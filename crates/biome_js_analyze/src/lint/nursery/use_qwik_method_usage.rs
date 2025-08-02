use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsFunction, JsCallExpression, JsVariableDeclarator};
use biome_rowan::AstNode;
use biome_rule_options::use_qwik_method_usage::UseQwikMethodUsageOptions;
declare_lint_rule! {
    /// Disallow use* hooks outside of component$ or other use* hooks.
    ///
    /// Ensures Qwik's lifecycle hooks are only used in valid reactive contexts.
    /// See [Qwik Component Lifecycle](https://qwik.dev/docs/components/lifecycle/) for proper hook usage.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// export const Counter = () => {
    ///   const count = useSignal(0);
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// export const Counter = component$(() => {
    ///   const count = useSignal(0);
    /// });
    ///
    /// export const useCounter = () => {
    ///   const count = useSignal(0);
    ///   return count;
    /// };
    /// ```
    pub UseQwikMethodUsage {
        version: "1.0.0",
        name: "useQwikMethodUsage",
        language: "js",
        sources: &[RuleSource::EslintQwik("use-qwik-method-usage").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseQwikMethodUsage {
    type Query = Ast<JsCallExpression>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseQwikMethodUsageOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();

        if !is_qwik_hook(call)? {
            return None;
        }

        if is_inside_component_or_hook(call) {
            return None;
        }

        if is_in_named_function(call) {
            return None;
        }
        Some(call.range())
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Invalid usage of "<Emphasis>"use*"</Emphasis>" method"
            },
        )
            .note(markup! {
            "Qwik's "<Emphasis>"use*"</Emphasis>" methods are special hooks that:"
            "\n- Require component context for proper reactivity"
            "\n- Need access to Qwik's runtime capabilities"
            "\n- Must follow Qwik's resumable execution rules"
        })
            .note(markup! {
            "Check the "<Hyperlink href="https://qwik.dev/docs/components/state/#use-methods">"Qwik documentation"</Hyperlink>"."
        })
            .into()
    }
}

fn is_qwik_hook(call: &JsCallExpression) -> Option<bool> {
    let binding = call
        .callee()
        .ok()?
        .as_js_reference_identifier()?
        .value_token()
        .ok()?;
    let name = binding.text();
    Some(name.starts_with("use") && name.chars().nth(3).map_or(false, |c| c.is_uppercase()))
}

fn is_inside_component_or_hook(call: &JsCallExpression) -> bool {
    call.syntax()
        .ancestors()
        .find_map(AnyJsFunction::cast)
        .and_then(|function| {
            function
                .syntax()
                .ancestors()
                .find_map(JsCallExpression::cast)
        })
        .and_then(|call_expr| {
            call_expr
                .callee()
                .ok()
                .and_then(|callee| callee.as_js_reference_identifier())
                .and_then(|ident| ident.value_token().ok())
                .map(|token| {
                    let name = token.text();
                    name == "component$"
                        || (name.starts_with("use")
                            && name.chars().nth(3).map_or(false, |c| c.is_uppercase()))
                })
        })
        .unwrap_or(false)
}

fn is_in_named_function(call: &JsCallExpression) -> bool {
    call.syntax()
        .ancestors()
        .find_map(AnyJsFunction::cast)
        .and_then(|func| match func {
            AnyJsFunction::JsFunctionDeclaration(decl) => decl
                .id()
                .ok()
                .and_then(|binding| binding.as_js_identifier_binding().cloned())
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.text().to_string()),
            AnyJsFunction::JsFunctionExpression(expr) => expr
                .id()
                .and_then(|binding| binding.as_js_identifier_binding().cloned())
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.text().to_string()),
            AnyJsFunction::JsArrowFunctionExpression(_) => func
                .syntax()
                .ancestors()
                .find(|a| JsVariableDeclarator::can_cast(a.kind()))
                .and_then(JsVariableDeclarator::cast)
                .and_then(|decl| decl.id().ok())
                .and_then(|binding_pattern| {
                    binding_pattern
                        .as_any_js_binding()
                        .and_then(|binding| binding.as_js_identifier_binding().cloned())
                })
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.text().to_string()),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(decl) => decl
                .id()
                .and_then(|binding| binding.as_js_identifier_binding().cloned())
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.text().to_string()),
        })
        .map(|name| {
            name == "component$"
                || (name.starts_with("use")
                    && name.chars().nth(3).map_or(false, |c| c.is_uppercase()))
        })
        .unwrap_or(false)
}
