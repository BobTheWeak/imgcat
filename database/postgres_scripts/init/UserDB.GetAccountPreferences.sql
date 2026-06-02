CREATE OR REPLACE FUNCTION UserDB.GetAccountPreferences(
	_id BIGINT
)
RETURNS TABLE (
	-- Core Profile
	account_id BIGINT,
	username TEXT,
	about_me TEXT,

	-- Content Flags
	content_level UserDB.CONTENT_LEVEL,
	legal_content_level UserDB.CONTENT_LEVEL,
	see_sexuality BOOL,
	legal_see_sexuality BOOL,
	see_gore BOOL,
	legal_see_gore BOOL,
	see_trauma BOOL,
	legal_see_trauma BOOL,
	
	-- Content Weights
	news_weight UserDB.CONTENT_WEIGHT,
	politics_weight UserDB.CONTENT_WEIGHT,
	creators_weight UserDB.CONTENT_WEIGHT,
	selfies_weight UserDB.CONTENT_WEIGHT,
	pets_weight UserDB.CONTENT_WEIGHT,
	ai_weight UserDB.CONTENT_WEIGHT,

	-- Visibility Flags
	about_me_visibility UserDB.VISIBILITY_LEVEL,
	legal_about_me_visibility UserDB.VISIBILITY_LEVEL,
	activity_visibility UserDB.VISIBILITY_LEVEL,
	legal_activity_visibility UserDB.VISIBILITY_LEVEL,
	dm_visibility UserDB.VISIBILITY_LEVEL,
	legal_dm_visibility UserDB.VISIBILITY_LEVEL
)
STABLE
LEAKPROOF
STRICT
SECURITY DEFINER
ROWS 1
LANGUAGE SQL
AS $$
	-- Return claims
	SELECT
		 a.id
		,a.username
		,c.about_me_content

		,b.content_level
		,b.legal_content_level
		,b.see_sexuality
		,b.legal_see_sexuality
		,b.see_gore
		,b.legal_see_gore
		,b.see_trauma
		,b.legal_see_trauma

		,b.news_weight
		,b.politics_weight
		,b.creators_weight
		,b.selfies_weight
		,b.pets_weight
		,b.ai_weight

		,c.about_me_visibility
		,c.legal_about_me_visibility
		,c.activity_visibility
		,c.legal_activity_visibility
		,c.dm_visibility
		,c.legal_dm_visibility
	FROM UserDB.Account a
	INNER JOIN UserDB.AccountContentPreferences b
		ON a.id = b.account_id
	INNER JOIN UserDB.AccountVisibilityPreferences c
		ON a.id = c.account_id
	WHERE a.id = _id
	LIMIT 1;
$$;
