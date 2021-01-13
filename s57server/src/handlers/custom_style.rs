use actix_web::{error, web};
use diesel::{ExpressionMethods, RunQueryDsl};
use diesel::prelude::*;
use futures::StreamExt;
use serde_json::Value;

use crate::{db, schema};
use crate::errors::{ErrMapper, Result};
use crate::schema::styles;

#[derive(Queryable, QueryableByName)]
#[table_name = "styles"]
pub struct CustomStyle {
    pub id: i64,
    pub name: String,
    pub style: Value,
}

#[derive(Deserialize)]
pub struct PathParam {
    pub name: String,
}

impl CustomStyle {

    pub fn query(name: &str) -> Result<CustomStyle> {
        db::db_conn().and_then(|conn|{
            styles::table.filter(styles::name.eq(name))
                .first::<CustomStyle>(&*conn)
                .map_not_found("style not found")
        })
    }

    pub async fn upsert(name: &String, mut payload: web::Payload) -> Result<Value> {
        let mut body = web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            if (body.len() + chunk.len()) > 262_144 { //256k
                return Err(error::ErrorPayloadTooLarge("slow your roll pal"));
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
