use std::path::{Path, PathBuf};
use std::fs;

pub fn check_out_dir(out_dir: &Path) {
    println!("rendering geojson to: {:?}", out_dir);
    if !out_dir.exists() {
        println!("creating directory: {:?}", out_dir);
        fs::create_dir_all(out_dir).ok();
    } else if !out_dir.is_dir() {
        let error = format!("{:?} exists and is not a directory", out_dir);
        panic!(error);
    }
}

pub fn write_json(out_dir: &Path, out_name: &str, contents: &String) {
    let mut json_out_path = PathBuf::from(out_dir);
    json_out_path.push(out_name);
    println!("writing to - {}", json_out_path.to_str().unwrap());
    if json_out_path.exists() {
        fs::remove_file(&json_out_path).ok();
    }
    fs::write(&json_out_path, contents).ok();
}
