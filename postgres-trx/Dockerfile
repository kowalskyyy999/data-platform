# refer to https://github.com/debezium/container-images/blob/main/postgres/15/Dockerfile
FROM postgres:15.3 AS build

ENV PLUGIN_VERSION=v3.1.0.Alpha1
ENV PROTOC_VERSION=1.5
ENV PG_MAJOR=15

# Install the packages which will be required to get everything to compile
RUN apt-get update \
    && apt-get install -f -y --no-install-recommends \
        software-properties-common \
        build-essential \
        pkg-config \
        git \
        clang \
        llvm \
        postgresql-server-dev-$PG_MAJOR \ 
    && add-apt-repository "deb http://ftp.debian.org/debian testing main contrib" \
    && apt-get update && apt-get install -f -y --no-install-recommends \
        libprotobuf-c-dev=$PROTOC_VERSION.* \
    && rm -rf /var/lib/apt/lists/*

ENV PATH=/usr/lib/postgresql/15.3/bin:$PATH
# Compile the plugin from sources and install it
RUN git clone https://github.com/debezium/postgres-decoderbufs.git \
    && cd /postgres-decoderbufs \
    && make && make install \
    && cd / \
    && rm -rf postgres-decoderbufs

FROM postgres:15.3 

RUN apt-get update \
    && apt-get install -f -y --no-install-recommends \
        software-properties-common \
    && add-apt-repository "deb http://ftp.debian.org/debian testing main contrib" \
    && apt-get update && apt-get install -f -y --no-install-recommends \
        libprotobuf-c1 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=build /usr/lib/postgresql/$PG_MAJOR/lib/decoderbufs.so /usr/lib/postgresql/$PG_MAJOR/lib/
COPY --from=build /usr/share/postgresql/$PG_MAJOR/extension/decoderbufs.control /usr/share/postgresql/$PG_MAJOR/extension/
# Copy the custom configuration which will be passed down to the server (using a .sample file is the preferred way of doing it by
# the base Docker image)
COPY postgresql.conf /usr/share/postgresql/postgresql.conf.sample

# Copy the script which will initialize the replication permissions
COPY /docker-entrypoint-initdb.d /docker-entrypoint-initdb.d
