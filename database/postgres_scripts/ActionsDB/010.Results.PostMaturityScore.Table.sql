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
	is_sexual
		REAL NOT NULL,
	is_gore
		REAL NOT NULL,
	is_trauma
		REAL NOT NULL,
	is_total_samples
		INT NOT NULL,

	PRIMARY KEY(post_id)
);
