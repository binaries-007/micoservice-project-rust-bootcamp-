FROM rust:1.74-alpine3.17 AS chef
USER root

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN apk add --no-cache musl-dev \
    & cargo install cargo-chef
WORKDIR /microservice-project


FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /microservice-project/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

RUN apk add --no-cache protoc
COPY . .
RUN cargo build --release --bin auth

FROM debian:buster-slim AS runtime
WORKDIR /microservice-project
COPY --from=builder /microservice-project/target/release/auth/ /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/auth" ]