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
		-- b.username,
		'TBD username' as username,
		a.upload_time as 'time',
		a.reply_to,
		a.link_v1 as 'link',
		a.comment
	FROM Comments.Comment a
	-- INNER JOIN UserDB.Account b
	-- 	ON a.user_id=b.id
	WHERE post_id = p_post_id;
END
$$
DELIMITER ;