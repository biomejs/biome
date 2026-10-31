use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsNamedImportSpecifier, JsArrayBindingPattern,
    JsArrayBindingPatternElement, JsArrayBindingPatternElementList, JsIdentifierBinding, JsImport,
    JsObjectBindingPattern, JsObjectBindingPatternPropertyList, JsObjectBindingPatternRest,
    JsObjectMemberList, JsSpread, JsVariableDeclarator,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_tanstack_query_rest_destructuring::NoTanstackQueryRestDestructuringOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow rest destructuring and object spread on TanStack Query results.
    ///
    /// The object returned by a TanStack Query function such as `useQuery` keeps track of the
    /// properties that a component reads. The component only updates when one of those
    /// properties changes.
    ///
    /// Collecting the remaining properties with a rest element (`...rest`), or copying the
    /// result into another object with a spread (`...query`), reads every property. The
    /// component then updates each time any part of the query changes, even the parts that
    /// the component doesn't use.
    ///
    /// This rule checks the results of the following functions, when they are imported from a
    /// TanStack Query package such as `@tanstack/react-query` or `@tanstack/svelte-query`:
    ///
    /// - `useQuery`, `useInfiniteQuery`, `useSuspenseQuery` and `useSuspenseInfiniteQuery`
    /// - `useQueries` and `useSuspenseQueries`
    /// - `createQuery`, `createInfiniteQuery` and `createQueries`, the names that
    ///   `@tanstack/svelte-query` uses
    ///
    /// :::note
    /// The rule only checks direct calls to these functions. It doesn't check the result of a
    /// custom function that wraps one of them.
    /// :::
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
    /// const result = { ...query, todos: query.data };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useQueries } from "@tanstack/react-query";
    ///
    /// const [{ data, ...rest }] = useQueries({
    ///     queries: [{ queryKey: ["todos"], queryFn: fetchTodos }],
    /// });
    /// ```
    ///
    /// ```svelte,expect_diagnostic
    /// <script>
    /// import { createQuery } from "@tanstack/svelte-query";
    ///
    /// const query = createQuery(() => ({ queryKey: ["todos"], queryFn: fetchTodos }));
    /// const { data, ...rest } = query;
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useQueries, useQuery } from "@tanstack/react-query";
    ///
    /// const { data, isPending } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    ///
    /// const query = useQuery({ queryKey: ["users"], queryFn: fetchUsers });
    /// const users = query.data;
    ///
    /// const [first, { data: posts }, ...others] = useQueries({
    ///     queries: [
    ///         { queryKey: ["todos"], queryFn: fetchTodos },
    ///         { queryKey: ["posts"], queryFn: fetchPosts },
    ///         { queryKey: ["users"], queryFn: fetchUsers },
    ///     ],
    /// });
    /// ```
    ///
    /// ```svelte
    /// <script>
    /// import { createQuery } from "@tanstack/svelte-query";
    ///
    /// const query = createQuery(() => ({ queryKey: ["todos"], queryFn: fetchTodos }));
    /// </script>
    ///
    /// <p>{query.data.length} todos</p>
    /// ```
    ///
    pub NoTanstackQueryRestDestructuring {
        version: "2.6.3",
        name: "noTanstackQueryRestDestructuring",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("no-rest-destructuring").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::TanstackQuery],
    }
}

declare_node_union! {
    pub AnyRestOrSpread = JsObjectBindingPatternRest | JsSpread
}

impl Rule for NoTanstackQueryRestDestructuring {
    type Query = Semantic<AnyRestOrSpread>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoTanstackQueryRestDestructuringOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let reads_every_property = match ctx.query() {
            AnyRestOrSpread::JsObjectBindingPatternRest(rest) => {
                let pattern = rest
                    .parent::<JsObjectBindingPatternPropertyList>()?
                    .parent::<JsObjectBindingPattern>()?;
                destructures_query_result(&pattern, model)?
            }
            AnyRestOrSpread::JsSpread(spread) => {
                // Spreading into an array or into call arguments iterates the value instead of
                // reading its properties.
                spread.parent::<JsObjectMemberList>()?;
                is_query_result(&spread.argument().ok()?, model)
            }
        };

        reads_every_property.then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let diagnostic = match node {
            AnyRestOrSpread::JsObjectBindingPatternRest(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This rest element collects every remaining property of a TanStack Query result."
                },
            )
            .note(markup! {
                "A query result only updates the component when a property that the component reads changes. Collecting the remaining properties reads all of them, so the component updates on every change to the query."
            })
            .note(markup! {
                "Remove the rest element and list the properties that you need."
            }),
            AnyRestOrSpread::JsSpread(_) => RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This spread copies every property of a TanStack Query result."
                },
            )
            .note(markup! {
                "A query result only updates the component when a property that the component reads changes. Copying the result reads all of its properties, so the component updates on every change to the query."
            })
            .note(markup! {
                "Remove the spread and copy the properties that you need."
            }),
        };

        Some(diagnostic)
    }
}

/// The shape of the value returned by a TanStack Query hook.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QueryHookResult {
    /// A single query result, as returned by `useQuery` or `createQuery`.
    Single,
    /// A list of query results, as returned by `useQueries` or `createQueries`.
    List,
}

/// The TanStack Query hooks that return query results, sorted by name.
///
/// The `create*` names are the ones exported by `@tanstack/svelte-query`.
const QUERY_HOOKS: &[(&str, QueryHookResult)] = &[
    ("createInfiniteQuery", QueryHookResult::Single),
    ("createQueries", QueryHookResult::List),
    ("createQuery", QueryHookResult::Single),
    ("useInfiniteQuery", QueryHookResult::Single),
    ("useQueries", QueryHookResult::List),
    ("useQuery", QueryHookResult::Single),
    ("useSuspenseInfiniteQuery", QueryHookResult::Single),
    ("useSuspenseQueries", QueryHookResult::List),
    ("useSuspenseQuery", QueryHookResult::Single),
];

/// Returns whether `pattern` destructures a single query result.
///
/// The pattern either belongs to a variable initialized with a single query result, or it is
/// an element of an array pattern initialized with a call to a hook that returns a list of
/// query results:
///
/// ```js
/// const { data, ...rest } = useQuery(options);
/// const [first, { data, ...rest }] = useQueries(options);
/// ```
///
/// Returns `None` if the pattern is in any other position, such as a function parameter.
fn destructures_query_result(
    pattern: &JsObjectBindingPattern,
    model: &SemanticModel,
) -> Option<bool> {
    if let Some(declarator) = pattern.parent::<JsVariableDeclarator>() {
        let initializer = declarator.initializer()?.expression().ok()?;
        return Some(is_query_result(&initializer, model));
    }

    let declarator = pattern
        .parent::<JsArrayBindingPatternElement>()?
        .parent::<JsArrayBindingPatternElementList>()?
        .parent::<JsArrayBindingPattern>()?
        .parent::<JsVariableDeclarator>()?;
    let initializer = declarator.initializer()?.expression().ok()?;

    Some(query_hook_result(&initializer, model) == Some(QueryHookResult::List))
}

/// Returns whether `expression` evaluates to a single query result.
///
/// The expression is either a call to a hook that returns a single query result, or a
/// reference to a variable that is initialized with such a call.
fn is_query_result(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    let Some(reference) = expression.as_js_reference_identifier() else {
        return query_hook_result(&expression, model) == Some(QueryHookResult::Single);
    };

    model
        .binding(&reference)
        .and_then(|binding| JsIdentifierBinding::cast(binding.syntax()))
        .and_then(|binding| binding.parent::<JsVariableDeclarator>())
        .and_then(|declarator| declarator.initializer()?.expression().ok())
        .is_some_and(|initializer| {
            query_hook_result(&initializer, model) == Some(QueryHookResult::Single)
        })
}

/// Returns the shape of the value that `expression` evaluates to, if `expression` is a call
/// to a hook imported by name from a TanStack Query package.
///
/// The hook is identified by its exported name, so a renamed import such as
/// `import { useQuery as useData }` is recognized too.
fn query_hook_result(
    expression: &AnyJsExpression,
    model: &SemanticModel,
) -> Option<QueryHookResult> {
    let expression = expression.inner_expression()?;
    let callee = expression
        .as_js_call_expression()?
        .callee()
        .ok()?
        .inner_expression()?;
    let binding = model.binding(&callee.as_js_reference_identifier()?)?;
    let specifier =
        JsIdentifierBinding::cast(binding.syntax())?.parent::<AnyJsNamedImportSpecifier>()?;
    if specifier.imports_only_types() {
        return None;
    }

    let imported_name = specifier.imported_name()?;
    let imported_name = imported_name.text_trimmed();
    let index = QUERY_HOOKS
        .binary_search_by(|(name, _)| (*name).cmp(imported_name))
        .ok()?;
    let (_, result) = QUERY_HOOKS.get(index)?;

    let source = specifier
        .import_clause()?
        .parent::<JsImport>()?
        .source_text()
        .ok()?;
    is_tanstack_query_package(source.text()).then_some(*result)
}

/// Returns whether `source` names a TanStack Query package, such as `@tanstack/react-query`
/// or `@tanstack/vue-query`.
fn is_tanstack_query_package(source: &str) -> bool {
    source
        .strip_prefix("@tanstack/")
        .is_some_and(|package| package.ends_with("-query"))
}

#[cfg(test)]
mod tests {
    use super::QUERY_HOOKS;

    #[test]
    fn query_hooks_are_sorted() {
        for pair in QUERY_HOOKS.windows(2) {
            assert!(
                pair[0].0.cmp(pair[1].0).is_lt(),
                "`{}` must come before `{}`",
                pair[0].0,
                pair[1].0
            );
        }
    }
}
