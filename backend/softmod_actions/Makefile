help:
	@echo "make [dev, release, docker]"

dev:
	cargo build

clean:
	docker rmi --force smact
	docker image prune --force
	docker builder prune --force

docker:
	docker build \
		-t smact \
		--secret id=ic-db-host,env=IC_DB_IS_MARIA \
		--secret id=ic-db-host,env=IC_DB_HOST \
		--secret id=ic-db-user,env=IC_DB_USER \
		--secret id=ic-db-pass,env=IC_DB_PASS \
		.

deploy:
	docker run -p 9000:8080 smact