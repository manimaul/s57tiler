use actix_web::{delete, get, HttpResponse, post, Responder, web, HttpRequest};
use actix_web::http::header::ContentType;
use mime::Mime;

use crate::handlers::about::About;
use crate::handlers::chart::{Chart, ChartInsert};
use crate::handlers::feature::FeatureInsert;
use crate::handlers::custom_style::CustomStyle;
use actix_http::error;

mod about;
mod custom_style;
mod chart;
pub mod feature;
mod files;
mod tilejson;
pub mod styler;

#[derive(Deserialize)]
pub struct PathParam {
    pub name: String,
}

#[derive(Deserialize)]
pub struct IdQuery {
    id: i64,
}

#[derive(Deserialize)]
pub struct GeoParams {
    chart_id: i64,
    name: String
}

#[derive(Deserialize)]
pub struct Tile {
    pub z: i32,
    pub x: i32,
    pub y: i32
}

///curl http://localhost:8080/v1/about | jq
#[get("/v1/about")]
pub async fn info() -> impl Responder {
    About::new()
}

///curl http://localhost:8081/v1/tile_json | jq
#[get("/v1/tile_json")]
pub async fn tile_json() -> impl Responder {
    HttpResponse::Ok().json(tilejson::tilejson())
}

///curl http://localhost:8081/v1/style/day/meters | jq
#[get("/v1/style/{color}/{depth}")]
pub async fn get_style(path_param: web::Path<(String, String)>) -> impl Responder {
    let color = String::from(&path_param.0.0);
    let depth = String::from(&path_param.0.1);
    styler::create_style(&depth, &color)
        .ok_or(error::ErrorNotFound("no such resource"))
        .map(|value| HttpResponse::Ok().json(value))
}

///curl http://localhost:8080/v1/custom_style/foo | jq
#[get("/v1/custom_style/{name}")]
pub async fn get_custom_style(path_param: web::Path<PathParam>) -> impl Responder {
    CustomStyle::query(&path_param.name).map(|style_record: CustomStyle| {
        HttpResponse::Ok().json(style_record.style)
    })
}

///curl -v -H "Content-Type: application/json" --request POST  --data '{"foo":"bar"}' 'http://localhost:8080/v1/custom_style/foo'
#[post("/v1/custom_style/{name}")]
pub async fn post_custom_style(
    payload: web::Payload,
    path_param: web::Path<PathParam>,
) -> impl Responder {
    info!("style name posted: {}", &path_param.name);
    CustomStyle::upsert(&path_param.name, payload).await.map(|value| HttpResponse::Ok().json(value))
}

///curl -v -H "Content-Type: application/json" --request POST  --data '{"name": "foo", "scale": 0, "file_name": "foo.000", "updated": "1979", "issued": "1980"}' http://localhost:8080/v1/chart
#[post("/v1/chart")]
pub async fn post_chart(
    payload: web::Json<ChartInsert>
) -> impl Responder {
    payload.into_inner().insert()
}

///curl http://localhost:8080/v1/chart?id=1 | jq
#[get("/v1/chart")]
pub async fn get_chart(
    id: web::Query<IdQuery>
) -> impl Responder {
    Chart::query(id.id).map(|chart| HttpResponse::Ok().json(chart))
}

///curl -v -X DELETE 'http://localhost:8080/v1/chart?id=1'
#[delete("/v1/chart")]
pub async fn delete_chart(
    id: web::Query<IdQuery>
) -> impl Responder {
    Chart::delete(id.id).map(|chart| HttpResponse::Ok().json(chart))
}

///curl -v -H "Content-Type: application/json" --request POST --data-binary "@data/BOYSPP.json" 'http://localhost:8080/v1/geojson?chart_id=8&name=BOYSPP'
#[post("/v1/geojson")]
pub async fn post_geojson(
    params: web::Query<GeoParams>,
    geo: web::Json<geojson::GeoJson>,
) -> impl Responder {
    FeatureInsert {
        params: params.0,
        geo: geo.0,
    }.insert().map(|_| HttpResponse::Ok())
}

///curl -v 'http://localhost:8080/v1/geojson?chart_id=1&name=BOYSPP'
#[get("/v1/geojson")]
pub async fn get_geojson(
    params: web::Query<GeoParams>
) -> impl Responder {
    feature::query(&params.0).map(|results| HttpResponse::Ok().json(results))
}

#[get("/v1/tile/{z}/{x}/{y}")]
pub async fn get_tile(
    tile: web::Path<Tile>
) -> impl Responder {
    feature::query_tile(tile.z, tile.x, tile.y).map(|resp| {
        let mut builder = HttpResponse::Ok();
        builder.set(
            ContentType("application/x-protobuf".parse::<Mime>().unwrap())
        );
        builder.body(resp)
    })
}

#[get("/res/{filename:.*}")]
pub async fn get_resource(req: HttpRequest) -> impl Responder {
    files::index(req)
}
