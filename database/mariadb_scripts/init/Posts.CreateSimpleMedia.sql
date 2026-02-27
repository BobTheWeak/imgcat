DELIMITER $$
CREATE OR REPLACE FUNCTION Posts.CreateSimpleMedia(
	p_user_id INT UNSIGNED,
	p_filename CHAR(12),
	p_mime_type TINYTEXT,
	p_sha256 BINARY(64)
)
RETURNS INT UNSIGNED
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	DECLARE v_media_id INT UNSIGNED;
	DECLARE v_type_id TINYINT UNSIGNED DEFAULT 0;

	SELECT media_id
	INTO v_media_id
	FROM Posts.MediaMetadata
	WHERE hash_sha256 = p_sha256;

	IF v_media_id IS NULL THEN

		-- Set the broad category, so we can change how it's rendered in browser
		-- HARDCODED: 0:Unknown, 1:raster image, 2:vector image, 3:animation, 4:video
		IF p_mime_type IN (
			'image/png',
			'image/jpeg',
			'image/webp'
		) THEN SET v_type_id = 1; END IF;
		IF p_mime_type IN (
			'image/svg+xml'
		) THEN SET v_type_id = 2; END IF;
		IF p_mime_type IN (
			'image/gif',
			'image/apng'
		) THEN SET v_type_id = 3; END IF;
		IF p_mime_type IN (
			'video/mp4',
			'video/webm'
		) THEN SET v_type_id = 4; END IF;

		INSERT INTO Posts.Media(type, link_v1)
		VALUES(v_type_id, p_filename);

		SET v_media_id = LAST_INSERT_ID();

		INSERT INTO Posts.MediaMetadata(media_id, user_id, hash_sha256, mime_type)
		VALUES (v_media_id, p_user_id, p_sha256, p_mime_type);

		-- We don't have this table (yet)
		-- INSERT INTO Posts.MediaRating(media_id)
		-- VALUES (v_media_id);
	END IF;

	RETURN v_media_id;
END
$$
DELIMITER ;