CREATE OR REPLACE FUNCTION UserDB.GetAccountId (
	_provider_name TEXT,
	_provider_num TEXT
)
RETURNS BIGINT
STABLE
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
DECLARE
	_provider_id SMALLINT := UserDB.GetProviderIdByName(_provider_name);
BEGIN
	RETURN a.account_id
	FROM UserDB.ProviderAccountLink a
	WHERE a.provider_id = _provider_id
		AND a.provider_num = _provider_num
	LIMIT 1;
END
$$;