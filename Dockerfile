FROM rust:alpine3.20 AS builder

WORKDIR /app

RUN : \
    && apk add --no-cache \
        musl-dev \
    && :

COPY Cargo.toml Cargo.lock ./

RUN : \
    && mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl --release \
    && rm -rf src \
    && :

COPY . .

RUN : \
    && touch -a -m ./src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.20.2

WORKDIR /app

RUN apk add --no-cache \
libgcc \
libstdc++ \
musl

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/burrito-server /app/

ENV ROCKET_ENV=production
ENV ROCKET_PORT=6969
ENV ROCKET_ADDRESS=0.0.0.0

CMD ["/app/burrito-server"]
