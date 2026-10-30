use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::AstNode;
use biome_rule_options::no_dynamic_delete::NoDynamicDeleteOptions;

declare_lint_rule! {
    /// Disallow using the `delete` operator with a property name that is computed at runtime.
    ///
    /// The `delete` operator removes a property from an object. When the property name
    /// is written inside square brackets and comes from a variable, a function call, or
    /// another expression, the code is using a plain object to store a changing set of keys.
    /// It is hard to tell which property gets removed just by reading the code.
    ///
    /// A [`Map`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Map)
    /// or a [`Set`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Set)
    /// is built for this use. Both have a `delete()` method for removing entries.
    ///
    /// This rule allows square brackets that contain a string literal, a number literal,
    /// or a negative number literal, because the property name is fixed in the code.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const name = "name";
    /// delete container[name];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// delete container[name.toUpperCase()];
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// delete container.aaa;
    /// delete container[7];
    /// delete container[-1];
    /// delete container["aaa"];
    /// delete container["Infinity"];
    ///
    /// const map = new Map();
    /// map.delete(name);
    /// ```
    ///
    /// ## See Also
    ///
    /// - If you want to disallow the `delete` operator on fixed property names, see [`noDelete`](https://biomejs.dev/linter/rules/no-delete/).
    ///
    pub NoDynamicDelete {
        version: "next",
        name: "noDynamicDelete",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-dynamic-delete").same()],
        recommended: false,
    }
}

impl Rule for NoDynamicDelete {
    type Query = Ast<JsUnaryExpression>;
    /// The expression inside the square brackets.
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = NoDynamicDeleteOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.operator().ok()? != JsUnaryOperator::Delete {
            return None;
        }
        let argument = node.argument().ok()?.omit_parentheses();
        let member = argument
            .as_js_computed_member_expression()?
            .member()
            .ok()?
            .omit_parentheses();
        (!is_fixed_key(&member)).then_some(member)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, member: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member.range(),
                markup! {
                    "Avoid using "<Emphasis>"delete"</Emphasis>" with a property name that is computed at runtime."
                },
            )
            .note(markup! {
                "Removing properties by a changing name makes it hard to tell which property gets removed."
            })
            .note(markup! {
                "Use a "<Emphasis>"Map"</Emphasis>" or a "<Emphasis>"Set"</Emphasis>" and call its "<Emphasis>"delete()"</Emphasis>" method instead."
            }),
        )
    }
}

/// Returns whether `member` is a string literal, a number literal, or a number
/// literal preceded by `-`.
fn is_fixed_key(member: &AnyJsExpression) -> bool {
    match member {
        AnyJsExpression::AnyJsLiteralExpression(
            AnyJsLiteralExpression::JsStringLiteralExpression(_)
            | AnyJsLiteralExpression::JsNumberLiteralExpression(_),
        ) => true,
        AnyJsExpression::JsUnaryExpression(unary) => {
            unary.operator() == Ok(JsUnaryOperator::Minus)
                && unary.argument().is_ok_and(|argument| {
                    matches!(
                        argument.omit_parentheses(),
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                        )
                    )
                })
        }
        _ => false,
    }
}
