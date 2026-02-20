use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssRule, CssSyntaxNode};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, Direction, TextRange};
use biome_rule_options::no_irregular_whitespace::NoIrregularWhitespaceOptions;

declare_lint_rule! {
    /// Disallows the use of irregular whitespace characters.
    ///
    /// Using irregular whitespace would lead to the failure of selecting the correct target.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// .firstClass.secondClass {
    ///   color: red;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// .firstClass .secondClass {
    ///   color: red;
    /// }
    /// ```
    ///
    pub NoIrregularWhitespace {
        version: "1.9.0",
        name: "noIrregularWhitespace",
        language: "css",
        sources: &[RuleSource::Stylelint("no-irregular-whitespace").same()],
        recommended: true,
        severity: Severity::Warning,
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyCssRule>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoIrregularWhitespaceOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        get_irregular_whitespace(ctx.query().syntax()).into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Irregular whitespace found."
                },
            )
            .note(markup! {
                    "Replace the irregular whitespace with normal whitespaces."
            }),
        )
    }
}

fn is_irregular_whitespace(c: char) -> bool {
    matches!(
        c,
        '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{00A0}' | '\u{1680}' | '\u{180E}' | '\u{2000}'
            ..='\u{200B}' | '\u{202F}' | '\u{205F}' | '\u{3000}' | '\u{FEFF}'
    )
}

fn get_irregular_whitespace(syntax: &CssSyntaxNode) -> Vec<TextRange> {
    if !syntax
        .text_with_trivia()
        .chars()
        .any(is_irregular_whitespace)
    {
        return vec![];
    }

    let mut results = vec![];
    for token in syntax.descendants_tokens(Direction::Next) {
        if !token.has_leading_comments()
            && !token.has_trailing_comments()
            && token.text().chars().any(is_irregular_whitespace)
        {
            results.push(token.text_range());
        }
    }
    results
}
