use failure::Error;

pub fn summarize_error(error: Error) -> String {
    let mut causes = error.causes();

    // Error struct guarantees at least one top-level cause.
    let top_level_cause = causes.next().unwrap();
    let summary = format!("fatal: {}", top_level_cause);

    match top_level_cause.cause() {
        // No second cause, done.
        None => summary,
        // If second cause is root cause, don't bother
        // enumerating the cause levels.
        Some(cause) if cause.cause().is_none() => format!("{}\ncause: {}", summary, cause),
        // But do so if more than one cause level.
        Some(_) => (1..).zip(causes).fold(summary, |acc, (idx, cause)| {
            format!(
                "{acc}\ncause {level}: {msg}",
                acc = acc,
                level = idx,
                msg = cause
            )
        }),
    }
}
