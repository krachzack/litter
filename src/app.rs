use clap::{App, Arg};

pub const VALUE_INPUT_FILES: &str = "INPUT_FILE";
pub const VALUE_OUTPUT_FILES: &str = "output";
pub const VALUE_FILTERS: &str = "filter";

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
                .takes_value(true),
        )
        .arg(
            Arg::with_name(VALUE_FILTERS)
                .required(false)
                .long("filter")
                .short("f")
                .help("Adds a transformation to perform after the input scenes have been combined.")
                .long_help("Adds transformations on top of the combined input scenes. For example, use --filter=grid,10x10x1 to duplicate the scene 100 times in a ten by ten grid. --filter=align,+,+,c will put all vertices in the positive X and Y range and puts half of the scene in negative Z and the rest in positive Z, default is c,+,c.")
                .use_delimiter(false)
                .value_name("FILTER_SPECIFICATION")
                .takes_value(true)
                .multiple(true)
        )
}
