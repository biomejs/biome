use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression,
    AnyJsNamedImportSpecifier, JsCallExpression, JsClassDeclaration, JsFunctionBody,
    JsFunctionDeclaration, JsIdentifierBinding, JsImport, JsLanguage, JsObjectExpression,
    JsReferenceIdentifier, JsReturnStatement, JsSyntaxToken, JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxKindSet, TextRange, WalkEvent};
use biome_rule_options::use_tanstack_query_query_options::UseTanstackQueryQueryOptionsOptions;

use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require TanStack Query options to be created with `queryOptions()`.
    ///
    /// TanStack Query stores the data of a query under its key (`queryKey`). When the key and
    /// the function that fetches the data (`queryFn`) are written again in each place that
    /// uses the query, the same key can end up with two different functions. `queryOptions()`
    /// and `infiniteQueryOptions()` create the options once, so the two stay together.
    ///
    /// This rule reports:
    ///
    /// - An object that sets `queryKey` or `queryFn`, written directly in a call to a query
    ///   hook such as `useQuery` or `useQueries`, or to a query client method such as
    ///   `fetchQuery`.
    /// - A query key written as an array, such as `["todos"]`, in a call to `useIsFetching`
    ///   or to a query client method such as `getQueryData` or `invalidateQueries`.
    ///
    /// :::note
    /// The rule only checks functions imported by name from a TanStack Query package, and
    /// query clients created in the same file with `useQueryClient()` or `new QueryClient()`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { useQuery } from "@tanstack/react-query";
    ///
    /// function Todo({ id }) {
    ///     const query = useQuery({
    ///         queryKey: ["todo", id],
    ///         queryFn: () => fetchTodo(id),
    ///     });
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useQueries } from "@tanstack/react-query";
    ///
    /// function Dashboard() {
    ///     const results = useQueries({
    ///         queries: [{ queryKey: ["todos"], queryFn: fetchTodos }, userOptions],
    ///     });
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useQueryClient } from "@tanstack/react-query";
    ///
    /// function useCachedTodo(id) {
    ///     const queryClient = useQueryClient();
    ///     return queryClient.getQueryData(["todo", id]);
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { useQueryClient } from "@tanstack/react-query";
    ///
    /// function useRefreshTodo(id) {
    ///     const queryClient = useQueryClient();
    ///     return () => queryClient.invalidateQueries({ queryKey: ["todo", id] });
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { queryOptions, useQuery, useQueryClient } from "@tanstack/react-query";
    ///
    /// function todoOptions(id) {
    ///     return queryOptions({
    ///         queryKey: ["todo", id],
    ///         queryFn: () => fetchTodo(id),
    ///     });
    /// }
    ///
    /// function Todo({ id }) {
    ///     const query = useQuery(todoOptions(id));
    ///     const title = useQuery({ ...todoOptions(id), select: (todo) => todo.title });
    /// }
    ///
    /// function useRefreshTodo(id) {
    ///     const queryClient = useQueryClient();
    ///     const cached = queryClient.getQueryData(todoOptions(id).queryKey);
    ///     return () => queryClient.invalidateQueries({ queryKey: todoOptions(id).queryKey });
    /// }
    /// ```
    ///
    pub UseTanstackQueryQueryOptions {
        version: "2.6.10",
        name: "useTanstackQueryQueryOptions",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("prefer-query-options").same()],
        recommended: true,
        severity: Severity::Warning,
        domains: &[RuleDomain::TanstackQuery],
    }
}

/// A part of a query that is written where the query is used, instead of being taken from
/// the result of `queryOptions()` or `infiniteQueryOptions()`.
pub enum InlineQueryPart {
    /// An options object that sets `queryKey` or `queryFn`.
    Options {
        range: TextRange,
        /// The function that should create the options instead.
        builder: OptionsBuilder,
    },
    /// A query key written as an array.
    QueryKey { range: TextRange },
}

impl Rule for UseTanstackQueryQueryOptions {
    type Query = Semantic<JsCallExpression>;
    type State = InlineQueryPart;
    type Signals = Box<[Self::State]>;
    type Options = UseTanstackQueryQueryOptionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        inline_query_parts(ctx.query(), ctx.model()).unwrap_or_default()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        // The two diagnostics share no text, and a message that interpolates a value can't be
        // selected by a `match` inside a single call.
        Some(match state {
            InlineQueryPart::Options { range, builder } => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The options of this query are written where the query is used."
                },
            )
            .note(markup! {
                "When "<Emphasis>"queryKey"</Emphasis>" and "<Emphasis>"queryFn"</Emphasis>" are written again in each place that uses the query, the same key can end up with two different functions. One place then receives the data that the other one fetched."
            })
            .note(markup! {
                "Create the options once with "<Emphasis>{builder}</Emphasis>", and pass the result here."
            }),
            InlineQueryPart::QueryKey { range } => RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This query key is written by hand."
                },
            )
            .note(markup! {
                "A key that is written again can differ from the key of the query, for example after one of them is changed. This call then no longer finds the query."
            })
            .note(markup! {
                "Create the options of the query with "<Emphasis>"queryOptions()"</Emphasis>", and read the "<Emphasis>"queryKey"</Emphasis>" property of the result here."
            }),
        })
    }
}

/// The TanStack Query function that creates the options of a query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OptionsBuilder {
    QueryOptions,
    InfiniteQueryOptions,
}

impl biome_console::fmt::Display for OptionsBuilder {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::QueryOptions => "queryOptions()",
            Self::InfiniteQueryOptions => "infiniteQueryOptions()",
        })
    }
}

/// What a TanStack Query function expects as its first argument.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FirstArgument {
    /// The options of one query, as in `useQuery(options)`.
    Options(OptionsBuilder),
    /// An object whose `queries` property lists the options of several queries, as in
    /// `useQueries({ queries })`.
    OptionsList,
    /// A query key, as in `queryClient.getQueryData(queryKey)`.
    QueryKey,
    /// Filters that select queries, optionally by their key, as in
    /// `queryClient.invalidateQueries({ queryKey })`.
    Filters,
}

/// The checked functions that are exported by the TanStack Query packages, sorted by name.
const FUNCTIONS: &[(&str, FirstArgument)] = &[
    (
        "useInfiniteQuery",
        FirstArgument::Options(OptionsBuilder::InfiniteQueryOptions),
    ),
    ("useIsFetching", FirstArgument::Filters),
    (
        "usePrefetchInfiniteQuery",
        FirstArgument::Options(OptionsBuilder::InfiniteQueryOptions),
    ),
    (
        "usePrefetchQuery",
        FirstArgument::Options(OptionsBuilder::QueryOptions),
    ),
    ("useQueries", FirstArgument::OptionsList),
    (
        "useQuery",
        FirstArgument::Options(OptionsBuilder::QueryOptions),
    ),
    (
        "useSuspenseInfiniteQuery",
        FirstArgument::Options(OptionsBuilder::InfiniteQueryOptions),
    ),
    ("useSuspenseQueries", FirstArgument::OptionsList),
    (
        "useSuspenseQuery",
        FirstArgument::Options(OptionsBuilder::QueryOptions),
    ),
];

/// The checked methods of a query client, sorted by name.
const QUERY_CLIENT_METHODS: &[(&str, FirstArgument)] = &[
    ("cancelQueries", FirstArgument::Filters),
    (
        "ensureInfiniteQueryData",
        FirstArgument::Options(OptionsBuilder::InfiniteQueryOptions),
    ),
    (
        "ensureQueryData",
        FirstArgument::Options(OptionsBuilder::QueryOptions),
    ),
    (
        "fetchInfiniteQuery",
        FirstArgument::Options(OptionsBuilder::InfiniteQueryOptions),
    ),
    (
        "fetchQuery",
        FirstArgument::Options(OptionsBuilder::QueryOptions),
    ),
    ("getQueriesData", FirstArgument::Filters),
    ("getQueryData", FirstArgument::QueryKey),
    ("getQueryDefaults", FirstArgument::QueryKey),
    ("getQueryState", FirstArgument::QueryKey),
    ("invalidateQueries", FirstArgument::Filters),
    ("isFetching", FirstArgument::Filters),
    (
        "prefetchInfiniteQuery",
        FirstArgument::Options(OptionsBuilder::InfiniteQueryOptions),
    ),
    (
        "prefetchQuery",
        FirstArgument::Options(OptionsBuilder::QueryOptions),
    ),
    ("refetchQueries", FirstArgument::Filters),
    ("removeQueries", FirstArgument::Filters),
    ("resetQueries", FirstArgument::Filters),
    ("setQueriesData", FirstArgument::Filters),
    ("setQueryData", FirstArgument::QueryKey),
    ("setQueryDefaults", FirstArgument::QueryKey),
];

/// The nodes that the search for the values returned by a function stops at: `return`
/// statements, and the nodes that can't contain a `return` statement of that function.
const RETURN_SEARCH_KINDS: SyntaxKindSet<JsLanguage> = JsReturnStatement::KIND_SET
    .union(AnyJsExpression::KIND_SET)
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsClassDeclaration::KIND_SET);

/// Returns the parts of a query that are written in the first argument of `call`.
///
/// Returns `None` if `call` isn't a call to one of the checked functions, or if its first
/// argument holds nothing to report.
fn inline_query_parts(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> Option<Box<[InlineQueryPart]>> {
    let [Some(AnyJsCallArgument::AnyJsExpression(argument))] =
        call.arguments().ok()?.get_arguments_by_index([0])
    else {
        return None;
    };

    // The argument is inspected before the callee, because most calls are ruled out by their
    // first argument without resolving any variable.
    match argument.inner_expression()? {
        AnyJsExpression::JsArrayExpression(key) => {
            (first_argument(call, model)? == FirstArgument::QueryKey)
                .then(|| [InlineQueryPart::QueryKey { range: key.range() }].into())
        }
        AnyJsExpression::JsObjectExpression(object) => match first_argument(call, model)? {
            FirstArgument::Options(builder) => sets_key_or_function(&object).then(|| {
                [InlineQueryPart::Options {
                    range: object.range(),
                    builder,
                }]
                .into()
            }),
            FirstArgument::OptionsList => {
                let queries = object.find_member("queries")?;
                let queries = queries.as_js_property_object_member()?.value().ok()?;
                let parts = listed_options(&queries)?
                    .into_iter()
                    .filter(sets_key_or_function)
                    .map(|options| InlineQueryPart::Options {
                        range: options.range(),
                        builder: OptionsBuilder::QueryOptions,
                    })
                    .collect();
                Some(parts)
            }
            FirstArgument::Filters => {
                let key = object.find_member("queryKey")?;
                let key = key.as_js_property_object_member()?.value().ok()?;
                let key = key.inner_expression()?;
                let key = key.as_js_array_expression()?;
                Some([InlineQueryPart::QueryKey { range: key.range() }].into())
            }
            FirstArgument::QueryKey => None,
        },
        _ => None,
    }
}

/// Returns what the first argument of `call` holds, if `call` calls one of the [`FUNCTIONS`]
/// or one of the [`QUERY_CLIENT_METHODS`].
fn first_argument(call: &JsCallExpression, model: &SemanticModel) -> Option<FirstArgument> {
    let callee = call.callee().ok()?.inner_expression()?;
    if let Some(reference) = callee.as_js_reference_identifier() {
        let name = tanstack_query_export_name(&reference, model)?;
        return find_by_name(FUNCTIONS, name.text_trimmed());
    }

    let member = AnyJsMemberExpression::cast(callee.into_syntax())?;
    let first_argument = find_by_name(QUERY_CLIENT_METHODS, member.member_name()?.text())?;
    is_query_client(member.object().ok()?, model).then_some(first_argument)
}

/// Returns the entry of `functions` for `name`. `functions` must be sorted by name.
fn find_by_name(functions: &[(&str, FirstArgument)], name: &str) -> Option<FirstArgument> {
    let index = functions
        .binary_search_by(|(function, _)| (*function).cmp(name))
        .ok()?;
    functions.get(index).map(|(_, first_argument)| *first_argument)
}

/// Returns the name under which a TanStack Query package, such as `@tanstack/react-query` or
/// `@tanstack/vue-query`, exports the value that `reference` refers to.
///
/// The result is the exported name, so `useQuery` is returned for a renamed import such as
/// `import { useQuery as useData }`. Returns `None` if `reference` doesn't refer to a value
/// that is imported by name from such a package.
fn tanstack_query_export_name(
    reference: &JsReferenceIdentifier,
    model: &SemanticModel,
) -> Option<JsSyntaxToken> {
    let binding = model.binding(reference)?;
    let specifier =
        JsIdentifierBinding::cast(binding.syntax())?.parent::<AnyJsNamedImportSpecifier>()?;
    if specifier.imports_only_types() {
        return None;
    }

    let source = specifier
        .import_clause()?
        .parent::<JsImport>()?
        .source_text()
        .ok()?;
    let package = source.text().strip_prefix("@tanstack/")?;
    if !package.ends_with("-query") {
        return None;
    }

    specifier.imported_name()
}

/// Returns whether `expression` evaluates to a TanStack Query client.
///
/// A query client is the result of `useQueryClient()` or `new QueryClient()`, where the
/// function is imported from a TanStack Query package. A variable that is initialized with a
/// query client, directly or through other variables, is a query client too:
///
/// ```js
/// const queryClient = useQueryClient();
/// const client = queryClient;
/// ```
///
/// Returns `false` for a client that comes from anywhere else, such as a function parameter.
fn is_query_client(expression: AnyJsExpression, model: &SemanticModel) -> bool {
    // Variables can be initialized with each other, as in `var a = b; var b = a;`.
    let mut visited_declarators = Vec::new();
    let mut expression = expression;
    loop {
        let (callee, constructor) = match expression.inner_expression() {
            Some(AnyJsExpression::JsIdentifierExpression(identifier)) => {
                let Some(declarator) = identifier
                    .name()
                    .ok()
                    .and_then(|reference| model.binding(&reference))
                    .and_then(|binding| JsIdentifierBinding::cast(binding.syntax()))
                    .and_then(|binding| binding.parent::<JsVariableDeclarator>())
                else {
                    return false;
                };
                if visited_declarators.contains(&declarator) {
                    return false;
                }
                let Some(initializer) = declarator
                    .initializer()
                    .and_then(|initializer| initializer.expression().ok())
                else {
                    return false;
                };
                visited_declarators.push(declarator);
                expression = initializer;
                continue;
            }
            Some(AnyJsExpression::JsCallExpression(call)) => (call.callee(), "useQueryClient"),
            Some(AnyJsExpression::JsNewExpression(new)) => (new.callee(), "QueryClient"),
            _ => return false,
        };

        return callee
            .ok()
            .and_then(|callee| callee.inner_expression())
            .and_then(|callee| callee.as_js_reference_identifier())
            .and_then(|reference| tanstack_query_export_name(&reference, model))
            .is_some_and(|name| name.text_trimmed() == constructor);
    }
}

/// Returns whether `options` has a member named `queryKey` or `queryFn`.
///
/// Every kind of member counts: a property, a shorthand property such as `{ queryKey }`, and
/// a method such as `{ queryFn() {} }`.
fn sets_key_or_function(options: &JsObjectExpression) -> bool {
    options.members().iter().flatten().any(|member| {
        member
            .name()
            .is_some_and(|name| matches!(name.text(), "queryKey" | "queryFn"))
    })
}

/// Returns the options objects that are written in `queries`, the list of queries of a
/// function such as `useQueries`.
///
/// The list is either an array, or a call to `.map()` whose callback returns the options of
/// one query:
///
/// ```js
/// useQueries({ queries: [{ queryKey: ["todos"], queryFn: fetchTodos }] });
/// useQueries({ queries: ids.map((id) => ({ queryKey: ["todo", id], queryFn: fetchTodo })) });
/// ```
///
/// Returns `None` if `queries` has neither form.
fn listed_options(queries: &AnyJsExpression) -> Option<Vec<JsObjectExpression>> {
    match queries.inner_expression()? {
        AnyJsExpression::JsArrayExpression(array) => Some(
            array
                .elements()
                .iter()
                .flatten()
                .filter_map(|element| {
                    let element = element.as_any_js_expression()?.inner_expression()?;
                    element.as_js_object_expression().cloned()
                })
                .collect(),
        ),
        AnyJsExpression::JsCallExpression(call) => {
            let callee = call.callee().ok()?.inner_expression()?;
            let callee = AnyJsMemberExpression::cast(callee.into_syntax())?;
            if callee.member_name()?.text() != "map" {
                return None;
            }
            let [Some(AnyJsCallArgument::AnyJsExpression(callback))] =
                call.arguments().ok()?.get_arguments_by_index([0])
            else {
                return None;
            };
            let returned_values = match callback.inner_expression()? {
                AnyJsExpression::JsArrowFunctionExpression(callback) => {
                    match callback.body().ok()? {
                        AnyJsFunctionBody::AnyJsExpression(value) => vec![value],
                        AnyJsFunctionBody::JsFunctionBody(body) => returned_values(&body),
                    }
                }
                AnyJsExpression::JsFunctionExpression(callback) => {
                    returned_values(&callback.body().ok()?)
                }
                _ => return None,
            };
            Some(possible_objects(returned_values))
        }
        _ => None,
    }
}

/// Returns the value of every `return` statement of the function that has the body `body`.
///
/// The `return` statements of the functions and classes nested in `body` are left out.
fn returned_values(body: &JsFunctionBody) -> Vec<AnyJsExpression> {
    let mut values = Vec::new();
    let mut preorder = body.syntax().preorder_of_kinds(RETURN_SEARCH_KINDS);
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(statement) = JsReturnStatement::cast_ref(&node) {
            values.extend(statement.argument());
        }
        preorder.skip_subtree();
    }
    values
}

/// Returns the object literals that one of `values` can evaluate to.
///
/// Each branch of a conditional expression and each side of a logical expression is a
/// possible result:
///
/// ```js
/// isEnabled ? { queryKey: ["todos"] } : fallbackOptions
/// cachedOptions ?? { queryKey: ["todos"] }
/// ```
fn possible_objects(values: Vec<AnyJsExpression>) -> Vec<JsObjectExpression> {
    let mut objects = Vec::new();
    let mut values = values;
    // The values are taken from the end, so they are reversed to find the objects in source
    // order.
    values.reverse();
    while let Some(value) = values.pop() {
        match value.inner_expression() {
            Some(AnyJsExpression::JsObjectExpression(object)) => objects.push(object),
            Some(AnyJsExpression::JsConditionalExpression(conditional)) => {
                values.extend(conditional.alternate());
                values.extend(conditional.consequent());
            }
            Some(AnyJsExpression::JsLogicalExpression(logical)) => {
                values.extend(logical.right());
                values.extend(logical.left());
            }
            _ => {}
        }
    }
    objects
}

#[cfg(test)]
mod tests {
    use super::{FUNCTIONS, QUERY_CLIENT_METHODS};

    #[test]
    fn functions_are_sorted() {
        for functions in [FUNCTIONS, QUERY_CLIENT_METHODS] {
            for pair in functions.windows(2) {
                assert!(
                    pair[0].0.cmp(pair[1].0).is_lt(),
                    "`{}` must come before `{}`",
                    pair[0].0,
                    pair[1].0
                );
            }
        }
    }
}
