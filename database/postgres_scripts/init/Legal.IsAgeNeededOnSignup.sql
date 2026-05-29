CREATE OR REPLACE FUNCTION Legal.IsAgeNeededOnSignup (
	_country CHAR(2),
	_state CHAR(2) -- Can be NULL
)
RETURNS BOOL
LANGUAGE PLPGSQL
IMMUTABLE
LEAKPROOF
SECURITY DEFINER
PARALLEL SAFE
AS $$
BEGIN
	_country := UPPER(_country);
	_state := UPPER(_state);

	-- Countries with privacy laws
	IF _country IN (
		 'GB','UK' -- United Kingdom
		,'AU' -- Australia
		,'BR' -- Brazil
	) THEN
		RETURN TRUE;
	END IF;

	RETURN FALSE;
END $$;