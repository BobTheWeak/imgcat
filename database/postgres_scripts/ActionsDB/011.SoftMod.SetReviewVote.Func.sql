CREATE OR REPLACE FUNCTION SoftMod.SetReviewVote(
	post_id  BIGINT,
	user_id  BIGINT,
	comment  TEXT
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	INSERT INTO SoftMod.NeedsReviewVote (
		post_id,
		user_id,
		comment
	)
	VALUES (
		SetReviewVote.post_id,
		SetReviewVote.user_id,
		SetReviewVote.comment
	)
	ON CONFLICT (post_id, user_id) DO UPDATE SET
		upload_time = CURRENT_TIMESTAMP,
		comment     = EXCLUDED.comment;

	-- The Upsert can't fail, so always return 1
	RETURN TRUE;
END;