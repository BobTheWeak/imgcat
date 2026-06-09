DELIMITER $$
CREATE OR REPLACE PROCEDURE Posts.SetPostPublic(
	p_user_id BIGINT,
	p_post_id INT UNSIGNED,
	p_is_public BOOL DEFAULT NULL
)
LANGUAGE SQL
NOT DETERMINISTIC
READS SQL DATA
SQL SECURITY DEFINER
BEGIN
	UPDATE Posts.Post a
	SET is_public = COALESCE(p_is_public, NOT(is_public))
	WHERE a.id = p_post_id
		AND a.user_id = p_user_id;
END
$$
DELIMITER ;