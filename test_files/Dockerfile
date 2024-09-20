FROM bitnami/kubectl:latest
MAINTAINER ruddickmg

ENV ETCD_VERSION 3.5.15
USER root

RUN apt-get update

# install hexdump
RUN apt-get install bsdmainutils -y

# install etcd
ADD https://github.com/coreos/etcd/releases/download/v${ETCD_VERSION}/etcd-v${ETCD_VERSION}-linux-amd64.tar.gz /tmp/etcd.tar.gz
RUN tar zxvf /tmp/etcd.tar.gz -C /tmp && \
    mv /tmp/etcd-v${ETCD_VERSION}-linux-amd64/etcdctl /usr/local/bin/etcdctl && \
    chmod a+x /usr/local/bin/etcdctl

ENV PATH="/usr/local/bin:$PATH"

CMD ["etcdctl"]
