use crate::{services::semantic::Semantic, JsRuleAction};
use biome_analyze::{context::RuleContext, declare_lint_rule, FixKind, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, static_value::StaticValue, AnyJsExpression, JsCallExpression,
    JsNewExpression,
};
use biome_rowan::{chain_trivia_pieces, AstNode, BatchMutationExt};

declare_lint_rule! {
    /// Disallow `new` operators with global non-constructor functions.
    ///
    /// Some global functions cannot be called using the new operator and
    /// will throw a `TypeError` if you attempt to do so. These functions are:
    ///
    /// - [`Symbol`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol)
    /// - [`BigInt`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let foo = new Symbol('foo');
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let bar = new BigInt(9007199254740991);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let foo = Symbol('foo');
    ///
    /// function baz(Symbol) {
    ///     const qux = new Symbol("baz");
    /// }
    /// ```
    ///
    /// ```js
    /// let bar = BigInt(9007199254740991);
    ///
    /// function quux(BigInt) {
    ///     const corge = new BigInt(9007199254740991);
    /// }
    /// ```
    pub NoInvalidNewBuiltin {
        version: "1.3.0",
        name: "noInvalidNewBuiltin",
        language: "js",
        recommended: false,
        deprecated: "Use the rule noInvalidBuiltinInstantiation instead.",
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoInvalidNewBuiltin {
    type Query = Semantic<JsNewExpression>;
    type State = StaticValue;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let callee = ctx.query().callee().ok()?;
        let (reference, name) = global_identifier(&callee)?;
        match name.text() {
            "Symbol" | "BigInt" => ctx.model().binding(&reference).is_none().then_some(name),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, builtin_name: &Self::State) -> Option<RuleDiagnostic> {
        let builtin_name = builtin_name.text();
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                <Emphasis>{builtin_name}</Emphasis>" cannot be called as a constructor."
            },
        ).note(markup! {
            "Calling "<Emphasis>{builtin_name}</Emphasis>" with the "<Emphasis>"new"</Emphasis>" operator throws a "<Emphasis>"TypeError"</Emphasis>"."
        }))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let call_expression = convert_new_expression_to_call_expression(node)?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia::<AnyJsExpression>(
            node.clone().into(),
            call_expression.into(),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Remove "<Emphasis>"new"</Emphasis>"." }.to_owned(),
            mutation,
        ))
    }
}

fn convert_new_expression_to_call_expression(expr: &JsNewExpression) -> Option<JsCallExpression> {
    let new_token = expr.new_token().ok()?;
    let mut callee = expr.callee().ok()?;
    if new_token.has_leading_comments() || new_token.has_trailing_comments() {
        callee = callee.prepend_trivia_pieces(chain_trivia_pieces(
            new_token.leading_trivia().pieces(),
            new_token.trailing_trivia().pieces(),
        ))?;
    }
    Some(make::js_call_expression(callee, expr.arguments()?).build())
}
