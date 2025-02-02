FROM rust:1.81-slim-bullseye as builder 

RUN mkdir -p /apps 

RUN apt-get update && \
    apt-get install -y make cmake libssl-dev libsasl2-dev libzstd-dev curl librdkafka-dev g++ ca-certificates pkg-config --fix-missing

WORKDIR /apps

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim as final

RUN apt-get update
RUN apt-get install -y libssl-dev libsasl2-dev ca-certificates

COPY --from=builder /usr/local/cargo/bin/sheet-connector /usr/local/bin/sheet-connector

COPY credentials.json .

ENV APPEND=false

RUN echo '#!/bin/bash' > entrypoint.sh && \
  echo 'if [ $APPEND = true ]; then /usr/local/bin/sheet-connector --sheet-id ${SHEET_ID} --table-name ${TABLE_NAME} --append' >> entrypoint.sh && \
  echo 'else /usr/local/bin/sheet-connector --sheet-id ${SHEET_ID} --table-name ${TABLE_NAME}' >> entrypoint.sh && \
  echo 'fi' >> entrypoint.sh && \
  chmod a+x entrypoint.sh 

ENTRYPOINT ["./entrypoint.sh"]
