CREATE OR REPLACE FUNCTION SoftMod.DeleteCategoryVote(
	post_id   BIGINT,
	user_id   BIGINT
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	DELETE FROM SoftMod.CategoryVote a
	WHERE a.post_id = DeleteCategoryVote.post_id
		AND a.user_id = DeleteCategoryVote.user_id;

	RETURN TRUE;
END;