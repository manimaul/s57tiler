use actix_web::web::Json;
use diesel::prelude::*;

use crate::db;
use crate::errors::{ErrMapper, Result};
use crate::schema::charts;

#[derive(Debug, Deserialize, Serialize, PartialEq, Queryable, QueryableByName)]
#[table_name = "charts"]
pub struct Chart {
    pub id: i64,
    pub name: String,
    pub scale: i32,
    pub file_name: String,
    pub updated: String,
    pub issued: String,
}

impl Chart {
    pub fn query(id: i64) -> Result<Self> {
        db::db_conn().and_then(|conn| {
            charts::table
                .filter(charts::id.eq(&id))
                .first::<Chart>(&*conn)
                .map_not_found("chart not found")
        })
    }

    pub fn delete(id: i64) -> Result<Self> {
        db::db_conn().and_then(|conn| {
            charts::table
                .filter(charts::id.eq(&id))
                .first::<Chart>(&*conn).and_then(|record| {
                diesel::delete(charts::table
                    .filter(charts::id.eq(&id)))
                    .execute(&*conn).map(|_| record)
            }).map_not_found("chart not found")
        })
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Insertable)]
#[table_name = "charts"]
pub struct ChartInsert {
    pub name: String,
    pub scale: i32,
    pub file_name: String,
    pub updated: String,
    pub issued: String,
}

impl ChartInsert {
    pub fn insert(&self) -> Result<Json<Chart>> {
        db::db_conn().and_then(|conn| {
            diesel::insert_into(charts::table)
                .values(self)
                .get_result(&*conn)
                .map(|chart| Json(chart))
                .map_bad_request("error inserting chart")
        })
    }
}
