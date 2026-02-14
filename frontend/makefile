help:
	@echo "make [frontend]"

frontend:
	docker build \
	-t icfe \
	--secret type=env,id=IC_DB_USER \
	--secret type=env,id=IC_DB_PASS \
	--secret type=env,id=IC_USERDB_USER \
	--secret type=env,id=IC_USERDB_PASS \
	--secret id=IC_JWT_PUB,src="${IC_JWT_PUB}" \
	--secret id=IC_JWT_PVT,src="${IC_JWT_PVT}" \
	--file frontend.container \
	.

deploy_frontend:
	docker run \
	--env IC_DB_USER \
	--env IC_DB_PASS \
	--env IC_USERDB_USER \
	--env IC_USERDB_PASS \
	--env IC_S3_URL \
	--env IC_S3_BUCKET \
	--env IC_S3_ACCESS_KEY \
	--env IC_S3_SECRET_KEY \
	--add-host host.docker.internal:host-gateway \
	-p 9720:8080 \
	icfe