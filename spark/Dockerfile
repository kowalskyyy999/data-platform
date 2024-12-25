FROM ubuntu:24.04

ENV PYTHONUNBUFFERED=1

ENV TZ="America/New_York"

RUN apt-get update && \
    apt-get upgrade -y && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y curl wget software-properties-common python3.12 python3-pip python3-venv

RUN pip3 install jupyter notebook --break-system-packages 
RUN apt-get update && \
    apt-get upgrade -y

RUN apt-get install -y openjdk-8-jdk-headless

ENV JAVA_HOME="/usr/lib/jvm/java-1.8.0-openjdk-amd64"

WORKDIR /opt 

RUN wget https://archive.apache.org/dist/spark/spark-3.5.2/spark-3.5.2-bin-hadoop3.tgz && \
  tar -zxvf spark-3.5.2-bin-hadoop3.tgz && \
  mv spark-3.5.2-bin-hadoop3 spark && \
  rm spark-3.5.2-bin-hadoop3.tgz && \
  mkdir spark/logs

RUN wget https://repo1.maven.org/maven2/org/apache/iceberg/iceberg-aws-bundle/1.5.0/iceberg-aws-bundle-1.5.0.jar -O spark/jars/iceberg-aws-bundle-1.5.0.jar && \
  wget https://repo1.maven.org/maven2/org/apache/iceberg/iceberg-spark-runtime-3.5_2.12/1.5.0/iceberg-spark-runtime-3.5_2.12-1.5.0.jar -O spark/jars/iceberg-spark-runtime-3.5_2.12-1.5.0.jar && \
  wget https://repo1.maven.org/maven2/org/postgresql/postgresql/42.7.3/postgresql-42.7.3.jar -O spark/jars/postgresql-42.7.3.jar && \
  wget https://repo1.maven.org/maven2/org/apache/hadoop/hadoop-aws/3.3.4/hadoop-aws-3.3.4.jar -O spark/jars/hadoop-aws-3.3.4.jar && \
  wget https://repo1.maven.org/maven2/com/amazonaws/aws-java-sdk-bundle/1.12.262/aws-java-sdk-bundle-1.12.262.jar -O spark/jars/aws-java-sdk-bundle-1.12.262.jar && \
  wget https://repo1.maven.org/maven2/org/apache/hadoop/hadoop-client/3.3.4/hadoop-client-3.3.4.jar -O spark/jars/hadoop-client-3.3.4.jar

RUN cp /opt/spark/conf/log4j2.properties.template /opt/spark/conf/log4j2.properties

ARG USER=spark
ARG USER_UID=1100

ARG USER_GID=${USER_UID}

RUN groupadd --gid ${USER_UID} ${USER} && \
    useradd --uid ${USER_UID} --gid $USER_GID -M --shell /usr/sbin/nologin ${USER} && \
    usermod -aG root ${USER}

ENV SPARK_HOME=/opt/spark
ENV SPARK_CONF="${SPARK_HOME}/conf"
ENV PATH="$PATH:${SPARK_HOME}/bin:${SPARK_HOME}/sbin"
ENV SPARK_LOG_DIR="${SPARK_HOME}/logs"
ENV SPARK_PID_DIR="${SPARK_HOME}/tmp"
ENV PYSPARK_PYTHON=/usr/bin/python3
ENV SPARK_MASTER_PORT=7077
ENV MODE=master
ENV PYSPARK_DRIVER_PYTHON="jupyter"
ENV PYSPARK_DRIVER_PYTHON_OPTS="notebook --ip=0.0.0.0 --no-browser --port=8989 --allow-root --NotebookApp.token=''"

ENV CATALOG_NAME=demo

ENV S3_WAREHOUSE=s3://warehouse 
ENV S3_HOST=minio
ENV S3_PORT=9000
ENV S3_PROTOCOL=http
ENV S3_ENDPOINT="${S3_PROTOCOL}://${S3_HOST}:${S3_PORT}"

ENV REST_HOST=rest
ENV REST_PORT=8181
ENV REST_PROTOCOL=http
ENV REST_URI="${REST_PROTOCOL}://${REST_HOST}:${REST_PORT}"

ENV CATALOG_NAME_REST=demo0
ENV CATALOG_NAME_JDBC=demo2
ENV DEFAULT_CATALOG_NAME="${CATALOG_NAME_REST}"

ENV JDBC_HOST=postgres
ENV JDBC_PORT=5432
ENV JDBC_DB=iceberg_metadata
ENV JDBC_USER=postgres
ENV JDBC_PASS=postgres

ENV AWS_JAVA_V1_DISABLE_DEPRECATION_ANNOUNCEMENT true

RUN echo '#!/bin/sh' > /opt/spark/config.sh && \
  echo 'set -x' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.extensions org.apache.iceberg.spark.extensions.IcebergSparkSessionExtensions > /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC} org.apache.iceberg.spark.SparkCatalog >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.type jdbc >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.uri jdbc:postgresql://${JDBC_HOST}:${JDBC_PORT}/${JDBC_DB} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.jdbc.user ${JDBC_USER} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.jdbc.password ${JDBC_PASS} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.warehouse ${S3_WAREHOUSE} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.s3.endpoint ${S3_ENDPOINT} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.jdbc.schema-version V1 >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_JDBC}.io-impl org.apache.iceberg.aws.s3.S3FileIO >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_REST} org.apache.iceberg.spark.SparkCatalog >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_REST}.catalog-impl org.apache.iceberg.rest.RESTCatalog >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \ 
  echo 'echo spark.sql.catalog.${CATALOG_NAME_REST}.uri ${REST_URI} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_REST}.io-impl org.apache.iceberg.aws.s3.S3FileIO >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_REST}.warehouse ${S3_WAREHOUSE} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalog.${CATALOG_NAME_REST}.s3.endpoint ${S3_ENDPOINT} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.defaultCatalog ${DEFAULT_CATALOG_NAME} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.eventLog.dir /opt/spark/events/spark-events >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.history.fs.logDirectory /opt/spark/events/spark-events >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.sql.catalogImplementation in-memory >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.hadoop.fs.s3a.access.key ${AWS_ACCESS_KEY_ID} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.hadoop.fs.s3a.secret.key ${AWS_SECRET_ACCESS_KEY} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.hadoop.fs.s3a.endpoint ${S3_HDFS_ENDPOINT} >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.hadoop.fs.s3a.path.style.access true >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.hadoop.fs.s3a.impl org.apache.hadoop.fs.s3a.S3AFileSystem >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'echo spark.hadoop.fs.s3a.connection.ssl.enabled false >> /opt/spark/conf/spark-defaults.conf' >> /opt/spark/config.sh && \
  echo 'mkdir /opt/spark/events' >> /opt/spark/config.sh && \
  echo 'mkdir /opt/spark/events/spark-events' >> /opt/spark/config.sh && \
  chmod a+x /opt/spark/config.sh

RUN echo '#!/bin/sh' > /opt/spark/entrypoint.sh && \
  echo 'set -x' >> /opt/spark/entrypoint.sh && \
  echo '/opt/spark/config.sh' >> /opt/spark/entrypoint.sh && \
  echo 'if [ $MODE = "master" ]; then ${SPARK_HOME}/bin/spark-class org.apache.spark.deploy.master.Master >> ${SPARK_HOME}/logs/spark-master.out' >> /opt/spark/entrypoint.sh && \
  echo 'elif [ $MODE = "worker" ]; then ${SPARK_HOME}/bin/spark-class org.apache.spark.deploy.worker.Worker spark://${SPARK_MASTER_HOST}:${SPARK_MASTER_PORT} >> ${SPARK_HOME}/logs/spark-worker.out' >> /opt/spark/entrypoint.sh && \
  echo 'elif [ $MODE = "notebook" ]; then ${SPARK_HOME}/bin/pyspark --master spark://${SPARK_MASTER_HOST}:${SPARK_MASTER_PORT}' >> /opt/spark/entrypoint.sh && \
  echo 'else echo "Mode not set !!!"' >> /opt/spark/entrypoint.sh && \
  echo 'fi' >> /opt/spark/entrypoint.sh && \
  chmod a+x /opt/spark/entrypoint.sh

RUN chown -R ${USER_UID}:${USER_GID} ${SPARK_HOME}

RUN mkdir /home/playground

RUN  rm -rf /var/lib/apt/lists/*

WORKDIR /home/playground

EXPOSE 7077 8080 4040 8081 8989 ${SPARK_MASTER_PORT}

ENTRYPOINT ["/opt/spark/entrypoint.sh"]
