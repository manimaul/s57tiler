FROM maptiler/tileserver-gl:latest

# This is just for the demo - don't do this - data should not be in the container image!
COPY ../data/fonts /data/fonts
COPY ../data/sprites /data/sprites
COPY ../data/styles /data/styles
COPY ../data/config.json /data/config.json
COPY ../data/config.json /data/config.json
COPY ../data/chart.mbtiles /data/chart.mbtiles
