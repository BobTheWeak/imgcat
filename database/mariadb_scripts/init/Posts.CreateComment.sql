DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.CreateComment(
	p_user_id BIGINT,
	p_post_id INT UNSIGNED,
	p_reply_to BIGINT UNSIGNED,
	p_link_v1 CHAR(12),
	p_comment TINYTEXT
)
RETURNS INT UNSIGNED
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	DECLARE v_comment_id INT UNSIGNED;

	-- TODO: We should have some check to see if a user has permissions to comment

	INSERT INTO Comments.Comment(user_id, post_id, reply_to, link_v1, comment)
	VALUES (p_user_id, p_post_id, p_reply_to, p_link_v1, p_comment);

	SET v_comment_id = LAST_INSERT_ID();

	INSERT INTO Comments.CommentVote(comment_id, user_id, type)
	VALUES (v_comment_id, p_user_id, 1); -- Hardcoded as a self-like

	RETURN v_comment_id;
END
$$
DELIMITER ;