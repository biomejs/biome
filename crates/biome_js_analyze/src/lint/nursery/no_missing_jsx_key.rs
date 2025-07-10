use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsCallExpression, jsx_ext::AnyJsxElement};
use biome_rowan::AstNode;
use biome_rule_options::no_missing_jsx_key::NoMissingJsxKeyOptions;

declare_lint_rule! {
    /// Disallow missing key props in iterators/collection literals.
    ///
    /// This rule is intended for use in Qwik applications to prevent missing key props in JSX elements inside iterators.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// {items.map(item => <li>{item}</li>)}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// {items.map(item => <li key={item.id}>{item}</li>)}
    /// ```
    pub NoMissingJsxKey {
        version: "1.0.0",
        name: "noMissingJsxKey",
        language: "js",
        sources: &[RuleSource::EslintQwik("jsx-key").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::None,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for NoMissingJsxKey {
    type Query = Ast<AnyJsxElement>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = NoMissingJsxKeyOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let mut current = element.syntax().parent();
        let mut is_in_map = false;
        while let Some(node) = current {
            if let Some(call) = JsCallExpression::cast(node.clone()) {
                if let Ok(AnyJsExpression::JsStaticMemberExpression(member)) = call.callee() {
                    let is_map = member
                        .member()
                        .ok()
                        .and_then(|name| {
                            name.as_js_name().and_then(|js_name| {
                                js_name
                                    .value_token()
                                    .ok()
                                    .map(|token| token.text() == "map")
                            })
                        })
                        .unwrap_or(false);
                    if is_map {
                        is_in_map = true;
                        break;
                    }
                }
            }
            current = node.parent();
        }
        if is_in_map {
            let has_key = element.find_attribute_by_name("key").is_some();
            if !has_key {
                return Some(element.range());
            }
        }
        None
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup!("Missing <Emphasis>key</Emphasis> prop for element in iterator."),
        ))
    }
}
