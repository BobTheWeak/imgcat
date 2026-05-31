DELIMITER $$
CREATE OR REPLACE FUNCTION Actions.ToggleFavPost(
	p_user_id BIGINT,
	p_post_id INT UNSIGNED,
	p_name TINYTEXT
)
RETURNS BOOL
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	DECLARE v_folder_id INT UNSIGNED DEFAULT NULL;

	-- Check if we're favoriting or unfavoriting a post
	IF NOT EXISTS(
		SELECT 1
		FROM Posts.FavoritePost
		WHERE user_id = p_user_id
			AND post_id = p_post_id
	) THEN
		IF p_name IS NOT NULL THEN
			SELECT id
			INTO v_folder_id
			FROM Posts.UserFolders
				WHERE user_id = p_user_id
				AND name = p_name;
		END IF;

		INSERT INTO Posts.FavoritePost(user_id, post_id, folder_id)
		VALUES (p_user_id, p_post_id, v_folder_id);

		RETURN TRUE;
	ELSE
		DELETE FROM Posts.FavoritePost
		WHERE user_id = p_user_id
			AND post_id = p_post_id;

		RETURN FALSE;
	END IF;
END
$$
DELIMITER ;


DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.ToggleFavPost(
	p_user_id BIGINT,
	p_post_id INT UNSIGNED,
	p_name TINYTEXT
)
RETURNS BOOL
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	RETURN Actions.ToggleFavPost(p_user_id, p_post_id, p_name);
END
$$
DELIMITER ;