CREATE OR REPLACE FUNCTION SoftMod.DeleteMaturityVote(
	post_id   BIGINT,
	user_id   BIGINT
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	DELETE FROM SoftMod.MaturityVote a
	WHERE a.post_id = DeleteMaturityVote.post_id
		AND a.user_id = DeleteMaturityVote.user_id;

	RETURN TRUE;
END;