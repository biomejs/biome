use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_css_syntax::{AnyCssUnknownAtRuleName, CssUnknownBlockAtRule, CssUnknownValueAtRule};
use biome_diagnostics::Severity;
use biome_languages::CssFileSource;
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_unknown_at_rules::NoUnknownAtRulesOptions;

/// Tailwind at-rules that Biome does not model as dedicated nodes, so they are
/// parsed as unknown at-rules. They are valid when Tailwind directives are
/// enabled and should not be reported.
const TAILWIND_AT_RULES: &[&str] = &["tailwind"];

declare_lint_rule! {
    /// Disallow unknown at-rules.
    ///
    /// For details on known at-rules, see the [MDN web docs](https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```css,expect_diagnostic
    /// @uNkNoWn {}
    /// ```
    ///
    /// ```css,expect_diagnostic
    /// @unknown-at-rule {
    ///   font-size: 14px;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```css
    /// @charset 'UTF-8';
    /// ```
    ///
     /// ```css
     /// @media (max-width: 960px) {
     ///   body {
     ///     font-size: 13px;
     ///   }
     /// }
     /// ```
     ///
     /// ## Options
     ///
     /// ### `ignore`
     ///
     /// A list of unknown at-rule names to ignore (case-insensitive).
     ///
     /// ```json,options
     /// {
     ///   "options": {
     ///     "ignore": [
     ///       "custom-at-rule",
     ///       "my-custom-rule"
     ///     ]
     ///   }
     /// }
     /// ```
     ///
     /// #### Valid
     ///
     /// ```css,use_options
     /// @custom-at-rule {}
     /// @my-custom-rule {
     ///   color: red;
     /// }
     /// ```
     pub NoUnknownAtRules {
        version: "2.0.0",
        name: "noUnknownAtRules",
        language: "css",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Stylelint("at-rule-no-unknown").same()],
    }
}

declare_node_union! {
  pub AnyUnknownAtRule = CssUnknownBlockAtRule | CssUnknownValueAtRule
}

/// Returns a static name for rules like `@unknown`; dynamic Sass names such as
/// `@#{$rule-name}` are not comparable with the configured ignore list.
fn static_unknown_at_rule_name(name: AnyCssUnknownAtRuleName) -> Option<(TextRange, String)> {
    let identifier = name.as_css_identifier()?;
    Some((
        identifier.range(),
        identifier.value_token().ok()?.text_trimmed().to_string(),
    ))
}

/// Determines if the given unknown at-rule name should be ignored.
fn should_ignore(name: &str, options: &NoUnknownAtRulesOptions) -> bool {
    for ignore_pattern in &options.ignore {
        if name.eq_ignore_ascii_case(ignore_pattern) {
            return true;
        }
    }
    false
}

pub struct NoUnknownAtRuleState {
    range: TextRange,
    name: String,
}

impl Rule for NoUnknownAtRules {
    type Query = Ast<AnyUnknownAtRule>;
    type State = NoUnknownAtRuleState;
    type Signals = Option<Self::State>;
    type Options = NoUnknownAtRulesOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let rule = match node {
            AnyUnknownAtRule::CssUnknownBlockAtRule(rule) => rule.name().ok()?,
            AnyUnknownAtRule::CssUnknownValueAtRule(rule) => rule.name().ok()?,
        };
        let (range, name) = static_unknown_at_rule_name(rule)?;

        // Check if this unknown at-rule should be ignored
        if should_ignore(&name, ctx.options()) {
            return None;
        }

        // Tailwind at-rules such as `@tailwind base;` are valid when Tailwind
        // directives are enabled, but Biome parses them as unknown at-rules.
        if ctx.source_type::<CssFileSource>().is_tailwind_css()
            && TAILWIND_AT_RULES
                .iter()
                .any(|rule| name.eq_ignore_ascii_case(rule))
        {
            return None;
        }

        Some(NoUnknownAtRuleState {
            range,
            name,
        })
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let span = state.range;
        let name = &state.name;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                span,
                markup! {
                    "Unexpected unknown at-rule: "<Emphasis>{ name }</Emphasis>" "
                },
            )
            .note(markup! {
                    ""<Emphasis>{ name }</Emphasis>" is not a standard CSS at-rule, which may lead to unexpected styling results or failure to interpret the styles as intended."
            })
            .note(markup! {
                "See "<Hyperlink href="https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule">"MDN web docs"</Hyperlink>" for a known list of at-rules."
            }).note(markup! {
            "To fix this issue, consider removing the unknown at-rule."
        })
        )
    }
}
