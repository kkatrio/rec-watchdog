FROM rust:1.82

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

CMD ["target/release/rec-watchdog"]
