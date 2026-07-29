help:
	@echo "make [dev, auth, deploy, clean]"

dev:
	cargo build --release
	docker build \
		-t auth \
		--file auth_from_local.container \
		.

auth:
	docker build \
		-t auth \
		--file auth.container \
		.

#deploy_auth:
#	docker run \
#		--env ORIGIN \
#		--env IC_JWT_PVT \
#		--env IC_JWT_PUB \
#		--env IC_JWT_PUB_ROTATED \
#		--env IC_JWT_ISS \
#		--env IC_JWT_AUD \
#		--env IC_UDB_HOST \
#		--env IC_UDB_PORT \
#		--env IC_UDB_DB \
#		--env IC_UDB_USER \
#		--env IC_UDB_PASS \
#		--env IC_REDIS_HOST \
#		--env IC_REDIS_PORT \
#		--env IC_REDIS_DB \
#		--env IC_OAUTH_GOOGLE_ID \
#		--env IC_OAUTH_GOOGLE_SECRET \
#		--detach \
#		-p 9730:8080 \
#		auth

deploy_auth:
	docker stop auth || true
	docker rm auth || true
	docker run \
		--env IC_ORIGIN \
		--env IC_JWT_ISS \
		--env IC_JWT_AUD \
		--env IC_UDB_HOST \
		--env IC_UDB_PORT \
		--env IC_UDB_DB \
		--env IC_UDB_USER \
		--env IC_UDB_PASS \
		--env IC_REDIS_HOST \
		--env IC_REDIS_PORT \
		--env IC_REDIS_DB \
		--env IC_OAUTH_GOOGLE_ID \
		--env IC_OAUTH_GOOGLE_SECRET \
		--volume ic-keys:/mnt/ic-keys:ro \
		--detach \
		--network ic-net \
		--name auth \
		auth

clean:
	docker rmi --force auth
	docker image prune --force
	docker builder prune --force



