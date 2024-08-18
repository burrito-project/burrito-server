FROM rust:alpine3.20

WORKDIR /app

ENV UID=1000
ENV GID=1000

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

COPY Cargo.toml Cargo.lock ./

RUN : \
    && mkdir -p src \
    && echo 'fn main() {}' > src/main.rs \
    && cargo build --target x86_64-unknown-linux-musl \
    && rm -rf src \
    && :

CMD ["/bin/sh"]
