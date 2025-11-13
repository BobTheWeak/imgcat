ARG PROJ_NAME="softmod_actions"

FROM rust:alpine AS build
ARG PROJ_NAME

ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN apk update
RUN apk add --no-cache musl-dev
RUN apk add curl
RUN apk add lld clang


# Build dependencies only
WORKDIR /app
#RUN cargo new --bin ${PROJ_NAME}
WORKDIR ${PROJ_NAME}
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
RUN mkdir src
RUN echo "fn main(){}" > ./src/main.rs
RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release


# Build application
COPY ./src ./src
RUN --mount=type=secret,id=ic-db-host,env=IC_DB_IS_MARIA \
    --mount=type=secret,id=ic-db-host,env=IC_DB_HOST \
    --mount=type=secret,id=ic-db-user,env=IC_DB_USER \
    --mount=type=secret,id=ic-db-pass,env=IC_DB_PASS \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release
RUN strip target/release/${PROJ_NAME}


FROM alpine AS runtime
ARG PROJ_NAME
RUN apk add --no-cache libgcc
# app specific environment variables
ENV APP_ENVIRONMENT=prod
# copy the binary, and config folders
WORKDIR /app
COPY --from=build /app/${PROJ_NAME}/target/release/${PROJ_NAME} ./main

ENTRYPOINT ["./main"]
EXPOSE 8080/tcp