FROM rust:alpine3.22

WORKDIR /usr/src/watch

COPY src/ src/
COPY Cargo.toml Cargo.toml

RUN cargo install --path .

CMD ["target/release/rec-watchdog"]
