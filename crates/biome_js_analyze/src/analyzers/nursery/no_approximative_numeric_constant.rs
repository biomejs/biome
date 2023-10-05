use std::f64::consts as f64;

use biome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::JsNumberLiteralExpression;
use biome_rowan::AstNode;

declare_rule! {
    /// Usually, the definition in the standard library is more precise than what people come up with or the used constant exceeds the maximum precision of the number type.
    ///
    /// Source: https://rust-lang.github.io/rust-clippy/master/#approx_constant
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let x = 3.141;
    /// ```
    /// ```js,expect_diagnostic
    /// let x = 2.302;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// let x = Math.PI;
    /// ```
    /// ```js
    /// let x = Math.LN10;
    /// ```
    ///
    pub(crate) NoApproximativeNumericConstant {
        version: "next",
        name: "noApproximativeNumericConstant",
        recommended: false,
    }
}

impl Rule for NoApproximativeNumericConstant {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if get_approximative_literal_diagnostic(node).is_some() {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        get_approximative_literal_diagnostic(node)
    }
}

// Tuples are of the form (constant, name, min_digits)
const KNOWN_CONSTS: [(f64, &str, usize); 8] = [
    (f64::E, "E", 4),
    (f64::LN_10, "LN10", 4),
    (f64::LN_2, "LN2", 4),
    (f64::LOG10_E, "LOG10E", 4),
    (f64::LOG2_E, "LOG2E", 4),
    (f64::PI, "PI", 4),
    (f64::FRAC_1_SQRT_2, "SQRT1_2", 4),
    (f64::SQRT_2, "SQRT2", 4),
];

fn get_approximative_literal_diagnostic(
    node: &JsNumberLiteralExpression,
) -> Option<RuleDiagnostic> {
    let binding = node.text();
    let s = binding.trim();
    if s.parse::<f64>().is_err() {
        return None;
    }

    for &(constant, name, min_digits) in &KNOWN_CONSTS {
        if is_approx_const(constant, s, min_digits) {
            return Some(
                RuleDiagnostic::new(
                    rule_category!(),
                    node.syntax().text_trimmed_range(),
                    markup! { "Prefer constants from the standard library." },
                )
                .note(markup! { "Use "<Emphasis>"Math."{ name }</Emphasis>" instead." }),
            );
        }
    }

    None
}

/// Returns `false` if the number of significant figures in `value` are
/// less than `min_digits`; otherwise, returns true if `value` is equal
/// to `constant`, rounded to the number of digits present in `value`.
fn is_approx_const(constant: f64, value: &str, min_digits: usize) -> bool {
    if value.len() <= min_digits {
        false
    } else if constant.to_string().starts_with(value) {
        // The value is a truncated constant
        true
    } else {
        let round_const = format!("{constant:.*}", value.len() - 2);
        value == round_const
    }
}
