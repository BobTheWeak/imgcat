#####################
###   Variables   ###
#####################

CODE_DIR = .
JWT_DIR = ~/.ssh

#########################
###   Documentation   ###
#########################
help:
	@echo This builds/deploys all of the standard infrastructure for ImgCat
	@echo INIT COMMANDS
	@echo  - make help - Show this help
	@echo  - make init_network - Create the network that all services sit in
	@echo  - make init_secrets - Create placeholder secrets and random passwords
	@echo  - make init_jwts - Create a new pub/pvt JWT cert
	@echo  - make rotate_jwts - Rotate the JWT cert \(services need restarts\)
	@echo  - make [postgres, actions_db, mariadb, redis] - Create and deploy persistent core services
	@echo DEV COMMANDS
	@echo  - make build_[all, auth, posts, softmod, users, maturity, frontend] - Build microservices
	@echo  - make deploy_[all, auth, posts, softmod, users, maturity, frontend] - Deploy microservices
	@echo  - make [proxy, proxy-localdev] - Redeploy the reverse proxy for microservices


#####################################
###   Server Init & Maintenance   ###
#####################################

init_network:
	podman network create ic-net

init_secrets:
	# JWT Secrets - "init_jwts" populates these properly
	printf "#" | podman secret create jwt_pub -
	printf "#" | podman secret create jwt_pvt -
	printf "#" | podman secret create jwt_pub_rot -
	# Oauth/OpenID
	printf "#" | podman secret create oauth_google_id -
	printf "#" | podman secret create oauth_google_secret -
	printf "#" | podman secret create oauth_microsoft_id -
	printf "#" | podman secret create oauth_microsoft_secret -
	# Storage
	printf "#" | podman secret create s3_access_key -
	printf "#" | podman secret create s3_bucket -
	printf "#" | podman secret create s3_secret_key -
	printf "#" | podman secret create s3_url -
	# DB Root Accounts
	printf $(openssl rand -base64 24) | podman secret create db_mariadb_root_pass -
	printf "#" | podman secret create db_postgres_root_user -
	printf $(openssl rand -base64 24) | podman secret create db_postgres_root_pass -
	# DB Service Accounts
	printf "#" | podman secret create db_mariadb_user -
	printf "#" | podman secret create db_auth_svc_user -
	printf "#" | podman secret create db_posts_svc_user -
	printf "#" | podman secret create db_users_svc_user -
	printf "#" | podman secret create db_softmod_svc_user -
	printf "#" | podman secret create db_maturity_svc_user -
	# DB Passwords
	printf $(openssl rand -base64 24) | podman secret create db_mariadb_pass -
	printf $(openssl rand -base64 24) | podman secret create db_auth_svc_pass -
	printf $(openssl rand -base64 24) | podman secret create db_posts_svc_pass -
	printf $(openssl rand -base64 24) | podman secret create db_users_svc_pass -
	printf $(openssl rand -base64 24) | podman secret create db_softmod_svc_pass -
	printf $(openssl rand -base64 24) | podman secret create db_maturity_svc_pass -

init_jwts:
	openssl genpkey -algorithm ed25519 -out ${JWT_DIR}/id_imgcat_jwt
	openssl pkey -in ${JWT_DIR}/id_imgcat_jwt -pubout -out ${JWT_DIR}/id_imgcat_jwt.pub
	podman secret create --replace jwt_pvt ${JWT_DIR}/id_imgcat_jwt
	podman secret create --replace jwt_pub ${JWT_DIR}/id_imgcat_jwt.pub
	podman secret create --replace jwt_pub_rot ${JWT_DIR}/id_imgcat_jwt.pub

rotate_jwts:
	podman secret create --replace jwt_pub_rot ${JWT_DIR}/id_imgcat_jwt.pub
	openssl genpkey -algorithm ed25519 -out ${JWT_DIR}/id_imgcat_jwt
	openssl pkey -in ${JWT_DIR}/id_imgcat_jwt -pubout -out ${JWT_DIR}/id_imgcat_jwt.pub
	podman secret create --replace jwt_pvt ${JWT_DIR}/id_imgcat_jwt
	podman secret create --replace jwt_pub ${JWT_DIR}/id_imgcat_jwt.pub


#########################
###   Core Services   ###
#########################

# This is the UserDB
postgres:
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env POSTGRES_DB=UserDB \
		--secret db_postgres_root_user,type=env,target=POSTGRES_USER \
		--secret db_postgres_root_pass,type=env,target=POSTGRES_PASSWORD \
		--volume ${CODE_DIR}/database/postgres_scripts/init/:/docker-entrypoint-initdb.d/:ro \
		--name ic-postgres \
		postgres:latest

actions_db:
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env POSTGRES_DB=ActionsDB \
		--secret db_postgres_root_user,type=env,target=POSTGRES_USER \
		--secret db_postgres_root_pass,type=env,target=POSTGRES_PASSWORD \
		--volume ${CODE_DIR}/database/postgres_scripts/ActionsDB/:/docker-entrypoint-initdb.d/:ro \
		--name ic-actions-db \
		postgres:latest

mariadb:
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--secret db_mariadb_root_pass,type=env,target=MARIADB_ROOT_PASSWORD \
		--volume ${CODE_DIR}/database/mariadb_scripts/init/:/docker-entrypoint-initdb.d/:ro \
		--name ic-mariadb \
		mariadb:latest

redis:
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--volume ./redis_ratelimiter.lua:/data/ratelimiter.lua \
		--name ic-redis \
		redis:latest
	podman exec -it ic-redis /bin/bash -c "cat /data/ratelimiter.lua | redis-cli -x FUNCTION LOAD REPLACE"

#nginx:
#	podman stop ic-nginx || true
#	podman rm ic-nginx || true
#	podman run \
#		--detach \
#		--network ic-net \
#		--publish 8080:80 \
#		--publish 8443:443 \
#		--restart on-failure:3 \
#		--volume ./nginx.conf:/etc/nginx/nginx.conf:copy \
#		--volume ./dhparam4096.pem:/etc/nginx/dhparam4096.pem:copy \
#		--volume /etc/letsencrypt/live:/etc/letsencrypt/live:ro \
#		--volume /etc/letsencrypt/archive:/etc/letsencrypt/archive:ro \
#		--volume /etc/letsencrypt/webroot:/var/www:ro \
#		--name ic-nginx \
#		nginx:latest

proxy:
	podman stop ic-proxy || true
	podman rm ic-proxy || true
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--volume ./nginx_internal.conf:/etc/nginx/nginx.conf:copy \
		--name ic-proxy \
		nginx:latest

# This proxy replaces the real proxy, when developing on a localhost computer
proxy-localdev:
	podman stop ic-proxy || true
	podman rm ic-proxy || true
	podman run \
		--detach \
		--network ic-net \
		--publish 8081:8081 \
		--publish 8082:8082 \
		--publish 8083:8083 \
		--volume ./nginx_localdev.conf:/etc/nginx/nginx.conf:copy \
		--name ic-proxy \
		nginx:latest

#########################
###   Microservices   ###
#########################

# All of the rust services share the same container-file, they just depend on
# specifying the sub-directory. This... *should* work.
build_auth:
	podman build \
		--tag ic-auth-svc \
		--build-arg PROJ_NAME=auth_service2 \
		--file ic-generic-rust-svc.container \
		${CODE_DIR}/backend/

build_posts:
	podman build \
		--tag ic-posts-svc \
		--build-arg PROJ_NAME=posts_service \
		--file ic-generic-rust-svc.container \
		${CODE_DIR}/backend/

build_softmod:
	podman build \
		--tag ic-softmod-svc \
		--build-arg PROJ_NAME=softmod_actions \
		--file ic-generic-rust-svc.container \
		${CODE_DIR}/backend/

build_users:
	podman build \
		--tag ic-users-svc \
		--build-arg PROJ_NAME=users_service \
		--file ic-generic-rust-svc.container \
		${CODE_DIR}/backend/

build_maturity:
	podman build \
		--tag ic-maturity-svc \
		--build-arg PROJ_NAME=maturity_service \
		--file ic-generic-rust-svc.container \
		${CODE_DIR}/backend/

deploy_auth:
	podman stop ic-auth-svc || true
	podman rm ic-auth-svc || true
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env-file env_defaults \
		--env RUST_LOG=debug \
		--secret jwt_pub,type=env,target=IC_JWT_PUB \
		--secret jwt_pvt,type=env,target=IC_JWT_PVT \
		--secret jwt_pub_rot,type=env,target=IC_JWT_PUB_ROTATED \
		--secret db_auth_svc_user,type=env,target=IC_USERS_SVC_USER \
		--secret db_auth_svc_pass,type=env,target=IC_USERS_SVC_PASS \
		--secret oauth_google_id,type=env,target=IC_OAUTH_GOOGLE_ID \
		--secret oauth_google_secret,type=env,target=IC_OAUTH_GOOGLE_SECRET \
		--secret oauth_microsoft_id,type=env,target=IC_OAUTH_MICROSOFT_ID \
		--secret oauth_microsoft_secret,type=env,target=IC_OAUTH_MICROSOFT_SECRET \
		--name ic-auth-svc \
		localhost/ic-auth-svc

deploy_posts:
	podman stop ic-posts-svc || true
	podman rm ic-posts-svc || true
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env-file env_defaults \
		--secret jwt_pub,type=env,target=IC_JWT_PUB \
		--secret jwt_pub_rot,type=env,target=IC_JWT_PUB_ROTATED \
		--secret db_posts_svc_user,type=env,target=IC_UDB_USER \
		--secret db_posts_svc_pass,type=env,target=IC_UDB_PASS \
		--secret db_mariadb_user,type=env,target=IC_DB_USER \
		--secret db_mariadb_pass,type=env,target=IC_DB_PASS \
		--name ic-posts-svc \
		localhost/ic-posts-svc

deploy_softmod:
	podman stop ic-softmod-svc || true
	podman rm ic-softmod-svc || true
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env-file env_defaults \
		--secret jwt_pub,type=env,target=IC_JWT_PUB \
		--secret jwt_pub_rot,type=env,target=IC_JWT_PUB_ROTATED \
		--secret db_softmod_svc_user,type=env,target=IC_SOFTMOD_SVC_USER \
		--secret db_softmod_svc_pass,type=env,target=IC_SOFTMOD_SVC_PASS \
		--name ic-softmod-svc \
		localhost/ic-softmod-svc

deploy_users:
	podman stop ic-users-svc || true
	podman rm ic-users-svc || true
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env-file env_defaults \
		--secret jwt_pub,type=env,target=IC_JWT_PUB \
		--secret jwt_pub_rot,type=env,target=IC_JWT_PUB_ROTATED \
		--secret db_users_svc_user,type=env,target=IC_USERS_SVC_USER \
		--secret db_users_svc_pass,type=env,target=IC_USERS_SVC_PASS \
		--name ic-users-svc \
		localhost/ic-users-svc

deploy_maturity:
	podman stop ic-maturity-svc || true
	podman rm ic-maturity-svc || true
	podman run \
		--detach \
		--network ic-net \
		--restart on-failure:3 \
		--env-file env_defaults \
		--secret jwt_pub,type=env,target=IC_JWT_PUB \
		--secret jwt_pub_rot,type=env,target=IC_JWT_PUB_ROTATED \
		--secret db_maturity_svc_user,type=env,target=IC_MATURITY_SVC_USER \
		--secret db_maturity_svc_pass,type=env,target=IC_MATURITY_SVC_PASS \
		--name ic-maturity-svc \
		localhost/ic-maturity-svc

###########################
###   Svelte Frontend   ###
###########################

build_frontend:
	podman build \
		--tag ic-frontend \
		--file frontend2.container \
		${CODE_DIR}/frontend/

deploy_frontend:
	podman stop ic-frontend || true
	podman rm ic-frontend || true
	podman run \
		--detach \
		--publish 8080:8080 \
		--network ic-net \
		--restart on-failure:3 \
		--env-file env_defaults \
		--secret jwt_pub,type=env,target=IC_JWT_PUB \
		--secret jwt_pub_rot,type=env,target=IC_JWT_PUB_ROTATED \
		--secret db_mariadb_user,type=env,target=IC_DB_USER \
		--secret db_mariadb_pass,type=env,target=IC_DB_PASS \
		--secret s3_url,type=env,target=IC_S3_URL \
		--secret s3_bucket,type=env,target=IC_S3_BUCKET \
		--secret s3_access_key,type=env,target=IC_S3_ACCESS_KEY \
		--secret s3_secret_key,type=env,target=IC_S3_SECRET_KEY \
		--secret ic_frontend_pvt_srv,type=env,target=IC_PVT_SVR \
		--name ic-frontend \
		localhost/ic-frontend


build_all: build_auth build_posts build_softmod build_users build_maturity build_frontend
deploy_all: deploy_auth deploy_posts deploy_softmod deploy_users deploy_maturity deploy_frontend proxy
