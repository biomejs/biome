use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsCallExpression, JsComputedMemberExpression, JsStaticMemberExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange};
use biome_rule_options::use_find::UseFindOptions;

use crate::services::typed::Typed;

declare_lint_rule! {
    /// Enforce the use of Array.prototype.find() over Array.prototype.filter() followed by [0] when looking for a single result.
    ///
    /// When searching for the first item in an array matching a condition, it may be tempting to use code like `arr.filter(x => x > 0)[0]`.
    /// However, it is simpler to use `Array.prototype.find()` instead, `arr.find(x => x > 0)`, which also returns the first entry matching a condition.
    /// Because the `.find()` only needs to execute the callback until it finds a match, it's also more efficient.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid.ts
    /// [1, 2, 3].filter(x => x > 1)[0];
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid2.ts
    /// [1, 2, 3].filter(x => x > 1).at(0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid.ts
    /// [1, 2, 3].find(x => x > 1);
    /// ```
    ///
    pub UseFind {
        version: "2.3.6",
        name: "useFind",
        language: "js",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("prefer-find").same()],
        domains: &[RuleDomain::Types],
    }
}

fn is_first_position(ctx: &RuleContext<UseFind>, express: &AnyJsExpression) -> bool {
    ctx.type_of_expression(express).is_number_literal(0.)
        || ctx.type_of_expression(express).is_bigint_literal(0)
}

impl Rule for UseFind {
    type Query = Typed<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseFindOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let binding_callee = binding.callee().ok()?;
        let binding_callee_static = binding_callee.as_js_static_member_expression()?;

        let member = binding_callee_static.member().ok()?;
        let member_name = member.as_js_name()?;
        let member_value = member_name.value_token().ok()?;
        if member_value.text_trimmed() != "filter" {
            return None;
        }

        let parent = binding.syntax().parent()?;

        // Handle .filter()[0]
        if JsComputedMemberExpression::can_cast(parent.kind()) {
            let express = JsComputedMemberExpression::cast(parent)?;
            let member = express.member().ok()?;

            if is_first_position(ctx, &member) {
                return Some(express.range());
            }

        // Handle .filter().at(0)
        } else if JsStaticMemberExpression::can_cast(parent.kind()) {
            let express = JsStaticMemberExpression::cast(parent)?;

            let member = express.member().ok()?;
            let value_token = member.value_token().ok()?;
            if value_token.text_trimmed() != "at" {
                return None;
            }

            let call_parent = express.syntax().parent()?;
            let call_parent_express = JsCallExpression::cast(call_parent)?;
            let arguments = call_parent_express.arguments().ok()?;
            let first_arg = arguments.args().first()?.ok()?;
            let first_arg_express = first_arg.as_any_js_expression()?;

            if is_first_position(ctx, first_arg_express) {
                return Some(call_parent_express.range());
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Prefer using Array#find() over Array#filter[0]."
                },
            )
            .note(markup! {
                "Use Array#find() instead of Array#filter[0] to improve readability."
            }),
        )
    }
}
