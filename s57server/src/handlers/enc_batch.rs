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
use glob::glob;
use sanitize_filename;
use uuid::Uuid;

use crate::errors::ErrMapper;

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

    fn zip_out_path(&self) -> Option<PathBuf> {
        self.uuid_dir_path().map(|dir| Path::new(&dir).join("zip_out"))
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

    fn unzip(&self, ctx: &mut ws::WebsocketContext<Self>) -> Option<PathBuf> {
        ctx.text(format!("Extracting geo file: {}", self.file));
        self.file_path().and_then(|file| {
            fs::File::open(file).ok()
        }).and_then(|fd| {
            zip::ZipArchive::new(fd).ok().and_then(|mut zip| {
                self.zip_out_path().and_then(|dir| {
                    zip.extract(&dir).ok().map(|_| dir)
                })
            })
        })
    }

    fn find_geo_records(&self, root: PathBuf) -> Vec<PathBuf> {
        fs::canonicalize(root).ok().and_then(|r| {
            let pattern = format!("{}/**/*.000", r.display().to_string());
            glob(&pattern).map(|ea| {
                ea.into_iter()
                    .filter(|ea| ea.is_ok())
                    .map(|ea| ea.unwrap())
                    .filter(|ea| {
                        ea.file_stem()
                            .and_then(|stem| stem.to_str())
                            .map(|stem| !stem.starts_with("."))
                            .unwrap_or(false)
                    })
                    .collect::<Vec<PathBuf>>()
            }).ok()
        }).unwrap_or(vec![])
    }

    fn process_geo_data(&self, ctx: &mut ws::WebsocketContext<Self>) {
        self.unzip(ctx).and_then(|zip_path| {
            ctx.text(format!("Processing geo data..."));
            let mut count = 0;
            for record in self.find_geo_records(zip_path) {
                ctx.text(format!("processing {:?}", &record.file_stem().unwrap()));
                //todo:(WK) - record is a path to a .000 s57 file
                count += 1;
            }
            ctx.text(format!("processed {} record(s)", count));
            Some(())
        }).or_else(|| {
            ctx.text(format!("Error processing geo data!"));
            None
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
            self.process_geo_data(ctx);
            self.cleanup();
            ctx.close(None)
        } else {}
    }

    fn finished(&mut self, _: &mut Self::Context) {
        info!("ws finished");
    }
}
