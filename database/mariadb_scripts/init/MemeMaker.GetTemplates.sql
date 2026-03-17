DELIMITER $$
CREATE OR REPLACE PROCEDURE MemeMaker.GetTemplates()
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
	ORDER BY id;

	-- Second dataset:
	-- The list of all text areas within the templates
	-- The middleware call will match these up & include them with the template JSON
	SELECT
		a.template_id  AS id,
		a.area_num     AS idx,
		a.height       AS y,
		a.is_top_text  AS dir,
		a.max_height   AS height,  -- Nullable
		a.pos_left     AS "left",  -- Nullable
		a.pos_right    AS "right", -- Nullable
		a.default_text AS "text"   -- Nullable
	FROM MemeMaker.TemplateTextArea a
	ORDER BY a.template_id, a.area_num;
	-- Order is important here so we can merge these two structures in a single loop

	-- NOTE: There was a discussion about adding search terms here,
	-- but it's better to put that in a seperate call

	-- NOTE: These two datasets do NOT have a common query, and may be out-of-whack
	-- This needs to be rectified in the MW layer, when it's stitching these together
END
$$
DELIMITER ;