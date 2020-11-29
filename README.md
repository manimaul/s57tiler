# S57 Chart to GeoJson & Mapbox Vector Tile (MVT) Renderer

The goal of this project is to render geojson and json meta data from S57 nautical chart files suitable for input to 
tippecanoe which transforms the data into MVT / MBTiles.

We will then serve the data with [tileserver-gl](https://github.com/maptiler/tileserver-gl) and work on styling the chart
with [maputnik](https://github.com/maputnik/editor/wiki).

Note: This project is still a WIP. I plan additionally creating an open source S57 theme based on 
[OpenCPN data](https://raw.githubusercontent.com/OpenCPN/OpenCPN/master/data/s57data/chartsymbols.xml). The theme will
be published in a TDB Github repo.

### Example

```shell script
docker build -t s57t .
docker run -v $(pwd)/data:/data s57t s57tiler -i /data/charts/US5WA22M/US5WA22M.000 -o /data
docker run --rm -v $(pwd)/data:/data -p 8080:80 maptiler/tileserver-gl
docker run -it --rm -p 8888:8888 maputnik/editor
```

### Screenshots
[US5WA22M](https://charts.noaa.gov/ENCs/ENCsIndv.shtml) rendered with [tileserver-gl](https://github.com/maptiler/tileserver-gl)

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

