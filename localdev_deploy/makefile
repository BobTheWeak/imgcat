
quick_help:
	@echo "make [build_*, deploy_*, kill_*, restart_*, ...]"
	@echo " - services: all, nginx, mariadb, frontend, softmod"

help:
	@echo "BUILD"
	@echo "make build_all - Build every image used for the default deploy"
	@echo "make build_nginx - Build the nginx image"
	@echo "DEPLOY"
	@echo "make deploy_all - Deploy the entire stack all at once"
	@echo "make deploy_nginx - Nginx acts as the localdev proxy that connects everything"
	@echo "make deploy_mariadb - The MariaDB instance for local development"
	@echo "make deploy_frontend - The Node.JS/Svelte frontend"
	@echo "make deploy_softmod - The Soft-Mod Actions API, for users flagging issues"

########################
###   Network Init   ###
########################

init_network:
	docker network create ic-net
	docker volume create ic-keys

#########################
###   Build actions   ###
#########################

# To keep the same style as other build files (ie. make frontend & make deploy_frontend)
nginx: build_nginx
redis: build_redis

build_nginx:
	docker build -t localdev-nginx --file nginx.container .

build_redis:
	docker build -t localdev-redis --file redis.container .

build_mariadb:
	make -C ../database/ mariadb

build_postgres:
	make -C ../database/ postgres

build_frontend:
	make -C ../frontend/ frontend

build_auth:
	make -C ../backend/auth_service/ auth

build_softmod:
	make -C ../backend/softmod_actions/ softmod

build_all: build_nginx build_redis build_mariadb build_frontend build_auth build_softmod


##########################
###   Deploy actions   ###
##########################

deploy_nginx:
	#docker build -t localdev-nginx --file nginx.container .
	docker stop localdev-nginx || true
	docker rm localdev-nginx || true
	docker run \
		--detach \
		--network ic-net \
		--add-host host.docker.internal:host-gateway \
		-p   80:80 \
		-p 8081:8081 \
		-p 8082:8082 \
		-p 8083:8083 \
		--name localdev-nginx \
		localdev-nginx

deploy_redis:
	docker stop localdev-redis || true
	docker rm localdev-redis || true
	docker run \
		--detach \
		--network ic-net \
		--name localdev-redis \
		localdev-redis
#	docker run \
#		--detach \
#		-p 9716:8080 \
#		localdev-redis

deploy_mariadb:
#	docker run --detach -p 9710:8080 --env MARIADB_RANDOM_ROOT_PASSWORD=1 localdev-mariadb
	make -C ../database/ deploy_mariadb

deploy_postgres:
	make -C ../database/ deploy_postgres

deploy_frontend:
	make -C ../frontend/ deploy_frontend

deploy_auth:
	make -C ../backend/auth_service/ deploy_auth

deploy_softmod:
	make -C ../backend/softmod_actions/ deploy_softmod

deploy_all: deploy_mariadb deploy_redis deploy_frontend deploy_auth deploy_softmod deploy_nginx


########################
###   Kill actions   ###
########################

kill_nginx:
	docker ps -q --filter "ancestor=localdev-nginx" | xargs -r docker stop

kill_mariadb:
	docker ps -q --filter "ancestor=localdev-mariadb" | xargs -r docker stop

kill_frontend:
	docker ps -q --filter "ancestor=icfe" | xargs -r docker stop

kill_softmod:
	docker ps -q --filter "ancestor=smact" | xargs -r docker stop

kill_all: kill_nginx kill_mariadb kill_frontend kill_softmod


###########################
###   Restart actions   ###
###########################

restart_nginx: kill_nginx build_nginx deploy_nginx

restart_mariadb: kill_mariadb build_mariadb deploy_mariadb

restart_frontend: kill_frontend build_frontend deploy_frontend

restart_softmod: kill_softmod build_softmod deploy_softmod

restart_all: restart_nginx restart_mariadb restart_frontend restart_softmod


########################
###   Misc actions   ###
########################

clean:
	docker stop $(docker ps -aq) -q || true
	docker rm $(docker ps -aq) || true
	docker container prune --force
	docker image prune --force
	docker buildx prune --force
