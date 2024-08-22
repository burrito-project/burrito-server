FROM rust:alpine3.20 AS builder

WORKDIR /app

RUN : \
    && apk add --no-cache \
        musl-dev \
        pkgconfig \
        perl \
        make \
        libgcc \
        libstdc++ \
        musl \
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
    && cargo install sqlx-cli --no-default-features --features postgres \
    && mv /app/target/x86_64-unknown-linux-musl/release/burrito-server /app/burrito-server \
    && :

ENV SQLX_OFFLINE=false

CMD ["/app/burrito-server"]
