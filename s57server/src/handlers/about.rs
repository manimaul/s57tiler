use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::Serialize;

use crate::constants;

#[derive(Serialize)]
pub struct About {
    description: String,
    version: String,
}

impl About {
    pub fn new() -> Self {
        About {
            description: String::from("s57server"),
            version: constants::version(),
        }
    }
}

impl Responder for About {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok().json(&self)))
    }
}