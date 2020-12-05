

You could use ogr2ogr to generate the geojson. However, we need to extract the z coordinate out of the SOUNDG layer.
```shell script
ogrinfo $(pwd)/data/charts/US5WA22M/US5WA22M.000 | cut -f2 -d ' '
export OGR_S57_OPTIONS="SPLIT_MULTIPOINT:ON,ADD_SOUNDG_DEPTH=ON,UPDATES=APPLY,LIST_AS_STRING=OFF"
ogr2ogr -t_srs 'EPSG:4326' -f GeoJSON $(pwd)/data/ogr_SOUNDG.json $(pwd)/data/charts/US5WA22M/US5WA22M.000 SOUNDG
```
