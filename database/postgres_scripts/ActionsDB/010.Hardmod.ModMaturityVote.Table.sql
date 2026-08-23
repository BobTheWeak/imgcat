CREATE TABLE HardMod.ModMaturityVote (
	post_id
		BIGINT NOT NULL,
	user_id
		BIGINT NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	maturity
		Public.MATURITY_LEVEL NOT NULL,
	is_sexual
		BOOL NOT NULL,
	is_gore
		BOOL NOT NULL,
	is_trauma
		BOOL NOT NULL,

	PRIMARY KEY(post_id, user_id)
);

CREATE INDEX ON HardMod.ModMaturityVote(post_id);

-- Triggers to insert the post_id we just updated into Public.LatestVotes
CREATE TRIGGER TRG_LatestVotes_I
	AFTER INSERT
	ON HardMod.ModMaturityVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes('P'); -- Insert as priority
CREATE TRIGGER TRG_LatestVotes_U
	AFTER UPDATE
	ON HardMod.ModMaturityVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes('P');
CREATE TRIGGER TRG_LatestVotes_D
	AFTER DELETE
	ON HardMod.ModMaturityVote
	REFERENCING OLD TABLE AS newtab -- Weird, on purpose
	EXECUTE FUNCTION Public.InsertLatestVotes('P');