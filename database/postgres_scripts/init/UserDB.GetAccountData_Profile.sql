CREATE OR REPLACE FUNCTION UserDB.GetAccountData_Profile(
	_id BIGINT
)
RETURNS TABLE(
	id BIGINT,
	username TEXT,
	content_level TEXT,
	see_sexuality BOOL,
	see_gore BOOL,
	see_trauma BOOL
)
ROWS 1
STABLE
LEAKPROOF
STRICT
SECURITY DEFINER
LANGUAGE SQL
AS $$
	SELECT
		a.id,
		a.username,
		b.content_level,
		b.see_sexuality,
		b.see_gore,
		b.see_trauma
	FROM UserDB.Account a
	INNER JOIN UserDB.AccountContentPreferences b
		ON a.id = b.account_id
	WHERE a.id = _id;
$$;