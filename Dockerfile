FROM ubuntu:18.04
RUN \
  apt-get update -y \
  && apt-get upgrade -y \
  && apt-get install curl build-essential -y
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /usr/src/myapp


