use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsCallArgumentList,
    JsCallExpression, JsSyntaxKind,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Enforce using the digits argument with `Number#toFixed()`.
    ///
    /// When using `Number#toFixed()` explicitly specify the number of digits you want to appear after the decimal point,
    /// to avoid unexpected results, rather than relying on its default value of 0.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const string = number.toFixed();
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const string = foo.toFixed(0);
    /// ```
    /// ```js
    /// const string = foo.toFixed(2);
    /// ```
    ///
    /// ## Caveats
    ///
    /// This rule always assumes that `toFixed` is called on a number.
    /// It does not check the type of the callee.
    ///
    pub UseNumberToFixedDigitsArgument {
        version: "1.8.0",
        name: "useNumberToFixedDigitsArgument",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("require-number-to-fixed-digits-argument")],
        recommended: false,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseNumberToFixedDigitsArgument {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_optional() || node.is_optional_chain() || node.arguments().ok()?.args().len() > 0
        {
            return None;
        }

        let callee = node.callee().ok()?;

        // TODO. This could be improved with type inference,
        // to only match if the callee is a number.
        if callee.get_callee_member_name()?.token_text() != "toFixed" {
            return None;
        }

        let is_new_expr = callee
            .as_js_static_member_expression()
            .and_then(|e| e.object().ok())
            .is_some_and(|e| e.syntax().kind() == JsSyntaxKind::JS_NEW_EXPRESSION);

        if is_new_expr {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query().arguments().ok()?;

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Specify the number of digits you want to appear after the decimal point."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        let zero_literal = AnyJsLiteralExpression::JsNumberLiteralExpression(
            make::js_number_literal_expression(make::js_number_literal('0')),
        );

        let arg = AnyJsCallArgument::AnyJsExpression(AnyJsExpression::AnyJsLiteralExpression(
            zero_literal,
        ));

        let previous_args = ctx.query().arguments().ok()?.args();
        let new_args = make::js_call_argument_list([arg], []);

        mutation.replace_node::<JsCallArgumentList>(previous_args, new_args);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            Applicability::MaybeIncorrect,
            markup! {
                "Add explicit digits argument to "<Emphasis>"toFixed"</Emphasis>" method."
            }
            .to_owned(),
            mutation,
        ))
    }
}
