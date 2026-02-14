ARG PROJ_NAME="imgcat_maria_database"

FROM mariadb:11.8.4 AS core
ARG PROJ_NAME

WORKDIR /app
COPY ./init/ /docker-entrypoint-initdb.d/

ENTRYPOINT ["docker-entrypoint.sh"]
EXPOSE 3306