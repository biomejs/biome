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
        version: "next",
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
        let is_valid_context = is_inside_component_or_hook(call) || is_in_named_function(call);
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

fn is_inside_component_or_hook(call: &JsCallExpression) -> bool {
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

    outer_call
        .and_then(|call_expr| {
            call_expr
                .callee()
                .ok()
                .and_then(|callee| callee.as_js_reference_identifier())
                .and_then(|ident| ident.value_token().ok())
        })
        .is_some_and(|token| is_component_or_hook_name(token.text_trimmed()))
}

/// Returns true if the given identifier name represents a Qwik hook name.
///
/// In Qwik, hooks are functions whose names start with `use` and where the
/// first character after `use` is uppercase, for example `useSignal`.
fn is_qwik_hook_name(name: &str) -> bool {
    name.starts_with("use") && name.chars().nth(3).is_some_and(|c| c.is_uppercase())
}

/// Returns true if the identifier name is either `component$` or a Qwik hook.
fn is_component_or_hook_name(name: &str) -> bool {
    name == "component$" || is_qwik_hook_name(name)
}

/// Determines whether `call` is enclosed in a named function whose identifier
/// designates a valid reactive context for Qwik hooks.
///
/// For this rule, a "named function" is any function with an explicit
/// identifier (function declaration, named function expression, or a variable
/// binding for an arrow/function expression). If that identifier is
/// `component$` or another Qwik hook (a name matching `use[A-Z]...`), we treat
/// the context as valid because hooks are allowed inside components or other
/// hooks.
fn is_in_named_function(call: &JsCallExpression) -> bool {
    let function_name = call
        .syntax()
        .ancestors()
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

    function_name.is_some_and(|name| is_component_or_hook_name(name.text()))
}

fn is_from_qwik(binding: &biome_js_semantic::Binding) -> bool {
    const QWIK_IMPORT_NAMES: [&str; 2] = ["@builder.io/qwik", "qwik"];
    binding
        .syntax()
        .ancestors()
        .find_map(|ancestor| JsImport::cast(ancestor)?.source_text().ok())
        .is_some_and(|source| QWIK_IMPORT_NAMES.contains(&source.text()))
}
