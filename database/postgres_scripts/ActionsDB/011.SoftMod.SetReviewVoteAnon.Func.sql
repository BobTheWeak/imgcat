CREATE OR REPLACE FUNCTION SoftMod.SetReviewVoteAnon(
	post_id  BIGINT,
	user_ip_address  INET,
	comment  TEXT
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	INSERT INTO SoftMod.NeedsReviewVoteAnon (
		post_id,
		user_ip_address,
		comment
	)
	VALUES (
		SetReviewVoteAnon.post_id,
		SetReviewVoteAnon.user_ip_address,
		SetReviewVoteAnon.comment
	)
	ON CONFLICT (post_id, user_ip_address) DO UPDATE SET
		upload_time = CURRENT_TIMESTAMP,
		comment     = EXCLUDED.comment;

	-- The Upsert can't fail, so always return 1
	RETURN TRUE;
END;