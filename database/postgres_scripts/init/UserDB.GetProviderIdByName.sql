CREATE OR REPLACE FUNCTION UserDB.GetProviderIdByName (
	_provider_name TEXT
)
RETURNS SMALLINT
 -- TODO: Once we finalize the list of providers, does STABLE => IMMUTABLE make sense?
STABLE
LEAKPROOF
SECURITY DEFINER
LANGUAGE SQL
AS $$
	SELECT id
	FROM UserDB.Provider
	WHERE name = _provider_name;
$$;