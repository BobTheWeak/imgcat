CREATE OR REPLACE FUNCTION SoftMod.SetMaturityVote(
	post_id   BIGINT,
	user_id   BIGINT,
	maturity  SMALLINT,
	is_sexual BOOL,
	is_gore   BOOL,
	is_trauma BOOL
)
RETURNS BOOL
LANGUAGE SQL
STRICT
SECURITY DEFINER
BEGIN ATOMIC
	INSERT INTO SoftMod.MaturityVote (
		post_id,
		user_id,
		maturity,
		is_sexual,
		is_gore,
		is_trauma
	)
	VALUES (
		SetMaturityVote.post_id,
		SetMaturityVote.user_id,
		CAST(SetMaturityVote.maturity AS MATURITY_LEVEL),
		SetMaturityVote.is_sexual,
		SetMaturityVote.is_gore,
		SetMaturityVote.is_trauma
	)
	ON CONFLICT (post_id, user_id) DO UPDATE SET
		upload_time = CURRENT_TIMESTAMP,
		maturity    = EXCLUDED.maturity,
		is_sexual   = EXCLUDED.is_sexual,
		is_gore     = EXCLUDED.is_gore,
		is_trauma   = EXCLUDED.is_trauma;

	-- The Upsert can't fail, so always return 1
	RETURN TRUE;
END;