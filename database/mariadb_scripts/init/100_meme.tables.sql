CREATE TABLE IF NOT EXISTS MemeMaker.Template (
	id
		BIGINT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	media_id
		INT UNSIGNED NOT NULL
		REFERENCES Posts.Media(id),
	thumbnail_id
		INT UNSIGNED NULL
		REFERENCES Posts.Media(id),

	height
		SMALLINT UNSIGNED NOT NULL,
	width
		SMALLINT UNSIGNED NOT NULL,
	name
		VARCHAR(255) NOT NULL,

	PRIMARY KEY(id)
);


CREATE TABLE IF NOT EXISTS MemeMaker.TemplateTextArea (
	template_id
		BIGINT UNSIGNED NOT NULL
		REFERENCES MemeMaker.Template(id),
	area_num
		TINYINT UNSIGNED NOT NULL,

	-- If it's top/bottom text, all we need is this, and we can calculate the rest
	is_top_text
		BOOL NOT NULL
		DEFAULT TRUE,

	-- If the textbox needs to be constrained, because the meme
	-- is a sign (aka: prove me wrong) or can't be top/bottom and
	-- automatically center-aligned, then we have display options
	x -- as pixels to the center (NULL: 50%)
		SMALLINT UNSIGNED NULL,
	y -- as pixels to the top of the first line (NULL: 0px from top)
		SMALLINT UNSIGNED NULL,
	height -- as max-allowed height (NULL: no-limit)
		SMALLINT UNSIGNED NULL,
	width -- as max-allowed width (NULL: img-width)
		SMALLINT UNSIGNED NULL,
	angle -- in degrees, clockwise (NULL: 0deg)
		SMALLINT NULL,

	-- If there's some default text we should pre-populate
	default_text
		VARCHAR(255) NULL,

	PRIMARY KEY(template_id, area_num)
);


CREATE TABLE IF NOT EXISTS MemeMaker.Tag (
	id
		BIGINT UNSIGNED NOT NULL
		AUTO_INCREMENT,
	name
		VARCHAR(50) NOT NULL,

	PRIMARY KEY(id),
	CONSTRAINT UNIQUE(name)
);


CREATE TABLE IF NOT EXISTS MemeMaker.TemplateTag (
	template_id
		BIGINT UNSIGNED NOT NULL
		REFERENCES MemeMaker.Template(id),
	tag_id
		BIGINT UNSIGNED NOT NULL
		REFERENCES MemeMaker.Tag(id),

	PRIMARY KEY(template_id, tag_id)
);


CREATE TABLE IF NOT EXISTS MemeMaker.EquivalentTags (
	tag_a
		BIGINT UNSIGNED NOT NULL
		REFERENCES MemeMaker.Tag(id),
	tag_b
		BIGINT UNSIGNED NOT NULL
		REFERENCES MemeMaker.Tag(id),

	-- Ensure bidirectional uniqueness
	CONSTRAINT CHECK (tag_a < tag_b),
	PRIMARY KEY(tag_a, tag_b)
);