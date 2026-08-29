CREATE OR REPLACE FUNCTION SoftMod.SetTagVote(
	post_id BIGINT,
	user_id BIGINT,
	tag     TEXT
)
RETURNS BOOL
LANGUAGE PLPGSQL
STRICT
SECURITY DEFINER
AS $$
DECLARE
	v_tag_id BIGINT := (SELECT id FROM Public.Tag WHERE name = SetTagVote.tag);
BEGIN

	-- Check if the tag exists
	IF v_tag_id IS NULL THEN
		INSERT INTO Public.Tag (name)
		VALUES (SetTagVote.tag)
		ON CONFLICT(name) DO NOTHING
		RETURNING id
		INTO v_tag_id;

		-- There is a race condition when two users insert the same tag at the same time
		-- It won't error, but tag_id remains null (DO NOTHING has precident over RETURNING?)
		IF v_tag_id IS NULL THEN
			v_tag_id := (SELECT id FROM Public.Tag WHERE name = SetTagVote.tag);
		END IF;
	END IF;


	INSERT INTO SoftMod.TagVote AS a (
		post_id,
		user_id,
		tag_id
	)
	VALUES(
		SetTagVote.post_id,
		SetTagVote.user_id,
		v_tag_id
	)
	ON CONFLICT
		ON CONSTRAINT tagvote_pkey
		DO UPDATE
		SET
			update_time = CURRENT_TIMESTAMP,
			post_id = SetTagVote.post_id,
			user_id = SetTagVote.user_id,
			tag_id = v_tag_id;

	-- No reason this can fail, always return true
	RETURN TRUE;
END;
$$;
