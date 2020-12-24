use crate::schema::styles;
use serde_json::Value;
use actix_web::{web, error};
use crate::constants::MAX_POST_SIZE;
use futures::StreamExt;
use crate::{db, schema};
use diesel::{ExpressionMethods, RunQueryDsl};
use crate::errors::{ErrMapper, Result};

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
