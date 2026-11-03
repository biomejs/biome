use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_number_argument_out_of_range::NoNumberArgumentOutOfRangeOptions;

use crate::utils::static_number_value;

declare_lint_rule! {
    /// Disallow number method arguments that are outside the range the method accepts.
    ///
    /// Some methods of numbers only accept a limited range of values as their first argument,
    /// and throw a `RangeError` when called with anything else:
    ///
    /// - [`toString()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toString) accepts a base (radix) between 2 and 36.
    /// - [`toFixed()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toFixed) accepts between 0 and 100 digits after the decimal point.
    /// - [`toExponential()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toExponential) accepts between 0 and 100 digits after the decimal point.
    /// - [`toPrecision()`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Number/toPrecision) accepts between 1 and 100 significant digits.
    ///
    /// The fractional part of the argument is dropped before the range is checked,
    /// so `toString(36.5)` is the same as `toString(36)`.
    ///
    /// This rule only checks arguments written as number literals.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const binary = value.toString(1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const text = value.toString(64);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const price = value.toFixed(-1);
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const rounded = value.toPrecision(0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const hex = value.toString(16);
    /// const price = value.toFixed(2);
    /// const scientific = value.toExponential(100);
    /// const rounded = value.toPrecision(3);
    /// const text = value.toString(base);
    /// ```
    ///
    /// ## Caveats
    ///
    /// This rule assumes that these methods are called on a number.
    /// It doesn't check the type of the value the method is called on.
    ///
    pub NoNumberArgumentOutOfRange {
        version: "next",
        name: "noNumberArgumentOutOfRange",
        language: "js",
        sources: &[RuleSource::Oxc("number-arg-out-of-range").inspired()],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoNumberArgumentOutOfRange {
    type Query = Ast<JsCallExpression>;
    type State = NumberMethod;
    type Signals = Option<Self::State>;
    type Options = NoNumberArgumentOutOfRangeOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let callee = call.callee().ok()?.omit_parentheses();
        let member_name = AnyJsMemberExpression::cast(callee.into_syntax())?.member_name()?;
        let method = NumberMethod::from_name(member_name.text())?;

        let argument = call.arguments().ok()?.args().first()?.ok()?;
        let value = static_number_value(argument.as_any_js_expression()?)?;
        // The methods convert the argument with `ToIntegerOrInfinity`, which drops the fractional part.
        let value = value.trunc();
        if (f64::from(method.min())..=f64::from(method.max())).contains(&value) {
            return None;
        }

        Some(method)
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let argument = ctx.query().arguments().ok()?.args().first()?.ok()?;
        let min = method.min();
        let max = method.max();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                argument.range(),
                markup! {
                    "The argument of "<Emphasis>{method}</Emphasis>" is out of range."
                },
            )
            .note(markup! {
                <Emphasis>{method}</Emphasis>" throws a "<Emphasis>"RangeError"</Emphasis>" when its first argument is not between "{min}" and "{max}"."
            })
            .note(markup! {
                "Pass a number between "{min}" and "{max}"."
            }),
        )
    }
}

/// A number method whose first argument must be in a fixed range.
#[derive(Clone, Copy, Debug)]
pub enum NumberMethod {
    ToExponential,
    ToFixed,
    ToPrecision,
    ToString,
}

impl NumberMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "toExponential" => Some(Self::ToExponential),
            "toFixed" => Some(Self::ToFixed),
            "toPrecision" => Some(Self::ToPrecision),
            "toString" => Some(Self::ToString),
            _ => None,
        }
    }

    /// The smallest value the method accepts.
    const fn min(self) -> u8 {
        match self {
            Self::ToExponential | Self::ToFixed => 0,
            Self::ToPrecision => 1,
            Self::ToString => 2,
        }
    }

    /// The largest value the method accepts.
    const fn max(self) -> u8 {
        match self {
            Self::ToExponential | Self::ToFixed | Self::ToPrecision => 100,
            Self::ToString => 36,
        }
    }
}

impl biome_console::fmt::Display for NumberMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::ToExponential => "toExponential()",
            Self::ToFixed => "toFixed()",
            Self::ToPrecision => "toPrecision()",
            Self::ToString => "toString()",
        })
    }
}
