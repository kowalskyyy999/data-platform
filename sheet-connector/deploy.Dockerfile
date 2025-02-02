FROM rust:1.81-slim-bullseye as builder 

RUN mkdir -p /apps 

RUN apt-get update && \
    apt-get install -y make cmake libssl-dev libsasl2-dev libzstd-dev curl librdkafka-dev g++ ca-certificates pkg-config --fix-missing

WORKDIR /apps

COPY . .

RUN cargo install --path .

FROM python:3.12-slim as deployment

RUN apt-get update
RUN apt-get install -y libssl-dev libsasl2-dev ca-certificates wget iputils-ping curl && \
  wget http://nz2.archive.ubuntu.com/ubuntu/pool/main/o/openssl/libssl1.1_1.1.1f-1ubuntu2.23_amd64.deb && \
  dpkg -i libssl1.1_1.1.1f-1ubuntu2.23_amd64.deb

COPY --from=builder /usr/local/cargo/bin/sheet-connector /usr/local/bin/sheet-connector

WORKDIR /app

COPY credentials.json .

ENV APPEND=false
ENV PREFECT_API_URL="http://localhost:4200/api"

RUN pip install prefect prefect-shell

RUN prefect config set PREFECT_API_URL=${PREFECT_API_URL}

COPY deploy.py .

CMD ["python", "deploy.py"]
# CMD sleep 3600
