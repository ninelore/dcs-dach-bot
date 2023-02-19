FROM rust:latest AS builder

WORKDIR /usr/src/dcs-dach-bot
ADD . ./

RUN cargo build --release

FROM debian:stable-slim
WORKDIR /opt

COPY --from=builder /usr/src/dcs-dach-bot/target/release/dcs-dach-bot /opt/dcs-dach-bot

CMD ["/opt/dcs-dach-bot"]