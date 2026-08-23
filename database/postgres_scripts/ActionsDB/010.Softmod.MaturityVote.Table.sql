CREATE TABLE SoftMod.MaturityVote (
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

CREATE INDEX ON SoftMod.MaturityVote(post_id);

-- Triggers to insert the post_id we just updated into Public.LatestVotes
CREATE TRIGGER TRG_LatestVotes_I
	AFTER INSERT
	ON SoftMod.MaturityVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes(); -- Insert as priority
CREATE TRIGGER TRG_LatestVotes_U
	AFTER UPDATE
	ON SoftMod.MaturityVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes();
CREATE TRIGGER TRG_LatestVotes_D
	AFTER DELETE
	ON SoftMod.MaturityVote
	REFERENCING OLD TABLE AS newtab -- Weird, on purpose
	EXECUTE FUNCTION Public.InsertLatestVotes();