CREATE TABLE Public.tag (
	id
		BIGSERIAL,
	name
		TEXT NOT NULL,

	PRIMARY KEY(id),
	UNIQUE(name),
	CHECK(octet_length(name) < 127) -- For efficient storage (UTF8-length, not char-length)
);
