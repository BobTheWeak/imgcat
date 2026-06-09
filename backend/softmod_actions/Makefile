help:
	@echo "make [dev, softmod, deploy, clean]"

dev:
	cargo build --features use_mariadb

softmod:
	docker build \
		-t smact \
		--file softmod.container \
		.

#deploy_softmod:
#	docker run --detach -p 9732:8080 smact

deploy_softmod:
	docker stop smact || true
	docker rm smact || true
	docker run \
		--env IC_ORIGIN \
		--env IC_JWT_ISS \
		--env IC_JWT_AUD \
		--env IC_DB_HOST \
		--env IC_DB_PORT \
		--env IC_DB_DB \
		--env IC_DB_USER \
		--env IC_DB_PASS \
		--volume ic-keys:/mnt/ic-keys:ro \
		--detach \
		--network ic-net \
		--name smact \
		smact

clean:
	docker rmi --force smact
	docker image prune --force
	docker builder prune --force