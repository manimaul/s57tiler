#[macro_use]
extern crate log;

use std::env;
use actix_web::{App, HttpServer, error, HttpResponse, web};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,actix_web=info");
    }
    env_logger::init();
    info!("starting s57server");
    HttpServer::new(|| {
        App::new()
            .service(s57server::handlers::info)
            .service(s57server::handlers::post_style)
            .service(s57server::handlers::get_style)
            .service(s57server::handlers::post_chart)
            .service(s57server::handlers::get_chart)
            .service(s57server::handlers::delete_chart)
            .service(s57server::handlers::post_geojson)
            .service(s57server::handlers::get_geojson)
            .service(s57server::handlers::get_tile)
            .app_data(
                web::JsonConfig::default()
                    // increase body payload size to accommodate large geojson
                    .limit(1024 * 1000 )
                    // accept text/plain content type
                    .content_type(|mime| {
                        mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
                    })
            )
    }).bind("127.0.0.1:8080")?.run().await
}
