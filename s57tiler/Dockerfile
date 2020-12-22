FROM rust:1.48-buster

WORKDIR /workdir
RUN apt update && \
    apt install -y libgdal-dev git build-essential

COPY . .

RUN cargo build --release

RUN cd / && \
    git clone https://github.com/mapbox/tippecanoe.git && \
    cd tippecanoe && \
    make

FROM debian:buster-slim

RUN apt update && \
    apt -y upgrade && \
    apt -y install --no-install-recommends libgdal20

RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=0 /workdir/target/release/s57tiler /usr/bin/s57tiler
COPY --from=0 /tippecanoe/tippecanoe /usr/bin/tippecanoe
COPY --from=0 /tippecanoe/tile-join /usr/bin/tile-join
COPY --from=0 /tippecanoe/tippecanoe-decode /usr/bin/tippecanoe-decode
COPY --from=0 /tippecanoe/tippecanoe-enumerate /usr/bin/tippecanoe-enumerate
COPY --from=0 /tippecanoe/tippecanoe-json-tool /usr/bin/tippecanoe-json-tool
