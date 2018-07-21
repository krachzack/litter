use app;
use clap::{ArgMatches, Values};
use failure::Error;
use litter::litter;
use std::path::PathBuf;

pub struct Launcher;

enum ResolveStyle {
    InputFile,
    OutputFile,
}
use self::ResolveStyle::*;

impl Launcher {
    pub fn launch() -> Result<(), Error> {
        let matches = app::new().get_matches();

        // Resolve first input and then output files, exit early if something happens.
        let input_obj_paths = Self::resolve_paths(&matches, app::VALUE_INPUT_FILES, InputFile)?;
        let output_obj_path = {
            let mut output_obj_paths =
                Self::resolve_paths(&matches, app::VALUE_OUTPUT_FILES, OutputFile)?;
            assert!(
                output_obj_paths.len() == 1,
                "Expected exactly one output OBJ"
            );
            output_obj_paths.remove(0)
        };

        litter(input_obj_paths, output_obj_path)
    }

    fn resolve_paths(
        matches: &ArgMatches,
        arg_name: &str,
        style: ResolveStyle,
    ) -> Result<Vec<PathBuf>, Error> {
        let matches: Result<Values, Error> = matches
            .values_of(arg_name)
            .ok_or_else(|| format_err!("{arg} has not been specified.", arg = arg_name));

        // Check if specified paths point to existing files and fail early
        // on errors.
        let resolved: Result<Vec<_>, _> = matches?
            .map(PathBuf::from)
            .map(|p| {
                if p.is_file() {
                    // If exists as ordinary file, that is fine for input and
                    //output files
                    Ok(p)
                } else if p.exists() {
                    Err(format_err!("Path {path:?} already exists but is not an ordinary file. It cannot be used as {arg}.", path = p, arg = arg_name))
                } else {
                    match (&style, p.parent().map(PathBuf::from)) {
                        // Non-existing output files without an explicit parent directory are valid
                        (OutputFile, Some(ref parent)) if parent.as_os_str().is_empty() => Ok(p),
                        // An existing directory as parent is fine too
                        (OutputFile, Some(ref parent)) if parent.is_dir() => Ok(p),
                        // Everything else is invalid
                        _ => Err(format_err!(
                            "Path {path:?} could not be resolved to be used as {arg}.",
                            path = p,
                            arg = arg_name
                        )),
                    }
                }
            })
            .collect();

        // Report error if the arguments where present but no values specified.
        // Note: This maybe could also interpreted as using stdout or stdin.
        resolved.and_then(|paths| {
            if !paths.is_empty() {
                Ok(paths)
            } else {
                Err(format_err!(
                    "Argument {arg} occurred but no values specified.",
                    arg = arg_name
                ))
            }
        })
    }
}
