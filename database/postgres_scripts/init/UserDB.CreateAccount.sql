CREATE OR REPLACE FUNCTION UserDB.CreateAccount (
	_provider_name TEXT,
	_provider_num TEXT,
	_username VARCHAR(40),
	-- Compliance Information
	_country char(2),
	_state char(2),
	_age SMALLINT
)
RETURNS BIGINT
VOLATILE
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
DECLARE
	_provider_id SMALLINT := UserDB.GetProviderIdByName(_provider_name);
	_legal Legal.GetLegalRestrictions_Result := Legal.GetLegalRestrictions(_country, _state, _age);
	_account_id BIGINT := UserDB.GetAccountId(_provider_name, _provider_num); -- Usually NULL
	_tour_claim SMALLINT := (SELECT id FROM UserDB.Claim WHERE name = 'New:tour');

	-- These are the defaults we use on insert (if allowed by legal restrictions)
	-- By default, set this to Normal/DUDE. If they're allowed to set it to LEWD, they can do it later.
	_default_content_level UserDB.CONTENT_LEVEL := 'DUDE';
	-- By default, yes. But it won't matter unless they set content_level higher
	_default_see_sexuality BOOL := TRUE;
	_default_see_gore BOOL := TRUE;
	_default_see_trauma BOOL := TRUE;
	-- By default, make profiles public
	_default_about_me_visibility UserDB.VISIBILITY_LEVEL := 'PUBLIC';
	_default_activity_visibility UserDB.VISIBILITY_LEVEL := 'PUBLIC';
	_default_dm_visibility UserDB.VISIBILITY_LEVEL := 'PUBLIC';
	-- By default, leave your profile blank
	_default_about_me_content TEXT := NULL;
BEGIN

	-- Does this account already exist?
	IF _account_id IS NOT NULL THEN
		-- RAISE INFO 'Account already exists';
		RETURN _account_id;
	END IF;

	-- Check if we can create an account (ie. kids)
	IF _legal IS NULL THEN
		RAISE EXCEPTION 'Legally restricted';
	END IF;

	-- Check the username
	IF NOT UserDB.IsUsernameFree(_username) THEN
		RAISE EXCEPTION 'Username taken';
	END IF;

	-- TODO: Check for IP bans & deeper stuff

	-- OK... Everything checks out... Let's create the account

	-- Header table
	INSERT INTO UserDB.Account (
		username
	) VALUES (
		_username
	)
	RETURNING id
	INTO _account_id;

	-- Provider link
	INSERT INTO UserDB.ProviderAccountLink (
		provider_id,
		provider_num,
		account_id
	) VALUES (
		_provider_id,
		_provider_num,
		_account_id
	);

	-- Insert the new-user tour claim
	INSERT INTO UserDB.AccountClaim (
		account_id,
		claim_id
	) VALUES (
		_account_id,
		_tour_claim
	);

	-- TODO: Are there other claims we need?
	-- There may be some country-specific claims we have to add


	-- Account content preferences (mess from compliance stuff)
	INSERT INTO UserDB.AccountContentPreferences (
		account_id,
		legal_content_level,
		legal_see_sexuality,
		legal_see_gore,
		legal_see_trauma,
		content_level,
		see_sexuality,
		see_gore,
		see_trauma
	) VALUES (
		_account_id,
		_legal.legal_content_level,
		_legal.legal_see_sexuality,
		_legal.legal_see_gore,
		_legal.legal_see_trauma,
		LEAST(_default_content_level, _legal.legal_content_level),
		_default_see_sexuality AND _legal.legal_see_sexuality,
		_default_see_gore AND _legal.legal_see_gore,
		_default_see_trauma AND _legal.legal_see_trauma
	);

	-- Account visibility (mess from compliance stuff)
	INSERT INTO UserDB.AccountVisibilityPreferences (
		account_id,
		legal_about_me_visibility,
		legal_activity_visibility,
		legal_dm_visibility,
		about_me_visibility,
		activity_visibility,
		dm_visibility,
		about_me_content
	) VALUES (
		_account_id,
		_legal.legal_about_me_visibility,
		_legal.legal_activity_visibility,
		_legal.legal_dm_visibility,
		LEAST(_default_about_me_visibility, _legal.legal_about_me_visibility),
		LEAST(_default_activity_visibility, _legal.legal_activity_visibility),
		LEAST(_default_dm_visibility, _legal.legal_dm_visibility),
		_default_about_me_content
	);

	-- Woo hoo! We did it!
	RETURN _account_id;
END $$;