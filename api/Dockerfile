FROM rust:latest

RUN mkdir /home/app
COPY . /home/app

WORKDIR /home/app

RUN apt update \
    && apt install build-essential \
    && apt install -y musl-tools musl-dev default-mysql-client

RUN rustup default nightly \
    && cargo install diesel_cli --no-default-features --features "mysql"

RUN cargo install cargo-watch

RUN diesel setup
RUN diesel migration run

COPY src/schema.rs /home/app/src/schema.rs

CMD ["cargo", "watch", "-x", "run"]
