CREATE OR REPLACE FUNCTION UserDB.GetAccountData_Claims(
	_id BIGINT
)
RETURNS SETOF TEXT
STABLE
LEAKPROOF
STRICT
SECURITY DEFINER
LANGUAGE SQL
AS $$
	-- Return claims
	SELECT b.name
	FROM UserDB.AccountClaim a
	INNER JOIN UserDB.Claim b
		ON a.claim_id = b.id
	WHERE a.account_id = _id;
$$;
