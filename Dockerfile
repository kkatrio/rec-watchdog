FROM rust:alpine3.22

WORKDIR /usr/src/watch

COPY src/ src/
COPY Cargo.toml Cargo.toml

ENV TZ=Europe/Athens
RUN apk add --no-cache tzdata && \
    ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone

RUN cargo install --path .

CMD ["target/release/rec-watchdog"]
