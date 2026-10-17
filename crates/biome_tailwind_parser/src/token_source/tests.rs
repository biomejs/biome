use super::TailwindTokenSource;
use crate::legacy::token_source::LegacyTokenSource;
use crate::lexer::{BASENAME_STORE, BaseNameStore, TailwindLexContext};
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use crate::{TailwindParserOptions, legacy, parse_tailwind_with_options};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{NthToken, TokenSource};
use biome_rowan::NodeCache;
use biome_tailwind_syntax::TailwindSyntaxKind::{self, EOF};
use biome_tailwind_syntax::TextRange;

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

/// The operations of a token source that [walk] takes.
trait Walk:
    TokenSource<Kind = TailwindSyntaxKind> + BumpWithContext<Context = TailwindLexContext>
{
    type Checkpoint;
    fn checkpoint(&self) -> Self::Checkpoint;
    fn rewind(&mut self, checkpoint: Self::Checkpoint);
    fn re_lex_current_in_context(&mut self, context: TailwindLexContext);
    fn has_colon_before_whitespace(&self) -> bool;
    fn nth_token(&mut self, n: usize) -> Option<(TailwindSyntaxKind, TextRange)>;
}

impl Walk for TailwindTokenSource<'_> {
    type Checkpoint = super::TailwindTokenSourceCheckpoint;

    fn checkpoint(&self) -> Self::Checkpoint {
        self.checkpoint()
    }

    fn rewind(&mut self, checkpoint: Self::Checkpoint) {
        self.rewind(checkpoint);
    }

    fn re_lex_current_in_context(&mut self, context: TailwindLexContext) {
        self.re_lex_current_in_context(context);
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

    fn checkpoint(&self) -> Self::Checkpoint {
        self.checkpoint()
    }

    fn rewind(&mut self, checkpoint: Self::Checkpoint) {
        self.rewind(checkpoint);
    }

    fn re_lex_current_in_context(&mut self, context: TailwindLexContext) {
        self.re_lex_current_in_context(context);
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
    let contexts = [
        TailwindLexContext::Regular,
        TailwindLexContext::Regular,
        TailwindLexContext::Regular,
        TailwindLexContext::VariantSegment,
        TailwindLexContext::VariantSegment,
        TailwindLexContext::SawNegative,
        TailwindLexContext::ArbitraryVariant,
        TailwindLexContext::ArbitraryCandidate,
        TailwindLexContext::CssValue,
        TailwindLexContext::CssUrlRawValue,
    ];
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
        let context = contexts[(next() % contexts.len() as u64) as usize];
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
                    tokens.re_lex_current_in_context(context);
                }
            }
            4 => tokens.skip_as_trivia_with_context(context),
            _ => {
                if tokens.current() == EOF && checkpoints.is_empty() {
                    break;
                }
                tokens.bump_with_context(context);
            }
        }
    }
    while tokens.current() != EOF {
        tokens.bump_with_context(TailwindLexContext::Regular);
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

/// Walks both token sources over the same sources with pseudo-random contexts, lookahead,
/// checkpoints, rewinds and re-lexing.
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
