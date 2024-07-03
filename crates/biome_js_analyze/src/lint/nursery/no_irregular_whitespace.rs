use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::AnyJsStatement;
use biome_rowan::{AstNode, Direction};

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
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyJsStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        // let syntax = node.syntax();

        // for tree in syntax.descendants_tokens(Direction::Next) {
        //     dbg!(tree.leading_trivia(), tree.trailing_trivia());
        // }

        get_irregular_whitespace(node)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
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

fn get_irregular_whitespace(node: &AnyJsStatement) -> Option<()> {
    let syntax = node.syntax();
    let has_irregular_whitespace = syntax.descendants_tokens(Direction::Next).any(|token| {
        let pieces = token
            .leading_trivia()
            .pieces()
            .chain(token.trailing_trivia().pieces());

        pieces.filter(|trivia| trivia.is_whitespace()).any(
            |trivia: biome_rowan::SyntaxTriviaPiece<biome_js_syntax::JsLanguage>| {
                IRREGULAR_WHITESPACES.iter().any(|irregular_whitespace| {
                    trivia
                        .text()
                        .chars()
                        .any(|char| &char == irregular_whitespace)
                })
            },
        )
    });

    has_irregular_whitespace.then_some(())
}
