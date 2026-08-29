CREATE TABLE SoftMod.NeedsReviewVote (
	post_id
		BIGINT NOT NULL,
	user_id
		BIGINT NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	comment
		TEXT NOT NULL,

	PRIMARY KEY(post_id, user_id),
	CHECK(octet_length(comment) < 127) -- For efficient storage
);

CREATE INDEX ON SoftMod.NeedsReviewVote(post_id);

-- NOTE: We don't want triggers on the NeedsReview tables
