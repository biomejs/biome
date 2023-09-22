use crate::{semantic_services::Semantic, JsRuleAction};
use biome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    global_identifier, static_value::StaticValue, AnyJsExpression, JsCallExpression,
    JsNewExpression,
};
use biome_rowan::{chain_trivia_pieces, AstNode, BatchMutationExt};

declare_rule! {
    /// Disallow `new` operators with global non-constructor functions and
    /// non-constructor built-in objects.
    ///
    /// Some global functions cannot be called using the new operator and
    /// will throw a `TypeError` if you attempt to do so. These functions are:
    ///
    /// - [`Symbol`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Symbol/Symbol)
    /// - [`BigInt`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/BigInt/BigInt)
    ///
    /// Several built-in objects cannot be instantiated and will throw a `TypeError`
    /// if you try to execute them as constructors. These objects are:
    ///
    /// - [`Math`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Math)
    /// - [`JSON`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/JSON)
    /// - [`Reflect`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Reflect)
    /// - [`Atomics`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Atomics)
    /// - [`Intl`](https://developer.mozilla.org/docs/Web/JavaScript/Reference/Global_Objects/Intl)
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-new-native-nonconstructor/
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var symbol = new Symbol('foo');
    /// var bigInt = new BigInt(9007199254740991);
    /// var math = new Math();
    /// var json = new JSON();
    /// var reflect = new Reflect();
    /// var atomics = new Atomics();
    /// var intl = new Intl();
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var symbol = Symbol('foo');
    /// var bigInt = BigInt(9007199254740991);
    ///
    /// // Ignores shadowed
    /// function foo(Symbol) {
    ///     const symbol = new Symbol('foo');
    /// }
    /// function bar(BigInt) {
    ///     const bigInt = new BigInt(9007199254740991);
    /// }
    /// function baz(Math) {
    ///     const math = new Math();
    /// }
    /// function qux(JSON) {
    ///     const json = new JSON();
    /// }
    /// function quux(Reflect) {
    ///     const reflect = new Reflect();
    /// }
    /// function corge(Intl) {
    ///     const intl = new Intl();
    /// }
    /// ```
    pub(crate) NoInvalidNewBuiltin {
        version: "next",
        name: "noInvalidNewBuiltin",
        recommended: true,
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
            "Symbol" | "BigInt" | "Math" | "JSON" | "Reflect" | "Atomics" | "Intl" => {
                ctx.model().binding(&reference).is_none().then_some(name)
            }
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

    fn action(ctx: &RuleContext<Self>, builtin_name: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let call_expression = convert_new_expression_to_call_expression(node)?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia::<AnyJsExpression>(
            node.clone().into(),
            call_expression.into(),
        );
        match builtin_name.text() {
            "Symbol" | "BigInt" => Some(JsRuleAction {
                category: ActionCategory::QuickFix,
                applicability: Applicability::MaybeIncorrect,
                message: markup! { "Remove "<Emphasis>"new"</Emphasis>"." }.to_owned(),
                mutation,
            }),
            _ => None,
        }
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
