use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsMemberExpression, JsCallExpression, JsSyntaxKind, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt};
use biome_rule_options::use_spread::UseSpreadOptions;

declare_lint_rule! {
    /// Enforce the use of the spread operator over `.apply()`.
    ///
    /// The `apply()` method is used to call a function with a given `this` value and arguments provided as an array.
    /// The spread operator `...` can be used to achieve the same result, which is more concise and easier to read.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo.apply(null, args);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.apply(undefined, args);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// obj.foo.apply(obj, args);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo(...args);
    ///
    /// obj.foo(...args);
    ///
    /// foo.apply(obj, [1, 2, 3]);
    ///
    /// foo.apply(null, [1, 2, 3]);
    /// ```
    ///
    pub UseSpread {
        version: "next",
        name: "useSpread",
        language: "js",
        sources: &[RuleSource::Eslint("prefer-spread").same()],
        recommended: true,
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSpread {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        let member_expr = AnyJsMemberExpression::cast(callee.into_syntax())?;
        if member_expr.member_name()?.text() != "apply" {
            return None;
        }

        let arguments = node.arguments().ok()?.args();
        if arguments.len() != 2 {
            return None;
        }

        let first_arg = arguments.first()?.ok()?;
        let this_arg = first_arg.as_any_js_expression()?;

        let second_arg = arguments.last()?.ok()?;
        let spread_candidate = second_arg.as_any_js_expression()?;

        // The rule should not flag `.apply()` calls where the second argument is an array literal.
        if spread_candidate.syntax().kind() == JsSyntaxKind::JS_ARRAY_EXPRESSION {
            return None;
        }

        let applied_object = member_expr.object().ok()?;

        let is_this_correct = if let Some(object_member) =
            AnyJsMemberExpression::cast(applied_object.clone().into_syntax())
        {
            object_member.object().ok()?.syntax().text_trimmed() == this_arg.syntax().text_trimmed()
        } else {
            // This handles cases like `foo.apply(null, args)` or `foo.apply(undefined, args)`
            this_arg
                .as_static_value()
                .is_some_and(|v| v.is_null_or_undefined())
        };

        if is_this_correct { Some(()) } else { None }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Use the spread operator instead of "<Emphasis>".apply()"</Emphasis>"."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let callee = node.callee().ok()?;
        let member_expr = AnyJsMemberExpression::cast(callee.into_syntax())?;
        let object = member_expr.object().ok()?;

        let arguments = node.arguments().ok()?;
        let arguments_list = arguments.args();

        let new_arguments = make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list(
                [AnyJsCallArgument::from(make::js_spread(
                    make::token(T![...]),
                    arguments_list.last()?.ok()?.as_any_js_expression()?.clone(),
                ))],
                [],
            ),
            make::token(T![')']),
        );

        let new_call_expression = make::js_call_expression(object, new_arguments).build();

        mutation.replace_node(node.clone(), new_call_expression);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the spread operator." }.to_owned(),
            mutation,
        ))
    }
}
