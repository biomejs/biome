use std::cmp::Ordering;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    numbers::split_into_radix_and_number, AnyJsExpression, AnyJsLiteralExpression,
    JsNumberLiteralExpression, T,
};
use biome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_lint_rule! {
    /// Use standard constants instead of approximated literals.
    ///
    /// Usually, the definition in the standard library is more precise than
    /// what people come up with or the used constant exceeds the maximum precision of the number type.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let x = 3.141;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let x = 2.302;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let x = Math.PI;
    /// let y = 3.14;
    /// ```
    ///
    /// ```js
    /// let x = Math.LN10;
    /// ```
    pub NoApproximativeNumericConstant {
        version: "1.3.0",
        name: "noApproximativeNumericConstant",
        language: "js",
        sources: &[RuleSource::Clippy("approx_constant")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for NoApproximativeNumericConstant {
    type Query = Ast<JsNumberLiteralExpression>;
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let token = ctx.query().value_token().ok()?;
        let num = token.text_trimmed();
        let (10, num) = split_into_radix_and_number(num) else {
            return None;
        };
        let (decimal, fraction) = num.split_once('.')?;
        if fraction.len() < (MIN_FRACTION_DIGITS as usize)
            || !matches!(decimal, "" | "0" | "1" | "2" | "3")
            || fraction.contains(['e', 'E'])
        {
            return None;
        }
        let num = num.trim_matches('0');
        for (constant, name) in KNOWN_CONSTS {
            let is_constant_approximated = match constant.len().cmp(&num.len()) {
                Ordering::Less => is_approx_const(num, constant),
                Ordering::Equal => constant == num,
                Ordering::Greater => is_approx_const(constant, num),
            };
            if is_constant_approximated {
                return Some(name);
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! { "Prefer constants from the standard library." },
        ))
    }

    fn action(ctx: &RuleContext<Self>, constant_name: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let new_node = make::js_static_member_expression(
            make::js_identifier_expression(make::js_reference_identifier(make::ident("Math")))
                .into(),
            make::token(T![.]),
            make::js_name(make::ident(constant_name)).into(),
        );
        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(node.clone())),
            AnyJsExpression::from(new_node),
        );
        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use "<Emphasis>"Math."{ constant_name }</Emphasis>" instead." }.to_owned(),
            mutation,
        ))
    }
}

const MIN_FRACTION_DIGITS: u8 = 3;

// Tuples are of the form (constant, name)
const KNOWN_CONSTS: [(&str, &str); 8] = [
    ("2.718281828459045", "E"),
    ("2.302585092994046", "LN10"),
    (".6931471805599453", "LN2"),
    (".4342944819032518", "LOG10E"),
    ("1.4426950408889634", "LOG2E"),
    ("3.141592653589793", "PI"),
    (".7071067811865476", "SQRT1_2"),
    ("1.4142135623730951", "SQRT2"),
];

/// Returns true if `value` is equal to `constant`,
/// or rounded to the number of digits present in `value`.
fn is_approx_const(constant: &str, value: &str) -> bool {
    if constant.starts_with(value) {
        // The value is a truncated constant
        return true;
    }
    let (digits, last_digit) = value.split_at(value.len() - 1);
    if constant.starts_with(digits) {
        let Ok(last_digit) = last_digit.parse::<u8>() else {
            return false;
        };
        let Ok(extra_constant_digit) = constant[value.len()..value.len() + 1].parse::<u8>() else {
            return false;
        };
        let can_be_rounded = extra_constant_digit < 5;
        if can_be_rounded {
            return false;
        }
        let Ok(constant_digit) = constant[digits.len()..digits.len() + 1].parse::<u8>() else {
            return false;
        };
        let rounded_constant_digit = constant_digit + 1;
        return last_digit == rounded_constant_digit;
    }
    false
}
