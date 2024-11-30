use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssRule, CssLanguage};
use biome_rowan::{AstNode, Direction, SyntaxToken, TextRange};

const IRREGULAR_WHITESPACES: &[char; 22] = &[
    '\u{c}', '\u{b}', '\u{85}', '\u{feff}', '\u{a0}', '\u{1680}', '\u{180e}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200a}', '\u{200b}', '\u{202f}', '\u{205f}', '\u{3000}',
];

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
    /// ```css,expect_diagnostic
    /// .firstClass .secondClass {
    ///   color:red;
    /// }
    /// ```
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
        recommended: false,
        sources: &[RuleSource::Stylelint("no-irregular-whitespace")],
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<AnyCssRule>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        get_irregular_whitespace(node).into_boxed_slice()
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

fn get_irregular_whitespace(node: &AnyCssRule) -> Vec<TextRange> {
    let syntax = node.syntax();
    let mut all_whitespaces_token: Vec<TextRange> = vec![];
    let matches_irregular_whitespace = |token: &SyntaxToken<CssLanguage>| {
        !token.has_leading_comments()
            && !token.has_trailing_comments()
            && token.text().chars().any(|char| {
                IRREGULAR_WHITESPACES
                    .iter()
                    .any(|irregular_whitespace| &char == irregular_whitespace)
            })
    };

    for token in syntax.descendants_tokens(Direction::Next) {
        if matches_irregular_whitespace(&token) {
            all_whitespaces_token.push(token.text_range());
        }
    }

    all_whitespaces_token
}
