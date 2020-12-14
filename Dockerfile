FROM debian:stretch-slim
MAINTAINER "Filip Bielejec" <filip@clashapp.co>

RUN apt-get update && apt-get install -y \
    libssl-dev ca-certificates \
    && rm -rf /tmp/* /var/{tmp,cache}/* /var/lib/{apt,dpkg}/

WORKDIR nagger

COPY target/release/nagger /nagger/nagger

ENTRYPOINT ["./nagger"]
