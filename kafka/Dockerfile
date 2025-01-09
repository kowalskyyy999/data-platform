FROM ubuntu:22.04

RUN apt-get clean

ENV TZ="Asia/jakarta"

RUN apt-get update && \
    apt-get upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y curl wget software-properties-common

RUN apt-get install -y openjdk-8-jdk-headless

ENV JAVA_HOME="/usr/lib/jvm/java-1.8.0-openjdk-amd64"

RUN wget https://archive.apache.org/dist/kafka/3.8.0/kafka_2.12-3.8.0.tgz

RUN tar -zxvf kafka_2.12-3.8.0.tgz && \
    mv kafka_2.12-3.8.0 /usr/local/etc/kafka && \
    mkdir -p /usr/local/etc/kafka/logs && \
    mkdir -p /usr/local/etc/kafka/plugins && \
    rm kafka_2.12-3.8.0.tgz

ENV KAFKA_HOME=/usr/local/etc/kafka
ENV PATH="$PATH:${KAFKA_HOME}/bin"

COPY server.properties $KAFKA_HOME/config/server.properties

WORKDIR ${KAFKA_HOME}

CMD $KAFKA_HOME/bin/kafka-server-start.sh $KAFKA_HOME/config/server.properties
