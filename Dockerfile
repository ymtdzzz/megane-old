FROM rust:1.46

RUN apt-get update \
  && apt-get install -y lcov 

RUN rustup install nightly \
  && cargo install grcov
