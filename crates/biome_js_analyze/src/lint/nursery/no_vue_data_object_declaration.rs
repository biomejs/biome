use biome_analyze::{Ast, FixKind, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{AnyJsExpression, AnyJsStatement, JsPropertyObjectMember, T};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce component's data property to be a function
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <script>
    /// export default {
    ///   /* ✗ BAD */
    ///   data: {
    ///     foo: null
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,expect_diagnostic
    /// <script>
    /// export default {
    ///   /* ✓ GOOD */
    ///   data() {
    ///     return {
    ///       foo: null
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    pub NoVueDataObjectDeclaration {
        version: "next",
        name: "noVueDataObjectDeclaration",
        language: "js",
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoVueDataObjectDeclaration {
    type Query = Ast<JsPropertyObjectMember>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let member = ctx.query();

        if let Ok(member_name) = member.name() {
            if let Some(name) = member_name.name() {
                if name.trim() == "data" {
                    if let Ok(value) = member.value() {
                        match value {
                            AnyJsExpression::JsIdentifierExpression(_)
                            | AnyJsExpression::JsArrowFunctionExpression(_)
                            | AnyJsExpression::JsFunctionExpression(_) => return None,
                            _ => return Some(()),
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
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
        let member = ctx.query();
        let mut mutation = ctx.root().begin();

        if let Ok(member_name) = member.name() {
            if let Some(name) = member_name.name() {
                if name.trim() == "data" {
                    if let Ok(value) = member.value() {
                        match value {
                            AnyJsExpression::JsIdentifierExpression(_)
                            | AnyJsExpression::JsArrowFunctionExpression(_)
                            | AnyJsExpression::JsFunctionExpression(_) => return None,
                            _ => {
                                let data_function = make::js_function_expression(
                                    make::token(T![function]),
                                    make::js_parameters(
                                        make::token(T!['(']),
                                        make::js_parameter_list(None, None),
                                        make::token(T![')']),
                                    ),
                                    make::js_function_body(
                                        make::token(T!['{']),
                                        make::js_directive_list(None),
                                        make::js_statement_list([
                                            AnyJsStatement::JsReturnStatement(
                                                make::js_return_statement(make::token(T![return]))
                                                    .with_argument(value)
                                                    .build(),
                                            ),
                                        ]),
                                        make::token(T!['}']),
                                    ),
                                )
                                .build();
                                let data_property_object_member = make::js_property_object_member(
                                    member_name,
                                    make::token(T![:]),
                                    AnyJsExpression::JsFunctionExpression(data_function),
                                );

                                mutation.replace_node(member.clone(), data_property_object_member);

                                return Some(JsRuleAction::new(
                                    ctx.metadata().action_category(ctx.category(), ctx.group()),
                                    ctx.metadata().applicability(),
                                    markup! { "Convert the data object to a function returning the data object" }.to_owned(),
                                    mutation,
                                ));
                            }
                        }
                    }
                }
            }
        }

        None
    }
}
