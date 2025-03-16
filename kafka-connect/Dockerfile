FROM kafka:3.8.0

ENV TZ="Asia/Jakarta"

RUN apt-get install unzip iputils-ping -y

COPY connect-distributed.properties $KAFKA_HOME/config/connect-distributed.properties

WORKDIR $KAFKA_HOME/plugins

RUN wget https://github.com/databricks/iceberg-kafka-connect/releases/download/v0.6.19/iceberg-kafka-connect-runtime-0.6.19.zip && \
    unzip iceberg-kafka-connect-runtime-0.6.19.zip && \
    mv iceberg-kafka-connect-runtime-0.6.19 iceberg-kafka-connect-runtime

WORKDIR $KAFKA_HOME

RUN curl https://repo1.maven.org/maven2/software/amazon/awssdk/bundle/2.20.18/bundle-2.20.18.jar -o ${KAFKA_HOME}/bundle-2.20.18.jar && \
    cp bundle-2.20.18.jar libs/ && \
    mv bundle-2.20.18.jar plugins/iceberg-kafka-connect-runtime/lib/

EXPOSE 8084

CMD $KAFKA_HOME/bin/connect-distributed.sh $KAFKA_HOME/config/connect-distributed.properties
