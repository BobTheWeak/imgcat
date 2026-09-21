CREATE OR REPLACE FUNCTION SoftMod.SetTagVote(
	post_id BIGINT,
	user_id BIGINT,
	tags    TEXT[]
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	-- Figure out if tags exist
	WITH tag_map(tag, tag_id) AS (
		SELECT
			a.a,
			b.id
		FROM unnest(Test.tags) a
		LEFT JOIN Public.Tag b
			ON a.a = b.name
	),
	
	-- Anything that's new, insert it
	new_tags(tag, tag_id) AS (
		INSERT INTO Public.Tag(name)
		SELECT tag
		FROM tag_map
		WHERE tag_id IS NULL
		ON CONFLICT(name) DO NOTHING
		RETURNING name, id
	),
	
	-- Join existing & new tags
	all_tags(tag, tag_id) AS (
		SELECT tag, tag_id
		FROM tag_map
		WHERE tag_id IS NOT NULL

		UNION ALL
		
		SELECT tag, tag_id
		FROM new_tags
	),

	-- TODO: This should be a MERGE
	delete_old_entries AS (
		DELETE FROM SoftMod.TagVote
		WHERE post_id = Test.post_id
			AND user_id = Test.user_id
			AND tag_id NOT IN (
				SELECT tag_id
				FROM tag_map
			)
	)

	-- Now, finally upsert
	INSERT INTO SoftMod.TagVote AS a (
		post_id,
		user_id,
		tag_id
	)
	SELECT
		Test.post_id,
		Test.user_id,
		a.tag_id
	FROM all_tags a
	ON CONFLICT
		ON CONSTRAINT tagvote_pkey
		DO UPDATE
		SET upload_time = CURRENT_TIMESTAMP;

	SELECT TRUE;
END;