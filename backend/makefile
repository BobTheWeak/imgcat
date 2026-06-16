help:
	@echo "make [dev, posts, deploy, clean]"

dev:
	cargo build --release
	docker build \
		-t posts-svc \
		--file posts_from_local.container \
		.

posts:
	docker build \
		-t posts-svc \
		--file posts.container \
		.

deploy_posts:
	docker stop posts-svc || true
	docker rm posts-svc || true
	docker run \
		--env IC_JWT_ISS \
		--env IC_JWT_AUD \
		--env IC_REDIS_HOST \
		--env IC_REDIS_PORT \
		--env IC_REDIS_DB \
		--env IC_DB_HOST \
		--env IC_DB_PORT \
		--env IC_DB_DB \
		--env IC_DB_USER \
		--env IC_DB_PASS \
		--env IC_UDB_HOST \
		--env IC_UDB_PORT \
		--env IC_UDB_DB \
		--env IC_UDB_USER \
		--env IC_UDB_PASS \
		--volume ic-keys:/mnt/ic-keys:ro \
		--detach \
		--network ic-net \
		--name posts-svc \
		posts-svc

clean:
	docker rmi --force posts
	docker image prune --force
	docker builder prune --force



