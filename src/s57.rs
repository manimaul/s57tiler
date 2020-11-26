use std::path::{Path, PathBuf};
use gdal::Dataset;
use serde_json;
use std::fs;
use crate::geojson_builder::feature_collection_from_layer;
use gdal::spatial_ref::SpatialRef;


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
        for layer in self.dataset.layers() {
            let target_sr = SpatialRef::from_epsg(4326).unwrap();
            if let Some(fc) = feature_collection_from_layer(&layer, &target_sr) {
                names.push(layer.name());
                let geo_json = if pretty {
                    serde_json::to_string_pretty(&fc).unwrap()
                } else {
                    fc.to_string()
                };
                Self::write_json(out_dir, format!("{}.json", layer.name()).as_str(), &geo_json);
            };
        }
        let meta = serde_json::json!({
            "layers": names
        });
        Self::write_json(out_dir, "meta.json", &serde_json::to_string_pretty(&meta).unwrap())
    }

    fn write_json(out_dir: &Path, name: &str, contents: &String) {
        let mut json_out_path = PathBuf::from(out_dir);
        json_out_path.push(name);
        println!("writing to - {:?}", json_out_path);
        if json_out_path.exists() {
            fs::remove_file(&json_out_path).ok();
        }
        fs::write(&json_out_path, contents).ok();
    }
}
