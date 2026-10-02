use crate::utils::is_node_equal;
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
    /// `.call()` and `.apply()` run a function and let you choose what `this` refers to
    /// inside it: their first argument becomes `this`. When that is the same value the
    /// function would get from a normal call, `.call()` and `.apply()` change nothing.
    /// `foo.call(undefined, a)` does the same thing as `foo(a)`, and
    /// `obj.foo.call(obj, a)` does the same thing as `obj.foo(a)`.
    ///
    /// Calling the function directly is also faster. `.call()` and `.apply()` add an extra
    /// step to every call, and `.apply()` creates an array only to hold the arguments.
    /// JavaScript engines, the programs that run your code in browsers and Node.js, can
    /// often skip this extra work, but a direct call never has it.
    ///
    /// This rule reports `.call()` and `.apply()` when their first argument is:
    ///
    /// - the object the function is read from, such as `obj` in `obj.foo.call(obj)`;
    /// - `null`, `undefined`, or `void` followed by any value, such as `void 0`, when the
    ///   function is not read from an object, such as `foo.call(null)`.
    ///
    /// `.apply()` is only reported when it has exactly two arguments and the second one is
    /// written out as an array, like `[a, b]`, because then the items of the array can be
    /// passed to the function directly.
    ///
    /// This rule looks at how the code is written, not at the values it produces. In
    /// `a[i++].foo.call(a[i++])`, both `a[i++]` are written the same way, so the call is
    /// reported even though they refer to different items.
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
    /// foo.apply(null, [1, 2, 3]);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// obj.foo.call(obj, 1, 2, 3);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// a.b.foo.apply(a.b, [1, 2, 3]);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // The `this` value is different.
    /// foo.call(obj, 1, 2, 3);
    /// foo.apply(obj, [1, 2, 3]);
    /// obj.foo.call(null, 1, 2, 3);
    /// obj.foo.apply(otherObj, [1, 2, 3]);
    ///
    /// // The arguments are not written out as an array.
    /// foo.apply(undefined, args);
    /// obj.foo.apply(obj, args);
    /// ```
    ///
    pub NoUselessCall {
        version: "next",
        name: "noUselessCall",
        language: "js",
        sources: &[RuleSource::Eslint("no-useless-call").same()],
        recommended: true,
    }
}

impl Rule for NoUselessCall {
    type Query = Ast<JsCallExpression>;
    type State = UselessCallMethod;
    type Signals = Option<Self::State>;
    type Options = NoUselessCallOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let AnyJsExpression::JsStaticMemberExpression(callee) =
            node.callee().ok()?.omit_parentheses()
        else {
            return None;
        };
        let member = callee.member().ok()?;
        let member_name = member.as_js_name()?.value_token().ok()?;
        let arguments = node.arguments().ok()?.args();
        let method = match member_name.text_trimmed() {
            "call" if !arguments.is_empty() => UselessCallMethod::Call,
            "apply"
                if arguments.len() == 2
                    && arguments
                        .last()?
                        .ok()?
                        .as_any_js_expression()?
                        .clone()
                        .omit_parentheses()
                        .as_js_array_expression()
                        .is_some() =>
            {
                UselessCallMethod::Apply
            }
            _ => return None,
        };

        let this_arg = arguments
            .first()?
            .ok()?
            .as_any_js_expression()?
            .clone()
            .omit_parentheses();
        let applied = callee.object().ok()?.omit_parentheses();
        let is_useless = match AnyJsMemberExpression::cast(applied.into_syntax()) {
            Some(applied) => {
                let expected_this = applied.object().ok()?.omit_parentheses();
                is_node_equal(expected_this.syntax(), this_arg.syntax())
            }
            None => is_null_or_undefined(&this_arg),
        };

        is_useless.then_some(method)
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This "<Emphasis>{state}</Emphasis>" is unnecessary."
                },
            )
            .note(markup! {
                "The function gets the same "<Emphasis>"this"</Emphasis>" value as it would if you called it directly."
            })
            .note(match state {
                UselessCallMethod::Call => markup! {
                    "Call the function directly instead."
                },
                UselessCallMethod::Apply => markup! {
                    "Call the function directly and pass the items of the array as arguments instead."
                },
            }),
        )
    }
}

pub enum UselessCallMethod {
    Call,
    Apply,
}

impl biome_console::fmt::Display for UselessCallMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Call => ".call()",
            Self::Apply => ".apply()",
        })
    }
}

/// Returns `true` when `expression` is `null`, `undefined`, or any `void` expression.
///
/// `undefined` is matched by name, so a local variable that shadows it still counts.
fn is_null_or_undefined(expression: &AnyJsExpression) -> bool {
    match expression {
        AnyJsExpression::JsUnaryExpression(unary) => unary.is_void().unwrap_or_default(),
        _ => expression
            .as_static_value()
            .is_some_and(|value| value.is_null_or_undefined()),
    }
}
