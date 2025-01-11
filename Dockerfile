FROM rust:1.82

WORKDIR /usr/src/app

COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY .env .env

RUN cargo install --path .

CMD ["target/release/rec-watchdog"]
