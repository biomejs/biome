use biome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, FixKind, Rule, RuleDiagnostic,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make::{ident, js_call_argument_list, js_call_arguments, js_name, token};
use biome_js_syntax::{
    AnyJsExpression, AnyJsMemberExpression, AnyJsName, JsCallExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow to use unnecessary callback on `flatMap`
    ///
    /// To achieve the same result (flattening an array) more concisely and efficiently, you should use flat() instead
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// array.flatMap((arr) => arr);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// array.flatMap((arr) => arr * 2);
    /// ```
    ///
    pub NoFlatMapIdentity {
        version: "next",
        name: "noFlatMapIdentity",
        recommended: false,
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
        let arguments = flat_map_call.arguments().ok()?.args();
        if let Some(arg) = arguments.first() {
            let arg = arg.ok()?;
            let arg = arg
                .as_any_js_expression()?
                .as_js_arrow_function_expression()?;
            let function_parameter = arg.parameters().ok()?.text();
            let function_body = arg.body().ok()?.text();
            let function_parameter = function_parameter.trim_matches(&['(', ')']);
            if function_parameter != function_body {
                return None;
            }
            let flat_map_expression =
                AnyJsMemberExpression::cast_ref(flat_map_call.callee().ok()?.syntax())?;
            if flat_map_expression.member_name()?.text() != "flatMap" {
                return None;
            }
        };

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "flat method can be used to simplify"
            },
        ))
    }
    fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let flat_member = js_name(ident("flat"));

        let Ok(AnyJsExpression::JsStaticMemberExpression(flat_expression)) = node.callee() else {
            return None;
        };

        let empty_argument = js_call_arguments(
            token(JsSyntaxKind::L_PAREN),
            js_call_argument_list(vec![], vec![token(JsSyntaxKind::COMMA)]),
            token(JsSyntaxKind::R_PAREN),
        );

        let flat_call = flat_expression.with_member(AnyJsName::JsName(flat_member));

        mutation.replace_node(
            node.clone(),
            node.clone()
                .with_arguments(empty_argument)
                .with_callee(AnyJsExpression::JsStaticMemberExpression(flat_call)),
        );

        Some(JsRuleAction {
            mutation,
            message: markup! {"Replace unnecessary flatMap call to flat instead"}.to_owned(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::Always,
        })
    }
}
