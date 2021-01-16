use std::env;
use std::io::Write;
use std::fs;

use actix_multipart::Multipart;
use actix_web::{App, Error, error, HttpResponse, HttpServer, middleware, web};
use futures::{StreamExt, TryStreamExt};
use sanitize_filename;
use uuid::Uuid;
use std::collections::HashSet;

use crate::errors;
use crate::errors::ErrMapper;

#[derive(Serialize)]
struct EncUpload {
    files: Vec<String>,
    uuid: String,
}

pub async fn save_enc_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let enc_upload = env::var("ENC_UPLOAD")
        .map_internal_server_error("ENC_UPLOAD not specified in .env")?;

    let my_uuid = Uuid::new_v4();
    let dir = format!("{}/{}", enc_upload, my_uuid);
    fs::create_dir_all(&dir);
    let mut files = HashSet::<String>::new();

    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        files.insert(String::from(filename));
        let filepath = format!("{}/{}", &dir, sanitize_filename::sanitize(&filename));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    let upload = EncUpload {
        files: files.into_iter().collect::<Vec<String>>(),
        uuid: format!("{}", my_uuid),
    };
    Ok(HttpResponse::Ok().json(upload))
}
