# Build Process

## Requirements

* Podman (or Docker)
* make
* S3 blob storage credentials
* OpenID Connect credentials

## Overview

The ImgCat technology stack is entirely containerized. The makefile contains all the commands needed to build and deploy everything.
* `make init_*` commands run once to bootstrap the environment
* `make redis, mariadb, postgres, actions_db` creates long-lived, core services
* `make build_*` builds various microservices from source
* `make deploy_*` stops and redeploys various microservices
* `make proxy` deploys a reverse proxy connecting the frontend to microservices

The typical development process is: (for example) `make build_posts deploy_posts proxy`. That will build a new container containing your updates, stop and redeploy it within the stack, then redeploy the proxy so networking picks up the changes.


## Initial Setup

1. Install Podman & make
2. Run `make init_network`, `make init_secrets`, `make init_jwts`
3. The following secrets must be set manually, using a command like: `printf "<NEW SECRET>" | podman secret create --replace <NAME> -`.
  - oauth_google_id
  - oauth_google_secret
  - oauth_microsoft_id
  - oauth_microsoft_secret
  - s3_access_key
  - s3_bucket
  - s3_secret_key
  - s3_url
4. We created default usernames for the DB. But these defaults are known, and create a **security risk** if deployed publicly. Please change anything starting with `LOCALDEV_`.
5. Run the following: `podman secret inspect --showsecret --format '{{.Spec.Name}}:\t{{.SecretData}}' $(podman secret ls -f name='db.*pass' --format '{{.Name}}')`, and store the results temporarially. You will need the randomly-generated passwords.
6. Setup Redis
  - Run `make redis`
  - Make sure `podman exec -it ic-redis redis-cli` connects to the server
  - Done
7. Setup MariaDB
  - Run `make mariadb`
  - Run and copy the results of: `sed -e "s/imgcat_posts/$(podman secret inspect --showsecret --format {{.SecretData}} db_mariadb_user)/gi" ./database/mariadb_scripts/perms/ZZZ_init_perms.sql` for a later step
  - Connect with `podman exec -it ic-redis mariadb --password`, using the `db_mariadb_root_pass` password
  - Run `CREATE USER LOCALDEV_frontend_svc IDENTIFIED BY '<PASSWORD>' WITH MAX_STATEMENT_TIME 1;`, using the `db_mariadb_pass` password
  - Run the commands you generated from the `sed` command above
  - Done
8. Setup PostgreSQL (UserDB)
  - Run `make postgres`
  - Run and copy the results of: `sed -e "s/IC_UDB_USER/$(podman secret inspect --showsecret --format {{.SecretData}} db_auth_svc_user)/gi" -e "s/IC_USERS_SVC_USER/$(podman secret inspect --showsecret --format {{.SecretData}} db_users_svc_user)/gi" ./database/postgres_scripts/perms/ZZZ_init_perms.sql` for a later step
  - Connect with `podman exec -it ic-postgres psql --user <USER> --db UserDB`, using the `db_postgres_root_user` user
  - Run `CREATE USER LOCALDEV_auth_svc WITH PASSWORD '<PASSWORD>';`, replacing db_auth_svc_pass
  - Run `CREATE USER LOCALDEV_users_svc WITH PASSWORD '<PASSWORD>';`, replacing db_users_svc_pass
  - Run the commands you generated from the `sed` command above
  - Done
9. Setup PostgreSQL (ActionsDB)
  - Run `make actions_db`
  - Run and copy the results of: `sed -e "s/IC_SOFTMOD_SVC_USER/$(podman secret inspect --showsecret --format {{.SecretData}} db_softmod_svc_user)/gi" -e "s/IC_SOFTMOD_SVC_PASS/$(podman secret inspect --showsecret --format {{.SecretData}} db_softmod_svc_pass)/gi" -e "s/IC_MATURITY_SVC_USER/$(podman secret inspect --showsecret --format {{.SecretData}} db_maturity_svc_user)/gi" -e "s/IC_MATURITY_SVC_PASS/$(podman secret inspect --showsecret --format {{.SecretData}} db_maturity_svc_pass)/gi" ./database/postgres_scripts/perms/ActionsDB.perms.sql` for a later step
  - Connect with `podman exec -it ic-actions-db psql --user <USER> --db ActionsDB`, using the `db_postgres_root_user` user
  - Run the commands you generated from the `sed` command above
  - Done
10. Run `make build_all`. This will take a while. Any build dependencies are built into the build containers.
11. Run `make deploy_all`. Verify everything is running with `podman ps -a`.
12. Run `make proxy` (or `make proxy-localdev` if you need debug access to services)
13. Visit `localhost:8080` in your browser. You should see the site homepage.

## Future improvements

1. Need a CI/CD process uploading to DockerHub, so folks don't have to build from scratch.
2. Need a local dev setup for S3 storage
3. Need a local dev setup for cloud identity providers for OpenID integration
4. The makefile should be converted to a Docker Compose file
5. `make proxy` should be smarter. It needs to be redeployed everytime the network changes.
6. Redis is not protected by a user/pass, but should be
7. The permissions setup for all databases can be automated, or at least made less terrible
