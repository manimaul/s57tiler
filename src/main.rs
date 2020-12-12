#![recursion_limit="256"]
#[macro_use]
extern crate lazy_static;

mod s57;
mod geojson_builder;
mod styler;
mod utils;
mod colors;
mod soundg;
mod seaare;
mod depare;
mod depcnt;

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
        .subcommand(SubCommand::with_name("style")
            .about("Generates a Mapbox Vector style for S57 marine charts")
            .arg(Arg::with_name("socket_address")
                .help("The socket address or host name where where your style will be served from. \n\
                          eg 'localhost:8080' or 's57dev.mxmariner.com`")
                .short("s")
                .long("s")
                .required(true)
                .takes_value(true)
            )
            .arg(Arg::with_name("tls")
                .help("Specifies whether the socket address uses https or the default http")
                .short("t")
                .long("tls")
                .required(false)
                .takes_value(false)
            )
            .arg(Arg::with_name("out_dir")
                .help("Sets the output directory")
                .short("o")
                .long("output")
                .required(true)
                .takes_value(true)
            )
        )
        .subcommand(SubCommand::with_name("config")
            .about("Generates a TileServer-GL Config")
            .arg(Arg::with_name("socket_address")
                .help("The socket address' or host name where where your style will be served from. \n\
                          This can be a comma delimited list.\n\
                          eg '127.0.0.1:8080,localhost:8080' or 's57dev.mxmariner.com`")
                .short("s")
                .long("s")
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
        )
        .get_matches();


    if let None = matches.subcommand_name() {
        println!("No command given, try help.");
    } else if let Some(matches) = matches.subcommand_matches("mbtiles") {
        mbtiles(matches);
    } else if let Some(matches) = matches.subcommand_matches("style") {
        style(matches);
    } else if let Some(matches) = matches.subcommand_matches("config") {
        config(matches);
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

fn style(matches: &ArgMatches) {
    let out_dir = matches.value_of("out_dir").unwrap();
    let socket_address = matches.value_of("socket_address").unwrap();
    let addess = if matches.is_present("tls") {
        format!("https://{}", socket_address)
    } else {
        format!("http://{}", socket_address)
    };
    styler::create_style(Path::new(out_dir), &addess);
}


fn config(matches: &ArgMatches) {
    let out_dir = matches.value_of("out_dir").unwrap();
    let sa_list: Vec<String> = matches.value_of("socket_address").unwrap()
        .split(",")
        .map(|s| String::from(s))
        .collect();
    styler::create_config(Path::new(out_dir), sa_list);
}
