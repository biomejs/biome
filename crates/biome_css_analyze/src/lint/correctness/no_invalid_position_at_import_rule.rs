use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssRule, CssRuleList};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};

declare_lint_rule! {
    /// Disallow the use of `@import` at-rules in invalid positions.
    ///
    /// Any `@import` rules must precede all other valid at-rules and style rules in a stylesheet (ignoring `@charset` and `@layer`), or else the `@import` rule is invalid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// a {}
    /// @import 'foo.css';
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @import 'foo.css';
    /// a {}
    /// ```
    ///
    pub NoInvalidPositionAtImportRule {
        version: "1.8.0",
        name: "noInvalidPositionAtImportRule",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("no-invalid-position-at-import-rule")],
    }
}

impl Rule for NoInvalidPositionAtImportRule {
    type Query = Ast<CssRuleList>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut is_invalid_position = false;
        let mut invalid_import_list = Vec::new();

        for rule in node {
            let any_css_at_rule = match rule {
                AnyCssRule::CssAtRule(item) => item.rule().ok(),
                _ => None,
            };

            if let Some(any_css_at_rule) = any_css_at_rule {
                // Ignore @charset, @layer
                if any_css_at_rule.as_css_charset_at_rule().is_some() {
                    continue;
                }
                if any_css_at_rule.as_css_layer_at_rule().is_some() {
                    continue;
                }

                let import_rule = any_css_at_rule.as_css_import_at_rule();
                if let Some(import_rule) = import_rule {
                    if is_invalid_position {
                        invalid_import_list.push(import_rule.range());
                    }
                } else {
                    is_invalid_position = true;
                }
            } else {
                is_invalid_position = true;
            }
        }
        invalid_import_list.into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "This "<Emphasis>"@import"</Emphasis>" is in the wrong position."
                },
            )
            .note(markup! {
                "Any "<Emphasis>"@import"</Emphasis>" rules must precede all other valid at-rules and style rules in a stylesheet (ignoring @charset and @layer), or else the "<Emphasis>"@import"</Emphasis>" rule is invalid."
            }).note(markup! {
                "Consider moving import position."
            })
        )
    }
}
