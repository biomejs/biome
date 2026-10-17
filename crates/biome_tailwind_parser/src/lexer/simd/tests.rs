use super::SimdLexer;
use crate::lexer::{BASENAME_STORE, BaseNameStore};
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use crate::token_source::TailwindTokenSource;
use biome_parser::token_source::TokenSource;
use biome_tailwind_syntax::TailwindSyntaxKind::*;

/// Asserts that [SimdLexer] answers the colon query of `source` at every offset like a scan of
/// its bytes.
fn check(source: &str, base_names: &BaseNameStore) {
    let simd = SimdLexer::new(source, base_names);
    let bytes = source.as_bytes();
    for offset in (0..=bytes.len()).filter(|&offset| source.is_char_boundary(offset)) {
        let expected = bytes[offset..]
            .iter()
            .take_while(|byte| !matches!(byte, b'\t' | b'\n' | 0x0B | 0x0C | b'\r' | b' '))
            .any(|&byte| byte == b':');
        assert_eq!(
            simd.has_colon_before_whitespace(offset),
            expected,
            "colon after {offset} of {source:?}"
        );
    }
}

#[test]
fn colon_query_matches_a_scan_on_fixtures() {
    let mut next = xorshift(0x2545_F491_4F6C_DD1D);
    for source in fixtures() {
        check(&source, &BASENAME_STORE);
        check(&mutate(&source, &mut next), &BASENAME_STORE);
    }
}

#[test]
fn colon_query_matches_a_scan_on_random_fragments() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..1500 {
        check(&random_fragments(&mut next), &BASENAME_STORE);
    }
}

#[test]
fn colon_query_matches_a_scan_across_blocks() {
    // Tokens that cross a 64-byte block boundary at every offset.
    for padding in 0..70 {
        let pad = "x".repeat(padding);
        for source in [
            format!("{pad} hover:bg-red-500/50 [&_p]:m-[calc(1px+2px)] \u{e9}%(x) w-1/2"),
            format!("{pad}%%%%(((\u{e9}\u{e9}hover-group/menu:\n\n  \x0B  data-[x]:p-4"),
            format!("{pad}{}", "a".repeat(130)),
            format!("{pad}{}", "%".repeat(70)),
            format!("{pad}[{}]", "[a]".repeat(30)),
            format!("{pad}[--x:url(a_b)_{}]", "1.5e+3px".repeat(10)),
        ] {
            check(&source, &BASENAME_STORE);
        }
    }
}

#[test]
fn colon_query_matches_a_scan_on_edge_cases() {
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
        "[--a_b\u{e9}:1e5] [a:'b\\'c'] [a:\"b c\"] [a:#fFf0] [a:.5%] [a:-.5] [a:+1] [a:1e] [a:1E-]",
        "[a:calc(100svh-var(--x))] [a:10px-1px] [a:a-b] [a:-a] [a:--a] [a:_-_] [a:1/2]",
        "bg-[url(a_b)] bg-[url('a')] bg-[url()] bg-[url(\u{e9})]",
    ] {
        check(source, &BASENAME_STORE);
    }
}

/// Runs one lexing pass over each input many times, for `perf stat` to measure. Reads the
/// inputs from `TW_LEX_FILE`, one per line if `TW_LEX_LINES` is set, and lexes them
/// `TW_LEX_ITERATIONS` times by bumping through the regular tokens of [TailwindTokenSource].
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
    let mut tokens = 0usize;
    for _ in 0..iterations {
        for input in &inputs {
            let input = std::hint::black_box(*input);
            let mut source = TailwindTokenSource::from_str(input, &BASENAME_STORE);
            while source.current() != EOF {
                source.bump();
                tokens += 1;
            }
        }
    }
    std::hint::black_box(tokens);
}
