use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsxAttributeValue, JsxAttribute};
use biome_rowan::AstNode;
use biome_rule_options::use_classlist::UseClasslistOptions;

declare_lint_rule! {
    /// Prefer using the classlist prop over the classnames helper.
    ///
    /// This rule is intended for use in Qwik applications to encourage the use of
    /// the built-in classlist prop instead of the classnames utility library.
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
    /// <div classlist={{ active: true, disabled: false }} />
    /// ```
 pub UseClasslist {
        version: "next",
        name: "useClasslist",
        language: "js",
        sources: &[RuleSource::EslintQwik("prefer-classlist").inspired()],
        recommended: true,
        severity: Severity::Warning,
        fix_kind: FixKind::None,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseClasslist {
    type Query = Ast<JsxAttribute>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseClasslistOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let attr = ctx.query();
        let name = attr.name().ok()?;
        let name_text = name.to_trimmed_text();
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
                "Use the classlist prop instead of using classnames. The classlist prop accepts an object { [class: string]: boolean } just like classnames."
            ),
        ))
    }
}
