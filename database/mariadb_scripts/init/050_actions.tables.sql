-- TODO: Move this to the Actions schema
CREATE TABLE IF NOT EXISTS Posts.View (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	user_id
		BIGINT NOT NULL,
	count
		INT UNSIGNED NOT NULL
		DEFAULT 1,

	PRIMARY KEY(post_id, user_id),
	INDEX(post_id),
	INDEX(user_id)
);


-- TODO: Move this to the Actions schema
CREATE TABLE IF NOT EXISTS Posts.Vote (
	id
		BIGINT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	user_id
		BIGINT NOT NULL,
	type
		TINYINT UNSIGNED NOT NULL,
		-- HARDCODED:
		-- 0: N/A (user voted, then cleared it)
		-- 1: +1
		-- 2: +1 Happy/Excited
		-- 3: +1 Love/Sympathy
		-- 4: +1 Top Quality
		-- 5: -1
		-- 6: -1 What is this?
		-- 7: -1 Spam
		-- 8: -1 Troll

	PRIMARY KEY(id),
	UNIQUE(post_id, user_id),
	CONSTRAINT CHECK (type IN (0,1,2,3,4,5,6,7,8)),
	INDEX(post_id),
	INDEX(user_id)
);


-- TODO: Move this to the Actions schema
-- This creates a dropdown menu for the user to pick an existing folder
CREATE TABLE IF NOT EXISTS Posts.UserFolder (
	id
		INT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	user_id
		BIGINT NOT NULL,
	name
		TINYTEXT NOT NULL,

	PRIMARY KEY(id),
	UNIQUE KEY(user_id, name),
	INDEX(user_id)
);


-- TODO: Move this to the Actions schema
CREATE TABLE IF NOT EXISTS Posts.FavoritePost (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	user_id
		BIGINT NOT NULL,
	folder_id
		INT UNSIGNED NULL
		REFERENCES Posts.UserFolder(id),

	PRIMARY KEY(user_id, post_id),
	INDEX(user_id),
	INDEX(folder_id)
);


CREATE TABLE IF NOT EXISTS Posts.FavoriteMedia (
	user_id
		BIGINT NOT NULL,
	media_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Media(id),
	folder_id
		INT UNSIGNED NULL
		REFERENCES Posts.UserFolder(id),

	PRIMARY KEY(user_id, media_id),
	INDEX(user_id),
	INDEX(folder_id)
);


-- TODO: Move this to the Actions schema
CREATE TABLE IF NOT EXISTS Posts.HidePost (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	user_id
		BIGINT NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	PRIMARY KEY(user_id, post_id),
	INDEX(user_id)
);


-- TODO: Move this to the Actions schema
CREATE TABLE IF NOT EXISTS Posts.Report (
	id
		INT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	user_id
		BIGINT NOT NULL,
	reason_code
		-- HARDCODED: TBD
		-- 1x: Hide this [post, tag, w/e TBD]
		-- 2x: Block this [user, etc.]
		-- 3x: Report incorrect rating
		-- 4x: Report for mod review
		-- 5x: Report illegal content
		TINYINT UNSIGNED NOT NULL,
	context
		TINYTEXT NULL,

	PRIMARY KEY(id)
);