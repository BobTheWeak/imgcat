CREATE OR REPLACE FUNCTION SoftMod.DeleteTagVote(
	post_id   BIGINT,
	user_id   BIGINT
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	DELETE FROM SoftMod.TagVote a
	WHERE a.post_id = DeleteTagVote.post_id
		AND a.user_id = DeleteTagVote.user_id;

	RETURN TRUE;
END;