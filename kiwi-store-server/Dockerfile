FROM rust:slim-bookworm AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/kiwi-store-server /usr/local/bin/kiwi-store-server
COPY .env .env

EXPOSE 5555

CMD ["kiwi-store-server"]
