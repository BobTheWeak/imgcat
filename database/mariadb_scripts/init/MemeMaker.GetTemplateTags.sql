DELIMITER $$
CREATE OR REPLACE PROCEDURE MemeMaker.GetTemplateTags()
LANGUAGE SQL
NOT DETERMINISTIC
READS SQL DATA
SQL SECURITY DEFINER
BEGIN

	-- First dataset:
	-- The list of tags/names
	SELECT
		id,
		name
	FROM MemeMaker.Tag
	ORDER BY id;

	-- Second dataset:
	-- The list of templates associated with each tag
	SELECT
		tag_id AS id,
		template_id AS template
	FROM TemplateTag
	ORDER BY tag_id;
END
$$
DELIMITER ;