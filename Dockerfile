FROM debian:stretch-slim
MAINTAINER "Filip Bielejec" <filip@clashapp.co>

WORKDIR nagger

COPY target/release/nagger /nagger/nagger

ENTRYPOINT ["./nagger"]

