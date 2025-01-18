FROM spark:3.5.2

ENV PYSPARK_PYTHON=/usr/bin/python3
ENV PYSPARK_DRIVER_PYTHON="python3"
ENV PYSPARK_DRIVER_PYTHON_OPTS=""

WORKDIR /opt/spark

RUN mkdir apps

COPY apps/* apps/
