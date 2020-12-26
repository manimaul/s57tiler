use actix_web::{delete, get, HttpResponse, post, Responder, web};

use crate::handlers::about::About;
use crate::handlers::chart::{Chart, ChartInsert};
use crate::handlers::style::{PathParam, Style};

mod about;
mod style;
mod chart;

///curl http://localhost:8080/v1/about | jq
#[get("/v1/about")]
pub async fn info() -> impl Responder {
    About::new()
}

///curl http://localhost:8080/v1/style/foo | jq
#[get("/v1/style/{name}")]
pub async fn get_style(path_param: web::Path<PathParam>) -> impl Responder {
    Style::query(&path_param.name).map(|style_record: Style| {
        HttpResponse::Ok().json(style_record.style)
    })
}

///curl -v -H "Content-Type: application/json" --request POST  --data '{"foo":"bar"}' http://localhost:8080/v1/style/foo
#[post("/v1/style/{name}")]
pub async fn post_style(
    payload: web::Payload,
    path_param: web::Path<PathParam>
) -> impl Responder {
    info!("style name posted: {}", &path_param.name);
    Style::upsert(&path_param.name, payload).await.map(|value| HttpResponse::Ok().json(value))
}

///curl -v -H "Content-Type: application/json" --request POST  --data '{"name": "foo", "scale": 0, "file_name": "foo.000", "updated": "1979", "issued": "1980"}' http://localhost:8080/v1/chart
#[post("/v1/chart")]
pub async fn post_chart(
    payload: web::Json<ChartInsert>
) -> impl Responder {
    payload.into_inner().insert()
}

#[derive(Deserialize)]
pub struct IdQuery {
    id: i64,
}

///curl http://localhost:8080/v1/chart?id=1 | jq
#[get("/v1/chart")]
pub async fn get_chart(
    id: web::Query<IdQuery>
) -> impl Responder {
    Chart::query(id.id).map(|chart| HttpResponse::Ok().json(chart))
}

///curl -v -X DELETE http://localhost:8080/v1/chart?id=1
#[delete("/v1/chart")]
pub async fn delete_chart(
    id: web::Query<IdQuery>
) -> impl Responder {
    Chart::delete(id.id).map(|chart| HttpResponse::Ok().json(chart))
}
