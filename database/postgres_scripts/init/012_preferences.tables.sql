
-- 1:1 required with Account table
-- Earlier versions included content_level directly in Account
CREATE TABLE IF NOT EXISTS UserDB.AccountContentPreferences (
	account_id
		BIGINT NOT NULL
		REFERENCES UserDB.Account(id),

	-- Legal maturity restrictions
	legal_content_level
		UserDB.CONTENT_LEVEL NOT NULL,
	legal_see_sexuality
		bool NOT NULL,
	legal_see_gore
		bool NOT NULL,
	legal_see_trauma
		bool NOT NULL,

	-- User maturity settings
	content_level
		UserDB.CONTENT_LEVEL NOT NULL
		CHECK(content_level <= legal_content_level),
	see_sexuality
		bool NOT NULL
		CHECK(see_sexuality <= legal_see_sexuality),
	see_gore
		bool NOT NULL
		CHECK(see_gore <= legal_see_gore),
	see_trauma
		bool NOT NULL
		CHECK(see_trauma <= legal_see_trauma),
	
	-- Content Topics
	-- NOTE: +1000 is twice as likely to recommend that topic, -1000 is half
	news_weight
		SMALLINT NOT NULL
		DEFAULT 0,
	politics_weight
		SMALLINT NOT NULL
		DEFAULT 0,
	creators_weight
		SMALLINT NOT NULL
		DEFAULT 0,
	selfies_weight
		SMALLINT NOT NULL
		DEFAULT 0,
	pets_weight
		SMALLINT NOT NULL
		DEFAULT 0,
	ai_weight
		SMALLINT NOT NULL
		DEFAULT 0,

	PRIMARY KEY(account_id)
);


CREATE TABLE IF NOT EXISTS UserDB.AccountVisibilityPreferences (
	account_id
		BIGINT NOT NULL
		REFERENCES UserDB.Account(id),

	-- Legal visibility restrictions
	legal_about_me_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL,
	legal_badges_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL,
	legal_dm_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL,
	
	-- User visibility settings
	about_me_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL
		DEFAULT 'PUBLIC'
		CHECK(about_me_visibility <= legal_about_me_visibility),
	badges_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL
		DEFAULT 'PUBLIC'
		CHECK(badges_visibility <= legal_badges_visibility),
	dm_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL
		DEFAULT 'PUBLIC'
		CHECK(dm_visibility <= legal_dm_visibility),

	-- The actual profile content
	about_me_content
		TEXT NULL,
	-- -- TBD -- --
	-- account_pic: Not sure how we want to do this. Probably select from a fixed list?
	-- badges: Or trophies? IDK how any of it works yet.
	-- direct messages: Other than Y/N, I'm not sure what we can set here.

	PRIMARY KEY(account_id)
);