use failure::{Error, Fail};
use filter::{Align, Anchor, Filter, Grid};
use std::str::FromStr;

pub fn filter_from_spec<S>(spec: S) -> Result<Box<dyn Filter>, Error>
where
    S: AsRef<str>,
{
    let spec = spec.as_ref();

    let mut spec_parts = spec.split(',');

    match spec_parts.next() {
        None => Err(format_err!("Filter spec appears to be empty")),
        Some("align") => make_align(spec_parts),
        Some("grid") => make_grid(spec_parts),
        Some(unknown) => Err(format_err!(
            "The type of filter {type} is unknown in the filter spec {spec}",
            type = unknown,
            spec = spec
        )),
    }
}

fn make_align<'a>(args: impl Iterator<Item = &'a str>) -> Result<Box<dyn Filter>, Error> {
    // e.g. c,+,c aligns everythign in positive z and around X/Y origin.
    let mut aligns = args.map(|align| match align {
        "+" => Anchor::Above,
        "-" => Anchor::Under,
        // nothing, c or any character counts as center
        _ => Anchor::Center,
    });

    let x_align = aligns.next().unwrap_or(Anchor::Center);
    // Above is nicer default for Y
    let y_align = aligns.next().unwrap_or(Anchor::Above);
    let z_align = aligns.next().unwrap_or(Anchor::Center);

    Ok(Box::new(Align::new(x_align, y_align, z_align)))
}

fn make_grid<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<Box<dyn Filter>, Error> {
    let dimension_clone_counts = args.next().ok_or_else(|| {
        format_err!("Filter grid must at least specify the amount of clones, e.g. \"-f grid,3x3x1\", but nothing found after grid.")
    })?;

    let mut dimension_clone_counts = dimension_clone_counts.split("x").map(|dim| {
        // For empty dimensions, fill  in 1, e.g. xx2 is 1x1x2
        if dim.is_empty() {
            Ok(1)
        } else {
            usize::from_str_radix(dim, 10)
                .map_err(|e| e.context("Amount of clones in a grid filter could not be read."))
        }
    });

    // Exit early if no args given to grid
    let x_clone_count = dimension_clone_counts.next().ok_or_else(|| {
        format_err!(
            "When using a grid filter, specify an argument with the amount of clones in at least on dimension."
        )
    })?;

    // Default to 1 if y unspecfied, optional
    let y_clone_count = dimension_clone_counts.next().unwrap_or(Ok(1));
    // Also for z
    let z_clone_count = dimension_clone_counts.next().unwrap_or(Ok(1));

    // If any of the three was specified but could not be parsed, exit with error
    let x = x_clone_count?;
    let y = y_clone_count?;
    let z = z_clone_count?;

    // Three optional arguments after a comma override cell dimensions
    // from the default of using the scene bounds with no gap.
    let cell_dims: Result<_, Error> = args.next()
        .map(|dims| {
            let mut dims = dims.split("x")
                .map(|d| f32::from_str(d)
                    .map(|d| Some(d))
                    .map_err(|e| e.context("Cell dimensions in a grid filter could not be read."))
                );

            Ok(
                (
                    // An empty argument after the comma is fine
                    dims.next().unwrap_or(Ok(None))?,
                    dims.next().unwrap_or(Ok(None))?,
                    dims.next().unwrap_or(Ok(None))?,
                )
            )
        })
        // Leaving it out is fine, just fill in None
        .unwrap_or(Ok((None, None, None)));

    // Exit with error when parsing failed
    let (cell_size_x, cell_size_y, cell_size_z) = cell_dims?;

    // Otherwise everything is fine, make the grid
    Ok(Box::new(Grid::new(
        x,
        y,
        z,
        cell_size_x,
        cell_size_y,
        cell_size_z,
    )))
}
