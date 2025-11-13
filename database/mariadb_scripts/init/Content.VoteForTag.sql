DELIMITER $$
CREATE OR REPLACE PROCEDURE Content.VoteForTag(
	p_user_id BIGINT UNSIGNED,
	p_post_id BIGINT UNSIGNED,
	p_tag TINYTEXT
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	DECLARE v_tag_id BIGINT UNSIGNED;

	-- TODO: Cleanup the tag...
	-- Trim spaces. Lowercase. Replace spaces with underscore. Etc.

	SELECT id
	INTO v_tag_id
	FROM Content.Tag
	WHERE name = p_tag;

	IF v_tag_id IS NULL THEN
		-- The tag is new & we need to insert it first
		INSERT INTO Content.Tag(name)
		VALUES (p_tag);

		SET v_tag_id = LAST_INSERT_ID();
	END IF;

	IF v_tag_id IS NOT NULL THEN
		INSERT INTO Content.TagVote(user_id, post_id, tag_id)
		VALUES(p_user_id, p_post_id, v_tag_id)
		ON DUPLICATE KEY UPDATE
			upload_time=CURRENT_TIMESTAMP;

		-- Insert a cache record, noting this post has changed
		INSERT INTO Content.VoteActionCache(post_id)
		VALUES(p_post_id);
	END IF;
END
$$
DELIMITER ;