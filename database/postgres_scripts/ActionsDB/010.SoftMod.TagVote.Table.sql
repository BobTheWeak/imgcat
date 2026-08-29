CREATE TABLE SoftMod.TagVote (
	post_id
		BIGINT NOT NULL,
	user_id
		BIGINT NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	tag_id
		BIGINT NOT NULL
		REFERENCES Public.Tag(id),

	PRIMARY KEY(post_id, user_id, tag_id)
);

-- Index on typical use-case "What tags does this post have?"
CREATE INDEX ON SoftMod.TagVote(post_id);
-- NOTE: Not doing a covering index or post-by-user, b/c this server isn't intended
-- for user API calls, just bulk server-to-server transfers.

-- Triggers to insert the post_id we just updated into Public.LatestVotes
CREATE TRIGGER TRG_LatestVotes_I
	AFTER INSERT
	ON SoftMod.TagVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes();
CREATE TRIGGER TRG_LatestVotes_U
	AFTER UPDATE
	ON SoftMod.TagVote
	REFERENCING NEW TABLE AS newtab
	EXECUTE FUNCTION Public.InsertLatestVotes();
CREATE TRIGGER TRG_LatestVotes_D
	AFTER DELETE
	ON SoftMod.TagVote
	REFERENCING OLD TABLE AS newtab -- Weird, on purpose
	EXECUTE FUNCTION Public.InsertLatestVotes();