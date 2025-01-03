FROM ubuntu:22.04

ENV TZ="Asia/Jakarta"

ARG USER=starrocks
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

ARG STARROCKS_VERSION
ARG PRIORITY_NETWORKS

RUN apt-get clean

RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y curl wget software-properties-common openjdk-11-jdk-headless ca-certificates mysql-client

ENV JAVA_HOME="/usr/lib/jvm/java-1.11.0-openjdk-amd64"
ENV STARROCKS_HOME="/opt/starrocks"

WORKDIR /opt

# RUN wget https://releases.starrocks.io/starrocks/StarRocks-3.3.7-ubuntu-amd64.tar.gz

RUN groupadd --gid ${USER_UID} ${USER} && \
    useradd --uid ${USER_UID} --gid $USER_GID -M --shell /usr/sbin/nologin ${USER} && \
    usermod -aG root ${USER}

COPY StarRocks-${STARROCKS_VERSION}-ubuntu-amd64.tar.gz .
COPY setup_db.py .

RUN tar -zxvf StarRocks-${STARROCKS_VERSION}-ubuntu-amd64.tar.gz && \
  mv StarRocks-${STARROCKS_VERSION}-ubuntu-amd64 starrocks && \
  rm StarRocks-${STARROCKS_VERSION}-ubuntu-amd64.tar.gz

RUN echo priority_networks = $PRIORITY_NETWORKS >> "/opt/starrocks/fe/conf/fe.conf" && \
  echo priority_networks = $PRIORITY_NETWORKS >> "/opt/starrocks/be/conf/be.conf" && \
  echo priority_networks = $PRIORITY_NETWORKS >> "/opt/starrocks/be/conf/cn.conf"


RUN apt-get install ca-certificates -y && \
    mkdir /etc/pki && \
    mkdir /etc/pki/tls && \
    mkdir /etc/pki/tls/certs && \
    cp /etc/ssl/certs/ca-certificates.crt /etc/pki/tls/certs/ca-bundle.crt

WORKDIR /opt/starrocks 

RUN echo python3 /opt/setup_db.py >> "setup_db.sh" && \ 
  echo "mysql -h fe-server -P 9030 -u root < sql_script.sql" >> "setup_db.sh" && \
  echo rm sql_script.sql >> "setup_db.sh"

RUN mkdir be/storage && \
    echo bash /opt/starrocks/be/bin/stop_be.sh --daemon > "be_server.sh" && \
    echo bash /opt/starrocks/be/bin/start_be.sh --daemon >> "be_server.sh" && \
    echo sleep 5 >> "be_server.sh" && \
    echo tail -f /opt/starrocks/be/log/be.INFO >> "be_server.sh" && \
    chmod a+x be_server.sh

RUN echo bash /opt/starrocks/be/bin/stop_cn.sh --daemon > "cn_server.sh" && \
    echo bash /opt/starrocks/be/bin/start_cn.sh --daemon >> "cn_server.sh" && \
    echo sleep 45 >> "cn_server.sh" && \
    echo tail -f /opt/starrocks/be/log/cn.INFO >> "cn_server.sh" && \
    chmod a+x cn_server.sh

RUN mkdir fe/meta  &&\
    echo bash /opt/starrocks/fe/bin/stop_fe.sh --daemon > "fe_server.sh" && \
    echo bash /opt/starrocks/fe/bin/start_fe.sh --daemon >> "fe_server.sh" && \
    echo sleep 5 >> "fe_server.sh" && \ 
    echo bash /opt/starrocks/setup_db.sh >> "fe_server.sh" && \
    echo sleep 10 >> "fe_server.sh" && \
    echo tail -f /opt/starrocks/fe/log/fe.log >> "fe_server.sh" && \
    chmod a+x fe_server.sh

RUN chown -R ${USER_UID}:${USER_GID} ${STARROCKS_HOME}

VOLUME [ "/opt/starrocks/fe" ]
VOLUME [ "/opt/starrocks/be" ]

USER root

EXPOSE 9030 8040 8030

RUN echo source ~/.bashrc >> "entrypoint.sh" && \
    chmod a+x entrypoint.sh

USER ${USER}
CMD ./entrypoint.sh
