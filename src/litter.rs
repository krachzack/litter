use aitios_asset::obj;
use aitios_scene::Entity;
use failure::Error;
use std::borrow::Borrow;
use std::path::{Path, PathBuf};

/// Combines the given input OBJ files and merges
/// them into a single OBJ/MTL pair to be persisted in the
/// given output path.
pub fn litter(
    input_objs: impl IntoIterator<Item = impl AsRef<Path>>,
    output_obj: impl AsRef<Path>,
) -> Result<(), Error> {
    // Check output path first because it would be super annoying to
    // get that hour when the processing is already done.
    let output_obj = output_obj.as_ref();
    verify_output_obj_path(output_obj)?;

    // Load and merge input scenes.
    let entities = load_all(input_objs)?;

    // And finally write everything to disk.
    persist(entities, output_obj)?;

    Ok(())
}

fn verify_output_obj_path(output_obj: &Path) -> Result<(), Error> {
    let output_ext = output_obj.extension().map(|e| {
        e.to_str()
            .expect("Non-UTF-8 file extension unsupported")
            .to_ascii_lowercase()
    });

    let output_ext = output_ext.as_ref().map(|e| e.as_str());

    match output_ext {
        Some("obj") => Ok(()),
        _ => Err(format_err!(
            "Expected output file to have .obj extension, but got {:?}",
            output_obj
        )),
    }
}

fn load_all(input_objs: impl IntoIterator<Item = impl AsRef<Path>>) -> Result<Vec<Entity>, Error> {
    // Load all input OBJs and exit early on errors
    let mut input_objs = input_objs
        .into_iter()
        .map(|p| obj::load(p.as_ref()))
        .collect::<Result<Vec<Vec<_>>, _>>()?
        .into_iter();

    // Merge the entities into a single vector
    let mut first = input_objs
        .next()
        .ok_or_else(|| format_err!("At least one input file needs to be specified"))?;

    while let Some(next_obj) = input_objs.next() {
        first.extend(next_obj)
    }

    Ok(first)
}

fn persist<I, E>(entities: I, output_obj: &Path) -> Result<(), Error>
where
    I: IntoIterator<Item = E>,
    E: Borrow<Entity>,
{
    let output_obj = output_obj.to_owned();
    let mut output_mtl = output_obj.clone();
    output_mtl.set_extension("mtl");

    obj::save(entities, Some(output_obj), Some(output_mtl))?;

    Ok(())
}
