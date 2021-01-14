#!/usr/bin/env bash

DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"

function setup_diesel() {
  pushd "${DIR}/s57server"
  if [ -f .env ]
  then
    export $(cat "${DIR}/s57server/.env" | sed 's/#.*//g' | xargs)
  fi
  db_url="postgres://${DB_USER}:${DB_PASS}@${DB_HOST}/${DB_NAME}"
  diesel --database-url ${db_url} migration run
  popd
}

function join_by {
  local d=$1; shift; local f=$1; shift; printf %s "$f" "${@/#/$d}";
}

function sample_chart_geojson() {
  layers=( LNDARE DEPARE DEPCNT SEAARE SLCONS PONTON HULKES SOUNDG BOYSPP LIGHTS )
  cargo run --package s57tiler --bin s57tiler -- mbtiles -g \
  -i "${DIR}"/data/charts/US5WA22M/US5WA22M.000 \
  -o "${DIR}"/data \
  -n $(join_by , "${layers[@]}")

  for i in "${layers[@]}"
  do
    curl -H "Content-Type: application/json" \
         --request POST --data-binary "@data/$i.json" \
         "http://localhost:8081/v1/geojson?chart_id=9&name=$i" > /dev/null
  done
}

#sample_chart_geojson

# todo delete tileserver configs
# todo https://github.com/mapbox/tilejson-spec/blob/master/2.2.0/schema.json
# todo https://github.com/mapbox/tilejson-spec/tree/master/2.2.0"