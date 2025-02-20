use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make::{ident, js_call_argument_list, js_call_arguments, js_name, token};
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression, AnyJsName, AnyJsStatement,
    JsCallExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Disallow to use unnecessary callback on `flatMap`.
    ///
    /// To achieve the same result (flattening an array) more concisely and efficiently, you should use `flat` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.flatMap((arr) => arr);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// array.flatMap((arr) => {return arr});
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// array.flatMap((arr) => arr * 2);
    /// ```
    ///
    pub NoFlatMapIdentity {
        version: "1.7.0",
        name: "noFlatMapIdentity",
        language: "js",
        severity: Severity::Information,
        recommended: true,
        sources: &[RuleSource::Clippy("flat_map_identity")],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for NoFlatMapIdentity {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let flat_map_call = ctx.query();

        let flat_map_expression =
            AnyJsMemberExpression::cast(flat_map_call.callee().ok()?.into_syntax())?;

        if flat_map_expression.object().is_err() {
            return None;
        }

        if flat_map_expression.member_name()?.text() != "flatMap" {
            return None;
        }

        let arguments = flat_map_call.arguments().ok()?.args();

        if let Some(arg) = arguments.first() {
            let arg = arg.ok()?;
            let (function_param, function_body) = match arg.as_any_js_expression()? {
                AnyJsExpression::JsArrowFunctionExpression(arg) => {
                    let parameter: String = match arg.parameters().ok()? {
                        biome_js_syntax::AnyJsArrowFunctionParameters::AnyJsBinding(p) => {
                            p.to_trimmed_string().trim_matches(['(', ')']).to_owned()
                        }
                        biome_js_syntax::AnyJsArrowFunctionParameters::JsParameters(p) => {
                            if p.items().len() == 1 {
                                if let Some(param) = p.items().into_iter().next() {
                                    param.ok()?.to_trimmed_string()
                                } else {
                                    return None;
                                }
                            } else {
                                return None;
                            }
                        }
                    };

                    let function_body: String = match arg.body().ok()? {
                        AnyJsFunctionBody::AnyJsExpression(body) => {
                            body.omit_parentheses().to_trimmed_string()
                        }
                        AnyJsFunctionBody::JsFunctionBody(body) => {
                            let mut statement = body.statements().into_iter();
                            match statement.next() {
                                Some(AnyJsStatement::JsReturnStatement(body)) => {
                                    let Some(AnyJsExpression::JsIdentifierExpression(
                                        return_statement,
                                    )) = body.argument()
                                    else {
                                        return None;
                                    };
                                    return_statement.name().ok()?.to_trimmed_string()
                                }
                                _ => return None,
                            }
                        }
                    };
                    (parameter, function_body)
                }
                AnyJsExpression::JsFunctionExpression(arg) => {
                    let function_parameter = arg.parameters().ok()?.to_trimmed_string();
                    let function_parameter = function_parameter.trim_matches(['(', ')']).to_owned();

                    let mut statement = arg.body().ok()?.statements().into_iter();
                    if let Some(AnyJsStatement::JsReturnStatement(body)) = statement.next() {
                        let Some(AnyJsExpression::JsIdentifierExpression(return_statement)) =
                            body.argument()
                        else {
                            return None;
                        };
                        (
                            function_parameter,
                            return_statement.name().ok()?.to_trimmed_string(),
                        )
                    } else {
                        return None;
                    }
                }
                _ => return None,
            };

            if function_param == function_body {
                return Some(());
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
                    "Avoid unnecessary callback in "<Emphasis>"flatMap"</Emphasis>" call."
                },
            )
            .note(markup! {"You can just use "<Emphasis>"flat"</Emphasis>" to flatten the array."}),
        )
    }
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let empty_argument = js_call_arguments(
            token(JsSyntaxKind::L_PAREN),
            js_call_argument_list(vec![], vec![]),
            token(JsSyntaxKind::R_PAREN),
        );

        let Ok(AnyJsExpression::JsStaticMemberExpression(flat_expression)) = node.callee() else {
            return None;
        };

        let flat_member = js_name(ident("flat"));
        let flat_call = flat_expression.with_member(AnyJsName::JsName(flat_member));

        mutation.replace_node(
            node.clone(),
            node.clone()
                .with_arguments(empty_argument)
                .with_callee(AnyJsExpression::JsStaticMemberExpression(flat_call)),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! {"Replace unnecessary "<Emphasis>"flatMap"</Emphasis>" call to "<Emphasis>"flat"</Emphasis>" instead."}.to_owned(),
            mutation,
        ))
    }
}
