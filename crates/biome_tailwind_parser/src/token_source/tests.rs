use super::{Grouping, TailwindTokenSource};
use crate::lexer::{BASENAME_STORE, BaseNameStore};
use crate::syntax::{
    group_css_unit, group_css_value, group_negative_base, group_regular, group_url_value,
    group_variant_segment,
};
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use crate::{TailwindParserOptions, parse_tailwind_with_options};
use biome_parser::token_source::TokenSource;
use biome_rowan::NodeCache;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_tailwind_syntax::{T, TextSize};
use quickcheck_macros::quickcheck;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

/// Returns the kind and length of each token of `source`, grouping each with `group`, including
/// trivia.
fn tokens(source: &str, group: Grouping) -> Vec<(TailwindSyntaxKind, TextSize)> {
    let mut tokens = TailwindTokenSource::from_str(source, &BASENAME_STORE);
    tokens.place_after(0, EOF);
    let mut result = Vec::new();
    loop {
        group(&mut tokens);
        if tokens.current() == EOF {
            return result;
        }
        result.push((tokens.current(), tokens.current_range().len()));
    }
}

/// Groups CSS values, with the unit after a dimension.
fn css(source: &mut TailwindTokenSource) {
    if source.current() == CSS_DIMENSION_VALUE {
        group_css_unit(source);
    } else {
        group_css_value(source);
    }
}

/// Groups regular tokens, with a negative candidate's basename after a `-`.
fn negative(source: &mut TailwindTokenSource) {
    if source.current() == T![-] {
        group_negative_base(source);
    } else {
        group_regular(source);
    }
}

/// Asserts that grouping a source gives tokens of the given kinds and lengths, which make up the
/// whole source.
macro_rules! assert_tokens {
    ($group:expr, $src:expr, $($kind:ident:$len:expr),* $(,)?) => {{
        let expected: Vec<(TailwindSyntaxKind, TextSize)> =
            vec![$(($kind, TextSize::from($len))),*];
        assert_eq!(tokens($src, $group), expected);
        assert_eq!(
            expected.iter().map(|(_, len)| *len).sum::<TextSize>(),
            TextSize::of($src),
        );
    }};
    ($src:expr, $($kind:ident:$len:expr),* $(,)?) => {
        assert_tokens!(group_regular, $src, $($kind:$len),*)
    };
}

// The tokens of a random string cover it, and grouping them terminates.
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
        .unwrap_or_else(|_| panic!("Grouping doesn't terminate on ->{string}<-"));

    let rebuilt: String = ranges.into_iter().map(|range| &string[range]).collect();
    string == rebuilt
}

#[test]
fn basic() {
    assert_tokens!("block", TW_BASE:5);
}

#[test]
fn basic_value() {
    assert_tokens!("text-sm", TW_BASE:4, DASH:1, TW_VALUE:2);
}

#[test]
fn basic_multiple() {
    assert_tokens!(
        "text-sm font-bold",
        TW_BASE:4,
        DASH:1,
        TW_VALUE:2,
        WHITESPACE:1,
        TW_BASE:4,
        DASH:1,
        TW_VALUE:4,
    );
}

#[test]
fn whitespace_includes_newlines() {
    assert_tokens!(" \t\r\n\n ", WHITESPACE:6);
}

#[test]
fn css_underscore_whitespace_has_its_own_kind() {
    assert_tokens!(css, "foo__bar", IDENT:3, CSS_WHITESPACE:2, IDENT:3);
}

#[test]
fn css_strings_stop_at_whitespace() {
    assert_tokens!(
        css,
        "\"foo bar\"",
        CSS_STRING_LITERAL:4,
        WHITESPACE:1,
        IDENT:3,
        CSS_STRING_LITERAL:1,
    );
}

#[test]
fn nested_brackets_stop_at_whitespace() {
    assert_tokens!(
        "[foo[bar baz]]",
        L_BRACKET:1,
        TW_SELECTOR:7,
        WHITESPACE:1,
        TW_BASE:5,
    );
}

#[test]
fn basic_modifier() {
    assert_tokens!(
        "bg-primary/10",
        TW_BASE:2,
        DASH:1,
        TW_VALUE:7,
        SLASH:1,
        TW_NUMBER:2,
    );
}

#[test]
fn basic_modifier_important() {
    assert_tokens!(
        "bg-primary/10!",
        TW_BASE:2,
        DASH:1,
        TW_VALUE:7,
        SLASH:1,
        TW_NUMBER:2,
        BANG:1,
    );
}

#[test]
fn variant() {
    assert_tokens!("hover:", TW_BASE:5, COLON:1);
}

#[test]
fn variant_multiple() {
    assert_tokens!("hover:focus:", TW_BASE:5, COLON:1, TW_BASE:5, COLON:1);
}

#[test]
fn variant_multiple_full() {
    assert_tokens!(
        "hover:focus:bg-red-500",
        TW_BASE:5,
        COLON:1,
        TW_BASE:5,
        COLON:1,
        TW_BASE:2,
        DASH:1,
        TW_VALUE:7,
    );
}

#[test]
fn negative_basename() {
    assert_tokens!(negative, "-mt", DASH:1, TW_BASE:2);
}

#[test]
fn negative_basename_cant_start_with_at() {
    assert_tokens!(negative, "-@x", DASH:1, ERROR_TOKEN:1, TW_BASE:1);
}

#[test]
fn byte_order_mark() {
    assert_tokens!("\u{FEFF}flex", UNICODE_BOM:3, TW_BASE:4);
}

#[test]
fn bracket_after_bracket() {
    assert_tokens!("[[a]", L_BRACKET:1, L_BRACKET:1, TW_SELECTOR:1, R_BRACKET:1);
}

#[test]
fn variant_segment_names() {
    assert_tokens!(
        group_variant_segment,
        "@sm-*%",
        TW_VARIANT_SEGMENT:3,
        DASH:1,
        TW_VARIANT_SEGMENT:2,
    );
    assert_tokens!(group_variant_segment, "%", ERROR_TOKEN:1);
}

#[test]
fn raw_urls_and_url_strings() {
    assert_tokens!(group_url_value, "a.png", CSS_URL_VALUE_RAW_LITERAL:5);
    assert_tokens!(group_url_value, "'a b'", CSS_STRING_LITERAL:2, WHITESPACE:1, CSS_URL_VALUE_RAW_LITERAL:2);
}

#[test]
fn css_punctuation() {
    assert_tokens!(css, "a=b,c", IDENT:1, EQ:1, IDENT:1, COMMA:1, IDENT:1);
}

#[test]
fn css_hex_colors_end_at_the_first_other_letter() {
    assert_tokens!(css, "#abcxyz", CSS_COLOR_LITERAL:4, IDENT:3);
}

#[test]
fn arbitrary_css_dimensions_and_percentages() {
    assert_tokens!(
        css,
        "12px 100% .5rem -1rem",
        CSS_DIMENSION_VALUE:2,
        PX_KW:2,
        WHITESPACE:1,
        CSS_PERCENTAGE_VALUE:3,
        PERCENT:1,
        WHITESPACE:1,
        CSS_DIMENSION_VALUE:2,
        REM_KW:3,
        WHITESPACE:1,
        CSS_DIMENSION_VALUE:2,
        REM_KW:3,
    );
}

#[test]
fn arbitrary_css_numbers_with_exponents() {
    assert_tokens!(
        css,
        "1e2px 1E+2% 1e-2 1e 1e+",
        CSS_DIMENSION_VALUE:3,
        PX_KW:2,
        WHITESPACE:1,
        CSS_PERCENTAGE_VALUE:4,
        PERCENT:1,
        WHITESPACE:1,
        CSS_NUMBER_LITERAL:4,
        WHITESPACE:1,
        CSS_DIMENSION_VALUE:1,
        IDENT:1,
        WHITESPACE:1,
        CSS_DIMENSION_VALUE:1,
        IDENT:1,
        PLUS:1,
    );
}

#[test]
fn arbitrary_css_underscores_are_whitespace() {
    assert_tokens!(
        css,
        "0px_2px_4px",
        CSS_DIMENSION_VALUE:1,
        PX_KW:2,
        CSS_WHITESPACE:1,
        CSS_DIMENSION_VALUE:1,
        PX_KW:2,
        CSS_WHITESPACE:1,
        CSS_DIMENSION_VALUE:1,
        PX_KW:2,
    );
}

#[test]
fn arbitrary_css_minus_after_value_is_an_operator() {
    assert_tokens!(
        css,
        "100svh-var 1px-2px 100-1 50%-1 (1)-1",
        CSS_DIMENSION_VALUE:3,
        SVH_KW:3,
        DASH:1,
        VAR_KW:3,
        WHITESPACE:1,
        CSS_DIMENSION_VALUE:1,
        PX_KW:2,
        DASH:1,
        CSS_DIMENSION_VALUE:1,
        PX_KW:2,
        WHITESPACE:1,
        CSS_NUMBER_LITERAL:3,
        DASH:1,
        CSS_NUMBER_LITERAL:1,
        WHITESPACE:1,
        CSS_PERCENTAGE_VALUE:2,
        PERCENT:1,
        DASH:1,
        CSS_NUMBER_LITERAL:1,
        WHITESPACE:1,
        L_PAREN:1,
        CSS_NUMBER_LITERAL:1,
        R_PAREN:1,
        DASH:1,
        CSS_NUMBER_LITERAL:1,
    );
}

/// Asserts that parsing `source` gives a tree whose text is `source`.
fn check_parse(source: &str, base_names: &BaseNameStore) {
    let parse = parse_tailwind_with_options(
        source,
        &mut NodeCache::default(),
        TailwindParserOptions::with_base_names(base_names),
    );
    assert_eq!(parse.syntax().to_string(), source);
}

/// Parses every fixture, and copies of each with a few edits that break it.
#[test]
fn fixtures_parse_losslessly() {
    let mut next = xorshift(0x2545_F491_4F6C_DD1D);
    for source in fixtures() {
        check_parse(&source, &BASENAME_STORE);
        for _ in 0..3 {
            check_parse(&mutate(&source, &mut next), &BASENAME_STORE);
        }
    }
}

/// Parses sequences of fragments that the groupings and the parser treat differently.
#[test]
fn random_fragments_parse_losslessly() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..3000 {
        check_parse(&random_fragments(&mut next), &BASENAME_STORE);
    }
}

#[test]
fn custom_base_names() {
    let base_names = BaseNameStore::new(&["slide-in-from-top"]);
    let source = TailwindTokenSource::from_str("slide-in-from-top-4", &base_names);
    assert_eq!(
        (source.current(), source.current_range().len()),
        (TW_BASE, TextSize::from(17))
    );
}
