use crate::errors;
use actix_web::{error, HttpRequest, HttpResponse};
use std::path::{PathBuf, Path};
use std::env;
use actix_files::NamedFile;

lazy_static! {
    static ref SERVE_ROOT: Option<String> = env::var("SERVE_ROOT").ok();
}

pub fn index(req: HttpRequest) -> errors::Result<HttpResponse> {
    SERVE_ROOT.as_ref().ok_or(error::ErrorNotFound("not found"))
        .and_then(|root| {
            let path: PathBuf = req.match_info().query("filename").parse().unwrap();
            let p = Path::new(&root).join(path);
            return if p.is_file() {
                let file = NamedFile::open(p)?;
                file.disable_content_disposition()
                    .use_etag(true)
                    .use_last_modified(true)
                    .into_response(&req)
            } else {
                let fbp = Path::new(&root).join("index.html");
                let file = NamedFile::open(fbp)?;
                file.disable_content_disposition()
                    .use_etag(true)
                    .use_last_modified(true)
                    .into_response(&req)
            };
        })
}
