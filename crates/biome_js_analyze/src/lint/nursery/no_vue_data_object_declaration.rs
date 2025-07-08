use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsStatement, JsCallExpression,
    JsExportDefaultExpressionClause, JsObjectExpression, T,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TriviaPieceKind};
use biome_rowan::{BatchMutationExt, declare_node_union};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce that Vue component `data` options are declared as functions.
    ///
    /// In Vue 3+, defining `data` as an object is deprecated because it leads to shared mutable state across component instances.
    /// This rule flags usages of `data: { … }` and offers an automatic fix to convert it into a function returning that object.
    ///
    /// See also:
    /// – Vue Migration Guide – Data Option: https://v3-migration.vuejs.org/breaking-changes/data-option.html :contentReference[oaicite:0]{index=0}
    /// – ESLint Plugin Vue: `no-deprecated-data-object-declaration`: https://eslint.vuejs.org/rules/no-deprecated-data-object-declaration :contentReference[oaicite:1]{index=1}
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// // root instance: shared state across all new Vue() calls
    /// new Vue({
    ///   /* ✗ BAD */
    ///   data: {
    ///     foo: null
    ///   }
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // global registration via Vue.component
    /// Vue.component('my-comp', {
    ///   /* ✗ BAD */
    ///   data: { count: 0 }
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Composition API helper also deprecated
    /// defineComponent({
    ///   /* ✗ BAD */
    ///   data: { message: 'hi' }
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// // Vue 3 entrypoint via createApp
    /// createApp({
    ///   /* ✗ BAD */
    ///   data: { active: true }
    /// }).mount('#app');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // component-local data via function
    /// export default {
    ///   /* ✓ GOOD */
    ///   data() {
    ///     return { foo: null };
    ///   }
    /// };
    /// ```
    ///
    /// ```js
    /// // global registration with function syntax
    /// Vue.component('my-comp', {
    ///   /* ✓ GOOD */
    ///   data: function () {
    ///     return { count: 0 };
    ///   }
    /// });
    /// ```
    ///
    /// ```js
    /// // Composition API and createApp entrypoints
    /// defineComponent({
    ///   /* ✓ GOOD */
    ///   data() {
    ///     return { message: 'hi' };
    ///   }
    /// });
    ///
    /// createApp({
    ///   /* ✓ GOOD */
    ///   data: function() {
    ///     return { active: true };
    ///   }
    /// }).mount('#app');
    /// ```
    ///
    pub NoVueDataObjectDeclaration {
        version: "next",
        name: "noVueDataObjectDeclaration",
        language: "vue",
        recommended: true,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Vue],
    }
}

// Anything that can hold a Vue Options object:
// `export default { … }`
// `createApp(...)`, `defineComponent(...)`, `.component(...)`
declare_node_union! {
  pub AnyVueOptionsLike = JsExportDefaultExpressionClause | JsCallExpression
}

fn get_options_object_expression(call: &JsCallExpression) -> Option<JsObjectExpression> {
    let callee = call.callee().ok()?;
    let options_argument = match callee {
        // createApp(...), defineComponent(...)
        AnyJsExpression::JsIdentifierExpression(_) => {
            let fn_name = call
                .callee()
                .ok()?
                .get_callee_member_name()?
                .token_text_trimmed();

            if fn_name == "createApp" || fn_name == "defineComponent" {
                call.arguments()
                    .ok()?
                    .args()
                    .elements()
                    .next()?
                    .into_node()
                    .ok()
            } else {
                None
            }
        }
        // dot access: `app.component(...)`
        AnyJsExpression::JsStaticMemberExpression(_) => {
            let fn_name = call
                .callee()
                .ok()?
                .get_callee_member_name()?
                .token_text_trimmed();
            if fn_name == "component" {
                call.arguments()
                    .ok()?
                    .args()
                    .elements()
                    .next()?
                    .into_node()
                    .ok()
            } else {
                None
            }
        }
        _ => return None,
    };

    JsObjectExpression::cast(options_argument?.syntax().clone())
}

pub struct RuleState {
    diagnostic_range: TextRange,
}

impl Rule for NoVueDataObjectDeclaration {
    type Query = Ast<AnyVueOptionsLike>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let vue_options_object_expression = match node {
            AnyVueOptionsLike::JsCallExpression(call) => get_options_object_expression(call),
            AnyVueOptionsLike::JsExportDefaultExpressionClause(export) => {
                if let Ok(AnyJsExpression::JsObjectExpression(object_expression)) =
                    export.expression()
                {
                    Some(object_expression)
                } else {
                    None
                }
            }
        };

        if let Some(options) = vue_options_object_expression {
            let options_object = JsObjectExpression::cast(options.syntax().clone())?;

            for member in options_object.members() {
                let member = member.ok()?;

                if let AnyJsObjectMember::JsPropertyObjectMember(property_object_member) = member {
                    return property_object_member
                        .name()
                        .ok()
                        .and_then(|n| n.name())
                        .filter(|ident| ident.text().trim() == "data")
                        .and_then(|_| property_object_member.value().ok())
                        .and_then(|value| match value {
                            AnyJsExpression::JsIdentifierExpression(_)
                            | AnyJsExpression::JsArrowFunctionExpression(_)
                            | AnyJsExpression::JsFunctionExpression(_) => None,
                            _ => Some(RuleState {
                                diagnostic_range: property_object_member.range(),
                            }),
                        });
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.diagnostic_range,
                markup! {
                    "Object declaration on 'data' property is deprecated. Using function declaration instead."
                },
            )
            .note(markup! {
                "When using the data property on a component, the value must be a function that returns an object."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();

        let vue_options_object_expression = match node {
            AnyVueOptionsLike::JsCallExpression(call) => get_options_object_expression(call),
            AnyVueOptionsLike::JsExportDefaultExpressionClause(export) => {
                export.expression().ok()?.as_js_object_expression().cloned()
            }
        };

        let options_object =
            JsObjectExpression::cast(vue_options_object_expression?.syntax().clone())?;

        for member in options_object.members() {
            let action = member.ok()?.as_js_property_object_member().and_then(|property_object_member| {
                    property_object_member
                        .name()
                        .ok()
                        .and_then(|object_member_name| object_member_name.name())
                        .filter(|object_member_name| object_member_name.text().trim() == "data")
                        .and_then(|_| property_object_member.value().ok())
                        .and_then(|value| match value {
                            AnyJsExpression::JsIdentifierExpression(_)
                            | AnyJsExpression::JsArrowFunctionExpression(_)
                            | AnyJsExpression::JsFunctionExpression(_) => None,
                            _ => {
                                let data_function = make::js_function_expression(
                                    make::token(T![function]),
                                    make::js_parameters(
                                        make::token(T!['(']),
                                        make::js_parameter_list(None, None),
                                        make::token(T![')']).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                                    ),
                                    make::js_function_body(
                                        make::token(T!['{']).with_trailing_trivia([(TriviaPieceKind::Newline, "\n")]),
                                        make::js_directive_list(None),
                                        make::js_statement_list([
                                            AnyJsStatement::JsReturnStatement(
                                                make::js_return_statement(make::token(
                                                    T![return],
                                                )
                                                .with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]))
                                                .with_argument(value.clone())
                                                .build(),
                                            ),
                                            ]),
                                            make::token(T!['}']).with_leading_trivia([(TriviaPieceKind::Newline, "\n")]),
                                        ),
                                    )
                                    .build();

                                let mut mutation = ctx.root().begin();
                                mutation.replace_node(
                                    value,
                                    AnyJsExpression::JsFunctionExpression(
                                        data_function,
                                    ),
                                );

                                Some(JsRuleAction::new(
                                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                                    ctx.metadata().applicability(),
                                    markup! { "Convert the data object to a function returning the data object" }.to_owned(),
                                    mutation,
                                ))
                        }})
                });

            if action.is_some() {
                return action;
            }
        }

        None
    }
}
