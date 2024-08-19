FROM rust:alpine3.20 AS builder

WORKDIR /app

RUN : \
    && apk add --no-cache \
        musl-dev \
        pkgconfig \
        perl \
        make \
    && :

COPY Cargo.toml Cargo.lock .sqlx ./

ENV SQLX_OFFLINE=true

RUN : \
    && mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl --release \
    && rm -rf src \
    && :

COPY . .

RUN : \
    && touch -a -m ./src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl --release \
    && :

FROM alpine:3.20.2

WORKDIR /app

RUN : \
    apk add --no-cache \
        libgcc \
        libstdc++ \
        musl \
    && :

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/burrito-server /app/
COPY --from=builder /app/.env /app/.env

CMD ["/app/burrito-server"]
