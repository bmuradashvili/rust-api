FROM rust:latest

RUN mkdir /home/app
COPY . /home/app

WORKDIR /home/app

RUN cp src/schema.rs src/schema.backup

RUN apt update \
    && apt install build-essential \
    && apt install -y musl-tools musl-dev default-mysql-client

RUN rustup default nightly \
    && cargo install diesel_cli --no-default-features --features "mysql"

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
