DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.SetView(
	p_post_id INT UNSIGNED,
	p_user_id BIGINT
)
RETURNS INT SIGNED
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	INSERT INTO Posts.View(post_id, user_id)
	VALUES (p_post_id, p_user_id)
	ON DUPLICATE KEY UPDATE
		count = count+1;

	-- JENKY-ASS BULLSHIT... MariaDB doesn't support:
	-- RETURNING id INTO variable;
	-- Or just returning that as the function result.
	RETURN (
		SELECT count
		FROM Posts.View
		WHERE post_id = p_post_id
			AND user_id = p_user_id
	);
END
$$
DELIMITER ;