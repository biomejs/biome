use biome_analyze::{Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsExpression, JsBinaryExpression, JsBinaryOperator, JsCallExpression,
    JsComputedMemberExpression, JsRegexLiteralExpression, JsStaticMemberExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};

declare_lint_rule! {
    /// Enforce using `String.startsWith()` and `String.endsWith()` over more complex alternatives.
    ///
    /// There are multiple ways to verify if a string starts or ends with a certain string,
    /// such as `foo.indexOf('bar') === 0` or `foo.slice(0, 3) === 'bar'`.
    ///
    /// This rule is aimed to enforce a consistent and more readable style using `startsWith()` and `endsWith()` methods.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = "hello";
    /// foo[0] === 'h';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = "hello";
    /// foo.charAt(0) === 'h';
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = "hello";
    /// foo.indexOf('bar') === 0;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const foo = "hello";
    /// /^bar/.test(foo);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = "hello";
    /// foo.startsWith('h');
    /// ```
    ///
    /// ```js
    /// const foo = "hello";
    /// foo.endsWith('o');
    /// ```
    ///
    pub UseStartsEndsWith {
        version: "next",
        name: "useStartsEndsWith",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        sources: &[RuleSource::Eslint("prefer-string-starts-ends-with").same()],
    }
}

declare_node_union! {
    pub AnyStringCheckExpression = JsBinaryExpression | JsCallExpression
}

#[derive(Debug, Clone)]
pub enum StartsEndsWithVariant {
    StartsWith,
    EndsWith,
}

#[derive(Debug, Clone)]
pub struct UseStartsEndsWithState {
    pub variant: StartsEndsWithVariant,
    pub pattern: String,
}

impl Rule for UseStartsEndsWith {
    type Query = Ast<AnyStringCheckExpression>;
    type State = UseStartsEndsWithState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            AnyStringCheckExpression::JsBinaryExpression(binary) => check_binary_expression(binary),
            AnyStringCheckExpression::JsCallExpression(call) => check_call_expression(call),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let method_name = match state.variant {
            StartsEndsWithVariant::StartsWith => "startsWith",
            StartsEndsWithVariant::EndsWith => "endsWith",
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use `"{method_name}"()` method for string comparison."
                },
            )
            .note(markup! {
                "The current code uses a complex pattern to check if a string starts or ends with a specific substring."
            })
            .note(markup! {
                "Using `"{method_name}"('"{ &state.pattern }"')` is more readable and easier to maintain."
            }),
        )
    }
}

fn check_binary_expression(binary: &JsBinaryExpression) -> Option<UseStartsEndsWithState> {
    let operator = binary.operator().ok()?;

    if !matches!(
        operator,
        JsBinaryOperator::StrictEquality
            | JsBinaryOperator::Equality
            | JsBinaryOperator::StrictInequality
            | JsBinaryOperator::Inequality
    ) {
        return None;
    }

    let left = binary.left().ok()?;
    let right = binary.right().ok()?;

    check_binary_sides(&left, &right).or_else(|| check_binary_sides(&right, &left))
}

fn check_call_expression(call: &JsCallExpression) -> Option<UseStartsEndsWithState> {
    let callee = call.callee().ok()?;

    let AnyJsExpression::JsStaticMemberExpression(member_expr) = callee else {
        return None;
    };

    let object = member_expr.object().ok()?;
    let member = member_expr.member().ok()?;
    let method = member.value_token().ok()?;

    if method.text_trimmed() == "test"
        && let AnyJsExpression::AnyJsLiteralExpression(literal) = object
        && let Some(regex) = literal.as_js_regex_literal_expression()
    {
        return check_regex(regex);
    }

    None
}

fn check_binary_sides(
    expr: &AnyJsExpression,
    pattern: &AnyJsExpression,
) -> Option<UseStartsEndsWithState> {
    if let AnyJsExpression::JsComputedMemberExpression(computed) = expr {
        return check_computed_member(computed, pattern);
    }

    if let AnyJsExpression::JsCallExpression(call) = expr {
        return check_method_call(call, pattern);
    }

    None
}

fn check_computed_member(
    computed: &JsComputedMemberExpression,
    pattern: &AnyJsExpression,
) -> Option<UseStartsEndsWithState> {
    let member = computed.member().ok()?;
    let pattern_str = extract_string_literal(pattern)?;

    if is_number_literal(&member, 0.0) {
        return Some(UseStartsEndsWithState {
            variant: StartsEndsWithVariant::StartsWith,
            pattern: pattern_str,
        });
    }

    if let AnyJsExpression::JsBinaryExpression(binary) = member
        && binary.operator().ok()? == JsBinaryOperator::Minus
    {
        let left = binary.left().ok()?;
        let right = binary.right().ok()?;

        let computed_object = computed.object().ok()?;
        if is_same_object_length_access(&left, &computed_object) && is_number_literal(&right, 1.0) {
            return Some(UseStartsEndsWithState {
                variant: StartsEndsWithVariant::EndsWith,
                pattern: pattern_str,
            });
        }
    }

    None
}

fn check_method_call(
    call: &JsCallExpression,
    pattern_expr: &AnyJsExpression,
) -> Option<UseStartsEndsWithState> {
    let callee = call.callee().ok()?;

    if let AnyJsExpression::JsStaticMemberExpression(member_expr) = callee {
        let args = call.arguments().ok()?;
        return check_string_method(&member_expr, pattern_expr, &args);
    }

    None
}

fn check_string_method(
    member_expr: &JsStaticMemberExpression,
    pattern_expr: &AnyJsExpression,
    args: &biome_js_syntax::JsCallArguments,
) -> Option<UseStartsEndsWithState> {
    let member = member_expr.member().ok()?;
    let method = member.value_token().ok()?;
    let method_text = method.text_trimmed();

    match method_text {
        "charAt" => {
            let pattern = extract_string_literal(pattern_expr)?;
            let first_arg = args.args().first()?.ok()?;
            let arg_expr = first_arg.as_any_js_expression()?;

            if is_number_literal(arg_expr, 0.0) {
                return Some(UseStartsEndsWithState {
                    variant: StartsEndsWithVariant::StartsWith,
                    pattern,
                });
            }

            if let AnyJsExpression::JsBinaryExpression(binary) = arg_expr
                && binary.operator().ok()? == JsBinaryOperator::Minus
            {
                let left = binary.left().ok()?;
                let right = binary.right().ok()?;
                if is_length_access(&left) && is_number_literal(&right, 1.0) {
                    return Some(UseStartsEndsWithState {
                        variant: StartsEndsWithVariant::EndsWith,
                        pattern,
                    });
                }
            }

            None
        }
        "indexOf" => {
            if is_number_literal(pattern_expr, 0.0) {
                let pattern_arg = args.args().first()?.ok()?;
                let pattern = extract_string_literal(pattern_arg.as_any_js_expression()?)?;
                return Some(UseStartsEndsWithState {
                    variant: StartsEndsWithVariant::StartsWith,
                    pattern,
                });
            }
            None
        }
        "lastIndexOf" => {
            let pattern_arg = args.args().first()?.ok()?;
            let pattern = extract_string_literal(pattern_arg.as_any_js_expression()?)?;

            if let AnyJsExpression::JsBinaryExpression(binary) = pattern_expr
                && binary.operator().ok()? == JsBinaryOperator::Minus
            {
                let left = binary.left().ok()?;
                if is_length_access(&left) {
                    // Validate that the right operand is a numeric literal equal to pattern length
                    let right = binary.right().ok()?;
                    let pattern_length = pattern.chars().count();
                    
                    if let AnyJsExpression::AnyJsLiteralExpression(literal) = right
                        && let Some(number) = literal.as_js_number_literal_expression()
                        && let Ok(value) = number.value_token()
                        && let Ok(parsed) = value.text_trimmed().parse::<usize>()
                        && parsed == pattern_length
                    {
                        return Some(UseStartsEndsWithState {
                            variant: StartsEndsWithVariant::EndsWith,
                            pattern,
                        });
                    }
                }
            }

            None
        }
        "slice" => {
            let pattern = extract_string_literal(pattern_expr)?;
            let first_arg = args.args().first()?.ok()?;

            if is_number_literal(first_arg.as_any_js_expression()?, 0.0) {
                return Some(UseStartsEndsWithState {
                    variant: StartsEndsWithVariant::StartsWith,
                    pattern,
                });
            }

            if let AnyJsExpression::JsUnaryExpression(unary) = first_arg.as_any_js_expression()? {
                use biome_js_syntax::JsUnaryOperator;
                if unary.operator().ok()? == JsUnaryOperator::Minus {
                    return Some(UseStartsEndsWithState {
                        variant: StartsEndsWithVariant::EndsWith,
                        pattern,
                    });
                }
            }

            None
        }
        "substring" => {
            let pattern = extract_string_literal(pattern_expr)?;
            let first_arg = args.args().first()?.ok()?;
            let arg_expr = first_arg.as_any_js_expression()?;

            if is_number_literal(arg_expr, 0.0) {
                return Some(UseStartsEndsWithState {
                    variant: StartsEndsWithVariant::StartsWith,
                    pattern,
                });
            }

            if let AnyJsExpression::JsBinaryExpression(binary) = arg_expr
                && binary.operator().ok()? == JsBinaryOperator::Minus
            {
                let left = binary.left().ok()?;
                if is_length_access(&left) {
                    return Some(UseStartsEndsWithState {
                        variant: StartsEndsWithVariant::EndsWith,
                        pattern,
                    });
                }
            }

            None
        }
        "match" => {
            let pattern_arg = args.args().first()?.ok()?;
            if let AnyJsExpression::AnyJsLiteralExpression(literal) =
                pattern_arg.as_any_js_expression()?
                && let Some(regex) = literal.as_js_regex_literal_expression()
            {
                return check_regex(regex);
            }
            None
        }
        _ => None,
    }
}

fn check_regex(regex: &JsRegexLiteralExpression) -> Option<UseStartsEndsWithState> {
    let token = regex.value_token().ok()?;
    let text = token.text_trimmed();

    let first_slash = text.find('/')?;
    let last_slash = text.rfind('/')?;

    if first_slash >= last_slash {
        return None;
    }

    let pattern = &text[first_slash + 1..last_slash];

    let has_start = pattern.starts_with('^') && !pattern.starts_with(r"\^");
    let has_end = pattern.ends_with('$') && !pattern.ends_with(r"\$");

    if has_start {
        let clean_pattern = pattern.trim_start_matches('^');
        return Some(UseStartsEndsWithState {
            variant: StartsEndsWithVariant::StartsWith,
            pattern: clean_pattern.to_string(),
        });
    }

    if has_end {
        let clean_pattern = pattern.trim_end_matches('$');
        return Some(UseStartsEndsWithState {
            variant: StartsEndsWithVariant::EndsWith,
            pattern: clean_pattern.to_string(),
        });
    }

    None
}

fn is_number_literal(expr: &AnyJsExpression, expected: f64) -> bool {
    if let AnyJsExpression::AnyJsLiteralExpression(literal) = expr
        && let Some(number) = literal.as_js_number_literal_expression()
        && let Ok(value) = number.value_token()
        && let Ok(parsed) = value.text_trimmed().parse::<f64>()
    {
        return (parsed - expected).abs() < f64::EPSILON;
    }
    false
}

fn extract_string_literal(expr: &AnyJsExpression) -> Option<String> {
    if let AnyJsExpression::AnyJsLiteralExpression(literal) = expr
        && let Some(string) = literal.as_js_string_literal_expression()
    {
        return Some(string.inner_string_text().ok()?.to_string());
    }
    None
}

fn is_length_access(expr: &AnyJsExpression) -> bool {
    if let AnyJsExpression::JsStaticMemberExpression(member) = expr
        && let Ok(prop) = member.member()
        && let Ok(token) = prop.value_token()
    {
        return token.text_trimmed() == "length";
    }
    false
}

fn is_same_object_length_access(expr: &AnyJsExpression, target_object: &AnyJsExpression) -> bool {
    if let AnyJsExpression::JsStaticMemberExpression(member) = expr
        && let Ok(prop) = member.member()
        && let Ok(token) = prop.value_token()
        && token.text_trimmed() == "length"
        && let Ok(length_object) = member.object()
    {
        // Compare the two objects by their syntax text representation
        return length_object.syntax().text_trimmed() == target_object.syntax().text_trimmed();
    }
    false
}
