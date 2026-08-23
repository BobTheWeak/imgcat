CREATE TABLE SoftMod.CategoryVote (
	post_id
		BIGINT NOT NULL,
	user_id
		BIGINT NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	is_news     BOOL NOT NULL,
	is_politics BOOL NOT NULL,
	is_oc_art   BOOL NOT NULL,
	is_selfie   BOOL NOT NULL,
	is_animal   BOOL NOT NULL,
	is_ai       BOOL NOT NULL,

	PRIMARY KEY(post_id, user_id)
);

CREATE INDEX ON SoftMod.CategoryVote(post_id);

-- Triggers to insert the post_id we just updated into Public.LatestVotes
CREATE TRIGGER TRG_LatestVotes_I
	AFTER INSERT
	ON SoftMod.CategoryVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes(); -- Insert as priority
CREATE TRIGGER TRG_LatestVotes_U
	AFTER UPDATE
	ON SoftMod.CategoryVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes();
CREATE TRIGGER TRG_LatestVotes_D
	AFTER DELETE
	ON SoftMod.CategoryVote
	REFERENCING OLD TABLE AS newtab -- Weird, on purpose
	EXECUTE FUNCTION Public.InsertLatestVotes();
