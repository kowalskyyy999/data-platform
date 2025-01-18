FROM rust:1.80-slim-bullseye as builder 

RUN mkdir -p /app 

RUN apt-get update && \
    apt-get install -y make cmake libssl-dev libsasl2-dev libzstd-dev curl librdkafka-dev g++ ca-certificates pkg-config --fix-missing

WORKDIR /app

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim as final

RUN apt-get update
RUN apt-get install -y libssl-dev libsasl2-dev ca-certificates

COPY --from=builder /usr/local/cargo/bin/finnhub-connector /usr/local/bin/finnhub-connector
