CREATE OR REPLACE FUNCTION UsersSvc.GetUserBadges(
	ids BIGINT[]
)
RETURNS TABLE (
	id BIGINT,
	username VARCHAR(40),
	link CHAR(12),
	badge CHAR(1)
)
STABLE
LEAKPROOF
STRICT
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
BEGIN
	RETURN QUERY
	SELECT
		a.id,
		a.username,
		a.link,
		CASE
			WHEN s.account_id IS NOT NULL THEN 'S'::CHAR(1)
			WHEN m.account_id IS NOT NULL THEN 'M'::CHAR(1)
			WHEN o.account_id IS NOT NULL THEN 'O'::CHAR(1)
			WHEN v.account_id IS NOT NULL THEN 'V'::CHAR(1)
			ELSE 'U'::CHAR(1)
		END CASE
	FROM UserDB.Account a
	LEFT JOIN UserDB.AccountClaim s
		ON a.id = s.account_id
		AND s.claim_id = ANY(UsersSvc.GetUserBadges_Internal('STAFF'::UsersSvc.BADGE_TYPE))
	LEFT JOIN UserDB.AccountClaim m
		ON a.id = m.account_id
		AND m.claim_id = ANY(UsersSvc.GetUserBadges_Internal('MODERATOR'::UsersSvc.BADGE_TYPE))
	LEFT JOIN UserDB.AccountClaim o
		ON a.id = o.account_id
		AND o.claim_id = ANY(UsersSvc.GetUserBadges_Internal('OFFICIAL_ORG'::UsersSvc.BADGE_TYPE))
	LEFT JOIN UserDB.AccountClaim v
		ON a.id = v.account_id
		AND v.claim_id = ANY(UsersSvc.GetUserBadges_Internal('VERIFIED_USER'::UsersSvc.BADGE_TYPE))
	WHERE a.id = ANY(GetUserBadges.ids);
END;
$$;


-- This is an internal helper function categorizing the types of claims into badge categories
-- It's IMMUTABLE, so this will cache the result without actually doing the query each time
CREATE OR REPLACE FUNCTION UsersSvc.GetUserBadges_Internal(
	t UsersSvc.BADGE_TYPE
)
RETURNS SMALLINT[]
IMMUTABLE
LEAKPROOF
LANGUAGE PLPGSQL AS $$
BEGIN
	RETURN CASE t
		WHEN 'STAFF'::UsersSvc.BADGE_TYPE THEN
			(SELECT array_agg(id) FROM UserDB.Claim WHERE name=ANY('{R:staff}'))
		WHEN 'MODERATOR'::UsersSvc.BADGE_TYPE THEN
			(SELECT array_agg(id) FROM UserDB.Claim WHERE name=ANY('{R:mod,R:vol}'))
		WHEN 'OFFICIAL_ORG'::UsersSvc.BADGE_TYPE THEN
			(SELECT array_agg(id) FROM UserDB.Claim WHERE name=ANY('{V:govt,V:news}'))
		WHEN 'VERIFIED_USER'::UsersSvc.BADGE_TYPE THEN
			(SELECT array_agg(id) FROM UserDB.Claim WHERE name=ANY('{V:biz,V:pol,V:jrnl,V:pop}'))
		ELSE '{}'
	END CASE;
END $$;
