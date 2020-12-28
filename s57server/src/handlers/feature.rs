use diesel::{Connection, PgConnection, RunQueryDsl};
use diesel::result::Error;
use diesel::sql_types::{BigInt, Text};
use geojson::GeoJson;
use serde_json::{Map, Value};

use crate::{db, errors};
use crate::errors::ErrMapper;
use crate::handlers::GeoParams;

pub type JsonObject = Map<String, Value>;

pub fn query(params: &GeoParams) -> errors::Result<geojson::FeatureCollection> {
    db::db_conn2().and_then(|mut conn| {
        conn.query("
            SELECT
                row_to_json(f)::JSON AS feature
            FROM (
             SELECT
                 id AS id,
                 layer AS layer,
                 'Feature' AS type,
                 ST_AsGeoJSON(geom)::JSON AS geometry,
                 props AS properties
             FROM features
             WHERE layer = $1 AND chart_id = $2
            ) f;
        ", &[&params.name, &params.chart_id])
            .map_internal_server_error("geojson query failed")
    }).map(|rows| rows.iter()
        .filter(|row| row.len() > 0)
        .map(|row| {
            let geo_json: serde_json::Value = row.get(0);
            geojson::Feature::from_json_value(geo_json).expect("could not create geojson feature")
        }).collect()
    ).map(|features| {
        geojson::FeatureCollection {
            bbox: None,
            features,
            foreign_members: None
        }
    })
}

pub struct FeatureInsert {
    pub params: GeoParams,
    pub geo: GeoJson,
}

impl FeatureInsert {
    fn insert_geom(
        conn: &PgConnection,
        params: &GeoParams,
        geom: &geojson::Geometry,
        props: &Option<JsonObject>,
    ) -> Result<(), Error> {
        let p = match props {
            None => String::from("{}"),
            Some(v) => Value::Object(v.clone()).to_string()
        };
        diesel::sql_query("
            INSERT INTO features (layer, geom, props, chart_id)
                VALUES (
                    $1,
                    ST_SetSRID(ST_GeomFromGeoJSON($2), 4326),
                    $3::TEXT::JSON,
                    $4
                );
        ").bind::<Text, _>(&params.name)
            .bind::<Text, _>(geom.to_string())
            .bind::<Text, _>(p)
            .bind::<BigInt, _>(params.chart_id)
            .execute(conn).map(|_| ()).map_err(|_| Error::RollbackTransaction)
    }

    fn insert_feature(
        conn: &PgConnection,
        params: &GeoParams,
        feat: &geojson::Feature,
    ) -> Result<(), Error> {
        if let Some(geom) = &feat.geometry {
            Self::insert_geom(conn, params, geom, &feat.properties)
        } else {
            warn!("feature without a geometry {}", params.name);
            Ok(())
        }
    }

    fn insert_feature_c(
        conn: &PgConnection,
        params: &GeoParams,
        fc: &geojson::FeatureCollection,
    ) -> Result<(), Error> {
        for feat in &fc.features {
            if let Err(_) = Self::insert_feature(conn, params, &feat) {
                return Err(Error::RollbackTransaction);
            }
        }
        Ok(())
    }

    pub fn insert(&self) -> errors::Result<()> {
        db::db_conn().map_err(|_| Error::RollbackTransaction)
            .and_then(|conn| {
                conn.transaction::<(), _, _>(|| {
                    match &self.geo {
                        GeoJson::Geometry(g) => Self::insert_geom(&*conn, &self.params, g, &None),
                        GeoJson::Feature(f) => Self::insert_feature(&*conn, &self.params, f),
                        GeoJson::FeatureCollection(fc) => Self::insert_feature_c(&*conn, &self.params, fc)
                    }
                })
            }).map_internal_server_error("transaction failed")
    }
}

