CREATE OR REPLACE FUNCTION SoftMod.SetCategoryVote(
	post_id   BIGINT,
	user_id   BIGINT,
	is_news     BOOL,
	is_politics BOOL,
	is_oc_art   BOOL,
	is_selfie   BOOL,
	is_animal   BOOL,
	is_ai       BOOL
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	INSERT INTO SoftMod.CategoryVote (
		post_id,
		user_id,
		is_news,
		is_politics,
		is_oc_art,
		is_selfie,
		is_animal,
		is_ai
	)
	VALUES (
		SetCategoryVote.post_id,
		SetCategoryVote.user_id,
		SetCategoryVote.is_news,
		SetCategoryVote.is_politics,
		SetCategoryVote.is_oc_art,
		SetCategoryVote.is_selfie,
		SetCategoryVote.is_animal,
		SetCategoryVote.is_ai
	)
	ON CONFLICT (post_id, user_id) DO UPDATE SET
		upload_time = CURRENT_TIMESTAMP,
		is_news = EXCLUDED.is_news,
		is_politics = EXCLUDED.is_politics,
		is_oc_art = EXCLUDED.is_oc_art,
		is_selfie = EXCLUDED.is_selfie,
		is_animal = EXCLUDED.is_animal,
		is_ai = EXCLUDED.is_ai;

	-- The Upsert can't fail, so always return 1
	RETURN TRUE;
END;
