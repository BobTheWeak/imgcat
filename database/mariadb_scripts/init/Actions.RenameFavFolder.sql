DELIMITER $$
CREATE OR REPLACE PROCEDURE Actions.RenameFavFolder(
	p_user_id BIGINT,
	p_folder_id INT UNSIGNED,
	p_name TINYTEXT
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	UPDATE Posts.UserFolders
	SET name = p_name
	WHERE id = p_folder_id
		AND user_id = p_user_id;
END
$$
DELIMITER ;


-- DEPRECATED... Migrate All UserActions to Actions schema
DELIMITER $$
CREATE OR REPLACE PROCEDURE Posts.RenameFavFolder(
	p_user_id BIGINT,
	p_folder_id INT UNSIGNED,
	p_name TINYTEXT
)
LANGUAGE SQL
NOT DETERMINISTIC
MODIFIES SQL DATA
SQL SECURITY DEFINER
BEGIN
	CALL Actions.RenameFavFolder(p_user_id, p_folder_id, p_name);
END
$$
DELIMITER ;