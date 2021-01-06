FROM maptiler/tileserver-gl:latest

# This is just for the demo - don't do this - data should not be in the container image!
COPY ../s57server/web/fonts /data/fonts
COPY ../data/sprites /data/sprites
COPY ../data/web/styles /data/styles
COPY ../data/config.json /data/config.json
COPY ../data/config.json /data/config.json
COPY ../data/chart.mbtiles /data/chart.mbtiles
