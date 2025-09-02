CREATE USER imgcat_jwt
IDENTIFIED BY '<password here>'
WITH
	MAX_STATEMENT_TIME 1;

CREATE USER imgcat_posts
IDENTIFIED BY '<password here>'
WITH
	MAX_STATEMENT_TIME 5;

CREATE USER imgcat_mods
IDENTIFIED BY '<password here>'
WITH
	MAX_STATEMENT_TIME 5;

CREATE USER imgcat_system
IDENTIFIED BY '<password here>'
WITH
	MAX_STATEMENT_TIME 5;

# Moved GRANT block to ./init/init_perms.sql