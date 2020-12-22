use std::path::{Path, PathBuf};
use gdal::Dataset;
use serde_json;
use std::fs;
use crate::geojson_builder::feature_collection_from_layer;
use gdal::spatial_ref::SpatialRef;
use std::process::Command;
use std::collections::HashSet;
use crate::utils;
use std::ffi::CString;


pub struct S57 {
    dataset: Dataset
}

impl S57 {
    pub fn open(path: &Path) -> Option<S57> {
        let key = CString::new("OGR_S57_OPTIONS").unwrap();
        // https://gdal.org/drivers/vector/s57.html
        let value = CString::new("SPLIT_MULTIPOINT:ON,ADD_SOUNDG_DEPTH=OFF,UPDATES=APPLY,LIST_AS_STRING=OFF").unwrap();
        unsafe {
            gdal_sys::GDALAllRegister();
            gdal_sys::CPLSetConfigOption(key.as_ptr(), value.as_ptr());
        }
        Dataset::open(path).map(|ds| S57 { dataset: ds }).ok()
    }

    pub fn render_geojson(
        &self,
        out_dir: &Path,
        pretty: bool,
        ex_layers: Option<Vec<&str>>,
        in_layers: Option<Vec<&str>>,
    ) -> Vec<String> {
        println!("rendering geojson to: {:?}", out_dir);
        utils::check_out_dir(out_dir);

        let mut names: Vec<String> = vec![];
        let target_sr = SpatialRef::from_epsg(4326).unwrap();
        let layer_ex_set: Option<HashSet<_>> = ex_layers.map(|ea| ea.iter().cloned().collect());
        let layer_in_set: Option<HashSet<_>> = in_layers.map(|ea| ea.iter().cloned().collect());

        for layer in self.dataset.layers() {
            let name = layer.name();
            if let Some(exclude) = &layer_ex_set {
                if exclude.contains(&name.as_str()) {
                    println!("excluding layer: {}", &name);
                    continue;
                }
            };

            if let Some(include) = &layer_in_set {
                if !include.contains(&name.as_str()) {
                    println!("skipping layer: {}", &name);
                    continue;
                }
            };
            if let Some(fc) = feature_collection_from_layer(&layer, &target_sr) {
                let layer_json = format!("{}.json", &name);
                let mut json_out_path = PathBuf::from(out_dir);
                json_out_path.push(layer_json.clone());
                names.push(format!("{}", json_out_path.to_str().unwrap()));
                let geo_json = if pretty {
                    serde_json::to_string_pretty(&fc).unwrap()
                } else {
                    fc.to_string()
                };
                utils::write_json(out_dir, layer_json.as_str(), &geo_json);
            };
        }
        let meta = serde_json::json!({
            "layers": names
        });
        utils::write_json(out_dir, "meta.json", &serde_json::to_string_pretty(&meta).unwrap());
        return names;
    }

    pub fn generate_mbtiles(out_dir: &Path, geojson_files: &Vec<String>) {
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
        println!("{}", std::str::from_utf8(output.stdout.as_slice()).unwrap());
        println!("{}", std::str::from_utf8(output.stderr.as_slice()).unwrap());
    }
}
