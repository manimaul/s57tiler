FROM rust:1.48-buster

WORKDIR /workdir
RUN apt update && \
    apt install -y libgdal-dev

COPY . .

RUN cargo build --release

FROM debian:buster-slim

RUN apt update && \
    apt -y upgrade && \
    apt -y install --no-install-recommends libgdal20

RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=0 /workdir/target/release/s57_tiler /usr/bin/s57tiler
