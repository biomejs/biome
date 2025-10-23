use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, AnyJsxAttributeValue, JsxAttribute};
use biome_rowan::AstNode;
use biome_rule_options::use_qwik_classlist::UseQwikClasslistOptions;

declare_lint_rule! {
    /// Prefer using the `class` prop as a classlist over the `classnames` helper.
    ///
    /// This rule encourages the use of `class` prop which natively supports strings, objects, and arrays, enabling fine-grained reactivity and optimal performance. Using utilities like `classnames` can interfere with Qwik's reactivity model and prevent the framework from optimizing component updates. Prefer using the built-in `class` prop for best results.
    ///
    /// For more information, see: [Qwik documentation on class bindings](https://qwik.dev/docs/components/rendering/#class-and-style-bindings)
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
        version: "2.1.4",
        name: "useQwikClasslist",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("prefer-classlist").same()],
        recommended: true,
        severity: Severity::Error,
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
            let expr_attr = match value {
                AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_attr) => expr_attr,
                _ => return None,
            };
            let call = match expr_attr.expression().ok()? {
                AnyJsExpression::JsCallExpression(call) => call,
                _ => return None,
            };
            let callee = call.callee().ok()?;
            let ident = callee.as_js_reference_identifier()?;
            if ident.value_token().ok()?.text_trimmed() == "classnames" {
                return Some(attr.range());
            }
        }
        None
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Avoid third-party "<Emphasis>"classnames"</Emphasis>" utilities with Qwik components"
            },
        )
        .note(markup! {
            "Qwik's built-in "<Emphasis>"class"</Emphasis>" prop handles:"
            "\n- Conditional classes via objects: "<Emphasis>"class={{ active: isActive }}"</Emphasis>
            "\n- Dynamic string concatenation"
            "\n- Array combinations"
            "\n\nExternal utilities break Qwik's:"
            "\n- Fine-grained reactivity tracking"
            "\n- Resumability optimizations"
        })
        .note(markup! {
            "Use native Qwik class binding as shown in "<Hyperlink href="https://qwik.dev/docs/components/tasks/">"Qwik Rendering: Class Bindings (Official Docs)."</Hyperlink>
        })
        .into()
    }
}
