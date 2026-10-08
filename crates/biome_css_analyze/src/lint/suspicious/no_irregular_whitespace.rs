use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssRoot, CssSyntaxKind, CssSyntaxNode};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, Direction, TextRange};
use biome_rule_options::no_irregular_whitespace::NoIrregularWhitespaceOptions;

declare_lint_rule! {
    /// Disallow whitespace characters that CSS does not treat as normal spaces.
    ///
    /// Some Unicode and control characters look like spaces but can change how a selector is
    /// parsed. The invalid example contains a vertical tab between the class selectors, so it does
    /// not behave like a normal descendant separator.
    ///
    /// Irregular whitespace inside strings and comments is allowed, because it doesn't affect how
    /// the stylesheet is parsed.
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
    /// ```css
    /// .firstClass::before {
    ///   content: "　";
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
    type Query = Ast<AnyCssRoot>;
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
        // The byte order mark is U+FEFF, but it's an encoding marker rather than whitespace.
        if token.kind() == CssSyntaxKind::UNICODE_BOM
            || token.has_leading_comments()
            || token.has_trailing_comments()
        {
            continue;
        }

        // Irregular whitespace inside a string is part of its value, so only its trivia is checked.
        if matches!(
            token.kind(),
            CssSyntaxKind::CSS_STRING_LITERAL | CssSyntaxKind::SCSS_STRING_CONTENT_LITERAL
        ) {
            for trivia in token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces())
            {
                if trivia.text().chars().any(is_irregular_whitespace) {
                    results.push(trivia.text_range());
                }
            }
        } else if token.text().chars().any(is_irregular_whitespace) {
            results.push(token.text_range());
        }
    }
    results
}
