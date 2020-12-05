#!/usr/bin/env sh

cargo run -- mbtiles -i "$(pwd)"/data/charts/US5WA22M/US5WA22M.000 -o "$(pwd)"/data -n LNDARE,DEPARE,SEAARE,SLCONS,PONTON,HULKES,SOUNDG,BOYSPP
cargo run -- config -o "$(pwd)"/data -s localhost:8080,127.0.0.1:8080
cargo run -- style -o "$(pwd)"/data/styles -s 127.0.0.1:8080
docker run --rm -v "$(pwd)"/data:/data -p 8080:80 maptiler/tileserver-gl