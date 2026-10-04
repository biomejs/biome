use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{
    AnyCssAtRule, AnyCssRootItem, AnyCssRule, AnyScssImportItem, CssRootItemList,
};
use biome_diagnostics::Severity;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_invalid_position_at_import_rule::NoInvalidPositionAtImportRuleOptions;

declare_lint_rule! {
    /// Disallow `@import` after other rules.
    ///
    /// An `@import` must appear before style rules and most at-rules. Only `@charset` and `@layer`
    /// may appear before it. Browsers ignore an `@import` placed later in the stylesheet.
    /// Sass load imports are ignored because they don't emit CSS `@import` rules.
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
        sources: &[RuleSource::Stylelint("no-invalid-position-at-import-rule").same(), RuleSource::EslintCss("no-invalid-at-rule-placement").inspired()],
    }
}

impl Rule for NoInvalidPositionAtImportRule {
    type Query = Ast<CssRootItemList>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoInvalidPositionAtImportRuleOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let mut is_invalid_position = false;
        let mut invalid_import_list = Vec::new();

        for item in node {
            let at_rule = match item {
                AnyCssRootItem::AnyCssRule(AnyCssRule::CssAtRule(at_rule)) => at_rule.rule().ok(),
                AnyCssRootItem::ScssVariableDeclaration(_) => continue,
                _ => {
                    is_invalid_position = true;
                    continue;
                }
            };

            match at_rule {
                Some(AnyCssAtRule::CssCharsetAtRule(_) | AnyCssAtRule::CssLayerAtRule(_)) => {}
                Some(AnyCssAtRule::CssImportAtRule(import_rule)) => {
                    if is_invalid_position {
                        invalid_import_list.push(import_rule.range());
                    }
                }
                Some(AnyCssAtRule::ScssImportAtRule(import_rule)) => {
                    if is_invalid_position {
                        invalid_import_list.extend(import_rule.imports().into_iter().filter_map(
                            |item| {
                                let AnyScssImportItem::ScssPlainImport(import) = item.ok()? else {
                                    return None;
                                };
                                Some(import.range())
                            },
                        ));
                    }
                }
                Some(rule) if is_scss_at_rule(&rule) => {}
                _ => is_invalid_position = true,
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

fn is_scss_at_rule(rule: &AnyCssAtRule) -> bool {
    matches!(
        rule,
        AnyCssAtRule::ScssAtRootAtRule(_)
            | AnyCssAtRule::ScssContentAtRule(_)
            | AnyCssAtRule::ScssDebugAtRule(_)
            | AnyCssAtRule::ScssEachAtRule(_)
            | AnyCssAtRule::ScssErrorAtRule(_)
            | AnyCssAtRule::ScssExtendAtRule(_)
            | AnyCssAtRule::ScssForAtRule(_)
            | AnyCssAtRule::ScssForwardAtRule(_)
            | AnyCssAtRule::ScssFunctionAtRule(_)
            | AnyCssAtRule::ScssIfAtRule(_)
            | AnyCssAtRule::ScssImportAtRule(_)
            | AnyCssAtRule::ScssIncludeAtRule(_)
            | AnyCssAtRule::ScssMixinAtRule(_)
            | AnyCssAtRule::ScssReturnAtRule(_)
            | AnyCssAtRule::ScssUseAtRule(_)
            | AnyCssAtRule::ScssWarnAtRule(_)
            | AnyCssAtRule::ScssWhileAtRule(_)
    )
}
