

You could use ogr2ogr to generate the geojson. However, we need to extract the z coordinate out of the SOUNDG layer.
```shell script
ogrinfo $HOME/Charts/ENC_ROOT/US3WA01M/US3WA01M.000 | cut -f2 -d ' '
ogr2ogr -t_srs 'EPSG:4326' -f GeoJSON $HOME/source/madrona/s57_tiler/geojson_out/ADMARE.json $HOME/Charts/ENC_ROOT/US3WA01M/US3WA01M.000 ADMARE
```
