use biome_analyze::{context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic};
use biome_analyze::{RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_rowan::{AstNode, Direction, SyntaxTriviaPiece, TextRange};

const IRREGULAR_WHITESPACES: &[char; 22] = &[
    '\u{c}', '\u{b}', '\u{85}', '\u{feff}', '\u{a0}', '\u{1680}', '\u{180e}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200a}', '\u{200b}', '\u{202f}', '\u{205f}', '\u{3000}',
];

declare_lint_rule! {
    /// Disallows the use of irregular whitespace characters.
    ///
    /// Invalid or irregular whitespace causes issues with various parsers and also makes code harder to debug.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// letcount;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let foo;
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
        version: "1.9.0",
        name: "noIrregularWhitespace",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-irregular-whitespace")],
        source_kind: RuleSourceKind::SameLogic,
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyJsRoot>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        get_irregular_whitespace(ctx.query().syntax()).into_boxed_slice()
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
            .note(markup!{
                "Irregular whitespaces can cause issues to other parsers, and make the code harder to debug."
            })
            .note(markup! {
                "Replace the irregular whitespaces with normal whitespaces or tabs."
            }),
        )
    }
}

fn get_irregular_whitespace(syntax: &JsSyntaxNode) -> Vec<TextRange> {
    let mut all_whitespaces_trivia: Vec<SyntaxTriviaPiece<JsLanguage>> = vec![];
    let is_whitespace = |trivia: &SyntaxTriviaPiece<JsLanguage>| {
        trivia.is_whitespace() && !trivia.text().replace(' ', "").is_empty()
    };

    for token in syntax.descendants_tokens(Direction::Next) {
        let leading_trivia_pieces = token.leading_trivia().pieces();
        let trailing_trivia_pieces = token.trailing_trivia().pieces();

        for trivia in leading_trivia_pieces {
            if is_whitespace(&trivia) {
                all_whitespaces_trivia.push(trivia);
            }
        }

        for trivia in trailing_trivia_pieces {
            if is_whitespace(&trivia) {
                all_whitespaces_trivia.push(trivia);
            }
        }
    }

    all_whitespaces_trivia
        .iter()
        .filter_map(|trivia| {
            let has_irregular_whitespace = trivia.text().chars().any(|char| {
                IRREGULAR_WHITESPACES
                    .iter()
                    .any(|irregular_whitespace| &char == irregular_whitespace)
            });
            has_irregular_whitespace.then(|| trivia.text_range())
        })
        .collect::<Vec<TextRange>>()
}
