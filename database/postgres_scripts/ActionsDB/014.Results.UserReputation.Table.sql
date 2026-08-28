CREATE TABLE Results.UserReputation (
	user_id
		BIGINT NOT NULL,
	last_updated
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	samples
		INT NOT NULL,
	weight
		REAL NOT NULL,
	mat_bias
		REAL NOT NULL,

	PRIMARY KEY(user_id)
);