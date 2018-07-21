use clap::{App, Arg};

pub const VALUE_INPUT_FILES: &str = "INPUT_FILE";
pub const VALUE_OUTPUT_FILES: &str = "output";

pub fn new<'a, 'b>() -> App<'a, 'b> {
    App::new("litter")
        .version(crate_version!())
        .author("krachzack <hello@phstadler.com>")
        .about("Takes scenes and combines them into more complex scenes")
        .arg(
            Arg::with_name(VALUE_INPUT_FILES)
                .required(true)
                .help("One or more paths to OBJ files")
                .index(1)
                .multiple(true),
        )
        .arg(
            Arg::with_name(VALUE_OUTPUT_FILES)
                .required(true)
                .long("output")
                .short("o")
                .help("Sets the output file path for the combined scene.")
                .value_name("OUTPUT_FILE")
                .takes_value(true)
                .multiple(true),
        )
}
