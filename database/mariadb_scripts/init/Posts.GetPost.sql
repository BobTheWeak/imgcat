DELIMITER $$
CREATE OR REPLACE PROCEDURE Posts.GetPost(
	p_post_link CHAR(12),
	p_user_content_level TINYINT UNSIGNED,
	p_user_id INT UNSIGNED,
	p_register_view BOOL
)
LANGUAGE SQL
NOT DETERMINISTIC
READS SQL DATA
SQL SECURITY DEFINER
BEGIN
	DECLARE v_post_id INT UNSIGNED;

	SELECT id
	INTO v_post_id
	FROM Posts.Post a
	INNER JOIN Posts.PostRating b
		ON a.id=b.post_id
	WHERE a.link = p_post_link
		AND (a.is_public = true OR a.user_id = p_user_id)
		AND b.category <= p_user_content_level
	LIMIT 1;

	-- Verify the user is allowed to see the post at all
	IF v_post_id IS NOT NULL THEN
		IF p_register_view THEN
			DO Posts.SetView(v_post_id, p_user_id);
		END IF;

		SELECT
			a.id,
			a.title,
			a.user_id,
			b.username,
			a.upload_time as 'time',
			a.is_public,
			a.link
		FROM Posts.Post a
		INNER JOIN UserDB.Account b
			ON a.user_id=b.id
		WHERE a.id = v_post_id
		LIMIT 1;

		SELECT
			b.link_v1 AS link,
			-- HARDCODED: 0:Unknown, 1:raster image, 2:vector image, 3:animation, 4:video
			b.type,
			c.mime_type,
			a.description
		FROM Posts.Attachment a
		INNER JOIN Posts.Media b
			ON a.media_id = b.id
		INNER JOIN Posts.MediaMetadata c
			ON a.media_id = c.media_id
		WHERE a.post_id = v_post_id
		ORDER BY order_id ASC;
	END IF;
END
$$
DELIMITER ;