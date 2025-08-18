/// Checks the given `num_types` against a threshold and prints a warning to
/// `stderr` if the threshold is reached.
///
/// Returns `true` when the threshold is reached.
pub(crate) fn reached_too_many_types(num_types: usize) -> bool {
    const MAX_NUM_TYPES: usize = 100_000;

    if num_types < MAX_NUM_TYPES {
        return false;
    }

    // format with thousand separators for readability:
    let num = MAX_NUM_TYPES
        .to_string()
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(|bytes| std::str::from_utf8(bytes).unwrap())
        .collect::<Vec<_>>()
        .join(",");

    eprintln!(
        "Biome encountered an unusually large amount of types ({num}).\n\
        \n\
        Either you are analyzing _very_ large files (did you make sure to \
        exclude your build/ or dist/ folder?), or you've encountered a bug in \
        Biome.\n\
        \n\
        Please follow these instructions to discover if you are accidentally \
        analyzing large files and what to do about them: \
        https://biomejs.dev/guides/investigate-slowness/\n\
        \n\
        If you believe this is a bug, please follow the instructions in the \
        link above first. Then report it to \
        https://github.com/biomejs/biome/issues/ and include (a relevant \
        snippet of) the file that triggered this issue. Without a \
        reproduction, unfortunately we cannot resolve the issue.\n",
    );

    true
}
