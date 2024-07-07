use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_analyze::{RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::JsModule;
use biome_rowan::{AstNode, Direction, TextRange};

const IRREGULAR_WHITESPACES: &[char; 22] = &[
    '\u{c}', '\u{b}', '\u{85}', '\u{feff}', '\u{a0}', '\u{1680}', '\u{180e}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200a}', '\u{200b}', '\u{202f}', '\u{205f}', '\u{3000}',
];

declare_lint_rule! {
    /// Disallows the use of irregular whitespace characters.
    ///
    /// Invalid or irregular whitespace causes issues with ECMAScript 5 parsers and also makes code harder to debug.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// constcount=1;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = 'thing';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const count = 1;
    /// ```
    ///
    /// ```js
    /// const foo = '';
    /// ```
    ///
    pub NoIrregularWhitespace {
        version: "next",
        name: "noIrregularWhitespace",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-irregular-whitespace")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<JsModule>;
    type State = TextRange;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        get_irregular_whitespace(node)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Irregular whitespaces found."
                },
            )
            .note(markup! {
                "Replace the irregular whitespaces with normal whitespaces or tabs."
            }),
        )
    }
}

fn get_irregular_whitespace(node: &JsModule) -> Vec<TextRange> {
    let syntax = node.syntax();

    let all_whitespaces_trivia = syntax
        .descendants_tokens(Direction::Next)
        .flat_map(|token| {
            token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
                .filter(|trivia| {
                    trivia.is_whitespace() && !trivia.text().replace(' ', "").is_empty()
                })
        });

    all_whitespaces_trivia
        .filter_map(|trivia| {
            let has_irregular_whitespace = trivia.text().chars().any(|char| {
                IRREGULAR_WHITESPACES
                    .iter()
                    .any(|irregular_whitespace| &char == irregular_whitespace)
            });
            has_irregular_whitespace.then_some(trivia.text_range())
        })
        .collect::<Vec<TextRange>>()
}
