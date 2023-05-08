mod mergedirs;

use clap::{Arg, App};
use std::path::Path;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(Arg::with_name("source")
            .short('s')
            .long("source")
            .required(true)
            .takes_value(true)
            .help("Sets the source file"))
        .arg(Arg::with_name("dest")
            .short('d')
            .long("dest")
            .required(true)
            .takes_value(true)
            .help("Sets the destination file"))
        .arg(Arg::with_name("mode")
            .short('m')
            .long("mode")
            .required(true)
            .takes_value(true)
            .possible_values(&["copy", "append", "delete"])
            .help("Sets the mode"))
        .get_matches();

    let src_dir = Path::new(matches.value_of("source").unwrap());
    let dst_dir = Path::new(matches.value_of("dest").unwrap());
    let mode = match matches.value_of("mode").unwrap() {
        "copy" => mergedirs::Mode::Copy,
        "append" => mergedirs::Mode::Append,
        "delete" => mergedirs::Mode::Delete,
        _ => unreachable!(),
    };

    mergedirs::merge_dirs(src_dir, dst_dir, &mode).unwrap_or_else(|error| panic!("Error: {}", error))
}
