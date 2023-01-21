FROM rust:latest AS builder

WORKDIR /usr/src/dcs-dach-bot
COPY . .

RUN cargo install --path .

CMD ["cargo", "run", "--release"]
