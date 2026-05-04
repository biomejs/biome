use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, AnyJsMemberExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_useless_call::NoUselessCallOptions;

declare_lint_rule! {
    /// Disallow unnecessary `.call()` and `.apply()`.
    ///
    /// `Function.prototype.call()` and `Function.prototype.apply()` can be used to call a function with
    /// an explicit `this` value. When the provided `this` value is the same as the value JavaScript would
    /// use for a normal function call, these methods are unnecessary.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// foo.call(undefined, 1, 2, 3);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.call(null, 1, 2, 3);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.apply(undefined, [1, 2, 3]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// foo.apply(null, [1, 2, 3]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// obj.foo.call(obj, 1, 2, 3);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// obj.foo.apply(obj, [1, 2, 3]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// foo.call(obj, 1, 2, 3);
    /// foo.apply(obj, [1, 2, 3]);
    ///
    /// obj.foo.call(null, 1, 2, 3);
    /// obj.foo.apply(otherObj, [1, 2, 3]);
    ///
    /// foo.apply(undefined, args);
    /// obj.foo.apply(obj, args);
    /// ```
    ///
    pub NoUselessCall {
        version: "2.4.11",
        name: "noUselessCall",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-call").same()],
        recommended: false,
    }
}

impl Rule for NoUselessCall {
    type Query = Ast<JsCallExpression>;
    type State = UselessCallState;
    type Signals = Option<Self::State>;
    type Options = NoUselessCallOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let member_expr = AnyJsMemberExpression::cast_ref(callee.syntax())?;
        let method = CallMethod::from_name(member_expr.member_name()?.text())?;

        let arguments = call.arguments().ok()?.args();
        if arguments.is_empty() {
            return None;
        }

        let this_arg = arguments.first()?.ok()?;
        let this_arg = this_arg.as_any_js_expression()?;

        if method == CallMethod::Apply && !has_array_argument(call)? {
            return None;
        }

        let target = member_expr.object().ok()?.omit_parentheses();
        let reason = if let AnyJsExpression::JsStaticMemberExpression(target_member) = &target
        {
            target_member
                .object()
                .ok()
                .is_some_and(|object| are_same_simple_reference(&object, this_arg))
                .then_some(UselessCallReason::SameReceiver)
        } else {
            this_arg
                .as_static_value()
                .is_some_and(|value| value.is_null_or_undefined())
                .then_some(UselessCallReason::NullishReceiver)
        };

        Some(UselessCallState {
            method,
            reason: reason?,
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let call = ctx.query();
        let method = state.method.name();

        let detail = match state.reason {
            UselessCallReason::NullishReceiver => markup! {
                "The receiver is "<Emphasis>"null"</Emphasis>" or "<Emphasis>"undefined"</Emphasis>", so "<Emphasis>"."{method}"()"</Emphasis>" does not change the relevant call semantics."
            },
            UselessCallReason::SameReceiver => markup! {
                "The receiver is the same object that a direct call would use, so "<Emphasis>"."{method}"()"</Emphasis>" does not change the relevant call semantics."
            },
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            call.range(),
            markup! {
                "This "<Emphasis>"."{method}"()"</Emphasis>" call is unnecessary."
            },
        )
        .detail(call.range(), detail)
        .note(markup! {
            "Use a direct function call instead."
        }))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UselessCallState {
    method: CallMethod,
    reason: UselessCallReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum UselessCallReason {
    NullishReceiver,
    SameReceiver,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CallMethod {
    Apply,
    Call,
}

impl CallMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "apply" => Some(Self::Apply),
            "call" => Some(Self::Call),
            _ => None,
        }
    }

    const fn name(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::Call => "call",
        }
    }
}

fn has_array_argument(call: &JsCallExpression) -> Option<bool> {
    let arguments = call.arguments().ok()?.args();
    if arguments.len() != 2 {
        return Some(false);
    }

    let array_argument = arguments.last()?.ok()?;
    let array_argument = array_argument
        .as_any_js_expression()?
        .clone()
        .omit_parentheses();

    Some(matches!(array_argument, AnyJsExpression::JsArrayExpression(_)))
}

fn are_same_simple_reference(left: &AnyJsExpression, right: &AnyJsExpression) -> bool {
    match (left.clone().omit_parentheses(), right.clone().omit_parentheses()) {
        (AnyJsExpression::JsIdentifierExpression(left), AnyJsExpression::JsIdentifierExpression(right)) => {
            left.name()
                .ok()
                .zip(right.name().ok())
                .is_some_and(|(left, right)| {
                    left.value_token()
                        .ok()
                        .zip(right.value_token().ok())
                        .is_some_and(|(left, right)| left.text_trimmed() == right.text_trimmed())
                })
        }
        _ => false,
    }
}
