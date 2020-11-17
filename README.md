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
s57tiler $HOME/Charts/ENC_ROOT/US5WA44M/US5WA44M.000 $HOME/Charts/Geojson/US5WA44M
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

### MacOS Dev Setup

```shell script
brew install gdal cmake tippecanoe
```

### Linux Setup

```shell script
#todo: (WK)
```

