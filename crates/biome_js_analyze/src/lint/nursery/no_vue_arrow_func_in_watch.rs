use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsFormalParameter, AnyJsParameter,
    JsArrowFunctionExpression, JsFunctionBody, JsFunctionExpression, JsParameters,
    JsPropertyObjectMember, T,
};
use biome_rowan::{
    AstNode, AstSeparatedList, BatchMutationExt, TriviaPieceKind, declare_node_union,
};
use biome_rule_options::no_vue_arrow_func_in_watch::NoVueArrowFuncInWatchOptions;

use crate::{
    JsRuleAction,
    frameworks::vue::vue_component::{
        AnyPotentialVueComponent, AnyVueComponent, VueComponent, VueComponentDeclarations,
        VueDeclarationCollectionFilter,
    },
    services::semantic::Semantic,
};

declare_lint_rule! {
    /// Disallows using arrow functions when defining a watcher.
    ///
    /// When using the Options API in Vue.js, defining watchers with arrow functions is discouraged. This is because arrow functions bind to their parent context, which means that the `this` keyword inside the arrow function does not refer to the Vue instance as expected. Instead, it refers to the context in which the arrow function was defined, which can be confusing.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   watch: {
    ///     foo: (val, oldVal) => {
    ///       console.log('new: %s, old: %s', val, oldVal)
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   watch: {
    ///     foo: {
    ///       handler: (val, oldVal) => {
    ///         console.log('new: %s, old: %s', val, oldVal)
    ///       }
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   watch: {
    ///     a: function (val, oldVal) {
    ///       console.log('new: %s, old: %s', val, oldVal)
    ///     },
    ///     b: 'someMethod',
    ///     c: {
    ///       handler: function (val, oldVal) { /* ... */ },
    ///       deep: true
    ///     },
    ///     d: {
    ///       handler: 'someMethod',
    ///       immediate: true
    ///     },
    ///     e: [
    ///       'handle1',
    ///       function handle2 (val, oldVal) { /* ... */ },
    ///       {
    ///         handler: function handle3 (val, oldVal) { /* ... */ },
    ///         /* ... */
    ///       }
    ///     ],
    ///     'e.f': function (val, oldVal) { /* ... */ }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// References:
    /// - <https://vuejs.org/api/options-state.html#watch>
    ///
    pub NoVueArrowFuncInWatch {
        version: "next",
        name: "noVueArrowFuncInWatch",
        language: "js",
        recommended: true,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-arrow-functions-in-watch").same()],
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoVueArrowFuncInWatch {
    type Query = Semantic<AnyPotentialVueComponent>;
    type State = JsArrowFunctionExpression;
    type Signals = Box<[Self::State]>;
    type Options = NoVueArrowFuncInWatchOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            ctx.model(),
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return [].into();
        };

        let mut violations = vec![];
        let watchers = match component.kind() {
            AnyVueComponent::OptionsApi(component) => {
                component.declarations(VueDeclarationCollectionFilter::Watcher.into())
            }
            AnyVueComponent::DefineComponent(component) => {
                component.declarations(VueDeclarationCollectionFilter::Watcher.into())
            }
            _ => return [].into(),
        }
        .into_iter()
        .filter_map(|declaration| declaration.as_watcher().cloned());

        for watcher in watchers {
            let Some(handler) = extract_watcher_handler(&watcher) else {
                continue;
            };
            if let AnyJsFunctionExpression::JsArrowFunctionExpression(arrow_func) = handler {
                violations.push(arrow_func);
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.syntax().text_trimmed_range(),
                markup! {
                    "Found arrow function in Vue watcher definition."
                },
            )
            .note(markup! {
                "Using an arrow function here means that the `this` context will not refer to the Vue instance, which is probably not what you want."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        // Preserve async, type parameters, return type, parameters, and body semantics
        let async_token = state.async_token();
        let type_parameters = state.type_parameters();
        let return_type_annotation = state.return_type_annotation();

        // Convert arrow parameters into function parameters
        let arrow_params = state.parameters().ok()?;
        let function_params: JsParameters =
            if let Some(params) = arrow_params.as_js_parameters().cloned() {
                params
            } else {
                // Single binding like `x => {}` must become formal parameter list `(x)`
                let binding = arrow_params
                    .as_any_js_binding()?
                    .clone()
                    .trim_trailing_trivia()?;
                let formal = make::js_formal_parameter(
                    make::js_decorator_list([]),
                    AnyJsBindingPattern::AnyJsBinding(binding),
                )
                .build();
                make::js_parameters(
                    make::token(T!['(']),
                    make::js_parameter_list(
                        [AnyJsParameter::AnyJsFormalParameter(
                            AnyJsFormalParameter::JsFormalParameter(formal),
                        )],
                        [],
                    ),
                    make::token(T![')']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                )
            };

        // Convert body: expression bodies must be wrapped into a block with a return
        let arrow_body = state.body().ok()?;
        let function_body: JsFunctionBody =
            if let Some(body) = arrow_body.as_js_function_body().cloned() {
                body
            } else {
                let expr = AnyJsExpression::cast_ref(arrow_body.syntax())?.clone();
                make::js_function_body(
                    make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Newline, "\n")]),
                    make::js_directive_list(None),
                    make::js_statement_list([make::js_return_statement(
                        make::token(T![return])
                            .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                    )
                    .with_argument(expr)
                    .build()
                    .into()]),
                    make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Newline, "\n")]),
                )
            };

        // Build the function expression
        let mut func_builder =
            make::js_function_expression(make::token(T![function]), function_params, function_body);
        if let Some(async_tok) = async_token {
            func_builder = func_builder.with_async_token(async_tok);
        }
        if let Some(tp) = type_parameters {
            func_builder = func_builder.with_type_parameters(tp);
        }
        if let Some(rt) = return_type_annotation {
            func_builder = func_builder.with_return_type_annotation(rt);
        }
        let func_expr = func_builder.build();

        // Apply mutation: replace the arrow function with the new function expression
        let mut mutation = ctx.root().begin();
        let old_expr = AnyJsExpression::JsArrowFunctionExpression(state.clone());
        mutation.replace_node(old_expr, AnyJsExpression::JsFunctionExpression(func_expr));

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Rewrite the arrow function into a function expression." }.to_owned(),
            mutation,
        ))
    }
}

declare_node_union! {
    pub AnyJsFunctionExpression = JsArrowFunctionExpression | JsFunctionExpression
}

/// Extract the handler function from a watcher declaration.
///
/// This only works if the handler is a function of some kind. If its a string, then this will return None.
fn extract_watcher_handler(watcher: &JsPropertyObjectMember) -> Option<AnyJsFunctionExpression> {
    let initializer = watcher.value().ok()?;
    if let Some(func) = AnyJsFunctionExpression::cast(initializer.clone().into_syntax()) {
        return Some(func);
    }

    if let Some(object) = initializer.as_js_object_expression() {
        for member in object.members().iter() {
            let Ok(member) = member else {
                continue;
            };
            let Some(member) = member.as_js_property_object_member() else {
                continue;
            };
            let Some(name) = member.name().ok().and_then(|name| name.name()) else {
                continue;
            };
            if name != "handler" {
                continue;
            }
            if let Ok(value) = member.value()
                && let Some(func) = AnyJsFunctionExpression::cast(value.into_syntax())
            {
                return Some(func);
            }
        }
    }

    None
}
