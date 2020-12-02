# S57 Chart to GeoJson & Mapbox Vector Tile (MVT) Renderer

The goal of this project is to render geojson and json meta data from S57 nautical chart files suitable for input to 
tippecanoe which transforms the data into MVT / MBTiles.

We will then serve the data with [tileserver-gl](https://github.com/maptiler/tileserver-gl) and work on styling the chart
with [maputnik](https://github.com/maputnik/editor/wiki).

Note: This project is still very much a *WORK IN PROGRESS*.

### Example

```shell script
cargo run -- -i $(pwd)/data/charts/US5WA22M/US5WA22M.000 -o $(pwd)/data -n LNDARE,DEPARE,SEAARE,SLCONS,PONTON,HULKES,SOUNDG,BOYSPP
#OR
docker build -t s57t .
docker run -v $(pwd)/data:/data s57t s57tiler -i /data/charts/US5WA22M/US5WA22M.000 -o /data -n LNDARE,DEPARE,SEAARE,SLCONS,PONTON,HULKES,SOUNDG,BOYSPP

docker run --rm -v $(pwd)/data:/data -p 8080:80 maptiler/tileserver-gl
docker run -it --rm -p 8888:8888 maputnik/editor
```

### Screenshots
[US5WA22M](https://charts.noaa.gov/ENCs/ENCsIndv.shtml) rendered with [tileserver-gl](https://github.com/maptiler/tileserver-gl)

You can find a live demo here: [https://s57dev.mxmariner.com/styles/day_bright_style/#14/47.27888/-122.41757](https://s57dev.mxmariner.com/styles/day_bright_style/#14/47.27888/-122.41757) 

|Un-styled|Styled (WIP)|
|---------|------------|
|![Screenshot](./screenshots/US5WA22M_data.png)|![Screenshot](./screenshots/US5WA22M.png)|
 

###  Dev Setup

MacOS
```shell script
brew install gdal tippecanoe
```

Linux
```shell script
apt install -y libgdal-dev
git clone https://github.com/mapbox/tippecanoe.git && \
    cd tippecanoe && \
    make
sudo cp /tippecanoe/tippecanoe /usr/local/bin/tippecanoe
sudo cp /tippecanoe/tile-join /usr/local/bin/tile-join
sudo cp /tippecanoe/tippecanoe-decode /usr/local/bin/tippecanoe-decode
sudo cp /tippecanoe/tippecanoe-enumerate /usr/local/bin/tippecanoe-enumerate
sudo cp /tippecanoe/tippecanoe-json-tool /usr/local/bin/tippecanoe-json-tool
```

[rustup](https://rustup.rs/)

