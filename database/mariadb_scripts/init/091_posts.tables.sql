CREATE TABLE IF NOT EXISTS Posts.PostTag (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	tag_id
		BIGINT UNSIGNED NOT NULL
		REFERENCES Content.Tag(id),

	PRIMARY KEY(post_id, tag_id)
);

CREATE TABLE IF NOT EXISTS Posts.PostMaturity (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	maturity_average
		FLOAT UNSIGNED NOT NULL,
	maturity_votes
		INT UNSIGNED NOT NULL,
	is_sexual_votes
		INT UNSIGNED NOT NULL,
	is_gore_votes
		INT UNSIGNED NOT NULL,
	is_trauma_votes
		INT UNSIGNED NOT NULL,

	PRIMARY KEY(post_id)
);

CREATE TABLE IF NOT EXISTS Posts.PostContent (
	post_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Post(id),
	is_sexual_votes
		INT UNSIGNED NOT NULL,
	is_gore_votes
		INT UNSIGNED NOT NULL,
	is_trauma_votes
		INT UNSIGNED NOT NULL,

	PRIMARY KEY(post_id)
);