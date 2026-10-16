use super::TailwindTokenSource;
use crate::lexer::{BASENAME_STORE, BaseNameStore, TailwindLexContext};
use crate::parse_with_parser;
use crate::parser::TailwindParser;
use crate::test_support::{fixtures, mutate, random_fragments, xorshift};
use biome_parser::prelude::BumpWithContext;
use biome_parser::token_source::{NthToken, TokenSource};
use biome_rowan::NodeCache;
use biome_tailwind_syntax::TailwindSyntaxKind::EOF;

/// Creates a token source over `source` with each lexer.
fn token_sources<'source>(
    source: &'source str,
    base_names: &'source BaseNameStore,
) -> [TailwindTokenSource<'source>; 2] {
    [
        TailwindTokenSource::legacy(source, base_names),
        TailwindTokenSource::simd(source, base_names),
    ]
}

/// Asserts that parsing `source` gives the same tree and diagnostics with both token sources.
fn check_parse(source: &str, base_names: &BaseNameStore) {
    let [legacy, simd] = token_sources(source, base_names).map(|token_source| {
        let parse = parse_with_parser(
            source,
            &mut NodeCache::default(),
            TailwindParser::with_token_source(token_source),
        );
        (
            format!("{:#?}", parse.syntax()),
            format!("{:?}", parse.diagnostics()),
        )
    });
    assert!(legacy.0 == simd.0, "the trees differ for {source:?}");
    assert!(legacy.1 == simd.1, "the diagnostics differ for {source:?}");
}

/// The token source's state after each step of a pseudo-random walk over its source, which ends
/// by bumping to the end of the source.
fn walk(mut tokens: TailwindTokenSource, seed: u64) -> Vec<String> {
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
                    log.push(format!(
                        "nth {n}: {:?} {:?} {}",
                        tokens.nth(n),
                        tokens.nth_range(n),
                        tokens.has_nth_preceding_line_break(n)
                    ));
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

/// Parses every fixture, and a copy of each with a few edits that break it, with both token
/// sources.
#[test]
fn fixtures_parse_the_same_with_both_token_sources() {
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
fn random_fragments_parse_the_same_with_both_token_sources() {
    let mut next = xorshift(0x9E37_79B9_7F4A_7C15);
    for _ in 0..3000 {
        check_parse(&random_fragments(&mut next), &BASENAME_STORE);
    }
}

#[test]
fn contexts_parse_the_same_with_both_token_sources() {
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
fn custom_base_names_parse_the_same_with_both_token_sources() {
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
            let [legacy, simd] =
                token_sources(&source, &BASENAME_STORE).map(|tokens| walk(tokens, seed));
            assert_eq!(legacy, simd, "{source:?} with seed {seed:#x}");
        }
    }
}
