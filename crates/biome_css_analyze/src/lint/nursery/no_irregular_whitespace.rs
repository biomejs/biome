use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssSelector, CssSelectorList};
use biome_rowan::{AstNode, TextRange};

const IRREGULAR_WHITESPACES: &[char; 22] = &[
    '\u{c}', '\u{b}', '\u{85}', '\u{feff}', '\u{a0}', '\u{1680}', '\u{180e}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200a}', '\u{200b}', '\u{202f}', '\u{205f}', '\u{3000}',
];

declare_lint_rule! {
    /// Disallows the use of irregular whitespace.
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
        version: "1.8.0",
        name: "noIrregularWhitespace",
        language: "css",
        recommended: false,
        sources: &[RuleSource::Stylelint("no-irregular-whitespace")],
    }
}

impl Rule for NoIrregularWhitespace {
    type Query = Ast<CssSelectorList>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        for selector in node.clone() {
            match selector.ok()? {
                AnyCssSelector::CssBogusSelector(tokens) => {
                    let irregular_whitespaces: Vec<&char> = IRREGULAR_WHITESPACES
                        .iter()
                        .filter(|irregular_whitespace| {
                            tokens
                                .text()
                                .chars()
                                .any(|char| &char == *irregular_whitespace)
                        })
                        .collect();

                    if !irregular_whitespaces.is_empty() {
                        return Some(tokens.range());
                    }
                }
                AnyCssSelector::CssComplexSelector(sel) => {
                    let token = sel.combinator().ok()?;

                    let irregular_whitespaces: Vec<&char> = IRREGULAR_WHITESPACES
                        .iter()
                        .filter(|irregular_whitespace| {
                            token
                                .text()
                                .chars()
                                .any(|char| &char == *irregular_whitespace)
                        })
                        .collect();

                    if !irregular_whitespaces.is_empty() {
                        return Some(token.text_range());
                    }
                }
                _ => continue,
            }
        }

        None
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
