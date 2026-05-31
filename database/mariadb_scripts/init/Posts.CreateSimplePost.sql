DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.CreateSimplePost(
	p_user_id BIGINT,
	p_title TINYTEXT,
	p_media_id INT UNSIGNED,
	p_media_desc TINYTEXT
)
RETURNS INT UNSIGNED
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	DECLARE v_post_id INT UNSIGNED;

	-- TODO: We should have some check to see if a user has permissions to post

	INSERT INTO Posts.Post(user_id, link)
	VALUES (p_user_id, Posts.GetFilename());

	SET v_post_id = LAST_INSERT_ID();

	-- TODO: We don't have this table (yet)
	-- INSERT INTO Posts.PostMetadata(id)
	-- VALUES (v_post_id);

	INSERT INTO Posts.PostRating(post_id)
	VALUES (v_post_id);

	INSERT INTO Posts.Attachment(post_id, media_id)
	VALUES (v_post_id, p_media_id);

	-- Auto-register a single view & upvote
	DO Posts.SetView(v_post_id, p_user_id);
	DO Posts.SetVote(v_post_id, p_user_id, 1);

	RETURN v_post_id;
END
$$
DELIMITER ;