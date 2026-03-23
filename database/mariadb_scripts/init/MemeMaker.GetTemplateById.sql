DELIMITER $$
CREATE OR REPLACE PROCEDURE MemeMaker.GetTemplateById(
	p_template_id INT
)
LANGUAGE SQL
NOT DETERMINISTIC
READS SQL DATA
SQL SECURITY DEFINER
BEGIN
	-- First dataset:
	-- The list of all templates
	SELECT
		a.id,
		a.name,
		a.height,
		a.width,
		b.link_v1 AS image,
		c.link_v1 AS thumb  -- Nullable
	FROM MemeMaker.Template a
	INNER JOIN Posts.Media b
		ON a.media_id = b.id
	LEFT JOIN Posts.Media c
		ON a.thumbnail_id = c.id
	WHERE a.id = p_template_id;

	-- Second dataset:
	-- The list of all text areas within the templates
	-- The middleware call will match these up & include them with the template JSON
	SELECT
		a.template_id  AS id,
		a.area_num     AS idx,
		a.is_top_text  AS dir,
		a.x,       -- nullable
		a.y,       -- nullable
		a.height,  -- nullable
		a.width,   -- nullable
		a.angle,   -- nullable
		a.default_text AS "text"  -- nullable
	FROM MemeMaker.TemplateTextArea a
	WHERE a.template_id = p_template_id;
	-- NOTE: There was a discussion about adding search terms here,
	-- but it's better to put that in a seperate call
END
$$
DELIMITER ;