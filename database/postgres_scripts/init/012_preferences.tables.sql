
-- 1:1 required with Account table
-- Earlier versions included content_level directly in Account
CREATE TABLE IF NOT EXISTS UserDB.AccountContentPreferences (
	account_id
		BIGINT NOT NULL
		REFERENCES UserDB.Account(id),

	-- Legal maturity restrictions
	-- Due to defaults (usually), or an age-check on account creation,
	-- we set the maximum legally-allowed levels for these fields.
	-- TODO: This is NOT efficient and should change to a system of standard templates
	legal_content_level
		UserDB.CONTENT_LEVEL NOT NULL,
	legal_see_sexuality
		bool NOT NULL,
	legal_see_gore
		bool NOT NULL,
	legal_see_trauma
		bool NOT NULL,

	-- User maturity settings
	-- These are what the user has set.
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
	-- NOTE: +1000 is twice as likely to recommend that topic, -1000 is half (range: 32x to 1/32nd)
	-- TODO: This list probably can't stay hardcoded like this & needs to be expanded
	news_weight -- News and current events (facts, not opinions)
		UserDB.CONTENT_WEIGHT NOT NULL
		DEFAULT 'NORMAL',
	politics_weight -- Politics & advocacy (heavy opinions)
		UserDB.CONTENT_WEIGHT NOT NULL
		DEFAULT 'NORMAL',
	creators_weight -- OC creators (good), but also small-business promotion
		UserDB.CONTENT_WEIGHT NOT NULL
		DEFAULT 'NORMAL',
	selfies_weight -- Individual self-pics, cosplay, body positivity, thirst traps
		UserDB.CONTENT_WEIGHT NOT NULL
		DEFAULT 'NORMAL',
	pets_weight -- Pictures of our fuzzy wuzzy family (always good)
		UserDB.CONTENT_WEIGHT NOT NULL
		DEFAULT 'NORMAL',
	ai_weight -- AI generated pictures (always bad)
		UserDB.CONTENT_WEIGHT NOT NULL
		DEFAULT 'NORMAL',

	PRIMARY KEY(account_id)
);


CREATE TABLE IF NOT EXISTS UserDB.AccountVisibilityPreferences (
	account_id
		BIGINT NOT NULL
		REFERENCES UserDB.Account(id),

	-- Legal visibility restrictions
	legal_about_me_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL,
	legal_activity_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL,
	legal_dm_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL,
	
	-- User visibility settings
	-- When another user clicks on a user profile, is that public?
	about_me_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL
		DEFAULT 'PUBLIC'
		CHECK(about_me_visibility <= legal_about_me_visibility),
	-- When another user looks at comment/post histories, is that public?
	activity_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL
		DEFAULT 'PUBLIC'
		CHECK(activity_visibility <= legal_activity_visibility),
	-- Does the user allow DMs from random people, from a list of friends, or not at all?
	dm_visibility
		UserDB.VISIBILITY_LEVEL NOT NULL
		DEFAULT 'PUBLIC'
		CHECK(dm_visibility <= legal_dm_visibility),

	-- The actual profile content
	about_me_content
		TEXT NULL,
	-- -- TBD -- --
	-- account_pic: Not sure how we want to do this. Probably select from a fixed list?
	-- activity: Or trophies? IDK how any of it works yet.
	-- direct messages: Other than Y/N, I'm not sure what we can set here.

	PRIMARY KEY(account_id)
);