CREATE SCHEMA IF NOT EXISTS Content
	DEFAULT CHARACTER SET = utf8mb3;


CREATE TABLE IF NOT EXISTS Content.Tag (
	id
		BIGINT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	name
		varchar(255),

	PRIMARY KEY(id),
	INDEX(name)
);


-- TagRelationships should be bi-directional. But let the software check
-- the inverse. It can be a weekly check, who cares?
-- If A=>B, but B=>A doesn't exist, auto-insert with the same weight?
CREATE TABLE IF NOT EXISTS Content.TagRelationship (
	tag_to
		BIGINT UNSIGNED NOT NULL
		REFERENCES Content.Tag(id),
	tag_from
		BIGINT UNSIGNED NOT NULL
		REFERENCES Content.Tag(id),
	weight FLOAT NOT NULL,

	PRIMARY KEY(tag_to, tag_from)
);









-- If a user feels a tag is missing from a post, they can add it in this queue
CREATE TABLE IF NOT EXISTS Content.TagVote (
	user_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	tag_id
		BIGINT UNSIGNED NOT NULL
		REFERENCES Content.Tag(id),

	PRIMARY KEY(user_id, post_id, tag_id),
	INDEX(post_id, tag_id),
	INDEX(upload_time) -- To allow entries/votes to "time out"
);


-- If a user feels a post is spicy/NSFW, or incorrectly classified, they can add it in this queue
CREATE TABLE IF NOT EXISTS Content.MaturityVote (
	user_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	maturity
		-- 0: Kid-safe
		-- 1: Normal
		-- 2: Spicy
		-- 3: NSFW
		-- 4: Illegal
		TINYINT UNSIGNED NOT NULL
		CHECK(maturity IN (0,1,2,3,4)),
	is_sexual
		BOOL NOT NULL,
	is_gore
		BOOL NOT NULL,
	is_trauma
		BOOL NOT NULL,

	PRIMARY KEY(user_id, post_id),
	INDEX(post_id),
	INDEX(upload_time)
);


-- If a user feels a post is one of several divisive categories, they can add it in this queue
CREATE TABLE IF NOT EXISTS Content.CategoryVote (
	user_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	is_news_politics
		BOOL NOT NULL,
	is_thirst_trap
		BOOL NOT NULL,
	is_creator_content
		BOOL NOT NULL,

	PRIMARY KEY(user_id, post_id),
	INDEX(post_id),
	INDEX(upload_time)
);


-- If a user feels the post needs to be reviewed by a mod, they can add it in this queue
-- We do require a comment, but otherwise we don't know why they flagged it
CREATE TABLE IF NOT EXISTS Content.ModReviewVote (
	user_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	comment
		TINYTEXT NOT NULL,

	PRIMARY KEY(user_id, post_id),
	INDEX(post_id),
	INDEX(upload_time)
);


-- This table holds anonymous complaints about posts, from anyone on the internet
-- For compliance purposes, we have to give anon folks an option to report content
-- But honestly, it might be total crap data (aka: AI bots), but thems the rules...
-- 
-- If it gets too bad, we can remove this option from the post itself, and move reporting to a
-- "help link" section of the site. aka: "Tell us the ImgCat URL where you found bad stuff: ___"
CREATE TABLE IF NOT EXISTS Content.ModReviewVoteAnon (
	-- NOTE: MariaDB versions: INET6 (10.5), INET4 (10.10), Store INET4 inside INET6 (11.3)
	user_ip_address
		INET6 NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	comment
		TINYTEXT NOT NULL,

	PRIMARY KEY(user_ip_address, post_id),
	INDEX(user_ip_address),
	INDEX(post_id),
	INDEX(upload_time)
);


-- This is a list of any recent actions taken
-- ID is useful for "SELECT all posts since I checked last",
-- Or for keeping a running total of history, aka: Kafka
CREATE TABLE IF NOT EXISTS Content.VoteActionCache (
	id
		BIGINT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),

	PRIMARY KEY(id),
	INDEX(upload_time),
	INDEX(post_id)
);
