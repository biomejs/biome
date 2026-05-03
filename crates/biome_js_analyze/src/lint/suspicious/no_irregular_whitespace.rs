use biome_analyze::RuleSource;
use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsRoot, JsSyntaxNode};
use biome_rowan::{AstNode, Direction, TextRange};
use biome_rule_options::no_irregular_whitespace::NoIrregularWhitespaceOptions;

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
        sources: &[RuleSource::Eslint("no-irregular-whitespace").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyJsRoot>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoIrregularWhitespaceOptions;

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

fn is_irregular_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{000B}'
            | '\u{000C}'
            | '\u{0085}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{180E}'
            | '\u{2000}'..='\u{200B}'
            | '\u{202F}'
            | '\u{205F}'
            | '\u{3000}'
            | '\u{FEFF}'
    )
}

fn get_irregular_whitespace(syntax: &JsSyntaxNode) -> Vec<TextRange> {
    if !syntax.text_with_trivia().chars().any(is_irregular_whitespace) {
        return vec![];
    }

    let mut results = vec![];
    for token in syntax.descendants_tokens(Direction::Next) {
        for trivia in token
            .leading_trivia()
            .pieces()
            .chain(token.trailing_trivia().pieces())
        {
            if trivia.is_whitespace()
                && trivia.text().chars().any(is_irregular_whitespace)
            {
                results.push(trivia.text_range());
            }
        }
    }
    results
}
