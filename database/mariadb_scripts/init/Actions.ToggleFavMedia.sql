DELIMITER $$
CREATE OR REPLACE FUNCTION Actions.ToggleFavMedia(
	p_user_id BIGINT,
	p_media_id INT UNSIGNED,
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
		FROM Posts.FavoriteMedia
		WHERE user_id = p_user_id
			AND media_id = p_media_id
	) THEN
		IF p_name IS NOT NULL THEN
			SELECT id
			INTO v_folder_id
			FROM Posts.UserFolders
				WHERE user_id = p_user_id
				AND name = p_name;
		END IF;

		INSERT INTO Posts.FavoriteMedia(user_id, media_id, folder_id)
		VALUES (p_user_id, p_media_id, v_folder_id);

		RETURN TRUE;
	ELSE
		DELETE FROM Posts.FavoriteMedia
		WHERE user_id = p_user_id
			AND media_id = p_post_id;

		RETURN FALSE;
	END IF;
END
$$
DELIMITER ;


-- DEPRECATED... Migrate All UserActions to Actions schema
DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.ToggleFavMedia(
	p_user_id BIGINT,
	p_media_id INT UNSIGNED,
	p_name TINYTEXT
)
RETURNS BOOL
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	RETURN Actions.ToggleFavMedia(p_user_id, p_media_id, p_name);
END
$$
DELIMITER ;