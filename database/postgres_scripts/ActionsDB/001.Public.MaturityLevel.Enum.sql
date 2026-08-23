CREATE TYPE Public.MATURITY_LEVEL AS ENUM (
	-- Kids
	 'PRUDE'   -- Confirmed kid-safe (as pretty much everything 'normal' is)
	-- Adult users
	,'DUDE'    -- Normal or unclassified content (originally 'NORMAL', but we like whimsy)
	,'LEWD'    -- Mature but SFW, like selfies & TT
	-- Mods & staff
	,'NUDE'    -- Mature and NSFW (not supported ATM)
	,'ILLEGAL' -- Straight up illegal, like terrorist beheading videos
);


-- MATURITY_LEVEL is a comparable number, so create a FUNC & CAST for that purpose
CREATE OR REPLACE FUNCTION Public.ConvertContentLevelToSmallInt (
	val Public.MATURITY_LEVEL
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
		WHEN 'PRUDE'::Public.MATURITY_LEVEL THEN 1
		WHEN 'DUDE'::Public.MATURITY_LEVEL THEN 2
		WHEN 'LEWD'::Public.MATURITY_LEVEL THEN 3
		WHEN 'NUDE'::Public.MATURITY_LEVEL THEN 4
		WHEN 'ILLEGAL'::Public.MATURITY_LEVEL THEN 5
		ELSE NULL
	END CASE;
END $$;

CREATE OR REPLACE FUNCTION Public.ConvertSmallIntToContentLevel (
	val SMALLINT
)
RETURNS Public.MATURITY_LEVEL
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
STRICT
SECURITY DEFINER
PARALLEL SAFE
AS $$
BEGIN
	RETURN CASE val
		WHEN 1 THEN 'PRUDE'::Public.MATURITY_LEVEL
		WHEN 2 THEN 'DUDE'::Public.MATURITY_LEVEL
		WHEN 3 THEN 'LEWD'::Public.MATURITY_LEVEL
		WHEN 4 THEN 'NUDE'::Public.MATURITY_LEVEL
		WHEN 5 THEN 'ILLEGAL'::Public.MATURITY_LEVEL
		ELSE NULL
	END CASE;
END $$;
CREATE CAST(Public.MATURITY_LEVEL AS SMALLINT)
	WITH FUNCTION Public.ConvertContentLevelToSmallInt(Public.MATURITY_LEVEL)
	AS ASSIGNMENT;
CREATE CAST(SMALLINT AS Public.MATURITY_LEVEL)
	WITH FUNCTION Public.ConvertSmallIntToContentLevel(SMALLINT)
	AS ASSIGNMENT;