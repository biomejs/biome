//! Drives [BulkTokenSource] and the token source over [JsLexer] alike, and compares them.

use super::*;
use crate::lexer::JsxRawTextElement;
use crate::token_source::JsTokenSource;
use biome_parser::token_source::{BumpWithContext, NthToken, TokenSource, TokenSourceCheckpoint};
use std::path::Path;

/// A pseudo-random sequence, fixed by its seed.
struct Random(u64);

impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, bound: usize) -> usize {
        (self.next() % bound as u64) as usize
    }
}

fn context(random: &mut Random) -> JsLexContext {
    match random.below(16) {
        0 => JsLexContext::TemplateElement { tagged: false },
        1 => JsLexContext::TemplateElement { tagged: true },
        2 => JsLexContext::JsxChild { astro: false },
        3 => JsLexContext::JsxChild { astro: true },
        4 => JsLexContext::JsxAttributeValue { astro: false },
        5 => JsLexContext::JsxAttributeValue { astro: true },
        6 => JsLexContext::JsxRawText(JsxRawTextElement::Script),
        _ => JsLexContext::Regular,
    }
}

fn re_lex_context(random: &mut Random) -> JsReLexContext {
    match random.below(8) {
        0 => JsReLexContext::Regex,
        1 => JsReLexContext::BinaryOperator,
        2 => JsReLexContext::TypeArgumentLessThan,
        3 => JsReLexContext::JsxIdentifier,
        4 => JsReLexContext::JsxChild { astro: false },
        5 => JsReLexContext::AstroHtmlComment,
        6 => JsReLexContext::AstroJsxAttributeName,
        _ => JsReLexContext::JsxChild { astro: true },
    }
}

/// Applies `steps` random operations to both token sources over `source`, and asserts that they
/// agree after each, and on their trivia and diagnostics at the end.
fn compare(source: &str, source_type: JsFileSource, seed: u64, steps: usize) {
    let mut reference = JsTokenSource::from_str(source, &[]);
    let mut bulk = BulkTokenSource::new(source, &source_type, &[]);
    let mut random = Random(seed | 1);
    let mut checkpoints: Vec<(TokenSourceCheckpoint<_>, _)> = Vec::new();
    let mut log = Vec::new();
    for step in 0..steps {
        let operation = random.below(20);
        log.push(operation);
        let describe =
            || format!("step {step} {log:?} of seed {seed} in {source:?} ({source_type:?})");
        match operation {
            0..=7 => {
                let context = if operation < 5 {
                    JsLexContext::Regular
                } else {
                    context(&mut random)
                };
                reference.bump_with_context(context);
                bulk.bump_with_context(context);
            }
            8 | 9 => {
                let mode = re_lex_context(&mut random);
                let start = reference.current_range().start();
                assert_eq!(
                    reference.re_lex(mode),
                    bulk.re_lex(mode),
                    "re-lex {mode:?}, {}",
                    describe()
                );
                // Re-lexing can drop the diagnostics of the token it replaces, below the count
                // that a checkpoint at that token holds. [JsLexer] would then keep the
                // diagnostics of lookahead past a rewind to it, which the parser doesn't rely on.
                checkpoints.retain(|(checkpoint, _)| checkpoint.current_start() != start);
            }
            10 | 11 => {
                let n = 1 + random.below(4);
                assert_eq!(reference.nth(n), bulk.nth(n), "nth {n}, {}", describe());
                assert_eq!(
                    reference.nth_range(n),
                    bulk.nth_range(n),
                    "nth range {n}, {}",
                    describe()
                );
                assert_eq!(
                    reference.has_nth_preceding_line_break(n),
                    bulk.has_nth_preceding_line_break(n),
                    "nth line break {n}, {}",
                    describe()
                );
            }
            12 => assert_eq!(
                reference.has_next_preceding_trivia(),
                bulk.has_next_preceding_trivia(),
                "{}",
                describe()
            ),
            13 | 14 => checkpoints.push((reference.checkpoint(), bulk.checkpoint())),
            15 | 16 if !checkpoints.is_empty() => {
                let index = random.below(checkpoints.len());
                let (to_reference, to_bulk) = checkpoints.swap_remove(index);
                // Later checkpoints may lie past the rewound position.
                checkpoints.retain(|(checkpoint, _)| {
                    checkpoint.trivia_position() <= to_reference.trivia_position()
                        && checkpoint.current_start() <= to_reference.current_start()
                });
                reference.rewind(to_reference);
                bulk.rewind(to_bulk);
            }
            17 => {
                let context = context(&mut random);
                reference.skip_as_trivia_with_context(context);
                bulk.skip_as_trivia_with_context(context);
            }
            _ => {}
        }
        assert_eq!(
            reference.current(),
            bulk.current(),
            "current, {}",
            describe()
        );
        assert_eq!(
            reference.current_range(),
            bulk.current_range(),
            "range, {}",
            describe()
        );
        assert_eq!(
            reference.has_preceding_line_break(),
            bulk.has_preceding_line_break(),
            "line break, {}",
            describe()
        );
        assert_eq!(
            reference.has_unicode_escape(),
            bulk.has_unicode_escape(),
            "unicode escape, {}",
            describe()
        );
        if reference.current() == EOF && random.below(4) == 0 {
            break;
        }
    }
    let (reference_trivia, reference_diagnostics) = reference.finish();
    let (bulk_trivia, bulk_diagnostics) = bulk.finish();
    let trivia = |trivia: Vec<Trivia>| -> Vec<_> {
        trivia
            .iter()
            .map(|trivia| (trivia.kind(), trivia.text_range(), trivia.trailing()))
            .collect()
    };
    assert_eq!(
        trivia(reference_trivia),
        trivia(bulk_trivia),
        "trivia of seed {seed} in {source:?}"
    );
    let diagnostics = |diagnostics: Vec<ParseDiagnostic>| -> Vec<_> {
        diagnostics
            .iter()
            .map(|diagnostic| format!("{diagnostic:?}"))
            .collect()
    };
    assert_eq!(
        diagnostics(reference_diagnostics),
        diagnostics(bulk_diagnostics),
        "diagnostics of seed {seed} in {source:?}"
    );
}

fn source_types() -> [JsFileSource; 4] {
    [
        JsFileSource::js_module(),
        JsFileSource::ts(),
        JsFileSource::jsx(),
        JsFileSource::tsx(),
    ]
}

#[test]
fn operations_on_snippets() {
    for source in [
        "a >>= b; a >>>= b; a >= b; a >> b; a >>> b; a > b; a > > b; a >\n= b;",
        "type T = A<B<C>>; let x: Array<Array<number>> = []; f<<T>() => T>(); a << b; 1 << 0;",
        "x = /re/g; y = a / b / c; z = (a) / 2; w = [1] / 2; v = {} / 2; u = `${a}` / 2;",
        "if (a) /'/.test(b); c = 'd'; e = f\n/g/h; return /a/; typeof /b/; x++ / 2;",
        "a = /[/]/; b = /\\//; c = /a/gg; d = /a/uv; e = /\n/; f = a /= 2; g = /=/;",
        "a = `b${c}d${`e${f}`}g`; h = tag`\\unicode ${i}`; j = `${{ k: 1 }}`; l = `\\x4`;",
        "<div className=\"a\" {...b}>text's {c} &amp; <br/> </div>; <A<B>>(c) => <d-e f:g='h'/>;",
        "<>{a}</>; <a b={c}/>; <a>{/* c */}</a>; x = <a>{`t`}</a>; y = a </b>c/; <a>></a>",
        "\u{FEFF}#!/usr/bin/env node\nlet a = 1 ?? 2; \\u0061 = \\u{62}; é = 1; '\\x4'; 0x;",
        "<script>if (a < b) {}</script><!-- c --><d 'e=1 f=g/>{h}<i is:raw>{</i>",
    ] {
        for source_type in source_types() {
            for seed in 0..64 {
                compare(source, source_type, seed, 200);
            }
        }
    }
}

#[test]
fn operations_on_spec_fixtures() {
    let mut count = 0;
    let mut stack = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("tests")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let Some(source_type) = camino::Utf8Path::from_path(&path)
                .and_then(|path| JsFileSource::try_from(path).ok())
            else {
                continue;
            };
            let Ok(source) = std::fs::read_to_string(&path) else {
                continue;
            };
            for seed in 0..4 {
                compare(&source, source_type, seed, 400);
            }
            count += 1;
        }
    }
    assert!(count > 700, "only found {count} fixtures");
}

#[test]
fn operations_on_random_fragments() {
    const FRAGMENTS: &[&str] = &[
        " ", "\n", "a", "if", "return", "1", "1.5", "'s'", "\"", "`", "`t`", "${", "}", "{", "(",
        ")", ";", ",", ".", "=", "==", "=>", "+", "++", "/", "/=", "<", "<<", "<=", ">", ">>",
        ">=", ">>=", "?", "?.", ":", "#", "//c\n", "/*c*/", "/a/g", "\\u0061", "é", "<div>",
        "</div>", "/>", "<T>", "a-b", "<!--", "-->", "\u{2028}", "'\\x4'",
    ];
    let mut random = Random(0x2545_F491_4F6C_DD1D);
    for round in 0..2000 {
        let count = 1 + random.below(30);
        let source: String = (0..count)
            .map(|_| FRAGMENTS[random.below(FRAGMENTS.len())])
            .collect();
        for source_type in source_types() {
            compare(&source, source_type, round, 300);
        }
    }
}
