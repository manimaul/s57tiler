#[macro_use]
extern crate log;

use std::env;
use actix_web::{App, HttpServer, web};
use actix_web::middleware::Logger;
use s57server::handlers;
use s57server::constants;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info,actix_web=info");
    }
    env_logger::init();
    info!("starting s57server");
    handlers::feature::create_mvt_query();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(handlers::info)
            .service(handlers::post_custom_style)
            .service(handlers::get_custom_style)
            .service(handlers::get_style)
            .service(handlers::post_chart)
            .service(handlers::get_chart)
            .service(handlers::delete_chart)
            .service(handlers::post_geojson)
            .service(handlers::get_geojson)
            .service(handlers::get_tile)
            .service(handlers::get_resource)
            .service(handlers::tile_json)
            .service(handlers::post_enc_save)
            .app_data(
                web::JsonConfig::default()
                    // increase body payload size to accommodate large geojson
                    .limit(1024 * 1000)
                    // accept text/plain content type
                    .content_type(|mime| {
                        mime.type_() == mime::TEXT && mime.subtype() == mime::PLAIN
                    })
            )
    }).bind(constants::local_socket_address())?.run().await
}
