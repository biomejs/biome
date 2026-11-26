use biome_analyze::RuleSource;
use biome_analyze::{
    FixKind, Rule, RuleDiagnostic, RuleDomain, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, AnyJsStatement, JsObjectExpression, T,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TriviaPieceKind};
use biome_rowan::{BatchMutationExt, SyntaxNodeCast};
use biome_rule_options::no_vue_data_object_declaration::NoVueDataObjectDeclarationOptions;

use crate::JsRuleAction;
use crate::services::vue::VueComponent;

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
    /// ```js
    /// // component-local data via function
    /// export default {
    ///   /* ✗ BAD */
    ///   data: { foo: null },
    /// };
    /// ```
    ///
    /// ```js
    /// // Composition API helper also deprecated
    /// defineComponent({
    ///   /* ✗ BAD */
    ///   data: { message: 'hi' }
    /// });
    /// ```
    ///
    /// ```js
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
        version: "2.1.4",
        name: "noVueDataObjectDeclaration",
        language: "js",
        recommended: true,
        fix_kind: FixKind::Safe,
        domains: &[RuleDomain::Vue],
        sources: &[
            RuleSource::EslintVueJs("no-deprecated-data-object-declaration").inspired(),
            RuleSource::EslintVueJs("no-shared-component-data").inspired(),
        ],
        severity: Severity::Error,
    }
}

pub struct State {
    /// The range around the entire data declaration.
    data_decl_range: TextRange,

    /// The object expression representing the value of the data declaration.
    object_expression: JsObjectExpression,
}

impl Rule for NoVueDataObjectDeclaration {
    type Query = VueComponent;
    type State = State;
    type Signals = Option<Self::State>;
    type Options = NoVueDataObjectDeclarationOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.vue_model().node(&ctx.query().options_object_node()?)?;
        let data_decl = JsObjectExpression::cast(options)?
            .members()
            .iter()
            .flatten()
            .find(|member| {
                matches!(
                    member,
                    AnyJsObjectMember::JsPropertyObjectMember(_)
                        | AnyJsObjectMember::JsMethodObjectMember(_)
                ) && member.name().is_some_and(|name| name.text() == "data")
            })?;
        let AnyJsObjectMember::JsPropertyObjectMember(property) = &data_decl else {
            return None;
        };
        let object_expression = property
            .value()
            .ok()?
            .omit_parentheses()
            .as_js_object_expression()
            .cloned()?;
        Some(State {
            data_decl_range: data_decl.range(),
            object_expression,
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.data_decl_range,
                markup! {
                    "Found an object declaration for "<Emphasis>"`data`"</Emphasis>" in this component."
                },
            )
            .note(markup! {
                "Using an object declaration for "<Emphasis>"`data`"</Emphasis>" is deprecated, and can result in different component instances sharing the same data."
            }),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let data_expr = state
            .object_expression
            .syntax()
            .clone()
            .cast::<AnyJsExpression>()?;

        let mut mutation = ctx.root().begin();
        mutation.wrap_node(data_expr, |data_expr| {
            AnyJsExpression::JsFunctionExpression(
                make::js_function_expression(
                    make::token(T![function]),
                    make::js_parameters(
                        make::token(T!['(']),
                        make::js_parameter_list(None, None),
                        make::token(T![')']).with_trailing_space(),
                    ),
                    make::js_function_body(
                        make::token(T!['{'])
                            .with_trailing_trivia([(TriviaPieceKind::Newline, "\n")]),
                        make::js_directive_list(None),
                        make::js_statement_list([AnyJsStatement::JsReturnStatement(
                            make::js_return_statement(
                                make::token(T![return]).with_trailing_space(),
                            )
                            .with_argument(data_expr)
                            .build(),
                        )]),
                        make::token(T!['}']).with_leading_newline(),
                    ),
                )
                .build(),
            )
        });

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Refactor the data object into a function returning the data object" }
                .to_owned(),
            mutation,
        ))
    }
}
