FROM debian:buster

WORKDIR /workdir
RUN apt update && \
    apt install -y libgdal-dev python3-pip build-essential

RUN pip3 install cmake --upgrade

COPY . .

RUN mkdir cmake-build-docker && \
    cd cmake-build-docker && \
    /usr/local/bin/cmake .. && \
    make

FROM debian:buster-slim

RUN apt update && \
    apt -y upgrade && \
    apt -y install --no-install-recommends libgdal20

RUN apt-get clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=0 /workdir/cmake-build-docker/s57tiler /usr/bin/s57tiler
