use clap::{App, Arg};

pub fn new<'a, 'b>() -> App<'a, 'b> {
    App::new("litter")
        .version(crate_version!())
        .author("krachzack <hello@phstadler.com>")
        .about("Takes scenes and combines them into more complex scenes")
        .arg(
            Arg::with_name("INPUT")
                .required(true)
                .help("One or more paths to OBJ files")
                .index(1)
                .multiple(true),
        )
        .arg(
            Arg::with_name("output")
                .required(true)
                .short("o")
                .long("output")
                .help("Sets the output file path for the combined scene.")
                .value_name("OUTPUT_FILE")
                .takes_value(true),
        )
}
