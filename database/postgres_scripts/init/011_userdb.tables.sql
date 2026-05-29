CREATE TABLE IF NOT EXISTS UserDB.Account (
	id
		BIGSERIAL,
	date_created
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	username
		varchar(40) NOT NULL,
	link
		char(12) NOT NULL
		DEFAULT Public.GetLinkId(),

	PRIMARY KEY(id)
		INCLUDE(username)
);

CREATE INDEX IF NOT EXISTS account_link_cix
	ON UserDB.Account(link)
	INCLUDE(id, username);


CREATE TABLE IF NOT EXISTS UserDB.Provider (
	id
		SMALLSERIAL,
	name
		text NOT NULL,

	PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS provider_name_idx ON UserDB.Provider(name);

INSERT INTO UserDB.Provider
	(name)
VALUES
	('google');


CREATE TABLE IF NOT EXISTS UserDB.ProviderAccountLink (
	provider_id
		SMALLINT NOT NULL
		REFERENCES UserDB.Provider,
	provider_num
		TEXT,
	account_id
		BIGINT NOT NULL
		REFERENCES UserDB.Account(id),

	PRIMARY KEY(provider_id, provider_num)
		INCLUDE(account_id)
);


CREATE TABLE IF NOT EXISTS UserDB.Claim (
	id
		SMALLSERIAL,
	name -- These should be short-codes b/c they're stored in cookies & re-transmitted every request
		VARCHAR(40) NOT NULL,
	description
		TEXT NOT NULL,

	PRIMARY KEY(id),
	UNIQUE(name)
);

-- NOTE: None of this stuff is implemented yet
INSERT INTO UserDB.Claim
	(name, description)
VALUES
	-- Team roles (Gold badge)
	('R:staff', 'Staff account'),
	-- Team roles (Red badge)
	('R:mod',   'Paid moderator with special powers'),
	('R:vol',   'Volunteer moderator with special powers'),
	-- Official Accounts (Blue badge)
	('V:govt',  'Verified govt organization'),
	('V:news',  'Verified news agency'),
	-- Verified Accounts (Green badge)
	('V:biz',   'Verified company or brand'),
	('V:pol',   'Verified govt official or politician'),
	('V:jrnl',  'Verified journalist, reporter, correspondent'),
	('V:pop',   'Verified pop culture celebrities'),
	-- New acct tutorials
	('New:tour', 'Would you like to take a tour of your new acct?');


CREATE TABLE IF NOT EXISTS UserDB.AccountClaim (
	account_id
		BIGSERIAL
		REFERENCES UserDB.Account(id),
	claim_id
		SMALLSERIAL
		REFERENCES UserDB.Claim(id),

	PRIMARY KEY(account_id, claim_id)
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
