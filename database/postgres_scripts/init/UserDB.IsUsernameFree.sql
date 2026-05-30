CREATE OR REPLACE FUNCTION UserDB.IsUsernameFree(
	_username varchar(40)
)
RETURNS BOOL
STABLE
LEAKPROOF
STRICT
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
BEGIN
	IF EXISTS (
		SELECT 1
		FROM UserDB.Account
		-- TODO: This works for now, but do a deeper dive into encoding/collation
		WHERE username ILIKE _username
	) THEN 
		RETURN FALSE;
	ELSE
		RETURN TRUE;
	END IF;
END $$;