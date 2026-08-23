-- This is a standard trigger function for anything that needs to insert into this table
CREATE OR REPLACE FUNCTION Public.InsertLatestVotes()
RETURNS TRIGGER
LANGUAGE PLPGSQL
SECURITY DEFINER
AS $$
BEGIN

	IF TG_NARGS = 0 THEN
		-- By default, insert without priority flag
		INSERT INTO Public.LatestVotes(post_id, priority)
		SELECT post_id, false
		FROM newtab
		ON CONFLICT DO NOTHING;
	ELSE
		-- If we created the trigger with any param(!), insert as priority
		INSERT INTO Public.LatestVotes(post_id, priority)
		SELECT post_id, true
		FROM newtab
		ON CONFLICT (post_id) 
			DO UPDATE
			SET priority=TRUE;
	END IF;

	RETURN NULL;
END $$;
