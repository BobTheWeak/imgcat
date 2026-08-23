CREATE OR REPLACE FUNCTION Results.CalculateMaturityScore(
	ids bigint[] DEFAULT NULL,
	-- NOTE: We can handle much bigger batch sizes, but IDK how this will scale to tens-of-millions, etc.
	batch_size int DEFAULT 50
)
-- Returns the number of post_ids (re-)calculated.
-- If this returns 0, the queue is empty and the service should sleep
RETURNS INT
LANGUAGE PLPGSQL
SECURITY DEFINER
AS $$
DECLARE
	-- When a user doesn't have a reputation (aka: they're new) what weight should they have?
	-- This should be low. We don't want someone to spin up a botnet of new accounts and
	-- be able to nuke things in usersub. Eventually, it'll get corrected, but still...
	NEW_USER_REPUTATION REAL := 0.25; -- DEFAULT: 0.25
	
	-- How much extra voting power does a mod have than a regular user?
	-- Mods participate in the same weight-based system, and do have reputation, etc.
	MOD_REPUTATION_MULT REAL := 20.0; -- DEFAULT: 20.0
	
	-- What percentage of votes are blended into the current maturity level?
	-- This smooths the curve out a bit, and has a slight bias towards higher-maturity
	-- MATH NOTE: IDK what range of values "feel ok", but 0.3 is probably at the higher end of that range
	COLUMN_BLEND_PERCENT REAL := 0.30; -- DEFAULT: 0.30
BEGIN
	-- If we didn't specify a list of post_ids in the params, then grab some off the queue
	IF CalculateMaturityScore.ids IS NULL THEN
		WITH queue AS (
			SELECT post_id
			FROM Public.LatestVotes
			ORDER BY priority DESC -- BOOL-DESC sorts T,F,NULL
			FOR UPDATE SKIP LOCKED
			LIMIT CalculateMaturityScore.batch_size
		), delete AS (
			DELETE FROM Public.LatestVotes
			WHERE post_id IN (
				SELECT post_id FROM queue
			)
		)
		SELECT array_agg(post_id)
		INTO CalculateMaturityScore.ids
		FROM queue;
	END IF;

	-- If we don't have anything from the queue, we're done (IDK why it returns NULL instead of 0)
	IF array_length(CalculateMaturityScore.ids, 1) IS NULL THEN
		RETURN 0;
	END IF;

	-- This is a big honkin' CTE, here's what it's doing:
	-- 1) [raw_data] pulls in various maturity votes, from a variety of sources
	-- 2) [blended_totals] aggregates that data and does some data prep
	-- 3) [raw_maturity_score] calculates raw maturity on a [1-5] scale
	-- 4) [clean_results] cleans up that raw score, sets a category, aggregates flags, etc
	-- 5) The resulting query will upsert the results into the PostMaturityScore table
	-- NOTE: We don't need numerical accuracy, so we're using REAL (aka: float/single/f32) for speed
	WITH raw_data AS (
		SELECT
			a.post_id::BIGINT,
			-- Count maturity votes
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'PRUDE'), 0.0)::REAL AS kid,
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'DUDE'), 0.0)::REAL AS normal,
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'LEWD'), 0.0)::REAL AS spicy,
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'NUDE'), 0.0)::REAL AS nsfw,
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'ILLEGAL'), 0.0)::REAL AS illegal,
			COUNT(*)::INT AS samples,
			-- Count flags
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.is_sexual), 0.0)::REAL AS is_sexual,
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.is_gore), 0.0)::REAL AS is_gore,
			COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.is_trauma), 0.0)::REAL AS is_trauma,
			COUNT(*) FILTER (WHERE a.is_sexual OR a.is_gore OR a.is_trauma) AS is_samples
		FROM SoftMod.MaturityVote a
		LEFT JOIN Results.UserReputation b
			ON a.user_id=b.user_id
		WHERE post_id = ANY(CalculateMaturityScore.ids)
		GROUP BY post_id

		-- Append anything we found in the MOD tables (multiplying by that multiplier)
		UNION ALL SELECT
			a.post_id::BIGINT,
			-- Count maturity votes
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'PRUDE'), 0.0)::REAL AS kid,
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'DUDE'), 0.0)::REAL AS normal,
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'LEWD'), 0.0)::REAL AS spicy,
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'NUDE'), 0.0)::REAL AS nsfw,
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.maturity = 'ILLEGAL'), 0.0)::REAL AS illegal,
			COUNT(*)::INT AS samples,
			-- Count flags
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.is_sexual), 0.0)::REAL AS is_sexual,
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.is_gore), 0.0)::REAL AS is_gore,
			MOD_REPUTATION_MULT * COALESCE(SUM(COALESCE(b.weight, NEW_USER_REPUTATION)) FILTER (WHERE a.is_trauma), 0.0)::REAL AS is_trauma,
			COUNT(*) FILTER (WHERE a.is_sexual OR a.is_gore OR a.is_trauma) AS is_samples
		FROM HardMod.ModMaturityVote a
		LEFT JOIN Results.UserReputation b
			ON a.user_id=b.user_id
		WHERE post_id = ANY(CalculateMaturityScore.ids)
		GROUP BY post_id

		-- Append anything we found in the automation tables
		-- TDB

	), blended_totals AS (
		-- This is the mathy part of our algorithm...
		-- * We merge columns, blending their values together a bit
		--   This makes the maturity curve "rounder" and makes the data a little less discrete
		-- * We're using a blending coefficient of 0.3
		--   There's no real reason for this proportion, it "felt right" when testing the data. It can change.
		-- * Blending a bit of the lower maturity into a higher one shifts the maturity upwards
		--   The goal is to capture 50/50 votes, and if there's any doubts push it higher for a margin of safety
		-- * Kid-safe is reversed, and pulls a partial value from normal instead
		--   This is mostly to keep column weights similar. Doing (1.3*kid) would be OK too; but blending is good.
		--   Realistically, the only difference between Normal & Kid-safe is just that Kid-safe is "certified".
		--   Normal is really more of a default: "Nobody's complained about it being spicy, so it's probably fine".
		-- * nsfw & illegal votes have extra weight
		--   If we get maturity wrong, it mostly matters one-way, so nsfw votes should count more
		--   We might want to weight kid votes like this too, so we get a Nike swoosh curve.
		-- * c2/normal gets a +1
		--   This serves as a default category if we don't have any data, and to avoid division-by-zero
		SELECT
			post_id,
			SUM(kid)         + COLUMN_BLEND_PERCENT*SUM(normal) AS c1,
			SUM(normal) + 1  + COLUMN_BLEND_PERCENT*SUM(kid)    AS c2,
			1.25*SUM(spicy)  + COLUMN_BLEND_PERCENT*SUM(normal) AS c3,
			1.5*SUM(nsfw)    + COLUMN_BLEND_PERCENT*SUM(spicy)  AS c4,
			2.0*SUM(illegal) + COLUMN_BLEND_PERCENT*SUM(nsfw)   AS c5,
			SUM(samples)                      AS samples,
			SUM(is_sexual)  AS is_sexual,
			SUM(is_gore)    AS is_gore,
			SUM(is_trauma)  AS is_trauma,
			SUM(is_samples) AS is_samples,
			SUM(is_sexual + is_gore + is_trauma) + 0.0001 AS is_total
		FROM raw_data
		GROUP BY post_id
	), raw_maturity_score AS (
		-- This is the maturity rating, out of a [1-5] scale
		SELECT
			post_id,
			((1.0*c1 + 2.0*c2 + 3.0*c3 + 4.0*c4 + 5.0*c5) / (c1+c2+c3+c4+c5))::REAL AS raw,
			is_sexual / is_total AS is_sexual,
			is_gore / is_total AS is_gore,
			is_trauma / is_total AS is_trauma
		FROM blended_totals
	), clean_results AS (
		-- This is the final prep phase, pulling all the data together
		SELECT
			a.post_id,
			a.raw AS raw_score,
			-- The adjusted range is [0.0 - 5.333], where 0-1:kid, 1-2:norm, 2-3:spicy, 3-4:nsfw, 4+:illegal
			(a.raw - 1.0)*(1.3333) AS adjusted_score,
			-- Category values DO use the adjusted score, but the values are precalculated, working backwards (solving for X, not Y)
			CASE WHEN a.raw < 1.75 THEN 'PRUDE'::Public.MATURITY_LEVEL
			     WHEN a.raw < 2.50 THEN 'DUDE'::Public.MATURITY_LEVEL
			     WHEN a.raw < 3.25 THEN 'LEWD'::Public.MATURITY_LEVEL
			     WHEN a.raw < 4.00 THEN 'NUDE'::Public.MATURITY_LEVEL
			     ELSE 'ILLEGAL'::Public.MATURITY_LEVEL
			END AS discrete,
			b.samples,
			-- Maturity flags are just the weighted proportion of votes. Something later will turn that into a bool.
			b.is_sexual / b.is_total AS is_sexual,
			b.is_gore   / b.is_total AS is_gore,
			b.is_trauma / b.is_total AS is_trauma,
			b.is_samples
		FROM raw_maturity_score a
		INNER JOIN blended_totals b
			ON a.post_id=b.post_id
	)
	INSERT INTO Results.PostMaturityScore(
		post_id,
		mat_category,
		mat_score,
		mat_samples,
		is_sexual,
		is_gore,
		is_trauma,
		is_total_samples
	)
	SELECT
		post_id,
		discrete AS mat_category,
		adjusted_score AS mat_score,
		samples AS mat_samples,
		is_sexual AS is_sexual,
		is_gore AS is_gore,
		is_trauma AS is_trauma,
		is_samples AS is_total_samples
	FROM clean_results
	ON CONFLICT(post_id)
	DO UPDATE SET
		last_updated = CURRENT_TIMESTAMP,
		mat_category = EXCLUDED.mat_category,
		mat_score = EXCLUDED.mat_score,
		mat_samples = EXCLUDED.mat_samples,
		is_sexual = EXCLUDED.is_sexual,
		is_gore = EXCLUDED.is_gore,
		is_trauma = EXCLUDED.is_trauma,
		is_total_samples = EXCLUDED.is_total_samples;

	-- Return the number of post_ids we processed
	RETURN array_length(CalculateMaturityScore.ids, 1);
END $$;
