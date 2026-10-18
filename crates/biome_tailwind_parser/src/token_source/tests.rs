use super::TailwindTokenSource;
use crate::lexer::{BASENAME_STORE, BaseNameStore};
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use crate::{TailwindParserOptions, parse_tailwind, parse_tailwind_with_options};
use biome_parser::token_source::TokenSource;
use biome_rowan::NodeCache;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Returns the kind and text of each letter that the lexer writes for `source`.
fn letters(source: &str) -> Vec<(TailwindSyntaxKind, &str)> {
    let mut tokens = TailwindTokenSource::from_str(source, &BASENAME_STORE);
    let mut result = Vec::new();
    while tokens.current() != EOF {
        result.push((tokens.current(), &source[tokens.current_range()]));
        tokens.bump();
    }
    result
}

/// Returns the kind and text of each token in the tree of `source`, without trivia.
fn tree_tokens(source: &str) -> Vec<(TailwindSyntaxKind, String)> {
    parse_tailwind(source)
        .syntax()
        .descendants_tokens(biome_rowan::Direction::Next)
        .filter(|token| token.kind() != EOF)
        .map(|token| (token.kind(), token.text_trimmed().to_string()))
        .collect()
}

macro_rules! assert_letters {
    ($src:expr, $($kind:ident $text:literal),* $(,)?) => {
        assert_eq!(letters($src), vec![$(($kind, $text)),*]);
    };
}

macro_rules! assert_tree_tokens {
    ($src:expr, $($kind:ident $text:literal),* $(,)?) => {
        assert_eq!(tree_tokens($src), vec![$(($kind, $text.to_string())),*]);
    };
}

// The letters of a random string cover it, and lexing it terminates.
#[quickcheck]
fn losslessness(string: String) -> bool {
    let cloned = string.clone();
    let (sender, receiver) = channel();
    thread::spawn(move || {
        let mut source = TailwindTokenSource::from_str(&cloned, &BASENAME_STORE);
        let mut ranges = vec![];
        while source.current() != EOF {
            ranges.push(source.current_range());
            source.bump();
        }
        sender
            .send(ranges)
            .expect("Could not send tokens to receiver");
    });
    let ranges = receiver
        .recv_timeout(Duration::from_secs(2))
        .unwrap_or_else(|_| panic!("Lexing doesn't terminate on ->{string}<-"));

    let rebuilt: String = ranges.into_iter().map(|range| &string[range]).collect();
    string == rebuilt
}

#[test]
fn letters_are_sequences_of_one_class_or_single_chars() {
    assert_letters!(
        "bg-red-500/50 \t\n%\u{e9}\u{e9}__x",
        TW_BASE "bg",
        DASH "-",
        TW_BASE "red",
        DASH "-",
        TW_NUMBER "500",
        SLASH "/",
        TW_NUMBER "50",
        WHITESPACE " \t\n",
        PERCENT "%",
        ERROR_TOKEN "\u{e9}",
        ERROR_TOKEN "\u{e9}",
        CSS_WHITESPACE "__",
        TW_BASE "x",
    );
}

#[test]
fn words_before_a_colon_are_variant_segments() {
    assert_letters!(
        "group-hover/menu:flex data-[a]:x data",
        TW_VARIANT_SEGMENT "group",
        DASH "-",
        TW_VARIANT_SEGMENT "hover",
        SLASH "/",
        TW_VARIANT_SEGMENT "menu",
        COLON ":",
        TW_BASE "flex",
        WHITESPACE " ",
        TW_VARIANT_SEGMENT "data",
        DASH "-",
        L_BRACKET "[",
        IDENT "a",
        R_BRACKET "]",
        COLON ":",
        TW_BASE "x",
        WHITESPACE " ",
        TW_BASE "data",
    );
}

#[test]
fn words_inside_brackets_are_css_identifiers() {
    assert_letters!(
        "bg-[color:red] w-[10px]",
        TW_BASE "bg",
        DASH "-",
        L_BRACKET "[",
        IDENT "color",
        COLON ":",
        IDENT "red",
        R_BRACKET "]",
        WHITESPACE " ",
        TW_BASE "w",
        DASH "-",
        L_BRACKET "[",
        CSS_NUMBER_LITERAL "10",
        PX_KW "px",
        R_BRACKET "]",
    );
}

#[test]
fn whitespace_closes_brackets() {
    assert_letters!(
        "w-[1 hover:x",
        TW_BASE "w",
        DASH "-",
        L_BRACKET "[",
        CSS_NUMBER_LITERAL "1",
        WHITESPACE " ",
        TW_VARIANT_SEGMENT "hover",
        COLON ":",
        TW_BASE "x",
    );
}

#[test]
fn hex_colors_end_inside_a_word() {
    assert_letters!(
        "[#abcxyz]",
        L_BRACKET "[",
        HASH "#",
        IDENT "abc",
        IDENT "xyz",
        R_BRACKET "]",
    );
}

#[test]
fn byte_order_mark() {
    assert_letters!("\u{FEFF}flex", UNICODE_BOM "\u{FEFF}", TW_BASE "flex");
}

#[test]
fn bad_whitespace_is_reported_once() {
    let parse = parse_tailwind("a\x0Bb");
    assert_eq!(parse.diagnostics().len(), 1);
}

#[test]
fn values_and_modifiers_merge_letters() {
    assert_tree_tokens!(
        "text-sm bg-primary/10! bg-red-500/[.5]",
        TW_BASE "text",
        DASH "-",
        TW_VALUE "sm",
        WHITESPACE " ",
        TW_BASE "bg",
        DASH "-",
        TW_VALUE "primary",
        SLASH "/",
        TW_NUMBER "10",
        BANG "!",
        WHITESPACE " ",
        TW_BASE "bg",
        DASH "-",
        TW_VALUE "red-500",
        SLASH "/",
        L_BRACKET "[",
        CSS_NUMBER_LITERAL ".5",
        R_BRACKET "]",
    );
}

#[test]
fn variants_keep_their_segments() {
    assert_tree_tokens!(
        "hover:focus:bg-red-500 -mt-4",
        TW_VARIANT_SEGMENT "hover",
        COLON ":",
        TW_VARIANT_SEGMENT "focus",
        COLON ":",
        TW_BASE "bg",
        DASH "-",
        TW_VALUE "red-500",
        WHITESPACE " ",
        DASH "-",
        TW_BASE "mt",
        DASH "-",
        TW_NUMBER "4",
    );
}

#[test]
fn selectors_merge_letters() {
    assert_tree_tokens!(
        "[[data-state=open]_&]:flex has-[>svg]:p-0",
        L_BRACKET "[",
        TW_SELECTOR "[data-state=open]_&",
        R_BRACKET "]",
        COLON ":",
        TW_BASE "flex",
        WHITESPACE " ",
        TW_VARIANT_SEGMENT "has",
        DASH "-",
        L_BRACKET "[",
        TW_SELECTOR ">svg",
        R_BRACKET "]",
        COLON ":",
        TW_BASE "p",
        DASH "-",
        TW_NUMBER "0",
    );
}

#[test]
fn css_dimensions_percentages_and_exponents() {
    assert_tree_tokens!(
        "w-[12px_100%_.5rem_-1rem_1e2px_1E+2%_1e-2]",
        TW_BASE "w",
        DASH "-",
        L_BRACKET "[",
        CSS_NUMBER_LITERAL "12",
        IDENT "px",
        CSS_NUMBER_LITERAL "100",
        PERCENT "%",
        CSS_NUMBER_LITERAL ".5",
        IDENT "rem",
        CSS_NUMBER_LITERAL "-1",
        IDENT "rem",
        CSS_NUMBER_LITERAL "1e2",
        IDENT "px",
        CSS_NUMBER_LITERAL "1E+2",
        PERCENT "%",
        CSS_NUMBER_LITERAL "1e-2",
        R_BRACKET "]",
    );
}

#[test]
fn css_minus_after_a_value_is_an_operator() {
    assert_tree_tokens!(
        "w-[calc(100svh-var(--x))]",
        TW_BASE "w",
        DASH "-",
        L_BRACKET "[",
        IDENT "calc",
        L_PAREN "(",
        CSS_NUMBER_LITERAL "100",
        IDENT "svh",
        DASH "-",
        IDENT "var",
        L_PAREN "(",
        IDENT "--x",
        R_PAREN ")",
        R_PAREN ")",
        R_BRACKET "]",
    );
}

#[test]
fn css_strings_colors_and_urls() {
    assert_tree_tokens!(
        "content-['a_b'] bg-[#abc] bg-[url(a.png)]",
        TW_BASE "content",
        DASH "-",
        L_BRACKET "[",
        CSS_STRING_LITERAL "'a_b'",
        R_BRACKET "]",
        WHITESPACE " ",
        TW_BASE "bg",
        DASH "-",
        L_BRACKET "[",
        CSS_COLOR_LITERAL "#abc",
        R_BRACKET "]",
        WHITESPACE " ",
        TW_BASE "bg",
        DASH "-",
        L_BRACKET "[",
        URL_KW "url",
        L_PAREN "(",
        CSS_URL_VALUE_RAW_LITERAL "a.png",
        R_PAREN ")",
        R_BRACKET "]",
    );
}

#[test]
fn css_strings_escape_quotes_and_colors_end_before_other_letters() {
    assert_tree_tokens!(
        "content-['a\\'b'] bg-[#abcxyz]",
        TW_BASE "content",
        DASH "-",
        L_BRACKET "[",
        CSS_STRING_LITERAL "'a\\'b'",
        R_BRACKET "]",
        WHITESPACE " ",
        TW_BASE "bg",
        DASH "-",
        L_BRACKET "[",
        CSS_COLOR_LITERAL "#abc",
        IDENT "xyz",
        R_BRACKET "]",
    );
}

#[test]
fn properties_run_to_the_colon() {
    let tokens = tree_tokens("[--a[--b)c:var(--d)]");
    assert_eq!(tokens[1], (TW_PROPERTY, "--a[--b)c".to_string()));
}

#[test]
fn data_is_a_basename() {
    assert!(parse_tailwind("data data-mt-4").diagnostics().is_empty());
    assert_tree_tokens!(
        "data data-mt-4",
        TW_BASE "data",
        WHITESPACE " ",
        TW_BASE "data",
        DASH "-",
        TW_VALUE "mt-4",
    );
}

#[test]
fn custom_base_names() {
    let base_names = BaseNameStore::new(&["slide-in-from-top"]);
    let parse = parse_tailwind_with_options(
        "slide-in-from-top-4",
        &mut NodeCache::default(),
        TailwindParserOptions::with_base_names(&base_names),
    );
    let base = parse
        .syntax()
        .descendants_tokens(biome_rowan::Direction::Next)
        .next()
        .unwrap();
    assert_eq!((base.kind(), base.text()), (TW_BASE, "slide-in-from-top"));
}

/// Asserts that parsing `source` gives a tree whose text is `source`.
fn check_parse(source: &str) {
    assert_eq!(parse_tailwind(source).syntax().to_string(), source);
}

/// Parses every fixture, and copies of each with a few edits that break it.
#[test]
fn fixtures_parse_losslessly() {
    let mut next = xorshift(0x2545_F491_4F6C_DD1D);
    for source in fixtures() {
        check_parse(&source);
        for _ in 0..3 {
            check_parse(&mutate(&source, &mut next));
        }
    }
}

/// Parses sequences of fragments that the lexer and the parser treat differently.
#[test]
fn random_fragments_parse_losslessly() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..3000 {
        check_parse(&random_fragments(&mut next));
    }
}
