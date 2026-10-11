use biome_analyze::{Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayBindingPatternElement, AnyJsBinding, AnyJsBindingPattern, AnyJsNamedImportSpecifier,
    AnyJsObjectBindingPatternMember, JsArrayBindingPattern, JsCallExpression, JsIdentifierBinding,
    JsIdentifierExpression, JsImport, JsInitializerClause, JsObjectBindingPattern, JsSpread,
    JsSyntaxNode, JsSyntaxToken, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_tanstack_query_rest_destructuring::NoTanstackQueryRestDestructuringOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow rest destructuring on the result of a TanStack Query hook.
    ///
    /// TanStack Query re-renders a component only when a field it reads changes.
    /// Rest destructuring or spreading a query result reads every field, so the component
    /// re-renders whenever any of them changes.
    ///
    /// The rule checks `useQuery`, `useInfiniteQuery`, `useSuspenseQuery`,
    /// `useSuspenseInfiniteQuery`, `useQueries` and `useSuspenseQueries` imported from a
    /// `@tanstack/*-query` package, including results stored in a variable.
    /// Custom hooks that wrap these calls are not checked.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// const { data, ...rest } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// const query = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    /// const { data, ...rest } = query;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// const query = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    /// const props = { ...query };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// const { data, isPending } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    /// ```
    ///
    pub NoTanstackQueryRestDestructuring {
        version: "next",
        name: "noTanstackQueryRestDestructuring",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("no-rest-destructuring").same()],
        recommended: false,
    }
}

/// The kind of result returned by a query hook.
#[derive(Clone, Copy)]
enum QueryHook {
    /// The hook returns one query result object.
    Single,
    /// The hook returns an array of query result objects.
    Multi,
}

/// Query hooks checked by the rule, with the kind of result each one returns.
const QUERY_HOOKS: &[(&str, QueryHook)] = &[
    ("useQuery", QueryHook::Single),
    ("useInfiniteQuery", QueryHook::Single),
    ("useSuspenseQuery", QueryHook::Single),
    ("useSuspenseInfiniteQuery", QueryHook::Single),
    ("useQueries", QueryHook::Multi),
    ("useSuspenseQueries", QueryHook::Multi),
];

/// How a query result is read in full.
pub enum ResultUse {
    RestDestructuring,
    Spread,
}

/// A place where a query result is read in full, and the range the diagnostic points at.
pub struct RuleState {
    range: TextRange,
    kind: ResultUse,
}

impl Rule for NoTanstackQueryRestDestructuring {
    type Query = Semantic<JsCallExpression>;
    type State = RuleState;
    type Signals = Vec<Self::State>;
    type Options = NoTanstackQueryRestDestructuringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        // Check the cheap syntactic shape first: most calls are not variable initializers, and
        // resolving the callee requires a semantic model lookup.
        let Some(declarator) = declarator_of(call.syntax()) else {
            return Vec::new();
        };
        let Ok(id) = declarator.id() else {
            return Vec::new();
        };
        let Some(hook) = query_hook(call, model) else {
            return Vec::new();
        };

        match (hook, id) {
            (QueryHook::Single, AnyJsBindingPattern::JsObjectBindingPattern(pattern)) => {
                rest_state(&pattern).into_iter().collect()
            }
            (QueryHook::Multi, AnyJsBindingPattern::JsArrayBindingPattern(pattern)) => {
                array_rest_states(&pattern)
            }
            (_, AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding))) => {
                stored_result_uses(&binding, model)
            }
            _ => Vec::new(),
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = match state.kind {
            ResultUse::RestDestructuring => {
                markup! { "Rest destructuring of a TanStack Query result is not allowed." }
            }
            ResultUse::Spread => {
                markup! { "Spreading a TanStack Query result is not allowed." }
            }
        };
        Some(
            RuleDiagnostic::new(rule_category!(), state.range, message)
                .note(markup! {
                    "Reading every field of the result makes the component re-render whenever any of them changes."
                })
                .note(markup! {
                    "Destructure only the fields you use."
                }),
        )
    }
}

/// Returns the declarator that `expression` is the initializer of, if any.
fn declarator_of(expression: &JsSyntaxNode) -> Option<JsVariableDeclarator> {
    JsInitializerClause::cast(expression.parent()?)?.parent::<JsVariableDeclarator>()
}

/// Returns the kind of TanStack Query hook that `call` invokes, if it invokes one.
///
/// The imported name takes precedence over the local name, so
/// `import { useQueries as useQuery }` is treated as `useQueries`.
fn query_hook(call: &JsCallExpression, model: &SemanticModel) -> Option<QueryHook> {
    let callee = call.callee().ok()?.omit_parentheses();
    let reference = callee.as_js_reference_identifier()?;
    let binding = model.binding(&reference)?;
    let ident = JsIdentifierBinding::cast_ref(&binding.syntax())?;
    let specifier = ident.parent::<AnyJsNamedImportSpecifier>()?;

    let find_hook = |name: &str| {
        QUERY_HOOKS
            .iter()
            .find_map(|(hook_name, hook)| (*hook_name == name).then_some(*hook))
    };
    let imported_name = imported_name(&specifier, &ident)?;
    let local_name = ident.name_token().ok()?;
    let hook =
        find_hook(imported_name.text_trimmed()).or_else(|| find_hook(local_name.text_trimmed()))?;

    let import = specifier.import_clause()?.parent::<JsImport>()?;
    let source = import.source_text().ok()?;
    is_tanstack_query_source(source.text()).then_some(hook)
}

/// Returns the name under which `specifier` imports `ident`, ignoring any local alias.
fn imported_name(
    specifier: &AnyJsNamedImportSpecifier,
    ident: &JsIdentifierBinding,
) -> Option<JsSyntaxToken> {
    match specifier {
        AnyJsNamedImportSpecifier::JsNamedImportSpecifier(named) => named.name().ok()?.value().ok(),
        AnyJsNamedImportSpecifier::JsShorthandNamedImportSpecifier(_) => ident.name_token().ok(),
        AnyJsNamedImportSpecifier::JsBogusNamedImportSpecifier(_) => None,
    }
}

/// Returns `true` if `source` is a TanStack Query package such as `@tanstack/react-query`.
///
/// This matches the import check of the upstream ESLint rule, which accepts every framework
/// package of the form `@tanstack/<framework>-query`.
fn is_tanstack_query_source(source: &str) -> bool {
    source.starts_with("@tanstack/") && source.ends_with("-query")
}

/// Returns a state for the rest element of `pattern`, if it has one.
fn rest_state(pattern: &JsObjectBindingPattern) -> Option<RuleState> {
    pattern.properties().into_iter().find_map(|member| {
        let Ok(AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest)) = member else {
            return None;
        };
        Some(RuleState {
            range: rest.range(),
            kind: ResultUse::RestDestructuring,
        })
    })
}

/// Returns a state for every element of `pattern` that is an object pattern with a rest element.
fn array_rest_states(pattern: &JsArrayBindingPattern) -> Vec<RuleState> {
    pattern
        .elements()
        .into_iter()
        .filter_map(|element| {
            let AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(element) =
                element.ok()?
            else {
                return None;
            };
            match element.pattern().ok()? {
                AnyJsBindingPattern::JsObjectBindingPattern(pattern) => rest_state(&pattern),
                _ => None,
            }
        })
        .collect()
}

/// Returns the rest destructurings and spreads of the query result stored in `binding`.
fn stored_result_uses(binding: &JsIdentifierBinding, model: &SemanticModel) -> Vec<RuleState> {
    model
        .as_binding(binding)
        .all_references()
        .filter_map(|reference| {
            let expression = reference
                .syntax()
                .parent()
                .and_then(JsIdentifierExpression::cast)?;
            if let Some(spread) = expression.parent::<JsSpread>() {
                return Some(RuleState {
                    range: spread.range(),
                    kind: ResultUse::Spread,
                });
            }
            match declarator_of(expression.syntax())?.id().ok()? {
                AnyJsBindingPattern::JsObjectBindingPattern(pattern) => rest_state(&pattern),
                _ => None,
            }
        })
        .collect()
}
