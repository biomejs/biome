use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsxAttributeValue, JsxAttribute};
use biome_rowan::AstNode;
use biome_rule_options::use_qwik_classlist::UseQwikClasslistOptions;

declare_lint_rule! {
    /// Prefer using the `class` prop as a classlist over the classnames helper.
    ///
    /// This rule is intended for use in Qwik applications to encourage the use of
    /// the built-in `class` prop (which accepts a string, object, or array) instead of the classnames utility library.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div class={classnames({ active: true, disabled: false })} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div class={{ active: true, disabled: false }} />
    /// ```
 pub UseQwikClasslist {
        version: "next",
        name: "useQwikClasslist",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("prefer-classlist").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::None,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseQwikClasslist {
    type Query = Ast<JsxAttribute>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseQwikClasslistOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();
        let name = attr.name().ok()?;
        let name_token = name.name_token().ok()?;
        let name_text = name_token.token_text();
        if name_text == "class" || name_text == "className" {
            let value = attr.initializer()?.value().ok()?;
            if let AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_attr) = value {
                if let AnyJsExpression::JsCallExpression(call) = expr_attr.expression().ok()? {
                    let callee = call.callee().ok()?;
                    if let Some(ident) = callee.as_js_reference_identifier() {
                        if ident.value_token().ok()?.text() == "classnames" {
                            return Some(attr.range());
                        }
                    }
                }
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
            markup!(
                "Avoid using the classnames utility. The Qwik class prop natively supports strings, objects, and arrays, which enables better static analysis and reactivity."
            ),
        ))
    }
}
