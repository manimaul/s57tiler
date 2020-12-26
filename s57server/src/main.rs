#[macro_use]
extern crate log;

use std::env;
use actix_web::{App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "debug,actix_web=debug");
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
    }).bind("127.0.0.1:8080")?.run().await
}
