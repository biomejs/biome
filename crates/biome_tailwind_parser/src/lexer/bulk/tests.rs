use super::{BulkTokens, LINE_BREAK, SLOW, kind};
use crate::lexer::{BASENAME_STORE, BaseNameStore, TailwindLexer};
use crate::token_source::TailwindLexContext;
use biome_parser::lexer::Lexer;
use biome_tailwind_syntax::TailwindSyntaxKind::EOF;
use std::path::Path;

/// Asserts that every bulk token of `source` is the token that [TailwindLexer] returns at the
/// same offset and after the same token.
fn check_tokens(source: &str, base_names: &BaseNameStore) {
    let tokens = BulkTokens::new(source, base_names);
    let mut lexer = TailwindLexer::from_str(source).with_base_names(base_names);

    let mut previous = EOF;
    let mut after_newline = false;
    for (start, end, raw) in tokens.iter() {
        let diagnostics = lexer.diagnostics_len();
        let lexed = lexer.lex_at(
            start as usize,
            previous,
            after_newline,
            TailwindLexContext::Regular,
        );
        let range = lexer.current_range();
        let at = || format!("regular token at {start} of {source:?}");
        assert_eq!(u32::from(range.start()), start, "{}", at());
        assert_eq!(u32::from(range.end()), end, "{}", at());
        if raw & SLOW == 0 {
            assert_eq!(kind(raw), lexed, "{}", at());
            assert_eq!(lexer.diagnostics_len(), diagnostics, "{}", at());
            assert_eq!(raw & LINE_BREAK != 0, lexer.after_newline(), "{}", at());
        } else {
            assert!(lexer.diagnostics_len() > diagnostics, "{}", at());
        }
        previous = lexed;
        after_newline = lexer.after_newline();
    }

    let bytes = source.as_bytes();
    for offset in tokens.segment_starts(source.len()) {
        assert!(
            source.is_char_boundary(offset),
            "variant segment token start inside a char at {offset} of {source:?}"
        );
    }

    for offset in (0..=bytes.len()).filter(|&offset| source.is_char_boundary(offset)) {
        let expected = bytes[offset..]
            .iter()
            .take_while(|byte| !matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' '))
            .any(|&byte| byte == b':');
        assert_eq!(
            tokens.has_colon_before_whitespace(bytes, offset),
            Some(expected),
            "colon after {offset} of {source:?}"
        );
    }

    // The variant segment tokens along the lexer's tokens from the start.
    let mut position = 0;
    while position < bytes.len() {
        let diagnostics = lexer.diagnostics_len();
        let lexed = lexer.lex_at(position, EOF, false, TailwindLexContext::VariantSegment);
        let end = usize::from(lexer.current_range().end());
        let at = || format!("variant segment token at {position} of {source:?}");
        match tokens.segment_token(bytes, position) {
            Some((raw, bulk_end)) => {
                assert_eq!(kind(raw), lexed, "{}", at());
                assert_eq!(bulk_end, end, "{}", at());
                assert_eq!(lexer.diagnostics_len(), diagnostics, "{}", at());
                assert_eq!(raw & LINE_BREAK != 0, lexer.after_newline(), "{}", at());
            }
            None => assert!(lexer.diagnostics_len() > diagnostics, "{}", at()),
        }
        position = end;
    }

    // The variant segment tokens at every other offset.
    for position in (0..bytes.len()).filter(|&offset| source.is_char_boundary(offset)) {
        if let Some((raw, bulk_end)) = tokens.segment_token(bytes, position) {
            let lexed = lexer.lex_at(position, EOF, false, TailwindLexContext::VariantSegment);
            let at = || format!("variant segment token at {position} of {source:?}");
            assert_eq!(kind(raw), lexed, "{}", at());
            assert_eq!(
                bulk_end,
                usize::from(lexer.current_range().end()),
                "{}",
                at()
            );
        }
    }
}

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
    assert!(sources.len() > 500, "only found {} fixtures", sources.len());
    sources
}

#[test]
fn bulk_tokens_match_the_lexer() {
    let mut next = xorshift(0x2545_F491_4F6C_DD1D);
    for source in fixtures() {
        check_tokens(&source, &BASENAME_STORE);
        for _ in 0..3 {
            check_tokens(&mutate(&source, &mut next), &BASENAME_STORE);
        }
    }
}

#[test]
fn bulk_tokens_of_random_fragments_match_the_lexer() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..3000 {
        check_tokens(&random_fragments(&mut next), &BASENAME_STORE);
    }
}

#[test]
fn bulk_tokens_match_the_lexer_across_blocks() {
    // Tokens that cross a 64-byte block boundary at every offset.
    for padding in 0..70 {
        let pad = "x".repeat(padding);
        for source in [
            format!("{pad} hover:bg-red-500/50 [&_p]:m-[calc(1px+2px)] \u{e9}%(x) w-1/2"),
            format!("{pad}%%%%(((\u{e9}\u{e9}hover-group/menu:\n\n  \x0B  data-[x]:p-4"),
            format!("{pad}{}", "a".repeat(130)),
            format!("{pad}{}", "%".repeat(70)),
            format!("{pad}[{}]", "[a]".repeat(30)),
        ] {
            check_tokens(&source, &BASENAME_STORE);
        }
    }
}

#[test]
fn bulk_tokens_use_the_given_base_names() {
    let base_names = BaseNameStore::new(&["slide-in-from-top", "bg-red", "data-x"]);
    for source in [
        "slide-in-from-top-4 bg-red-500 data-x-1 data-[a]:flex",
        "slide-in-from-top bg-red data-x hover:slide-in-from-top-2",
    ] {
        check_tokens(source, &base_names);
    }
}

#[test]
fn bulk_tokens_of_edge_cases_match_the_lexer() {
    for source in [
        "",
        " ",
        "\u{FEFF}flex",
        "\u{FEFF}",
        "bg-:flex",
        "bg-%",
        "a/:b",
        "a/!",
        "a/%",
        "*:flex **:p-4",
        "[[data-state=open]_&]:flex",
        "[a[b]c]:x [a[b",
        "bg-(--x)",
        "bg-(",
        "-mt-4",
        "\r\n\t\x0B\x0C",
        "data",
        "data-",
        "@container/sidebar",
        "\u{e9}\u{e9}:\u{e9}",
    ] {
        check_tokens(source, &BASENAME_STORE);
    }
}

/// Runs one lexing pass over each input many times, for `perf stat` to measure. Reads the
/// inputs from `TW_LEX_FILE`, one per line if `TW_LEX_LINES` is set, and lexes them
/// `TW_LEX_ITERATIONS` times with the bulk lexer, or with [TailwindLexer] in
/// [TailwindLexContext::Regular] if `TW_LEX_MODE` is `lexer`.
#[test]
#[ignore]
fn lexing_pass() {
    let text = std::fs::read_to_string(std::env::var("TW_LEX_FILE").unwrap()).unwrap();
    let inputs: Vec<&str> = if std::env::var_os("TW_LEX_LINES").is_some() {
        text.lines().collect()
    } else {
        vec![text.as_str()]
    };
    let iterations: usize = std::env::var("TW_LEX_ITERATIONS").unwrap().parse().unwrap();
    let lexer_mode = std::env::var("TW_LEX_MODE").is_ok_and(|mode| mode == "lexer");
    let mut tokens = 0usize;
    for _ in 0..iterations {
        for input in &inputs {
            let input = std::hint::black_box(*input);
            if lexer_mode {
                let mut lexer = TailwindLexer::from_str(input);
                while lexer.next_token(TailwindLexContext::Regular) != EOF {
                    tokens += 1;
                }
                tokens += lexer.finish().len();
            } else {
                let bulk = BulkTokens::new(input, &BASENAME_STORE);
                tokens += bulk.iter().count();
            }
        }
    }
    std::hint::black_box(tokens);
}
