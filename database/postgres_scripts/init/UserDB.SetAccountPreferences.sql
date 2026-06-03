CREATE OR REPLACE FUNCTION UserDB.SetAccountPreferences(
	account_id BIGINT

	-- Core Profile
	,username TEXT DEFAULT NULL
	,about_me TEXT DEFAULT NULL

	-- Content Flags
	,content_level UserDB.CONTENT_LEVEL DEFAULT NULL
	,see_sexuality BOOL DEFAULT NULL
	,see_gore      BOOL DEFAULT NULL
	,see_trauma    BOOL DEFAULT NULL
	
	-- Content Weights
	,news_weight     UserDB.CONTENT_WEIGHT DEFAULT NULL
	,politics_weight UserDB.CONTENT_WEIGHT DEFAULT NULL
	,creators_weight UserDB.CONTENT_WEIGHT DEFAULT NULL
	,selfies_weight  UserDB.CONTENT_WEIGHT DEFAULT NULL
	,pets_weight     UserDB.CONTENT_WEIGHT DEFAULT NULL
	,ai_weight       UserDB.CONTENT_WEIGHT DEFAULT NULL

	-- Visibility Flags
	,about_me_visibility UserDB.VISIBILITY_LEVEL DEFAULT NULL
	,activity_visibility UserDB.VISIBILITY_LEVEL DEFAULT NULL
	,dm_visibility       UserDB.VISIBILITY_LEVEL DEFAULT NULL
)
-- Returns the number of fields updated (ignoring duplicates)
RETURNS INT
SECURITY DEFINER
LANGUAGE PLPGSQL
AS $$
DECLARE
	_update_count_a INT := 0;
	_update_count_b INT := 0;
	_update_count_c INT := 0;
BEGIN
	-- Make sure the user exists
	IF NOT EXISTS(SELECT 1 FROM UserDB.Account WHERE id=SetAccountPreferences.account_id) THEN
		RETURN 0;
	END IF;

	RAISE DEBUG 'User % updating settings', account_id;

	-- UserDB.Account (username)
	IF username IS NOT NULL
	THEN
		UPDATE UserDB.Account t SET
			 username = COALESCE(SetAccountPreferences.username, t.username)
		WHERE t.id = SetAccountPreferences.account_id
		RETURNING (
			  CASE WHEN NEW.username <> OLD.username THEN 1 ELSE 0 END
		) INTO _update_count_a;
	END IF;

	-- UserDB.AccountContentPreferences (content_level, see_sexuality/gore/trauma, *_weight)
	IF (content_level IS NOT NULL OR
		COALESCE(see_sexuality, see_gore, see_trauma) IS NOT NULL OR
		COALESCE(news_weight, politics_weight, creators_weight, selfies_weight, pets_weight, ai_weight) IS NOT NULL)
	THEN
		UPDATE UserDB.AccountContentPreferences t SET
			 content_level =   COALESCE(SetAccountPreferences.content_level,   t.content_level)
			,see_sexuality =   COALESCE(SetAccountPreferences.see_sexuality,   t.see_sexuality)
			,see_gore =        COALESCE(SetAccountPreferences.see_gore,        t.see_gore)
			,see_trauma =      COALESCE(SetAccountPreferences.see_trauma,      t.see_trauma)
			,news_weight =     COALESCE(SetAccountPreferences.news_weight,     t.news_weight)
			,politics_weight = COALESCE(SetAccountPreferences.politics_weight, t.politics_weight)
			,creators_weight = COALESCE(SetAccountPreferences.creators_weight, t.creators_weight)
			,selfies_weight =  COALESCE(SetAccountPreferences.selfies_weight,  t.selfies_weight)
			,pets_weight =     COALESCE(SetAccountPreferences.pets_weight,     t.pets_weight)
			,ai_weight =       COALESCE(SetAccountPreferences.ai_weight,       t.ai_weight)
		WHERE t.account_id = SetAccountPreferences.account_id
		RETURNING (
			  CASE WHEN NEW.content_level <> OLD.content_level THEN 1 ELSE 0 END
			+ CASE WHEN NEW.see_sexuality <> OLD.see_sexuality THEN 1 ELSE 0 END
			+ CASE WHEN NEW.see_gore <> OLD.see_gore THEN 1 ELSE 0 END
			+ CASE WHEN NEW.see_trauma <> OLD.see_trauma THEN 1 ELSE 0 END
			+ CASE WHEN NEW.news_weight <> OLD.news_weight THEN 1 ELSE 0 END
			+ CASE WHEN NEW.politics_weight <> OLD.politics_weight THEN 1 ELSE 0 END
			+ CASE WHEN NEW.creators_weight <> OLD.creators_weight THEN 1 ELSE 0 END
			+ CASE WHEN NEW.selfies_weight <> OLD.selfies_weight THEN 1 ELSE 0 END
			+ CASE WHEN NEW.pets_weight <> OLD.pets_weight THEN 1 ELSE 0 END
			+ CASE WHEN NEW.ai_weight <> OLD.ai_weight THEN 1 ELSE 0 END
		) INTO _update_count_b;
	END IF;

	-- UserDB.AccountVisibilityPreferences (about_me, *_visibility)
	IF (about_me IS NOT NULL OR
		COALESCE(about_me_visibility, activity_visibility, dm_visibility) IS NOT NULL)
	THEN
		UPDATE UserDB.AccountVisibilityPreferences t SET
			 about_me_content =    COALESCE(SetAccountPreferences.about_me,            t.about_me_content)
			,about_me_visibility = COALESCE(SetAccountPreferences.about_me_visibility, t.about_me_visibility)
			,activity_visibility = COALESCE(SetAccountPreferences.activity_visibility, t.activity_visibility)
			,dm_visibility =       COALESCE(SetAccountPreferences.dm_visibility,       t.dm_visibility)
		WHERE t.account_id = SetAccountPreferences.account_id
		RETURNING (
			  CASE WHEN NEW.about_me_content <> OLD.about_me_content THEN 1 ELSE 0 END
			+ CASE WHEN NEW.about_me_visibility <> OLD.about_me_visibility THEN 1 ELSE 0 END
			+ CASE WHEN NEW.activity_visibility <> OLD.activity_visibility THEN 1 ELSE 0 END
			+ CASE WHEN NEW.dm_visibility <> OLD.dm_visibility THEN 1 ELSE 0 END
		) INTO _update_count_c;
	END IF;

	RETURN _update_count_a + _update_count_b + _update_count_c;
END;
$$;
