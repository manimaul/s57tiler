use actix_web::{error, Error, get, HttpResponse, post, Responder, web};
use diesel::{ExpressionMethods, RunQueryDsl};
use futures::StreamExt;
use serde_json::Value;

use crate::db;
use crate::errors::ErrMapper;
use crate::handlers::about::About;
use crate::handlers::style::PathParam;
use crate::schema;

mod about;
mod style;

#[get("/v1/about")]
pub async fn info() -> impl Responder {
    About::new()
}

const MAX_SIZE: usize = 262_144; // max payload size is 256k

///curl -v --header "Content-Type: application/json" \
//   --request POST \
//   --data '{"foo":"bar"}' \
//   http://localhost:8080/v1/style/foo
#[post("/v1/style/{name}")]
pub async fn post_style(
    mut payload: web::Payload,
    path_param: web::Path<PathParam>
) -> Result<HttpResponse, Error> {
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
            return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk);
    }
    info!("style name posted: {}", path_param.name);
    let new_style: Value = serde_json::from_slice(&body)?;
    let inserted_style: Value = db::db_conn().and_then(|conn| {
        diesel::insert_into(schema::styles::table)
            .values((
                schema::styles::name.eq(&path_param.name),
                schema::styles::style.eq(&new_style)
            ))
            .returning(schema::styles::style)
            .on_conflict(schema::styles::name)
            .do_update()
            .set(schema::styles::style.eq(&new_style))
            .get_result(&*conn)
            .map_internal_server_error("error getting db connection")
    })?;
    Ok(HttpResponse::Ok().json(inserted_style))
}
