use actix_web::{get, HttpResponse, post, Responder, web};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::db;
use crate::errors::ErrMapper;
use crate::handlers::about::About;
use crate::handlers::style::{PathParam, Style};
use crate::schema;

mod about;
mod style;

///curl http://localhost:8080/v1/about | jq
#[get("/v1/about")]
pub async fn info() -> impl Responder {
    About::new()
}

///curl http://localhost:8080/v1/style/foo | jq
#[get("/v1/style/{name}")]
pub async fn get_style(path_param: web::Path<PathParam>) -> impl Responder {
    db::db_conn().and_then(|conn|{
        schema::styles::table.filter(schema::styles::name.eq(&path_param.name))
            .first::<Style>(&*conn)
            .map_not_found(&format!("style {} not found", &path_param.name))
    }).map(|style_record: Style| {
        HttpResponse::Ok().json(style_record.style)
    })
}

///curl --request POST  --data '{"foo":"bar"}' http://localhost:8080/v1/style/foo | jq
#[post("/v1/style/{name}")]
pub async fn post_style(
    payload: web::Payload,
    path_param: web::Path<PathParam>
) -> impl Responder {
    info!("style name posted: {}", &path_param.name);
    Style::upsert(&path_param.name, payload).await.map(|value| HttpResponse::Ok().json(value))
}
