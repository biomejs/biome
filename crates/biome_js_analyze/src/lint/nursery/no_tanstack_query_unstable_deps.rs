use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression,
    AnyJsNamedImportSpecifier, JsArrayBindingPattern, JsArrayBindingPatternElementList,
    JsCallExpression, JsFunctionBody, JsIdentifierBinding, JsImport, JsLanguage,
    JsReferenceIdentifier, JsReturnStatement, JsVariableDeclarator,
    binding_ext::AnyJsBindingDeclaration,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, WalkEvent};
use biome_rule_options::no_tanstack_query_unstable_deps::NoTanstackQueryUnstableDepsOptions;

use crate::react::hooks::is_react_hook_name;
use crate::react::{ReactLibrary, is_react_call_api};
use crate::services::control_flow::AnyJsControlFlowRoot;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow using the result of a TanStack Query hook as a React hook dependency.
    ///
    /// TanStack Query hooks such as `useQuery` and `useMutation` can return a new object on every
    /// render. When that object is a dependency of `useEffect`, `useMemo` or `useCallback`, the hook
    /// runs again on every render. List the values you need from the object instead.
    ///
    /// `useQueries` and `useSuspenseQueries` are allowed when they are called with a `combine` option.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useCallback } from "react";
    /// import { useMutation } from "@tanstack/react-query";
    ///
    /// function Component() {
    ///     const mutation = useMutation({ mutationFn: (value) => value });
    ///     const callback = useCallback(() => {
    ///         mutation.mutate("hello");
    ///     }, [mutation]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useEffect } from "react";
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// function useTodos() {
    ///     return useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    /// }
    ///
    /// function Component() {
    ///     const todos = useTodos();
    ///     useEffect(() => {
    ///         console.log(todos.data);
    ///     }, [todos]);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { useCallback, useEffect, useMemo } from "react";
    /// import { useMutation, useQueries, useQuery } from "@tanstack/react-query";
    ///
    /// function Component() {
    ///     const { mutate } = useMutation({ mutationFn: (value) => value });
    ///     const callback = useCallback(() => {
    ///         mutate("hello");
    ///     }, [mutate]);
    ///
    ///     const { data } = useQuery({ queryKey: ["todos"], queryFn: fetchTodos });
    ///     useEffect(() => {
    ///         console.log(data);
    ///     }, [data]);
    ///
    ///     const combined = useQueries({
    ///         queries: [{ queryKey: ["users"], queryFn: fetchUsers }],
    ///         combine: (results) => results.map((result) => result.data),
    ///     });
    ///     const count = useMemo(() => combined.length, [combined]);
    /// }
    /// ```
    ///
    pub NoTanstackQueryUnstableDeps {
        version: "2.6.10",
        name: "noTanstackQueryUnstableDeps",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("no-unstable-deps").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::TanstackQuery],
    }
}

impl Rule for NoTanstackQueryUnstableDeps {
    type Query = Semantic<JsCallExpression>;
    type State = UnstableDependency;
    type Signals = Box<[Self::State]>;
    type Options = NoTanstackQueryUnstableDepsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        unstable_dependencies(ctx.query(), ctx.model()).unwrap_or_default()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let UnstableDependency {
            dependency,
            react_hook,
            query_hook,
        } = state;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                dependency.range(),
                markup! {
                    "The object returned by "<Emphasis>{query_hook}</Emphasis>" is a dependency of "<Emphasis>{react_hook}</Emphasis>"."
                },
            )
            .note(markup! {
                "This object can be a new object on every render, even when none of its values have changed. Because of this, "<Emphasis>{react_hook}</Emphasis>" treats its dependencies as changed on every render."
            })
            .note(markup! {
                "Take the values that you need out of this object, and list those values as dependencies instead."
            }),
        )
    }
}

/// A dependency of a React hook that holds the object returned by a TanStack Query hook.
pub struct UnstableDependency {
    dependency: JsReferenceIdentifier,
    /// The name of the React hook, such as `useEffect`.
    react_hook: &'static str,
    /// The name of the TanStack Query hook that returns the object, such as `useQuery`.
    query_hook: &'static str,
}

/// The React hooks whose dependency list is checked, sorted by name.
const REACT_HOOKS: &[&str] = &["useCallback", "useEffect", "useMemo"];

/// The TanStack Query hooks that return an object with a new identity on every render, sorted
/// by name.
const QUERY_HOOKS: &[&str] = &[
    "useInfiniteQuery",
    "useMutation",
    "useQueries",
    "useQuery",
    "useSuspenseInfiniteQuery",
    "useSuspenseQueries",
    "useSuspenseQuery",
];

/// The nodes that the search for `return` statements stops at: the `return` statements
/// themselves, and the functions whose `return` statements belong to another function.
const RETURN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> =
    JsReturnStatement::KIND_SET.union(AnyJsControlFlowRoot::KIND_SET);

/// Returns the dependencies of `call` that hold the result of a TanStack Query hook.
///
/// Returns `None` if `call` isn't a call to a React hook from [`REACT_HOOKS`] with an array
/// literal as its second argument.
fn unstable_dependencies(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Option<Box<[UnstableDependency]>> {
    let react_hook = react_hook(call.callee().ok()?, model)?;
    let [Some(dependencies)] = call.arguments().ok()?.get_arguments_by_index([1]) else {
        return None;
    };
    let dependencies = dependencies.as_any_js_expression()?.inner_expression()?;
    let dependencies = dependencies.as_js_array_expression()?;

    let unstable = dependencies
        .elements()
        .iter()
        .filter_map(|element| {
            let AnyJsArrayElement::AnyJsExpression(expression) = element.ok()? else {
                return None;
            };
            let dependency = expression.inner_expression()?.as_js_reference_identifier()?;
            let query_hook = query_hook_of_variable(&dependency, model)?;
            Some(UnstableDependency {
                dependency,
                react_hook,
                query_hook,
            })
        })
        .collect();
    Some(unstable)
}

/// Returns the name of the React hook from [`REACT_HOOKS`] that `callee` refers to.
///
/// The callee is either a member of the `React` namespace, such as `React.useEffect`, or a named
/// import from React, which may be renamed. A hook name that isn't declared anywhere in the file
/// is also accepted, because some projects make React hooks available as globals.
fn react_hook(callee: AnyJsExpression, model: &SemanticModel) -> Option<&'static str> {
    let callee = callee.omit_parentheses();
    let name = if let Some(reference) = callee.as_js_reference_identifier() {
        let Some(binding) = model.binding(&reference) else {
            return find_name(REACT_HOOKS, reference.value_token().ok()?.text_trimmed());
        };
        let specifier =
            JsIdentifierBinding::cast(binding.syntax())?.parent::<AnyJsNamedImportSpecifier>()?;
        let imported_name = specifier.imported_name()?;
        find_name(REACT_HOOKS, imported_name.text_trimmed())?
    } else {
        let member_name = AnyJsMemberExpression::cast_ref(callee.syntax())?.member_name()?;
        find_name(REACT_HOOKS, member_name.text())?
    };

    is_react_call_api(&callee, model, ReactLibrary::React, name).then_some(name)
}

/// Returns the name of the TanStack Query hook whose result is stored in the variable that
/// `reference` refers to.
///
/// The variable is either initialized with the call, or is an element of an array pattern
/// initialized with the call:
///
/// ```js
/// const query = useQuery(options);
/// const [first, ...others] = useQueries(options);
/// ```
fn query_hook_of_variable(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<&'static str> {
    let declarator = match model.binding(reference)?.tree().declaration()? {
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => declarator,
        AnyJsBindingDeclaration::JsArrayBindingPatternElement(element) => element
            .parent::<JsArrayBindingPatternElementList>()?
            .parent::<JsArrayBindingPattern>()?
            .parent::<JsVariableDeclarator>()?,
        AnyJsBindingDeclaration::JsArrayBindingPatternRestElement(element) => element
            .parent::<JsArrayBindingPatternElementList>()?
            .parent::<JsArrayBindingPattern>()?
            .parent::<JsVariableDeclarator>()?,
        _ => return None,
    };
    let initializer = declarator.initializer()?.expression().ok()?.inner_expression()?;
    let call = initializer.as_js_call_expression()?;

    direct_query_hook(call, model).or_else(|| custom_query_hook(call, model))
}

/// Returns the name of the TanStack Query hook that `call` calls, if the hook is imported from a
/// TanStack Query package.
///
/// The hook is identified by its exported name, so a renamed import such as
/// `import { useQuery as useData }` is recognized too. Calls to `useQueries` and
/// `useSuspenseQueries` with a `combine` option return `None`, because `combine` controls what
/// the hook returns.
fn direct_query_hook(call: &JsCallExpression, model: &SemanticModel) -> Option<&'static str> {
    let callee = call.callee().ok()?.inner_expression()?;
    let binding = model.binding(&callee.as_js_reference_identifier()?)?;
    let specifier =
        JsIdentifierBinding::cast(binding.syntax())?.parent::<AnyJsNamedImportSpecifier>()?;
    if specifier.imports_only_types() {
        return None;
    }

    let imported_name = specifier.imported_name()?;
    let hook = find_name(QUERY_HOOKS, imported_name.text_trimmed())?;

    let source = specifier
        .import_clause()?
        .parent::<JsImport>()?
        .source_text()
        .ok()?;
    if !is_tanstack_query_package(source.text()) {
        return None;
    }

    if matches!(hook, "useQueries" | "useSuspenseQueries") && has_combine_option(call) {
        return None;
    }

    Some(hook)
}

/// Returns the name of the TanStack Query hook whose result is returned by the custom hook that
/// `call` calls.
///
/// The custom hook must be a function declaration, or a variable initialized with a function,
/// whose name follows the React hook naming convention. Its body must either be an expression
/// that calls the TanStack Query hook, or contain a single `return` statement that does:
///
/// ```js
/// const useTodos = () => useQuery(options);
///
/// function useUsers() {
///     return useQuery(options);
/// }
/// ```
fn custom_query_hook(call: &JsCallExpression, model: &SemanticModel) -> Option<&'static str> {
    let callee = call.callee().ok()?.inner_expression()?;
    let reference = callee.as_js_reference_identifier()?;
    if !is_react_hook_name(reference.value_token().ok()?.text_trimmed()) {
        return None;
    }

    let returned = match model.binding(&reference)?.tree().declaration()? {
        AnyJsBindingDeclaration::JsFunctionDeclaration(function) => {
            single_returned_expression(&function.body().ok()?)?
        }
        AnyJsBindingDeclaration::JsFunctionExportDefaultDeclaration(function) => {
            single_returned_expression(&function.body().ok()?)?
        }
        AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
            match declarator.initializer()?.expression().ok()?.omit_parentheses() {
                AnyJsExpression::JsArrowFunctionExpression(function) => {
                    match function.body().ok()? {
                        AnyJsFunctionBody::AnyJsExpression(expression) => expression,
                        AnyJsFunctionBody::JsFunctionBody(body) => {
                            single_returned_expression(&body)?
                        }
                    }
                }
                AnyJsExpression::JsFunctionExpression(function) => {
                    single_returned_expression(&function.body().ok()?)?
                }
                _ => return None,
            }
        }
        _ => return None,
    };

    direct_query_hook(
        returned.inner_expression()?.as_js_call_expression()?,
        model,
    )
}

/// Returns the argument of the only `return` statement of the function whose body is `body`.
///
/// `return` statements of nested functions are ignored. Returns `None` if the function has no
/// `return` statement, or more than one.
fn single_returned_expression(body: &JsFunctionBody) -> Option<AnyJsExpression> {
    let mut returned = None;
    let mut preorder = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(statement) = JsReturnStatement::cast(node) {
            if returned.is_some() {
                return None;
            }
            returned = Some(statement.argument());
        }
        preorder.skip_subtree();
    }
    returned?
}

/// Returns whether the first argument of `call` is an object literal with a `combine` property.
fn has_combine_option(call: &JsCallExpression) -> bool {
    call.arguments()
        .ok()
        .and_then(|arguments| arguments.args().first()?.ok())
        .and_then(|argument| argument.as_any_js_expression()?.inner_expression())
        .and_then(|argument| argument.as_js_object_expression().cloned())
        .is_some_and(|options| {
            options.members().iter().any(|member| {
                member
                    .ok()
                    .and_then(|member| member.name())
                    .is_some_and(|name| name.text() == "combine")
            })
        })
}

/// Returns whether `source` names a TanStack Query package, such as `@tanstack/react-query`
/// or `@tanstack/vue-query`.
fn is_tanstack_query_package(source: &str) -> bool {
    source
        .strip_prefix("@tanstack/")
        .is_some_and(|package| package.ends_with("-query"))
}

/// Returns the entry of the sorted list `names` that is equal to `name`.
fn find_name(names: &[&'static str], name: &str) -> Option<&'static str> {
    let index = names.binary_search(&name).ok()?;
    names.get(index).copied()
}

#[cfg(test)]
mod tests {
    use super::{QUERY_HOOKS, REACT_HOOKS};

    #[test]
    fn hook_lists_are_sorted() {
        for names in [REACT_HOOKS, QUERY_HOOKS] {
            for pair in names.windows(2) {
                assert!(
                    pair[0].cmp(pair[1]).is_lt(),
                    "`{}` must come before `{}`",
                    pair[0],
                    pair[1]
                );
            }
        }
    }
}
