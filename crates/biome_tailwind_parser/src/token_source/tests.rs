use super::{Grouping, TailwindTokenSource, lex_css_value, lex_regular};
use crate::legacy::lexer::{TailwindLexContext, TailwindLexer};
use crate::legacy::token_source::LegacyTokenSource;
use crate::lexer::{BASENAME_STORE, BaseNameStore};
use crate::syntax::{
    group_negative_base, group_property, group_selector, group_url_value, group_variant_segment,
};
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use crate::{TailwindParserOptions, legacy, parse_tailwind_with_options};
use biome_parser::lexer::Lexer;
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{NthToken, TokenSource};
use biome_rowan::NodeCache;
use biome_tailwind_syntax::T;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, *};
use biome_tailwind_syntax::TextRange;

/// Kinds of the token before, covering every kind a lexer context checks for.
const PREVIOUS: &[TailwindSyntaxKind] = &[
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

/// Each context of the reference lexer, the [Grouping] that groups the tokens it lexes, and the
/// kinds of the token before after which the parser uses it.
const GROUPINGS: [(TailwindLexContext, Grouping, &[TailwindSyntaxKind]); 7] = [
    (TailwindLexContext::Regular, lex_regular, PREVIOUS),
    (
        TailwindLexContext::SawNegative,
        group_negative_base,
        &[T![-]],
    ),
    (
        TailwindLexContext::ArbitraryVariant,
        group_selector,
        &[T!['[']],
    ),
    (
        TailwindLexContext::ArbitraryCandidate,
        group_property,
        &[T!['[']],
    ),
    (
        TailwindLexContext::VariantSegment,
        group_variant_segment,
        &[EOF, T![-]],
    ),
    (TailwindLexContext::CssValue, lex_css_value, PREVIOUS),
    (
        TailwindLexContext::CssUrlRawValue,
        group_url_value,
        PREVIOUS,
    ),
];

/// The longest source whose every offset [check_groupings] groups.
const MAX_EXHAUSTIVE_LEN: usize = 600;

/// Asserts that, for sources up to [MAX_EXHAUSTIVE_LEN] bytes, each grouping in [GROUPINGS]
/// groups the token at every offset after each kind of token that the parser uses it after like
/// the reference lexer lexes it in the grouping's context.
pub(crate) fn check_groupings(source: &str, base_names: &BaseNameStore) {
    if source.len() > MAX_EXHAUSTIVE_LEN {
        return;
    }
    let mut tokens = TailwindTokenSource::from_str(source, base_names);
    let offsets = (0..=source.len()).filter(|&offset| source.is_char_boundary(offset));
    for offset in offsets {
        for (context, group, previous_kinds) in GROUPINGS {
            for &previous in previous_kinds {
                let mut legacy = TailwindLexer::from_str(source).with_base_names(base_names);
                let expected_kind = legacy.lex_at(offset, previous, false, context);
                tokens.place_after(offset, previous);
                group(&mut tokens);
                let at = || format!("{context:?} after {previous:?} at {offset} of {source:?}");
                assert_eq!(
                    (tokens.current, tokens.current_range()),
                    (expected_kind, legacy.current_range()),
                    "{}",
                    at()
                );
                assert_eq!(
                    format!("{:?}", tokens.diagnostics),
                    format!("{:?}", legacy.diagnostics()),
                    "{}",
                    at()
                );
                assert_eq!(tokens.after_newline, legacy.after_newline(), "{}", at());
            }
        }
    }
}

/// Asserts that the parser gives the same tree and diagnostics for `source` as the reference
/// implementation.
fn check_parse(source: &str, base_names: &BaseNameStore) {
    let parse = parse_tailwind_with_options(
        source,
        &mut NodeCache::default(),
        TailwindParserOptions::with_base_names(base_names),
    );
    let expected = legacy::parse(source, base_names);
    assert!(
        format!("{:#?}", parse.syntax()) == format!("{:#?}", expected.syntax()),
        "the trees differ for {source:?}"
    );
    assert!(
        format!("{:?}", parse.diagnostics()) == format!("{:?}", expected.diagnostics()),
        "the diagnostics differ for {source:?}"
    );
}

/// The operations of a token source that [walk] takes, where `grouping` is an index into
/// [GROUPINGS].
trait Walk: TokenSource<Kind = TailwindSyntaxKind> {
    type Checkpoint;
    fn bump_with(&mut self, grouping: usize);
    fn skip_as_trivia_with(&mut self, grouping: usize);
    fn checkpoint(&self) -> Self::Checkpoint;
    fn rewind(&mut self, checkpoint: Self::Checkpoint);
    fn regroup_current(&mut self, grouping: usize);
    fn has_colon_before_whitespace(&self) -> bool;
    fn nth_token(&mut self, n: usize) -> Option<(TailwindSyntaxKind, TextRange)>;
}

impl Walk for TailwindTokenSource<'_> {
    type Checkpoint = super::TailwindTokenSourceCheckpoint;

    fn bump_with(&mut self, grouping: usize) {
        self.bump_with_context(GROUPINGS[grouping].1);
    }

    fn skip_as_trivia_with(&mut self, grouping: usize) {
        self.skip_as_trivia_with_context(GROUPINGS[grouping].1);
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.checkpoint()
    }

    fn rewind(&mut self, checkpoint: Self::Checkpoint) {
        self.rewind(checkpoint);
    }

    fn regroup_current(&mut self, grouping: usize) {
        self.regroup_current(GROUPINGS[grouping].1);
    }

    fn has_colon_before_whitespace(&self) -> bool {
        self.has_colon_before_whitespace()
    }

    fn nth_token(&mut self, n: usize) -> Option<(TailwindSyntaxKind, TextRange)> {
        self.lookahead(n).map(|token| (token.kind, token.range))
    }
}

impl Walk for LegacyTokenSource<'_> {
    type Checkpoint = crate::legacy::token_source::LegacyTokenSourceCheckpoint;

    fn bump_with(&mut self, grouping: usize) {
        self.bump_with_context(GROUPINGS[grouping].0);
    }

    fn skip_as_trivia_with(&mut self, grouping: usize) {
        self.skip_as_trivia_with_context(GROUPINGS[grouping].0);
    }

    fn checkpoint(&self) -> Self::Checkpoint {
        self.checkpoint()
    }

    fn rewind(&mut self, checkpoint: Self::Checkpoint) {
        self.rewind(checkpoint);
    }

    fn regroup_current(&mut self, grouping: usize) {
        self.re_lex_current_in_context(GROUPINGS[grouping].0);
    }

    fn has_colon_before_whitespace(&self) -> bool {
        self.has_colon_before_whitespace()
    }

    fn nth_token(&mut self, n: usize) -> Option<(TailwindSyntaxKind, TextRange)> {
        self.nth_range(n).map(|range| (self.nth(n), range))
    }
}

/// The token source's state after each step of a pseudo-random walk over its source, which ends
/// by bumping to the end of the source.
fn walk(mut tokens: impl Walk, seed: u64) -> Vec<String> {
    // Indices into `GROUPINGS`, with the regular tokens and variant segments more often. A
    // grouping that the parser doesn't use after the current token falls back to the regular one.
    let groupings = [0, 0, 0, 4, 4, 1, 2, 3, 5, 6];
    let mut next = xorshift(seed);
    let mut checkpoints = Vec::new();
    let mut log = Vec::new();
    for _ in 0..300 {
        log.push(format!(
            "{:?} {:?} {} {}",
            tokens.current(),
            tokens.current_range(),
            tokens.has_preceding_line_break(),
            tokens.has_colon_before_whitespace(),
        ));
        let mut grouping = groupings[(next() % groupings.len() as u64) as usize];
        match next() % 12 {
            0 => {
                for n in 1..4 {
                    log.push(format!("nth {n}: {:?}", tokens.nth_token(n)));
                }
            }
            1 => checkpoints.push(tokens.checkpoint()),
            2 => {
                if let Some(checkpoint) = checkpoints.pop() {
                    tokens.rewind(checkpoint);
                }
            }
            3 => {
                if tokens.current() != EOF {
                    if !GROUPINGS[grouping].2.contains(&EOF) {
                        grouping = 0;
                    }
                    tokens.regroup_current(grouping);
                }
            }
            4 => {
                if !GROUPINGS[grouping].2.contains(&tokens.current()) {
                    grouping = 0;
                }
                tokens.skip_as_trivia_with(grouping);
            }
            _ => {
                if tokens.current() == EOF && checkpoints.is_empty() {
                    break;
                }
                if !GROUPINGS[grouping].2.contains(&tokens.current()) {
                    grouping = 0;
                }
                tokens.bump_with(grouping);
            }
        }
    }
    while tokens.current() != EOF {
        tokens.bump();
    }
    let (trivia, diagnostics) = tokens.finish();
    log.push(format!("{trivia:?}"));
    log.push(format!("{diagnostics:?}"));
    log
}

/// Parses every fixture, and a copy of each with a few edits that break it.
#[test]
fn fixtures_parse_like_the_reference() {
    let mut next = xorshift(0x2545_F491_4F6C_DD1D);
    for source in fixtures() {
        check_parse(&source, &BASENAME_STORE);
        for _ in 0..3 {
            check_parse(&mutate(&source, &mut next), &BASENAME_STORE);
        }
    }
}

/// Parses sequences of fragments that the lexers and the parser treat differently.
#[test]
fn random_fragments_parse_like_the_reference() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..3000 {
        check_parse(&random_fragments(&mut next), &BASENAME_STORE);
    }
}

#[test]
fn contexts_parse_like_the_reference() {
    for source in [
        "[[data-state=open]_&]:flex [&>div]:p-4 has-[>svg]:p-0 [(x)]:a [[a]b]:c",
        "[color:red] [mask:circle_at_center] [--x:calc(1px+2%)] [a:url(x)] [:x]",
        "-mt-4 -translate-x-1/2 !-m-4 -[x] -(--y) -@x -*",
        "bg-[url(a.png)] bg-[url(\\'a\\')] bg-[url( a )] bg-[url()] bg-[url(var(--x))]",
        "bg-(color:--brand) bg-(--a:b) w-(--x) text-(length:var(--y))/50",
        "grid-cols-[repeat(3,minmax(0,1fr))] w-[calc(100%-1rem)] aspect-[16/9] w-[100svh-var(--h)]",
        "group-hover/menu:flex @sm/main:p-4 @[400px]:flex *:p-1 **:m-0 data-[a=b]:x",
        "hover:\nfocus:\r\nflex\n\x0Bp-4",
        "bg-[url(\nx)] bg-[url(\r\n)] bg-[url( \n x )] bg-(url(\nx))",
        "bg-red-500/[0.5] text-[#555]/[.5] shadow-[0_1px_2px_black] content-['a_b']",
    ] {
        check_parse(source, &BASENAME_STORE);
    }
}

#[test]
fn custom_base_names_parse_like_the_reference() {
    let base_names = BaseNameStore::new(&["slide-in-from-top", "bg-red", "data-x"]);
    check_parse(
        "slide-in-from-top-4 bg-red-500 data-x-1 hover:slide-in-from-top-2 data-[a]:flex",
        &base_names,
    );
}

/// Walks the token source and the reference one over the same sources with pseudo-random
/// groupings, lookahead, checkpoints, rewinds and regrouping.
#[test]
fn random_walks_match() {
    let mut next = xorshift(0x5851_F42D_4C95_7F2D);
    let mut sources = fixtures();
    sources.extend((0..2000).map(|_| random_fragments(&mut next)));
    for source in sources {
        for _ in 0..3 {
            let seed = next() | 1;
            let expected = walk(LegacyTokenSource::from_str(&source, &BASENAME_STORE), seed);
            let actual = walk(
                TailwindTokenSource::from_str(&source, &BASENAME_STORE),
                seed,
            );
            assert_eq!(actual, expected, "{source:?} with seed {seed:#x}");
        }
    }
}
