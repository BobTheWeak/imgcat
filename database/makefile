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

postgres:
	docker build \
	-t localdev-postgres \
	--file postgres.container \
	--secret type=env,id=IC_UDB_USER \
	--secret type=env,id=IC_UDB_PASS \
	.

#deploy_mariadb:
#	docker run \
#		--detach \
#		-p 9710:8080 \
#		--env MARIADB_RANDOM_ROOT_PASSWORD=1 \
#		localdev-mariadb
#
#deploy_postgres:
#	docker run \
#		--detach \
#		-p 9712:8080 \
#		--env IC_UDB_HOST \
#		--env IC_UDB_PORT \
#		--env IC_UDB_DB \
#		--env IC_UDB_USER \
#		--env IC_UDB_PASS \
#		localdev-postgres

deploy_mariadb:
	docker stop localdev-mariadb || true
	docker rm localdev-mariadb || true
	docker run \
		--detach \
		--network ic-net \
		--env MARIADB_RANDOM_ROOT_PASSWORD=1 \
		--name localdev-mariadb \
		localdev-mariadb

deploy_postgres:
	docker stop localdev-postgres || true
	docker rm localdev-postgres || true
	docker run \
		--detach \
		--network ic-net \
		--env IC_UDB_HOST \
		--env IC_UDB_PORT \
		--env IC_UDB_DB \
		--env IC_UDB_USER \
		--env IC_UDB_PASS \
		--name localdev-postgres \
		localdev-postgres

clean:
	#docker rmi --force localdev-mariadb
	#docker rmi --force localdev-postgres
	docker container prune --force
	docker image prune --force
	docker buildx prune --force
#pg:
#	docker build -t localdev-pg --file pg.container . 
#
#deploy_pg:
#	docker run -p 9712:8080 localdev-pg

