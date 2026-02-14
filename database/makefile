quick_help:
	@echo "make [mariadb, deploy_mariadb, help, ...]"

help:
	@echo "make mariadb - Build a fresh dev version of MariaDB"
	@echo "make deploy_mariadb - Deploy the MariaDB image as your DB"
	#@echo "make pg - Build a fresh dev version of PostgreSQL"
	#@echo "make deploy_pg - Deploy the PostgreSQL image as your DB"

mariadb:
	docker build \
	-t localdev-mariadb \
	--file mariadb.container \
	--secret type=env,id=IC_DB_USER \
	--secret type=env,id=IC_DB_PASS \
	--secret type=env,id=IC_USERDB_USER \
	--secret type=env,id=IC_USERDB_PASS \
	.

deploy_mariadb:
	docker run --detach -p 9710:8080 --env MARIADB_RANDOM_ROOT_PASSWORD=1 localdev-mariadb

clean:
	docker rmi --force localdev-mariadb
	docker container prune --force
	docker image prune --force
	docker buildx prune --force
#pg:
#	docker build -t localdev-pg --file pg.container . 
#
#deploy_pg:
#	docker run -p 9712:8080 localdev-pg

