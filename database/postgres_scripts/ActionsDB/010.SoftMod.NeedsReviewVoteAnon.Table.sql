CREATE TABLE SoftMod.NeedsReviewVoteAnon (
	post_id
		BIGINT NOT NULL,
	user_ip_address
		INET NOT NULL,
	upload_time
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	comment
		TEXT NOT NULL,

	PRIMARY KEY(post_id, user_ip_address),
	CHECK(octet_length(comment) < 127) -- For efficient storage
);

CREATE INDEX ON SoftMod.NeedsReviewVoteAnon(post_id);

-- NOTE: We don't want triggers on the NeedsReview tables
