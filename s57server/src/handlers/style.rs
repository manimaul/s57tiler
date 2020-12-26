use actix_web::{error, web};
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;
use futures::StreamExt;
use serde_json::Value;

use crate::{db, schema};
use crate::constants::MAX_POST_SIZE;
use crate::errors::{ErrMapper, Result};
use crate::schema::styles;

#[derive(Queryable, QueryableByName)]
#[table_name = "styles"]
pub struct Style {
    pub id: i64,
    pub name: String,
    pub style: Value,
}

#[derive(Deserialize)]
pub struct PathParam {
    pub name: String,
}

impl Style {

    pub fn query(name: &str) -> Result<Style> {
        db::db_conn().and_then(|conn|{
            styles::table.filter(styles::name.eq(name))
                .first::<Style>(&*conn)
                .map_not_found("style not found")
        })
    }

    pub async fn upsert(name: &String, mut payload: web::Payload) -> Result<Value> {
        let mut body = web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            // limit max size of in-memory payload
            if (body.len() + chunk.len()) > MAX_POST_SIZE {
                return Err(error::ErrorBadRequest("overflow"));
            }
            body.extend_from_slice(&chunk);
        }
        let new_style: Value = serde_json::from_slice(&body)?;
        db::db_conn().and_then(|conn| {
            diesel::insert_into(schema::styles::table)
                .values((
                    schema::styles::name.eq(&name),
                    schema::styles::style.eq(&new_style)
                ))
                .returning(schema::styles::style)
                .on_conflict(schema::styles::name)
                .do_update()
                .set(schema::styles::style.eq(&new_style))
                .get_result(&*conn)
                .map_internal_server_error("error getting db connection")
        })
    }
}
