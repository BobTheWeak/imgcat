-- Define a composite return type. Normally not needed, but defining
-- mushy _DEFAULT_ADULT ROW := ROW(...) wasn't working as expected.
CREATE TYPE Legal.GetLegalRestrictions_Result AS (
	legal_content_level UserDB.CONTENT_LEVEL,
	legal_see_sexuality bool,
	legal_see_gore bool,
	legal_see_trauma bool,
	legal_about_me_visibility UserDB.VISIBILITY_LEVEL,
	legal_badges_visibility UserDB.VISIBILITY_LEVEL,
	legal_dm_visibility UserDB.VISIBILITY_LEVEL
);


-- Given the country, state, and age of the user return the
-- highest legally allowed values they're allowed to set.
CREATE OR REPLACE FUNCTION Legal.GetLegalRestrictions (
	_country CHAR(2),
	_state CHAR(2),
	_age SMALLINT
)
RETURNS Legal.GetLegalRestrictions_Result
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
SECURITY DEFINER
PARALLEL SAFE
AS $$
DECLARE
	-- Default settings
	-- These are the default profiles, but due to country-specific regs,
	-- like no DMs for kids, there may be customizations below

	-- Adults (18+)
	_DEFAULT_ADULT Legal.GetLegalRestrictions_Result := ROW(
		-- At the moment, on app startup, we DO NOT want to host
		-- ANY mature content AT ALL. NO NO NO NO NO. In the future,
		-- that option exists, theoretically, but likely never ever.
		'LEWD'::UserDB.CONTENT_LEVEL,
		TRUE,
		TRUE,
		TRUE,
		'PUBLIC'::UserDB.VISIBILITY_LEVEL,
		'PUBLIC'::UserDB.VISIBILITY_LEVEL,
		'PUBLIC'::UserDB.VISIBILITY_LEVEL
	);

	-- Teens (16+ only)
	-- NOTE: We will not support any kid/teen accounts on launch
	-- _DEFAULT_TEEN Legal.GetLegalRestrictions_Result := ROW(
	-- 	-- Most regs allow normal content to be served to (older) teens.
	-- 	-- We're being a little conservative and removing any bikini pics
	-- 	-- or spicy selfies. My apologies to horny teens. Go elsewhere.
	-- 	'DUDE'::UserDB.CONTENT_LEVEL,
	-- 	-- Since we don't allow spicy content (LEWD), this is already covered,
	-- 	-- but block anything sexual, violent, or emotionally traumatizing
	-- 	FALSE,
	-- 	FALSE,
	-- 	FALSE,
	-- 	-- The accounts should be visible, that's fine
	-- 	'PUBLIC'::UserDB.VISIBILITY_LEVEL,
	-- 	'PUBLIC'::UserDB.VISIBILITY_LEVEL,
	-- 	-- Allow DMs, but only to certified friends, not random strangers
	-- 	'FRIENDS ONLY'::UserDB.VISIBILITY_LEVEL
	-- );

	-- Kids (<16)
	-- NOTE: We DO NOT plan on supporting kid accounts, ever.
	-- _DEFAULT_KID Legal.GetLegalRestrictions_Result := ROW(
	-- 	-- Only show things that have been certified kid-friendly
	-- 	'PRUDE'::UserDB.CONTENT_LEVEL,
	-- 	FALSE,
	-- 	FALSE,
	-- 	FALSE,
	-- 	-- The accounts shouldn't be visible at all, so they can't write:
	-- 	-- I'm a student at blah-blah school.
	-- 	'PRIVATE'::UserDB.VISIBILITY_LEVEL,
	-- 	'PRIVATE'::UserDB.VISIBILITY_LEVEL,
	-- 	-- No DMs at all
	-- 	'PRIVATE'::UserDB.VISIBILITY_LEVEL
	-- );
BEGIN
	_country := UPPER(_country);
	_state := UPPER(_state);

	IF Legal.IsJurisdictionBanned(_country, _state) THEN
		RETURN NULL;
	END IF;

	-- This works for now (maybe), but this won't scale as needed
	-- NOTE: This function is currently IMMUTABLE, which will fail within a rules-engine
	CASE _country
		-- AU bans all social media accounts for anyone <16
		WHEN 'AU' THEN
			IF _age < 16 THEN
				RETURN NULL;
			END IF;

		-- BR requires accounts under 16 to be tied to parent's accounts.
		-- We can't support this now, but maybe later when we do alts.
		WHEN 'BR' THEN
			IF _age < 16 THEN
				RETURN NULL;
			END IF;

		-- UK is awful, and requires custom code/logic beyond this.
		-- Ages MUST be verified by some external mechanism (we're using Oauth)
		WHEN 'GB', 'UK' THEN
			-- Due to the complexity of the OSA, we are NOT allowing any kids whatsoever
			IF _age < 18 THEN
				RETURN NULL;
			END IF;

		-- The US has 50 different legal jurisdictions...
		-- WHEN 'US' THEN
			-- TBD TBD TBD - Need to do a full, state-by-state review
			-- CASE _state
			-- 	WHEN 'XX' THEN RETURN NULL;
			-- 	ELSE
			-- END CASE;
		
		ELSE -- A searched CASE requires an ELSE
			NULL; -- NOOP

	END CASE;


	-- If we haven't handled any special case, then do the default
	IF _age < 16 THEN
		RETURN NULL;
	ELSIF _age < 18 THEN
		-- While the site is starting up, do not allow kids at all
		RETURN NULL;
	ELSE
		RETURN _DEFAULT_ADULT;
	END IF;
END $$;