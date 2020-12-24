#[macro_use]
extern crate log;

use std::env;
use actix_web::{App, HttpServer};


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
    }).bind("127.0.0.1:8080")?.run().await
}
