//! Inputs for the tests that compare the two lexers and token sources.

use std::path::Path;

/// A pseudo-random sequence with a fixed seed.
pub(crate) fn xorshift(mut state: u64) -> impl FnMut() -> u64 {
    move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    }
}

/// Deletes, repeats or inserts text at a few positions picked by `next`.
pub(crate) fn mutate(source: &str, next: &mut impl FnMut() -> u64) -> String {
    const INSERTS: &[&str] = &[
        "[", "]", "(", ")", "-", "/", ":", "!", "%", " ", "\n", "\r\n", "\x0B", "\x0C", "_", "@",
        "*", "\u{e9}", "url(", "--", "data-", "[&", ".", "#", "'", "\"", "calc(", ",", "\u{FEFF}",
        "\\", "=", "+",
    ];
    let mut text: Vec<char> = source.chars().collect();
    for _ in 0..1 + next() % 3 {
        let at = (next() % (text.len() as u64 + 1)) as usize;
        let len = (next() % 12) as usize;
        match next() % 3 {
            0 => {
                let insert = INSERTS[(next() % INSERTS.len() as u64) as usize];
                text.splice(at..at, insert.chars());
            }
            1 => {
                text.drain(at..(at + len).min(text.len()));
            }
            _ => {
                let repeated: Vec<char> = text[at..(at + len).min(text.len())].to_vec();
                text.splice(at..at, repeated);
            }
        }
    }
    text.into_iter().collect()
}

/// Joins fragments that the lexer treats differently, chosen by `next`.
pub(crate) fn random_fragments(next: &mut impl FnMut() -> u64) -> String {
    const FRAGMENTS: &[&str] = &[
        "flex",
        "bg",
        "-",
        "red",
        "500",
        "/",
        "50",
        ":",
        "hover",
        "[",
        "]",
        "(",
        ")",
        "&",
        "_",
        "p",
        "!",
        "%",
        " ",
        "\n",
        "\t",
        "\x0C",
        "@",
        "*",
        "data",
        "url(",
        "var(--x)",
        "calc(1px+2px)",
        "1.5",
        "e5",
        "#fff",
        "'a b'",
        "\"",
        "\u{e9}",
        "group",
        "peer",
        "min-w",
        "border-t",
        "translate-x",
        "[[x]]",
        "-[",
        ",",
        ".",
        "=",
        "+",
        "~",
        ">",
        "\u{FEFF}",
        "md",
        "2xl",
        "[&_p]",
        "@container",
        "**",
        "sm",
        "a(b",
        "x)",
        "[color:red]",
        "bg-(--x)",
        "w-1/2",
        "\\",
    ];
    let count = 1 + next() % 40;
    (0..count)
        .map(|_| FRAGMENTS[(next() % FRAGMENTS.len() as u64) as usize])
        .collect()
}

/// The parser's spec fixtures, the benchmark fixtures and each class string of the short
/// benchmark fixtures.
pub(crate) fn fixtures() -> Vec<String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = Vec::new();
    let mut stack = vec![
        root.join("tests/tailwind_specs"),
        root.join("benches/fixtures"),
    ];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|extension| extension != "txt") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            if path
                .parent()
                .is_some_and(|parent| parent.ends_with("short"))
            {
                sources.extend(source.lines().map(str::to_string));
            } else {
                sources.push(source);
            }
        }
    }
    assert!(sources.len() > 200, "only found {} fixtures", sources.len());
    sources
}
