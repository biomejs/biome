use crate::JsRuleAction;
use biome_analyze::{
    Ast, FixKind, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsMemberExpression, JsCallExpression, JsLanguage, T,
};
use biome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxToken};
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
    /// foo.apply(null, [1, 2, 3]);
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
    /// ```
    ///
    pub UseSpread {
        version: "2.3.6",
        name: "useSpread",
        language: "js",
        sources: &[
            RuleSource::Eslint("prefer-spread").same(),
            RuleSource::EslintE18e("prefer-spread-syntax").inspired(),
        ],
        recommended: true,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseSpread {
    type Query = Ast<JsCallExpression>;
    type State = (AnyJsExpression, AnyJsExpression);
    type Signals = Option<Self::State>;
    type Options = UseSpreadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;

        let member_expr = AnyJsMemberExpression::cast_ref(callee.syntax())?;
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

        let applied_object = member_expr.object().ok()?;

        let is_same_reference = if let Some(object_member) =
            AnyJsMemberExpression::cast(applied_object.clone().into_syntax())
        {
            are_nodes_equal(&object_member.object().ok()?, this_arg)
        } else {
            // This handles cases like `foo.apply(null, args)` or `foo.apply(undefined, args)`
            this_arg
                .as_static_value()
                .is_some_and(|v| v.is_null_or_undefined())
        };

        if is_same_reference {
            Some((applied_object.clone(), spread_candidate.clone()))
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
               <Emphasis>"apply()"</Emphasis>" is used to call a function with arguments provided as an array."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let (object, spread_candidate) = state;

        let new_arguments = make::js_call_arguments(
            make::token(T!['(']),
            make::js_call_argument_list(
                [AnyJsCallArgument::from(make::js_spread(
                    make::token(T![...]),
                    spread_candidate.clone(),
                ))],
                [],
            ),
            make::token(T![')']),
        );

        let new_call_expression = make::js_call_expression(object.clone(), new_arguments).build();

        mutation.replace_node(node.clone(), new_call_expression);

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use the spread operator." }.to_owned(),
            mutation,
        ))
    }
}

fn get_identifier_token(node: &AnyJsExpression) -> Option<SyntaxToken<JsLanguage>> {
    match node {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .ok()
            .and_then(|name| name.value_token().ok()),
        _ => None,
    }
}

fn are_nodes_equal(node1: &AnyJsExpression, node2: &AnyJsExpression) -> bool {
    let object_token = get_identifier_token(node1);
    let this_token = get_identifier_token(node2);

    match (object_token, this_token) {
        (Some(object_token), Some(this_token)) => {
            object_token.text_trimmed() == this_token.text_trimmed()
        }
        _ => false,
    }
}
