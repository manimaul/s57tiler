

You could use ogr2ogr to generate the geojson. However, we need to extract the z coordinate out of the SOUNDG layer.
```shell script
ogrinfo $HOME/Charts/ENC_ROOT/US3WA01M/US3WA01M.000 | cut -f2 -d ' '
ogr2ogr -t_srs 'EPSG:4326' -f GeoJSON $(pwd)/data/ogr_DEPARE.json $(pwd)/data/charts//US5WA22M/US5WA22M.000 DEPARE
```
