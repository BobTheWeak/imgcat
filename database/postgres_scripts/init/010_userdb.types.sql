CREATE TYPE UserDB.CONTENT_LEVEL AS ENUM (
	-- Kids
	 'PRUDE'   -- Confirmed kid-safe (as pretty much everything 'normal' is)
	-- Adult users
	,'DUDE'    -- Normal or unclassified content (originally 'NORMAL', but we like whimsy)
	,'LEWD'    -- Mature but SFW, like selfies & TT
	-- Mods & staff
	,'NUDE'    -- Mature and NSFW (not supported)
	,'ILLEGAL' -- Straight up illegal, like terrorist beheading videos
);


CREATE TYPE UserDB.VISIBILITY_LEVEL AS ENUM (
	 'PRIVATE'         -- No visibility at all (except self)
	-- Reserved for later
	-- ,'LIMITED'         -- Kids: visible to parents, groups
	-- ,'LIMITED FRIENDS' -- Kids: visible to parents, groups, friends
	,'FRIENDS'         -- Adult: approval list
	,'PUBLIC'          -- Public, to other ImgCat users
	,'GLOBAL'          -- Public, to anyone surfing the interwebs
);


-- I don't know what these values mean (yet), but we don't want to store
-- actual weights in user preferences b/c the formulas will change.
-- NOTE: I'm not going to do casting translations either...
CREATE TYPE UserDB.CONTENT_WEIGHT AS ENUM (
	'NONE',
	'MUCH LESS',
	'LESS',
	'LITTLE LESS',
	'NORMAL',
	'LITTLE MORE',
	'MORE',
	'MUCH MORE'
);




-- CONTENT_LEVEL is a comparable number, so create a FUNC & CAST for that purpose
CREATE OR REPLACE FUNCTION UserDB.ConvertContentLevelToSmallInt (
	val UserDB.CONTENT_LEVEL
)
RETURNS SMALLINT
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
STRICT
SECURITY DEFINER
PARALLEL SAFE
AS $$
BEGIN
	RETURN CASE val
		WHEN 'PRUDE'::UserDB.CONTENT_LEVEL THEN 1
		WHEN 'DUDE'::UserDB.CONTENT_LEVEL THEN 2
		WHEN 'LEWD'::UserDB.CONTENT_LEVEL THEN 3
		WHEN 'NUDE'::UserDB.CONTENT_LEVEL THEN 4
		WHEN 'ILLEGAL'::UserDB.CONTENT_LEVEL THEN 5
		ELSE NULL
	END CASE;
END $$;

CREATE OR REPLACE FUNCTION UserDB.ConvertSmallIntToContentLevel (
	val SMALLINT
)
RETURNS UserDB.CONTENT_LEVEL
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
STRICT
SECURITY DEFINER
PARALLEL SAFE
AS $$
BEGIN
	RETURN CASE val
		WHEN 1 THEN 'PRUDE'::UserDB.CONTENT_LEVEL
		WHEN 2 THEN 'DUDE'::UserDB.CONTENT_LEVEL
		WHEN 3 THEN 'LEWD'::UserDB.CONTENT_LEVEL
		WHEN 4 THEN 'NUDE'::UserDB.CONTENT_LEVEL
		WHEN 5 THEN 'ILLEGAL'::UserDB.CONTENT_LEVEL
		ELSE NULL
	END CASE;
END $$;


CREATE CAST(UserDB.CONTENT_LEVEL AS SMALLINT)
	WITH FUNCTION UserDB.ConvertContentLevelToSmallInt(UserDB.CONTENT_LEVEL)
	AS ASSIGNMENT;
CREATE CAST(SMALLINT AS UserDB.CONTENT_LEVEL)
	WITH FUNCTION UserDB.ConvertSmallIntToContentLevel(SMALLINT)
	AS ASSIGNMENT;


-- VISIBILITY_LEVEL is a comparable number, so create a FUNC & CAST for that purpose
CREATE OR REPLACE FUNCTION UserDB.ConvertVisibilityLevelToSmallInt (
	val UserDB.VISIBILITY_LEVEL
)
RETURNS SMALLINT
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
STRICT
SECURITY DEFINER
PARALLEL SAFE
AS $$
BEGIN
	RETURN CASE val
		WHEN 'PRIVATE'::UserDB.VISIBILITY_LEVEL THEN 1
		-- Reserved for later
		-- WHEN 'LIMITED'::UserDB.VISIBILITY_LEVEL THEN 2
		-- WHEN 'LIMITED FRIENDS'::UserDB.VISIBILITY_LEVEL THEN 3
		WHEN 'FRIENDS'::UserDB.VISIBILITY_LEVEL THEN 4
		WHEN 'PUBLIC'::UserDB.VISIBILITY_LEVEL THEN 5
		ELSE NULL
	END CASE;
END $$;

CREATE OR REPLACE FUNCTION UserDB.ConvertSmallIntToVisibilityLevel (
	val SMALLINT
)
RETURNS UserDB.VISIBILITY_LEVEL
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
STRICT
SECURITY DEFINER
PARALLEL SAFE
AS $$
BEGIN
	RETURN CASE val
		WHEN 1 THEN 'PRIVATE'::UserDB.VISIBILITY_LEVEL
		-- Reserved for later
		-- WHEN 2 THEN 'LIMITED'::UserDB.VISIBILITY_LEVEL
		-- WHEN 3 THEN 'LIMITED FRIENDS'::UserDB.VISIBILITY_LEVEL
		WHEN 4 THEN 'FRIENDS'::UserDB.VISIBILITY_LEVEL
		WHEN 5 THEN 'PUBLIC'::UserDB.VISIBILITY_LEVEL
		ELSE NULL
	END CASE;
END $$;

CREATE CAST(UserDB.VISIBILITY_LEVEL AS SMALLINT)
	WITH FUNCTION UserDB.ConvertVisibilityLevelToSmallInt(UserDB.VISIBILITY_LEVEL)
	AS ASSIGNMENT;
CREATE CAST(SMALLINT AS UserDB.VISIBILITY_LEVEL)
	WITH FUNCTION UserDB.ConvertSmallIntToVisibilityLevel(SMALLINT)
	AS ASSIGNMENT;