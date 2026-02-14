use crate::services::semantic::Semantic;
use biome_analyze::{Rule, RuleDiagnostic, RuleDomain, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsFunction, JsCallExpression, JsImport, JsVariableDeclarator};
use biome_rowan::AstNode;
use biome_rule_options::use_qwik_method_usage::UseQwikMethodUsageOptions;
declare_lint_rule! {
    /// Disallow `use*` hooks outside of `component$` or other `use*` hooks in Qwik applications.
    ///
    /// Ensures Qwik's lifecycle hooks are only used in valid reactive contexts.
    /// See [Qwik Component Lifecycle](https://qwik.dev/docs/components/lifecycle/) for proper hook usage.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useSignal } from "@builder.io/qwik";
    ///
    /// export const Counter = () => {
    ///   const count = useSignal(0);
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { component$, useSignal } from "@builder.io/qwik";
    ///
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
        version: "2.2.6",
        name: "useQwikMethodUsage",
        language: "jsx",
        sources: &[RuleSource::EslintQwik("use-method-usage").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Qwik],
    }
}

impl Rule for UseQwikMethodUsage {
    type Query = Semantic<JsCallExpression>;
    type State = biome_rowan::TextRange;
    type Signals = Option<Self::State>;
    type Options = UseQwikMethodUsageOptions;

    fn run(ctx: &biome_analyze::context::RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();
        let is_hook = is_qwik_hook(call, model)?;
        if !is_hook {
            return None;
        }
        let is_valid_context =
            is_inside_component_or_hook(call, model) || is_inside_component_or_hook_name(call);
        if is_valid_context {
            None
        } else {
            Some(call.range())
        }
    }

    fn diagnostic(
        _: &biome_analyze::context::RuleContext<Self>,
        range: &Self::State,
    ) -> Option<RuleDiagnostic> {
        RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Qwik hook detected outside of an allowed scope."
            },
        )
            .note(markup! {
            "Qwik's reactive hooks (functions starting with  "<Emphasis>"use*"</Emphasis>" followed by uppercase letter) must be:"
            "\n- Used exclusively within `component$` functions"
            "\n- Or encapsulated within other valid Qwik hooks"
        })
            .note(markup! {
            "Check the "<Hyperlink href="https://qwik.dev/docs/components/state/#use-methods">"Qwik documentation"</Hyperlink>"."
        })
            .into()
    }
}

fn is_qwik_hook(call: &JsCallExpression, model: &SemanticModel) -> Option<bool> {
    let ident = call.callee().ok()?.as_js_reference_identifier()?;
    let token = ident.value_token().ok()?;
    let name = token.text_trimmed();
    if !is_qwik_hook_name(name) {
        return Some(false);
    }
    let binding = model.binding(&ident)?;
    Some(is_from_qwik(&binding))
}

fn is_inside_component_or_hook(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let outer_call = call
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsFunction::cast)
        .and_then(|function| {
            function
                .syntax()
                .ancestors()
                .skip(1)
                .find_map(JsCallExpression::cast)
        });

    if let Some(call_expr) = outer_call
        && let Ok(callee) = call_expr.callee()
        && let Some(ident) = callee.as_js_reference_identifier()
    {
        // Check if this is component$ or a hook by name
        if let Ok(token) = ident.value_token() {
            let name = token.text_trimmed();
            if is_component_or_hook_name(name) {
                return true;
            }
        }

        // Check if this identifier is bound to component$ from Qwik
        // This handles aliased imports like: import { component$ as MyComponent } from "@builder.io/qwik"
        if let Some(binding) = model.binding(&ident)
            && is_from_qwik(&binding)
        {
            // Walk up to find the import specifier
            let mut current = binding.syntax().clone();
            while let Some(parent) = current.parent() {
                // Check if we've reached an import specifier that contains "component$"
                let text = parent.text_trimmed();
                if text.to_string().contains("component$") {
                    return true;
                }
                // Stop at the import statement level
                if JsImport::can_cast(parent.kind()) {
                    break;
                }
                current = parent;
            }
        }
    }

    false
}

fn is_qwik_hook_name(name: &str) -> bool {
    name.starts_with("use") && name.chars().nth(3).is_some_and(|c| c.is_uppercase())
}

fn is_component_or_hook_name(name: &str) -> bool {
    name == "component$" || is_qwik_hook_name(name)
}

fn is_inside_component_or_hook_name(call: &JsCallExpression) -> bool {
    let function_name = call
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsFunction::cast)
        .and_then(|func| match func {
            AnyJsFunction::JsFunctionDeclaration(decl) => decl
                .id()
                .ok()
                .and_then(|binding| binding.as_js_identifier_binding().cloned())
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.token_text_trimmed()),
            AnyJsFunction::JsFunctionExpression(expr) => expr
                .id()
                .and_then(|binding| binding.as_js_identifier_binding().cloned())
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.token_text_trimmed()),
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
                .map(|token| token.token_text_trimmed()),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(decl) => decl
                .id()
                .and_then(|binding| binding.as_js_identifier_binding().cloned())
                .and_then(|ident| ident.name_token().ok())
                .map(|token| token.token_text_trimmed()),
        });

    function_name.is_some_and(|name| is_qwik_hook_name(name.text()))
}

fn is_from_qwik(binding: &biome_js_semantic::Binding) -> bool {
    const QWIK_IMPORT_NAMES: [&str; 2] = ["@builder.io/qwik", "qwik"];
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| {
            let source_text = source.text();
            QWIK_IMPORT_NAMES.contains(&source_text)
        })
}
