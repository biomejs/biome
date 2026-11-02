use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsArrayBindingPatternElement, AnyJsArrowFunctionParameters,
    AnyJsBindingPattern, AnyJsCallArgument, AnyJsExpression, AnyJsFunctionBody,
    AnyJsNamedImportSpecifier, JsIdentifierBinding, JsImport, JsInitializerClause,
    JsNewExpression, JsVariableDeclarator, T,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_rule_options::use_tanstack_query_stable_query_client::UseTanstackQueryStableQueryClientOptions;

use crate::JsRuleAction;
use crate::react::components::is_react_component_name;
use crate::react::hooks::is_react_hook_name;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require the TanStack Query `QueryClient` to be created once, not on every render.
    ///
    /// The `QueryClient` holds the cache that stores the results of your queries. A React
    /// component runs its function body again each time it renders. When the body creates the
    /// client with `new QueryClient()`, every render gets a new client with an empty cache, and
    /// the data that was already fetched is lost.
    ///
    /// Create the client outside of the component, or store it with `useState` so that React
    /// keeps the same client across renders.
    ///
    /// This rule checks `new QueryClient()` calls that are assigned to a variable inside a React
    /// component or a custom hook, when `QueryClient` is imported from `@tanstack/react-query`.
    /// A function counts as a component when its name starts with an uppercase letter, and as a
    /// hook when its name starts with `use` followed by an uppercase letter.
    ///
    /// Async components are allowed to create a client, because an async Server Component only
    /// runs once on the server for each request.
    ///
    /// The fix wraps the client in `React.useState`. It expects `React` to be available in the
    /// file.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
    ///
    /// function App() {
    ///     const queryClient = new QueryClient();
    ///     return (
    ///         <QueryClientProvider client={queryClient}>
    ///             <Home />
    ///         </QueryClientProvider>
    ///     );
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { QueryClient } from "@tanstack/react-query";
    ///
    /// const useQueryClient = () => {
    ///     const queryClient = new QueryClient();
    ///     return queryClient;
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
    /// import { useState } from "react";
    ///
    /// const sharedClient = new QueryClient();
    ///
    /// function App() {
    ///     const [queryClient] = useState(() => new QueryClient());
    ///     return (
    ///         <QueryClientProvider client={queryClient}>
    ///             <Home />
    ///         </QueryClientProvider>
    ///     );
    /// }
    ///
    /// async function ServerComponent() {
    ///     const queryClient = new QueryClient();
    ///     await queryClient.prefetchQuery(options);
    /// }
    /// ```
    ///
    pub UseTanstackQueryStableQueryClient {
        version: "2.6.5",
        name: "useTanstackQueryStableQueryClient",
        language: "js",
        sources: &[RuleSource::EslintTanstackQuery("stable-query-client").same()],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
        domains: &[RuleDomain::React, RuleDomain::TanstackQuery],
    }
}

impl Rule for UseTanstackQueryStableQueryClient {
    type Query = Semantic<JsNewExpression>;
    /// The variable declaration that stores the client.
    type State = JsVariableDeclarator;
    type Signals = Option<Self::State>;
    type Options = UseTanstackQueryStableQueryClientOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !is_react_query_client(node, ctx.model()) {
            return None;
        }

        let declarator = AnyJsExpression::from(node.clone())
            .outer_expression()?
            .parent::<JsInitializerClause>()?
            .parent::<JsVariableDeclarator>()?;

        let AnyFunctionLike::AnyJsFunction(function) = node
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(AnyFunctionLike::cast)?
        else {
            return None;
        };
        if function.is_async() {
            return None;
        }

        let binding = function.binding()?;
        let name = binding.as_js_identifier_binding()?.name_token().ok()?;
        let name = name.text_trimmed();
        (is_react_component_name(name) || is_react_hook_name(name)).then_some(declarator)
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"QueryClient"</Emphasis>" is created again each time the component renders."
                },
            )
            .note(markup! {
                "Each new client starts with an empty cache, so the data that was already fetched is lost on every render."
            })
            .note(markup! {
                "Create the client outside of the component, or store it with "<Emphasis>"useState"</Emphasis>"."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, declarator: &Self::State) -> Option<JsRuleAction> {
        let id = declarator.id().ok()?;
        let binding = id.as_any_js_binding()?;
        binding.as_js_identifier_binding()?;
        let initializer = declarator.initializer()?;

        let pattern = make::js_array_binding_pattern(
            make::token(T!['[']),
            make::js_array_binding_pattern_element_list(
                [AnyJsArrayBindingPatternElement::from(
                    make::js_array_binding_pattern_element(AnyJsBindingPattern::AnyJsBinding(
                        binding.clone().trim_trivia()?,
                    ))
                    .build(),
                )],
                [],
            ),
            make::token(T![']']).with_trailing_space(),
        );

        let factory = make::js_arrow_function_expression(
            AnyJsArrowFunctionParameters::from(make::js_parameters(
                make::token(T!['(']),
                make::js_parameter_list([], []),
                make::token(T![')']).with_trailing_space(),
            )),
            make::token(T![=>]).with_trailing_space(),
            AnyJsFunctionBody::AnyJsExpression(initializer.expression().ok()?.trim_trivia()?),
        )
        .build();
        let use_state = make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("React")))
                .into(),
            make::token(T![.]),
            make::js_name(make::ident("useState")).into(),
        );
        let call = make::js_call_expression(
            use_state.into(),
            make::js_call_arguments(
                make::token(T!['(']),
                make::js_call_argument_list([AnyJsCallArgument::AnyJsExpression(factory.into())], []),
                make::token(T![')']),
            ),
        )
        .build();

        // The type annotation of the client doesn't fit the array pattern, so it is dropped.
        let new_declarator = make::js_variable_declarator(pattern.into())
            .with_initializer(make::js_initializer_clause(
                initializer.eq_token().ok()?,
                call.into(),
            ))
            .build();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(declarator.clone(), new_declarator);
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Store the client with "<Emphasis>"React.useState"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

/// Returns whether `node` calls the `QueryClient` constructor imported by name from
/// `@tanstack/react-query`.
///
/// A renamed import such as `import { QueryClient as Client }` is recognized too.
fn is_react_query_client(node: &JsNewExpression, model: &SemanticModel) -> bool {
    let Some(reference) = node
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
        .and_then(|callee| callee.as_js_reference_identifier())
    else {
        return false;
    };
    let Some(specifier) = model
        .binding(&reference)
        .and_then(|binding| JsIdentifierBinding::cast(binding.syntax()))
        .and_then(|binding| binding.parent::<AnyJsNamedImportSpecifier>())
    else {
        return false;
    };
    if specifier.imports_only_types()
        || specifier
            .imported_name()
            .is_none_or(|name| name.text_trimmed() != "QueryClient")
    {
        return false;
    }

    specifier
        .import_clause()
        .and_then(|clause| clause.parent::<JsImport>())
        .and_then(|import| import.source_text().ok())
        .is_some_and(|source| source.text() == "@tanstack/react-query")
}
