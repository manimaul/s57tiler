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
docker run -v ${HOME}/Charts/:/charts -it s57t s57tiler -i /charts/ENC_ROOT/US5WA28M/US5WA28M.000 -o /charts/GEOJSON
# todo: (WK) read $HOME/Charts/Geojson/US5WA44M/meta.json with jq for layers list
pushd $HOME/Charts/Geojson/US5WA44M
tippecanoe -zg -o $HOME/Charts/MBTiles/US5WA44M.mbtiles \
           --coalesce-densest-as-needed --extend-zooms-if-still-dropping \
           ACHARE.json BOYSPP.json C_ASSO.json DEPARE.json FERYRT.json LNDMRK.json \
           M_NPUB.json M_SDAT.json PILPNT.json RIVERS.json SLCONS.json WATTUR.json \
           ADMARE.json BUAARE.json CBLARE.json DEPCNT.json LIGHTS.json LNDRGN.json \
           M_NSYS.json M_VDAT.json PRCARE.json SBDARE.json SOUNDG.json WEDKLP.json \
           BCNSPP.json BUISGL.json COALNE.json DMPGRD.json LNDARE.json MAGVAR.json \
           MORFAC.json OBSTRN.json RDOCAL.json SEAARE.json TWRTPT.json WRECKS.json \
           BOYLAT.json C_AGGR.json CTNARE.json DSID.json LNDELV.json M_COVR.json \
           M_QUAL.json OFSPLF.json RESARE.json SILTNK.json UWTROC.json
popd
docker run --rm -it -v $HOME/Charts/MBTiles:/data -p 8080:80 maptiler/tileserver-gl
docker run -it --rm -p 8888:8888 maputnik/editor
```

You could use ogr2ogr to generate the geojson. However, we need to extract the z coordinate out of the SOUNDG layer.
```shell script
ogrinfo $HOME/Charts/ENC_ROOT/US3WA01M/US3WA01M.000 | cut -f2 -d ' '
ogr2ogr -t_srs 'EPSG:4326' -f GeoJSON $HOME/source/madrona/s57_tiler/geojson_out/ADMARE.json $HOME/Charts/ENC_ROOT/US3WA01M/US3WA01M.000 ADMARE
```

###  Dev Setup

MacOS
```shell script
brew install gdal
```

Linux
```shell script
apt install -y libgdal-dev
```

[rustup](https://rustup.rs/)

