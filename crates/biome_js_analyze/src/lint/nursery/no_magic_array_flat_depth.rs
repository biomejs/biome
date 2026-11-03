use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, JsCallArgumentList, JsCallExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_magic_array_flat_depth::NoMagicArrayFlatDepthOptions;

declare_lint_rule! {
    /// Disallow unexplained numbers as the depth argument of `Array.prototype.flat()`.
    ///
    /// The depth passed to [`flat()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array/flat)
    /// is usually `1` (the default) or `Infinity`.
    /// Any other number doesn't say why the array needs to be flattened to that exact depth.
    /// Store the depth in a variable with a descriptive name, or add a comment inside the parentheses that explains it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const items = array.flat(2);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const items = array.flat(99);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const items = array.flat();
    /// const allItems = array.flat(Infinity);
    /// const allValues = array.flat(Number.POSITIVE_INFINITY);
    /// const rows = array.flat(depth);
    /// const cells = array.flat(/* The grid is always two levels deep. */ 2);
    /// ```
    ///
    pub NoMagicArrayFlatDepth {
        version: "2.6.4",
        name: "noMagicArrayFlatDepth",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-magic-array-flat-depth").same()],
        recommended: true,
    }
}

impl Rule for NoMagicArrayFlatDepth {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoMagicArrayFlatDepthOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.optional_chain_token().is_some() {
            return None;
        }

        let callee = call.callee().ok()?.omit_parentheses();
        let member = callee.as_js_static_member_expression()?.member().ok()?;
        if member.as_js_name()?.value_token().ok()?.text_trimmed() != "flat" {
            return None;
        }

        let arguments = call.arguments().ok()?;
        let depth = single_argument(&arguments.args())?;
        let depth = static_number_value(depth.as_any_js_expression()?)?;
        if depth == 1.0 || arguments.syntax().has_inner_comments() {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let depth = ctx.query().arguments().ok()?.args().first()?.ok()?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                depth.range(),
                markup! {
                    "The depth passed to "<Emphasis>"flat()"</Emphasis>" is an unexplained number."
                },
            )
            .note(markup! {
                "A bare number doesn't say why the array needs to be flattened to this depth."
            })
            .note(markup! {
                "Store the depth in a variable with a descriptive name, or add a comment inside the parentheses that explains it."
            }),
        )
    }
}

/// Returns the only argument of a call, or `None` if the call has zero or several arguments.
fn single_argument(arguments: &JsCallArgumentList) -> Option<AnyJsCallArgument> {
    let mut iter = arguments.iter();
    let first = iter.next()?.ok()?;
    iter.next().is_none().then_some(first)
}

/// Returns the value of a number literal, looking through parentheses, TypeScript type
/// wrappers, and unary `+` and `-`.
fn static_number_value(expression: &AnyJsExpression) -> Option<f64> {
    match expression.clone().omit_type_wrappers()? {
        AnyJsExpression::AnyJsLiteralExpression(literal) => {
            literal.as_js_number_literal_expression()?.as_number()
        }
        AnyJsExpression::JsUnaryExpression(unary) => {
            let value = static_number_value(&unary.argument().ok()?)?;
            match unary.operator().ok()? {
                JsUnaryOperator::Minus => Some(-value),
                JsUnaryOperator::Plus => Some(value),
                _ => None,
            }
        }
        _ => None,
    }
}
