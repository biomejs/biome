use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_css_syntax::{AnyCssRule, CssImportAtRule, CssRuleList};
use biome_rowan::AstNode;

declare_rule! {
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
        version: "next",
        name: "noInvalidPositionAtImportRule",
        recommended: true,
        sources: &[RuleSource::Stylelint("no-invalid-position-at-import-rule")],
    }
}

impl Rule for NoInvalidPositionAtImportRule {
    type Query = Ast<CssRuleList>;
    type State = CssImportAtRule;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let mut is_invalid_position = false;

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

                let import_rule = any_css_at_rule.as_css_import_at_rule().cloned();
                if let Some(import_rule) = import_rule {
                    if is_invalid_position {
                        return Some(import_rule);
                    }
                } else {
                    is_invalid_position = true;
                }
            } else {
                is_invalid_position = true;
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, node: &Self::State) -> Option<RuleDiagnostic> {
        let span = node.range();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "This @import is in the wrong position."
                },
            )
            .note(markup! {
                    "Consider moving import position."
            }).note(markup! {
                "Any @import rules must precede all other valid at-rules and style rules in a stylesheet (ignoring @charset and @layer), or else the @import rule is invalid."
            }),
        )
    }
}
