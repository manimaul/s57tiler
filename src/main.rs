mod s57;
mod geojson_builder;

use std::path::Path;
use gdal_sys;
use std::ffi::CString;

extern crate clap;
use clap::{Arg, App};
use std::fs;


fn main() {
    let matches = App::new("S57 Tiler")
        .version("1.0")
        .author("William Kamp <manimaul@gmail.com>")
        .about("Renders S57 marine chart files into geojson")
        .arg(Arg::with_name("in_file")
            .help("Sets the input S57 file which is usually ending in .000")
            .short("i")
            .long("input")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("out_dir")
            .help("Sets the output directory")
            .short("o")
            .long("output")
            .required(true)
            .takes_value(true)
        )
        .arg(Arg::with_name("keep_geojson")
            .help("Keep the generated geojson")
            .short("g")
            .long("geojson")
            .required(false)
        )
        .get_matches();

    let in_file = matches.value_of("in_file").unwrap();
    let out_dir = matches.value_of("out_dir").unwrap();
    let keep_geojson = matches.is_present("keep_geojson");

    let key = CString::new("OGR_S57_OPTIONS").unwrap();
    let value = CString::new("LNAM_REFS:ON,UPDATES:ON,SPLIT_MULTIPOINT:ON,PRESERVE_EMPTY_NUMBERS:ON,RETURN_LINKAGES:ON").unwrap();
    unsafe {
        gdal_sys::GDALAllRegister();
        gdal_sys::CPLSetConfigOption(key.as_ptr(), value.as_ptr());
    }

    let s57 = s57::S57::open(Path::new(in_file)).unwrap();
    let files = s57.render_geojson(Path::new(out_dir), false);
    s57::S57::generate_mbtiles(Path::new(out_dir), &files);

    if !keep_geojson {
        files.iter()
            .map(|f| Path::new(f))
            .for_each(|f|  fs::remove_file(f).expect("expected to remove file"));
    }
}
