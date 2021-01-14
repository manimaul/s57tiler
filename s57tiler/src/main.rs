use s57tiler::s57;
use s57tiler::geojson_builder;
use s57tiler::utils;
use s57tiler::colors;
use s57tiler::soundg;
use s57tiler::seaare;
use s57tiler::depare;
use s57tiler::depcnt;
use s57tiler::boyspp;
use s57tiler::lights;
use s57tiler::catspm;
use s57tiler::util;
use s57tiler::zfinder;

use std::path::Path;

extern crate clap;

use clap::{Arg, App, SubCommand, ArgMatches};
use std::fs;


fn main() {
    let matches = App::new("S57 Tiler")
        .version("1.0")
        .author("William Kamp <manimaul@gmail.com>")
        .about("Utility for generating S57 Mapbox vector tiles and styles")
        .subcommand(SubCommand::with_name("mbtiles")
            .about("Renders S57 marine chart files into geojson and/or Mapbox MBTiles")
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
            .arg(Arg::with_name("layer_ex")
                .help("Exclude the comma separated list of layers")
                .short("x")
                .long("exclude")
                .required(false)
                .takes_value(true)
            )
            .arg(Arg::with_name("layer_in")
                .help("Include only the comma separated list of layers")
                .short("n")
                .long("include")
                .required(false)
                .takes_value(true)
            )
        )
        .get_matches();


    if let None = matches.subcommand_name() {
        println!("No command given, try help.");
    } else if let Some(matches) = matches.subcommand_matches("mbtiles") {
        mbtiles(matches);
    }
}

fn mbtiles(matches: &ArgMatches) {
    let in_file = matches.value_of("in_file").unwrap();
    let out_dir = matches.value_of("out_dir").unwrap();
    let layer_ex = matches.value_of("layer_ex").map(|ex| ex.split(",").collect::<Vec<&str>>());
    let layer_in = matches.value_of("layer_in").map(|ex| ex.split(",").collect::<Vec<&str>>());
    let keep_geojson = matches.is_present("keep_geojson");
    let s57 = s57::S57::open(Path::new(in_file)).unwrap();
    let files = s57.render_geojson(
        Path::new(out_dir),
        false,
        layer_ex,
        layer_in,
    );
    s57::S57::generate_mbtiles(Path::new(out_dir), &files);
    if !keep_geojson {
        files.iter()
            .map(|f| Path::new(f))
            .for_each(|f| fs::remove_file(f).expect("expected to remove file"));
    }
}
