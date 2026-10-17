use super::{LINE_BREAK, SLOW, SimdLexer, kind};
use crate::legacy::lexer::TailwindLexer;
use crate::lexer::{BASENAME_STORE, BaseNameStore, TailwindLexContext};
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use biome_parser::lexer::Lexer;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};

const CONTEXTS: [TailwindLexContext; 7] = [
    TailwindLexContext::Regular,
    TailwindLexContext::SawNegative,
    TailwindLexContext::ArbitraryVariant,
    TailwindLexContext::ArbitraryCandidate,
    TailwindLexContext::VariantSegment,
    TailwindLexContext::CssValue,
    TailwindLexContext::CssUrlRawValue,
];

/// Kinds of the token before, covering every kind a lexer context checks for.
const PREVIOUS: [TailwindSyntaxKind; 12] = [
    EOF,
    T!['['],
    T!['('],
    T![-],
    T![/],
    T![:],
    T![']'],
    T![')'],
    CSS_DIMENSION_VALUE,
    WHITESPACE,
    TW_BASE,
    IDENT,
];

/// The longest source whose every offset [check] lexes in every context.
const MAX_EXHAUSTIVE_LEN: usize = 600;

/// Asserts that [SimdLexer] lexes `source` like [TailwindLexer]: its regular tokens, its colon
/// query, and, for sources up to [MAX_EXHAUSTIVE_LEN] bytes, the token at every offset in every
/// context after every kind of token.
fn check(source: &str, base_names: &BaseNameStore) {
    let simd = SimdLexer::new(source, base_names);
    let bytes = source.as_bytes();

    // The regular tokens, each after the one before it.
    let mut legacy = TailwindLexer::from_str(source).with_base_names(base_names);
    let mut previous = EOF;
    let mut after_newline = false;
    for (start, end, raw) in simd.regular_tokens() {
        let diagnostics = legacy.diagnostics().len();
        let lexed = legacy.lex_at(
            start as usize,
            previous,
            after_newline,
            TailwindLexContext::Regular,
        );
        let at = || format!("regular token at {start} of {source:?}");
        assert_eq!(u32::from(legacy.current_range().end()), end, "{}", at());
        assert_eq!(
            raw & SLOW != 0,
            legacy.diagnostics().len() > diagnostics,
            "{}",
            at()
        );
        if raw & SLOW == 0 {
            assert_eq!(kind(raw), lexed, "{}", at());
            assert_eq!(raw & LINE_BREAK != 0, legacy.after_newline(), "{}", at());
        }
        previous = lexed;
        after_newline = legacy.after_newline();
    }

    let offsets = (0..=bytes.len()).filter(|&offset| source.is_char_boundary(offset));
    for offset in offsets.clone() {
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

    if bytes.len() > MAX_EXHAUSTIVE_LEN {
        return;
    }
    for offset in offsets {
        for context in CONTEXTS {
            for previous in PREVIOUS {
                let mut legacy = TailwindLexer::from_str(source).with_base_names(base_names);
                let expected_kind = legacy.lex_at(offset, previous, false, context);
                let expected_end = usize::from(legacy.current_range().end());
                let mut diagnostics = Vec::new();
                let (actual_kind, actual_end) =
                    simd.lex(offset, context, previous, &mut diagnostics);
                let at = || format!("{context:?} after {previous:?} at {offset} of {source:?}");
                assert_eq!(
                    (actual_kind, actual_end),
                    (expected_kind, expected_end),
                    "{}",
                    at()
                );
                assert_eq!(
                    format!("{diagnostics:?}"),
                    format!("{:?}", legacy.diagnostics()),
                    "{}",
                    at()
                );
                assert_eq!(
                    actual_kind == WHITESPACE && simd.has_line_break(offset, actual_end),
                    legacy.after_newline(),
                    "{}",
                    at()
                );
            }
        }
    }
}

#[test]
fn simd_lexer_matches_the_legacy_lexer_on_fixtures() {
    let mut next = xorshift(0x2545_F491_4F6C_DD1D);
    for source in fixtures() {
        check(&source, &BASENAME_STORE);
        check(&mutate(&source, &mut next), &BASENAME_STORE);
    }
}

#[test]
fn simd_lexer_matches_the_legacy_lexer_on_random_fragments() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..1500 {
        check(&random_fragments(&mut next), &BASENAME_STORE);
    }
}

#[test]
fn simd_lexer_matches_the_legacy_lexer_across_blocks() {
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
fn simd_lexer_matches_the_legacy_lexer_on_edge_cases() {
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

#[test]
fn simd_lexer_uses_the_given_base_names() {
    let base_names = BaseNameStore::new(&["slide-in-from-top", "bg-red", "data-x"]);
    for source in [
        "slide-in-from-top-4 bg-red-500 data-x-1 data-[a]:flex",
        "slide-in-from-top bg-red data-x hover:slide-in-from-top-2",
    ] {
        check(source, &base_names);
    }
}

/// Runs one lexing pass over each input many times, for `perf stat` to measure. Reads the
/// inputs from `TW_LEX_FILE`, one per line if `TW_LEX_LINES` is set, and lexes them
/// `TW_LEX_ITERATIONS` times with [SimdLexer], or with [TailwindLexer] in
/// [TailwindLexContext::Regular] if `TW_LEX_MODE` is `legacy`.
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
    let legacy = std::env::var("TW_LEX_MODE").is_ok_and(|mode| mode == "legacy");
    let mut tokens = 0usize;
    for _ in 0..iterations {
        for input in &inputs {
            let input = std::hint::black_box(*input);
            if legacy {
                let mut lexer = TailwindLexer::from_str(input);
                while lexer.next_token(TailwindLexContext::Regular) != EOF {
                    tokens += 1;
                }
                tokens += lexer.finish().len();
            } else {
                tokens += SimdLexer::new(input, &BASENAME_STORE)
                    .regular_tokens()
                    .count();
            }
        }
    }
    std::hint::black_box(tokens);
}
