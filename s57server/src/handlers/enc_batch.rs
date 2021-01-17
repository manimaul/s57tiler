use std::collections::HashSet;
use std::env;
use std::env::VarError;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use actix::{Actor, StreamHandler};
use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, web};
use actix_web_actors::ws;
use futures::{StreamExt, TryStreamExt};
use sanitize_filename;
use uuid::Uuid;

use crate::errors::ErrMapper;
use actix_web_actors::ws::CloseReason;

#[derive(Serialize)]
struct EncUpload {
    files: Vec<String>,
    uuid: String,
}

fn enc_upload_dir() -> Result<String, VarError> {
    env::var("ENC_UPLOAD").map(|it| it.clone())
}

pub async fn save_enc_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let enc_upload = enc_upload_dir()
        .map_internal_server_error("ENC_UPLOAD not specified in .env")?;

    let my_uuid = Uuid::new_v4();
    let dir = format!("{}/{}", enc_upload, my_uuid);
    fs::create_dir_all(&dir)?;
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

#[derive(Deserialize)]
pub struct GeoUploadWs {
    pub uuid: String,
    pub file: String,
}

impl GeoUploadWs {

    pub fn is_folder_empty(path: impl AsRef<Path>) -> bool {
        fs::read_dir(path).map_or(false, |p| p.take(1).count() == 0)
    }

    fn uuid_dir_path(&self) -> Option<PathBuf> {
        enc_upload_dir().ok()
            .map(|dir| Path::new(&dir).join(&self.uuid))
    }

    fn file_path(&self) -> Option<PathBuf> {
        self.uuid_dir_path()
            .map(|dir| {
                dir.join(&self.file)
            })
            .filter(|it| it.exists())
    }

    fn cleanup(&self) {
        self.file_path().and_then(|f| {
            fs::remove_file(f).ok()
        });

        self.uuid_dir_path().filter(|dir| {
            Self::is_folder_empty(dir)
        }).and_then(|dir| {
            fs::remove_dir(dir).ok()
        });
    }
}

impl Actor for GeoUploadWs {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for GeoUploadWs {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        _ctx: &mut Self::Context,
    ) {
        info!("ws msg");
        match msg {
            Ok(ws::Message::Ping(_)) => info!("ping message received"),
            Ok(ws::Message::Text(text)) => info!("text message received: {}", text),
            Ok(ws::Message::Binary(bin)) => info!("binary message received num bytes: {}", bin.len()),
            _ => (),
        }
    }

    fn started(&mut self, ctx: &mut Self::Context) {
        if let Some(file_path) = self.file_path() {
            debug!("rendering file path {:?}", file_path);
            ctx.text(format!("Rendering file: {}/{}", &self.uuid, &self.file));
            //todo: perform render
            self.cleanup();
            ctx.close(None)
        } else {}
    }

    fn finished(&mut self, _: &mut Self::Context) {
        info!("ws finished");
    }
}
