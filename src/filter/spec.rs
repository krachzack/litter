use failure::{Error, Fail};
use filter::{Filter, Grid};

pub fn filter_from_spec<S>(spec: S) -> Result<Box<dyn Filter>, Error>
where
    S: AsRef<str>,
{
    let spec = spec.as_ref();

    let mut spec_parts = spec.split(',');

    match spec_parts.next() {
        None => Err(format_err!("Filter spec appears to be empty")),
        Some("grid") => make_grid(spec_parts),
        Some(unknown) => Err(format_err!(
            "The type of filter {type} is unknown in the filter spec {spec}",
            type = unknown,
            spec = spec
        )),
    }
}

fn make_grid<'a>(mut args: impl Iterator<Item = &'a str>) -> Result<Box<dyn Filter>, Error> {
    let dimension_clone_counts = args.next().ok_or_else(|| {
        format_err!("Filter grid must at least specify the amount of clones, e.g. \"-f grid,3x3x1\", but nothing found after grid.")
    })?;

    let mut dimension_clone_counts = dimension_clone_counts.split("x").map(|dim| {
        usize::from_str_radix(dim, 10)
            .map_err(|e| e.context("Amount of clones in a dimension could not be read."))
    });

    // Exit early if no args given to grid
    let x_clone_count = dimension_clone_counts.next().ok_or_else(|| {
        format_err!(
            "Filter grid must specify clone counts in at least one dimension as a whole number"
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

    // Otherwise everything is fine, make the grid
    Ok(Box::new(Grid::new(x, y, z)))
}
