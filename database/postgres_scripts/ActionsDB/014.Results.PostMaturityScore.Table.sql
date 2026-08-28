CREATE TABLE Results.PostMaturityScore (
	post_id
		BIGINT NOT NULL,
	last_updated
		TIMESTAMP NOT NULL
		DEFAULT CURRENT_TIMESTAMP,

	-- Maturity
	mat_category
		MATURITY_LEVEL NOT NULL,
	mat_score
		REAL NOT NULL,
	mat_samples
		INT NOT NULL,
	
	-- Maturity subcategories
	is_sexual_score
		REAL NOT NULL,
	is_gore_score
		REAL NOT NULL,
	is_trauma_score
		REAL NOT NULL,
	is_sexual
		BOOL NOT NULL,
	is_gore
		BOOL NOT NULL,
	is_trauma
		BOOL NOT NULL,
	is_total_samples
		INT NOT NULL,

	PRIMARY KEY(post_id)
);

-- Used for batch management, to grab any changes since I last asked
-- TODO: There is also a fancy way of doing this with xmin cols. But recovery & batches gets messier.
CREATE INDEX ON Results.PostMaturityScore(last_updated DESC);
