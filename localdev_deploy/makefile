
quick_help:
	@echo "make [build_all, deploy_all, help, ...]"

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

#########################
###   Build actions   ###
#########################

nginx: build_nginx

build_nginx:
	docker build -t localdev-nginx --file nginx.container .

build_all:
	make -C . build_nginx
	make -C ../database/ mariadb
	make -C ../backend/softmod_actions/ softmod


##########################
###   Deploy actions   ###
##########################

deploy_nginx:
#	docker build -t localdev-nginx --file nginx.container .
	docker run \
		--detach \
		--add-host host.docker.internal:host-gateway \
		-p 8080:8080 \
		-p 8081:8081 \
		localdev-nginx

deploy_mariadb:
#	docker run --detach -p 9710:8080 --env MARIADB_RANDOM_ROOT_PASSWORD=1 localdev-mariadb
	make -C ../database/ deploy_mariadb

deploy_frontend:
	docker run -p 9720:8080 icfe

deploy_softmod:
	docker run --detach -p 9732:8080 smact

deploy_all: deploy_mariadb deploy_frontend deploy_softmod deploy_nginx


########################
###   Misc actions   ###
########################

#clean:
#	docker stop $(docker ps -aq) -q || true
#	docker rm $(docker ps -aq) || true
#	docker container prune --force
#	docker image prune --force
#	docker buildx prune --force
