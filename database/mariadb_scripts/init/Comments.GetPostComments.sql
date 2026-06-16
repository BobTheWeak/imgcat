DELIMITER $$
CREATE OR REPLACE PROCEDURE Comments.GetPostComments(
	p_post_id INT UNSIGNED
)
LANGUAGE SQL
NOT DETERMINISTIC
READS SQL DATA
SQL SECURITY DEFINER
BEGIN
	SELECT
		a.id,
		a.upload_time as 'ts',
		a.reply_to,
		a.user_id,
		a.link_v1 as 'img',
		a.comment
	FROM Comments.Comment a
	-- INNER JOIN UserDB.Account b
	-- 	ON a.user_id=b.id
	WHERE post_id = p_post_id;
END
$$
DELIMITER ;