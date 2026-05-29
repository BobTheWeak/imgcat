help:
	@echo "make [dev, softmod, deploy, clean]"

dev:
	cargo build --features use_mariadb

softmod:
	docker build \
		-t smact \
		--secret id=ic-db-user,env=IC_DB_USER \
		--secret id=ic-db-pass,env=IC_DB_PASS \
		--file softmod.container \
		.

#deploy_softmod:
#	docker run --detach -p 9732:8080 smact

deploy_softmod:
	docker stop smact || true
	docker rm smact || true
	docker run \
		--detach \
		--network ic-net \
		--name smact \
		smact

clean:
	docker rmi --force smact
	docker image prune --force
	docker builder prune --force