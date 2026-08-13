use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsLogicalExpression, JsStaticMemberExpression,
    JsSyntaxKind::*,
};
use biome_rowan::AstNode;
use biome_rule_options::no_useless_length_check::NoUselessLengthCheckOptions;

declare_lint_rule! {
    /// Disallow useless array length checks
    ///
    /// `Array#some()` returns `false` for an empty array, and `Array#every()`
    /// returns `true`. There is no need to check whether the array is empty
    /// (or not) right next to those calls: the resulting value already covers
    /// the empty case.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (array.length === 0 || array.every(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length !== 0 && array.some(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.length > 0 && array.some(Boolean));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (array.every(Boolean) || array.length === 0);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (array.every(Boolean));
    /// ```
    ///
    /// ```js
    /// if (array.length > 0 && array.every(Boolean)) {}
    /// ```
    ///
    /// ```js
    /// if (array.length === 0 || array.some(Boolean));
    /// ```
    ///
    /// ```js
    /// if (foo().length === 0 || foo().every(Boolean));
    /// ```
    pub NoUselessLengthCheck {
        version: "next",
        name: "noUselessLengthCheck",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("no-useless-length-check").same()],
        recommended: false,
        severity: Severity::Warning,
    }
}

impl Rule for NoUselessLengthCheck {
    type Query = Ast<JsLogicalExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUselessLengthCheckOptions;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let logical = ctx.query();
        let is_or = logical.operator_token().ok()?.kind() == PIPE2;
        let is_and = logical.operator_token().ok()?.kind() == AMP2;
        if !is_or && !is_and {
            return None;
        }

        let left = logical.left().ok()?.omit_parentheses();
        let right = logical.right().ok()?.omit_parentheses();

        // Left side must be a usable length check.
        let (left_base, length_kind) = extract_length_check(left.clone())?;

        // Right side must call `.some()` / `.every()` on the same identifier.
        let (right_base, method) = extract_some_every(right.clone())?;

        // `.every()` is true on an empty array, `.some()` on an empty array.
        // So the length check is redundant only for the matching comparison.
        //   `length === 0 || array.every(...)`  -> every is true on empty
        //   `length !== 0 && array.some(...)`   -> some is false on empty
        //   `length > 0 && array.some(...)`     -> some is false on empty
        let matches = match (is_or, length_kind) {
            (true, LengthKind::EqualZero) => method == "every",
            (false, LengthKind::NotEqualZero | LengthKind::GreaterZero) => method == "some",
            _ => false,
        };
        if !matches {
            return None;
        }

        // The length check and the method call must target the same identifier.
        // Impure expressions (calls, computed members) are rejected: two
        // syntactically identical calls may still return different arrays.
        if !same_identifier(&left_base, &right_base) {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "This length check is useless"
                },
            )
            .note(markup! {
                "The "<Emphasis>".every()"</Emphasis>" method already returns "<Emphasis>"true"</Emphasis>" for an empty array, and "<Emphasis>".some()"</Emphasis>" already returns "<Emphasis>"false"</Emphasis>", so this check has no effect on the result."
            }),
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
enum LengthKind {
    EqualZero,
    NotEqualZero,
    GreaterZero,
}

/// Extracts the base expression and kind of a length comparison such as
/// `foo.length === 0`, `foo.length !== 0`, `foo.length > 0`, or `!foo.length`.
fn extract_length_check(expr: AnyJsExpression) -> Option<(AnyJsExpression, LengthKind)> {
    let inner = expr.omit_parentheses();

    if let AnyJsExpression::JsBinaryExpression(binary) = &inner {
        return extract_length_check_binary(binary);
    }

    // `!foo.length` reads as "length === 0".
    if let AnyJsExpression::JsUnaryExpression(unary) = &inner {
        if unary.operator_token().ok()?.kind() == BANG {
            let arg = unary.argument().ok()?.omit_parentheses();
            if let AnyJsExpression::JsStaticMemberExpression(static_member) = &arg {
                if is_length_member(static_member) {
                    let base = static_member.object().ok()?;
                    return Some((base, LengthKind::EqualZero));
                }
            }
        }
    }

    None
}

fn extract_length_check_binary(
    binary: &JsBinaryExpression,
) -> Option<(AnyJsExpression, LengthKind)> {
    let kind = match binary.operator_token().ok()?.kind() {
        EQ2 | EQ3 => LengthKind::EqualZero,
        NEQ | NEQ2 => LengthKind::NotEqualZero,
        R_ANGLE => LengthKind::GreaterZero,
        _ => return None,
    };
    let left = binary.left().ok()?.omit_parentheses();
    let right = binary.right().ok()?.omit_parentheses();

    if let AnyJsExpression::JsStaticMemberExpression(static_member) = &left {
        if is_length_member(static_member) && is_zero_literal(&right) {
            let base = static_member.object().ok()?;
            return Some((base, kind));
        }
    }
    None
}

fn is_length_member(member: &JsStaticMemberExpression) -> bool {
    member
        .member()
        .ok()
        .is_some_and(|name| name.syntax().text_trimmed() == "length")
}

fn is_zero_literal(expr: &AnyJsExpression) -> bool {
    match expr {
        AnyJsExpression::AnyJsLiteralExpression(lit) => {
            let text = lit.syntax().text_trimmed();
            text == "0" || text == "0.0" || text == "-0"
        }
        _ => false,
    }
}

/// Extracts the base expression and method name of a `.some()` / `.every()`
/// call.
fn extract_some_every(expr: AnyJsExpression) -> Option<(AnyJsExpression, &'static str)> {
    let inner = expr.omit_parentheses();
    let AnyJsExpression::JsCallExpression(call) = &inner else {
        return None;
    };
    let callee = call.callee().ok()?.omit_parentheses();
    let AnyJsExpression::JsStaticMemberExpression(member) = &callee else {
        return None;
    };
    // Compare the method token text directly, without allocating a String.
    let method = member.member().ok()?.syntax().text_trimmed();
    let method = if method == "some" {
        "some"
    } else if method == "every" {
        "every"
    } else {
        return None;
    };
    Some((member.object().ok()?, method))
}

/// Returns `true` only when both base expressions are the same identifier.
/// Anything else (calls, computed member access, literals) returns `false`,
/// because it cannot be proven to reference the same object.
fn same_identifier(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    let a_inner = a.clone().omit_parentheses();
    let b_inner = b.clone().omit_parentheses();
    let AnyJsExpression::JsIdentifierExpression(ai) = &a_inner else {
        return false;
    };
    let AnyJsExpression::JsIdentifierExpression(bi) = &b_inner else {
        return false;
    };
    match (ai.name().ok(), bi.name().ok()) {
        (Some(na), Some(nb)) => na.syntax().text_trimmed() == nb.syntax().text_trimmed(),
        _ => false,
    }
}