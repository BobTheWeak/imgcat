CREATE OR REPLACE FUNCTION UserDB.GetAccountData(
	INOUT id BIGINT,
	
	OUT username TEXT,
	OUT content_level TEXT,
	OUT see_sexuality BOOL,
	OUT see_gore BOOL,
	OUT see_trauma BOOL,

	-- Returns [TEXT]
	OUT claims_cur REFCURSOR
)
STABLE
LEAKPROOF
STRICT
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
BEGIN
	IF EXISTS(SELECT 1 FROM UserDB.Account a WHERE a.id = GetAccountData.id) THEN
		SELECT
			-- a.id, -- Not necessary b/c it keeps the same value
			a.username,
			b.content_level,
			b.see_sexuality,
			b.see_gore,
			b.see_trauma
		INTO
			-- GetAccountData.id,
			GetAccountData.username,
			GetAccountData.content_level,
			GetAccountData.see_sexuality,
			GetAccountData.see_gore,
			GetAccountData.see_trauma
		FROM UserDB.Account a
		INNER JOIN UserDB.AccountContentPreferences b
			ON a.id = b.account_id
		WHERE a.id = GetAccountData.id
		LIMIT 1;

		OPEN GetAccountData.claims_cur NO SCROLL FOR
			SELECT b.name
			FROM UserDB.AccountClaim a
			INNER JOIN UserDB.Claim b
				ON a.claim_id = b.id
			WHERE a.account_id = GetAccountData.id;
	ELSE
		GetAccountData.id := NULL;
	END IF;
END;
$$;