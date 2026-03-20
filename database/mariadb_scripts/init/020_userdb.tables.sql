CREATE TABLE IF NOT EXISTS UserDB.Account (
	id
		INT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	date_created
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,
	is_login_allowed BOOL NOT NULL
		DEFAULT 1,

	pass_hash
		BINARY(128) NOT NULL,
	pass_salt
		BINARY(16) NOT NULL,
	username
		VARCHAR(50) NOT NULL,
	email
		VARCHAR(255) NOT NULL,

	location
		CHAR(2) NOT NULL,
	age_category
		TINYINT NOT NULL,
	content_level
		TINYINT NOT NULL,

	PRIMARY KEY(id),
	UNIQUE KEY(username),
	UNIQUE KEY(email),
	INDEX(username(20)),
	INDEX(email(20))
);

-- Just index the first 20 characters to preserve space
-- TODO: Once we get some users, we can run some performance metrics
-- create or replace index IX_Account_username
-- 	ON UserDB.Account(username(20));
-- create or replace index IX_Account_email
-- 	ON UserDB.Account(email(20));


CREATE TABLE IF NOT EXISTS UserDB.Claim (
	id
		INT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	name
		VARCHAR(20) NOT NULL,
	description
		TINYTEXT NOT NULL,

	PRIMARY KEY(id),
	UNIQUE KEY(name)
);

-- NOTE: None of this stuff is implemented yet
INSERT INTO UserDB.Claim(name, description) VALUES
	('is_unverified', 'The email is unverified, strictly limiting what they can do'),
	('is_post_banned', 'Temporary ban on posting'),
	('is_comment_banned', 'Temporary ban on commenting'),
	('optin_news', 'User can see news & current events'),
	('optin_politics', 'User can see political content'),
	('mod_add_tags', 'Can add tags to a post'),
	('mod_set_content_lvl', 'Can update a posts content level'),
	('mod_ban_user', 'Can ban users');


CREATE TABLE IF NOT EXISTS UserDB.AccountClaim (
	account_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Account(id),
	claim_id
		INT UNSIGNED NOT NULL
		REFERENCES UserDB.Claim(id),

	UNIQUE KEY(account_id, claim_id),
	INDEX(account_id)
);

-- CREATE OR REPLACE INDEX IX_AccountClaim
-- 	ON UserDB.AccountClaim(account_id);


--
-- None of this SecurityLog stuff will work within SQL, it MUST be in the app layer.
-- The goal was to have higher-level intelligence there anyway, but do a *very* simple
-- ip ban & rate-limiter implementation here, as a fallback. But it won't work. We can
-- insert a "login failed" log message, and test rate-limits, etc., but then it all
-- gets rolled back when we raise/signal an error message (ie. "incorrect password").
-- So either we don't have error messages at all, or do a manual thing with hardcoded
-- error codes (ie. -4=AcctAlreadyExists) that we have to keep in sync at all times,
-- or... we just don't do it in SQL.
--
--CREATE TABLE IF NOT EXISTS UserDB.SecurityLog_CreateAccount(
--	id
--		INT UNSIGNED NOT NULL
--		AUTO_INCREMENT,
--	logged_time
--		TIMESTAMP NOT NULL
--		DEFAULT CURRENT_TIMESTAMP,
--	ip_address
--		INET6 NOT NULL,
--	username
--		varchar(50) NOT NULL,
--	email
--		varchar(255) NOT NULL,
--	was_successful
--		BOOL NOT NULL,
--
--	PRIMARY KEY(id)
--);
--
--CREATE OR REPLACE INDEX IX_SecurityLog_CreateAccount
--	ON UserDB.SecurityLogAccountCreation(ip_address);
--
--
--CREATE TABLE IF NOT EXISTS UserDB.SecurityLog_LogIn(
--	id
--		INT UNSIGNED NOT NULL
--		AUTO_INCREMENT,
--	logged_time
--		TIMESTAMP NOT NULL
--		DEFAULT CURRENT_TIMESTAMP,
--	ip_address
--		INET6 NOT NULL,
--	user_or_email
--		VARCHAR(255) NOT NULL,
--
--	was_successful
--		BOOL NOT NULL,
--
--	PRIMARY KEY(id)
--);
--
--CREATE OR REPLACE INDEX IX_SecurityLog_LogIn
--	ON UserDB.SecurityLogLoginAttempt(ip_address);

-- Removing IP address filtering from UserDB...
-- This needs to be in the app layer, not here
--CREATE TABLE IF NOT EXISTS UserDB.BannedIPs(
--	id
--		INT UNSIGNED NOT NULL
--		AUTO_INCREMENT,
--	date_created
--		TIMESTAMP NOT NULL
--		DEFAULT CURRENT_TIMESTAMP,
--	ip_address
--		INET6 NOT NULL,
--	ban_end
--		TIMESTAMP NULL,
--	notes
--		TEXT NULL,
--
--	PRIMARY KEY(id)
--);
--
--CREATE OR REPLACE INDEX IX_BannedIPs
--	ON UserDB.BannedIPs(ip_address);
