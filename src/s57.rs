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

        for layer in self.dataset.layers() {
            // if !layer.name().eq("SOUNDG")  {
            //     continue;
            // }
            let mut json_out_path = PathBuf::from(out_dir);
                json_out_path.push(format!("{}.json", layer.name()));
            println!("rendering layer {:?}", json_out_path);
            let target_sr = SpatialRef::from_epsg(4326).unwrap();
            if let Some(fc) = feature_collection_from_layer(&layer, &target_sr) {
                let geo_json = if pretty {
                    serde_json::to_string_pretty(&fc).unwrap()
                } else {
                    fc.to_string()
                };
                if json_out_path.exists() {
                    fs::remove_file(&json_out_path).ok();
                }
                fs::write(&json_out_path, geo_json).ok();
            };

        }
    }
}
