-- This table is an unlogged scratchpad for keeping track of what posts have gotten updates
-- Everytime someone does a softmod vote (aka: "This post is mature content"), we use triggers
-- to insert into this table. Backend services will then swing through this list and recalculate
-- the overall content settings for a particular post.

CREATE UNLOGGED TABLE Public.LatestVotes (
	post_id
		BIGINT NOT NULL,
	-- If a mod votes on a post, it needs to get recalculated in the next cycle
	priority
		BOOL NOT NULL
		DEFAULT FALSE,

	UNIQUE(post_id)
);

-- -- -- -- -- RECOVERY PROCEDURE -- -- -- -- --
-- Since this is unlogged, if there's been a crash or restart, we do need to repair this table.
-- It's not particularly urgent... This is probably one of the last things to do.
-- TODO: This is psudocode & needs to be tested properly. Please test & update me (before I'm needed).
-- 1) Grab the last transaction we can guarantee completed successfully
--    SELECT MIN(
--        (SELECT MAX(xmin) FROM Content.PostMaturity),
--        (SELECT MAX(xmin) FROM Content.PostMaturityScores),
--        (SELECT MAX(xmin) FROM Content.PostContent),
--        (SELECT MAX(xmin) FROM Content.PostContentScores),
--        -- Are there other tables we need?
--    ) INTO @xmin_value;
-- 2) Grab all votes older than that, and insert them into this table
--    INSERT INTO Content.LatestVotes
--    SELECT DISTINCT post_id
--    FROM {... each vote table ...}
--    WHERE xmin > @xmin_value
-- 3) Monitor the table to make sure the service can start emptying the queue