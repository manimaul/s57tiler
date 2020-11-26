use std::path::{Path, PathBuf};
use gdal::Dataset;
use serde_json;
use std::fs;
use crate::geojson_builder::feature_collection_from_layer;
use gdal::spatial_ref::SpatialRef;
use std::process::Command;


pub struct S57 {
    dataset: Dataset
}

impl S57 {
    pub fn open(path: &Path) -> Option<S57> {
        Dataset::open(path).map(|ds| S57 { dataset: ds }).ok()
    }

    pub fn render_geojson(&self, out_dir: &Path, pretty: bool) {
        println!("rendering geojson to: {:?}", out_dir);
        if !out_dir.exists() {
            println!("creating directory: {:?}", out_dir);
            fs::create_dir_all(out_dir).ok();
        } else if !out_dir.is_dir() {
            let error = format!("{:?} exists and is not a directory", out_dir);
            panic!(error);
        }

        let mut names: Vec<String> = vec![];
        let target_sr = SpatialRef::from_epsg(4326).unwrap();
        for layer in self.dataset.layers() {
            if let Some(fc) = feature_collection_from_layer(&layer, &target_sr) {
                let layer_json = format!("{}.json", layer.name());
                let mut json_out_path = PathBuf::from(out_dir);
                json_out_path.push(layer_json.clone());
                names.push(format!("{}", json_out_path.to_str().unwrap()));
                let geo_json = if pretty {
                    serde_json::to_string_pretty(&fc).unwrap()
                } else {
                    fc.to_string()
                };
                Self::write_json(out_dir, layer_json.as_str(), &geo_json);
            };
        }
        let meta = serde_json::json!({
            "layers": names
        });
        Self::write_json(out_dir, "meta.json", &serde_json::to_string_pretty(&meta).unwrap());
        Self::generate_mbtiles(out_dir, &names);
    }

    fn generate_mbtiles(out_dir: &Path, geojson_files: &Vec<String>) {
        let mb_tiles_out = format!("{}/chart.mbtiles", out_dir.to_str().unwrap());
        let mb_tiles_out_p = Path::new(&mb_tiles_out);
        if mb_tiles_out_p.exists() {
            fs::remove_file(&mb_tiles_out_p).expect("could not remove existing mbtiles file");
        };

        let output = Command::new("tippecanoe")
            .arg("-zg")
            .arg("-o")
            .arg(mb_tiles_out)
            .arg("--coalesce-densest-as-needed")
            .arg("--extend-zooms-if-still-dropping")
            .args(geojson_files)
            .output()
            .expect("failed to execute process");
        println!("output result {}", output.status);
        println!("out {}", std::str::from_utf8(output.stdout.as_slice()).unwrap());
        println!("err {}", std::str::from_utf8(output.stderr.as_slice()).unwrap());
    }

    fn write_json(out_dir: &Path, name: &str, contents: &String) {
        let mut json_out_path = PathBuf::from(out_dir);
        json_out_path.push(name);
        println!("writing to - {}", json_out_path.to_str().unwrap());
        if json_out_path.exists() {
            fs::remove_file(&json_out_path).ok();
        }
        fs::write(&json_out_path, contents).ok();
    }
}
