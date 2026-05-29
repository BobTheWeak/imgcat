CREATE OR REPLACE FUNCTION Legal.IsJurisdictionBanned (
	_country CHAR(2),
	_state CHAR(2)
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

	-- Banned countries
	IF _country IN (
		 'KP' -- North Korea
		,'IR' -- Iran
		,'CU' -- Cuba
		,'AF' -- Afghanistan
		,'SY' -- Syria
		,'LY' -- Libya
		,'RU' -- Russia
		,'BY' -- Belarus
	) THEN
		RETURN TRUE;
	END IF;

	RETURN FALSE;
END $$;