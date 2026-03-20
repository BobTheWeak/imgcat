DROP TABLE IF EXISTS Posts.Attachment;
DROP TABLE IF EXISTS Posts.MediaMetadata;
DROP TABLE IF EXISTS Posts.MediaRating;
DROP TABLE IF EXISTS Posts.Media;
DROP TABLE IF EXISTS Posts.PostRating;
DROP TABLE IF EXISTS Posts.Post;


CREATE TABLE Posts.Post (
	id
		INT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	is_public
		BOOL NOT NULL
		DEFAULT FALSE,
	link
		CHAR(12) NOT NULL,

	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,
	user_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),

	title
		TINYTEXT
		CHARACTER SET utf8mb3
		NULL,

	PRIMARY KEY(id),
	INDEX(id, is_public),
	INDEX(link)
);

-- CREATE INDEX IX_Post
-- 	ON Posts.Post(id, is_public);
-- CREATE INDEX IX_PostByLink
-- 	ON Posts.Post(link);


CREATE TABLE Posts.PostRating (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),

	category
		-- HARDCODED: 1:prude, 2:std, 3:lewd, 4:nude, 5:illegal
		TINYINT
		GENERATED ALWAYS AS
		(
			CASE
				-- Two reports of illegal content auto-hides it permenantly
				-- TODO: Make it 1 once moderating can override this
				WHEN illegal >= 2 THEN 5
				WHEN nude >= 5 THEN 4
				WHEN lewd >= 5 THEN 3
				-- Prude content (aka: kid safe) must have 5 OKs
				-- and no other reports of anything inappropreate
				WHEN prude >=5 AND (nude + lewd + illegal) = 0 THEN 1
				-- If we don't know, mark it standard
				ELSE 2
			END
		) PERSISTENT,

	prude
		INT UNSIGNED NOT NULL
		DEFAULT 0,
	std
		INT UNSIGNED NOT NULL
		DEFAULT 0,
	lewd
		INT UNSIGNED NOT NULL
		DEFAULT 0,
	nude
		INT UNSIGNED NOT NULL
		DEFAULT 0,
	illegal
		INT UNSIGNED NOT NULL
		DEFAULT 0,

	PRIMARY KEY(post_id)
);


CREATE TABLE Posts.Media (
	id
		INT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	type
		-- HARDCODED: 0:Unknown, 1:raster image, 2:vector image, 3:animation, 4:video
		TINYINT NOT NULL,
	link_v1
		-- Versioning b/c we'll need a way to serve from different locations/services
		-- Cloudflare's R2 (blob storage) is cheaper ($0.015/GB) than every other
		-- cloud provider AND doesn't charge for bandwidth. (As of Aug 2025)
		-- This should be our base-case, that everything supports, and other (future)
		-- cols will be higher priority, but may not exist.
		-- NOTE: It uses "https://i.imgcat.io/{link}" as the proxy
		VARCHAR(32)
		CHARACTER SET ascii
		NOT NULL,
	-- link_v2 - Will likely be Cloudflare Images, which can automatically handle 
	-- resizing, thumbnails, format conversions, compression, etc. But we still
	-- need to hold the original somewhere, so v1 & v2, etc.

	PRIMARY KEY(id),
	CONSTRAINT CHECK(type IN (0, 1,2,3,4)),
	INDEX(link_v1)
);

-- CREATE INDEX IX_Media
-- 	ON Posts.Media(link_v1);


-- Metadata stores lesser-used, analysis data. It's used on upload, to ensure
-- someone isn't uploading banned content. We just need a table of banned hashes.
-- We also use this to dedup content, so that flashmobs don't actually incur costs.
CREATE TABLE Posts.MediaMetadata (
	media_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Media(id),
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,
	user_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),
	hash_sha256
		BINARY(32) NOT NULL,
	mime_type
		VARCHAR(40) NULL,

	PRIMARY KEY(media_id),
	INDEX(user_id),
	INDEX(hash_sha256)
);

-- CREATE INDEX IX_MediaMetadataByUser
-- 	ON Posts.MediaMetadata(user_id);
-- CREATE INDEX IX_MediaMetadataByHash
-- 	ON Posts.MediaMetadata(hash_sha256);


-- CREATE TABLE Posts.MediaRating (
-- 	media_id
-- 		INT UNSIGNED NOT NULL
-- 		REFERENCES Posts.Media(id),
-- 
-- 	category
-- 		-- HARDCODED: 1:prude, 2:std, 3:lewd, 4:nude, 5:illegal
-- 		TINYINT
-- 		GENERATED ALWAYS AS
-- 		(
-- 			CASE
-- 				-- Two reports of illegal content auto-hides it permenantly
-- 				-- TODO: Make it 1 once moderating can override this
-- 				WHEN illegal >= 2 THEN 5
-- 				WHEN nude >= 5 THEN 4
-- 				WHEN lewd >= 5 THEN 3
-- 				-- Prude content (aka: kid safe) must have 5 OKs
-- 				-- and no other reports of anything inappropreate
-- 				WHEN prude >=5 AND (nude + lewd + illegal) = 0 THEN 1
-- 				-- If we don't know, mark it standard
-- 				ELSE 2
-- 			END
-- 		) PERSISTENT,
-- 
-- 	prude
-- 		INT UNSIGNED NOT NULL
-- 		DEFAULT 0,
-- 	std
-- 		INT UNSIGNED NOT NULL
-- 		DEFAULT 0,
-- 	lewd
-- 		INT UNSIGNED NOT NULL
-- 		DEFAULT 0,
-- 	nude
-- 		INT UNSIGNED NOT NULL
-- 		DEFAULT 0,
-- 	illegal
-- 		INT UNSIGNED NOT NULL
-- 		DEFAULT 0,
-- 
-- 	PRIMARY KEY(id)
-- );


CREATE TABLE Posts.Attachment (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	media_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Media(id),
	order_id
		TINYINT UNSIGNED NOT NULL
		DEFAULT 0,
	description
		TINYTEXT
		CHARACTER SET utf8mb3
		NULL,

	PRIMARY KEY(post_id, media_id, order_id),
	INDEX(post_id),
	INDEX(media_id)
);

-- CREATE INDEX IX_AttachmentByPost
-- 	ON Posts.Attachment(post_id);
-- CREATE INDEX IX_AttachmentByMedia
-- 	ON Posts.Attachment(media_id);



