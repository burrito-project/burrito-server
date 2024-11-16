FROM rust:alpine3.20

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

RUN : \
    && cargo install sqlx-cli --no-default-features --features postgres \
    && :

COPY . .

ENV SQLX_OFFLINE=true

RUN \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    : \
    && cargo build --target x86_64-unknown-linux-musl --release \
    && mv /app/target/x86_64-unknown-linux-musl/release/burrito-server /app/burrito-server \
    && :

ENV SQLX_OFFLINE=false

CMD ["/app/burrito-server"]
