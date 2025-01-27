use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsCallExpression, JsSyntaxKind, JsUnaryOperator,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, SyntaxNode, TextRange};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const MAX_ARRAY_LENGTH: i64 = (2_i64.pow(32)) - 1;

#[derive(
    Debug, Default, Clone, Serialize, Deserialize, Deserializable, PartialEq, Eq, JsonSchema,
)]
pub struct NoMagicNumbersConfig {
    detect_objects: bool,
    enforce_const: bool,
    ignore: Vec<i64>,
    ignore_array_indexes: bool,
    ignore_default_values: bool,
    ignore_class_field_initial_values: bool,
}

declare_lint_rule! {
    /// Disallow magic numbers
    ///
    /// This rule aims to make code more maintainable by ensuring that special numbers
    /// are declared as constants with meaningful names.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let total = price * 1.23; // Magic number for tax rate
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const TAX_RATE = 1.23;
    /// let total = price * TAX_RATE;
    /// ```
    pub NoMagicNumbers {
        version: "next",
        name: "noMagicNumbers",
        language: "js",
        sources: &[RuleSource::Eslint("no-magic-numbers")],
        recommended: false,
        fix_kind: FixKind::None,
    }
}

#[derive(Debug)]
pub struct NumberContext {
    range: TextRange,
    raw: String,
    is_const_violation: bool,
}

impl Rule for NoMagicNumbers {
    type Query = Ast<AnyJsExpression>;
    type State = NumberContext;
    type Signals = Option<Self::State>;
    type Options = NoMagicNumbersConfig;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let config = ctx.options();

        match node {
            AnyJsExpression::AnyJsLiteralExpression(literal_expr) => {
                if let Some(literal) = literal_expr.as_js_number_literal_expression() {
                    let value_token = literal.value_token().ok()?;
                    let value_text = value_token.text();
                    let value = value_text.parse::<i64>().ok()?;
                    if is_allowed_number(literal.syntax(), value, config) {
                        return None;
                    }

                    let is_const_violation = check_const_violation(literal.syntax(), config);

                    Some(NumberContext {
                        range: literal.range(),
                        raw: value_token.text().to_string(),
                        is_const_violation,
                    })
                } else {
                    return None;
                }
            }
            AnyJsExpression::JsUnaryExpression(unary) => {
                if let Ok(operator) = unary.operator() {
                    if operator == JsUnaryOperator::Minus {
                        if let Ok(argument) = unary.argument() {
                            if let Some(literal_expr) = argument.as_any_js_literal_expression() {
                                if let Ok(value_token) = literal_expr.value_token() {
                                    let value_text = value_token.text();
                                    let value = value_text.parse::<i64>().ok()?;
                                    let neg_value = -value;
                                    if is_allowed_number(unary.syntax(), neg_value, config) {
                                        return None;
                                    }
                                    let is_const_violation =
                                        check_const_violation(unary.syntax(), config);
                                    return Some(NumberContext {
                                        range: unary.range(),
                                        raw: format!("-{}", value_token.text()),
                                        is_const_violation,
                                    });
                                }
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = if state.is_const_violation {
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! { "Number constants declarations must use 'const'" },
            )
        } else {
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! { "No magic number: "{state.raw} },
            )
            .note(markup! {
                "Consider extracting this magic number into a named constant."
            })
        };
        Some(diagnostic)
    }
}

fn is_allowed_number(
    node: &SyntaxNode<biome_js_syntax::JsLanguage>,
    value: i64,
    config: &NoMagicNumbersConfig,
) -> bool {
    // Ignore specific values from config
    if config.ignore.contains(&value) {
        return true;
    }

    // Check parent nodes for various allowed contexts
    if let Some(parent) = node.parent() {
        match parent.kind() {
            // Array index check
            // k if k == JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            //     || k == JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION =>
            // {
            //     if config.ignore_array_indexes
            //         && JsMemberExpression::cast(parent)
            //             .map_or(false, |m| m.object().is_ok() && is_valid_array_index(value))
            //     {
            //         return true;
            //     }
            // }
            // parseInt radix check
            k if k == JsSyntaxKind::JS_CALL_EXPRESSION => {
                if let Some(call) = JsCallExpression::cast(parent) {
                    if let Ok(callee) = call.callee() {
                        if callee.syntax().text() == "parseInt" {
                            return true;
                        }
                    }
                }
            }
            // Default value check
            k if k == JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN
                || k == JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN =>
            {
                if config.ignore_default_values {
                    return true;
                }
            }
            // Class field check
            k if k == JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                if config.ignore_class_field_initial_values {
                    return true;
                }
            }
            _ => {}
        }
    }

    if !config.detect_objects {
        if let Some(parent) = node.parent() {
            match parent.kind() {
                JsSyntaxKind::JS_OBJECT_EXPRESSION | JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                    return true;
                }
                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    if let Some(expr) = JsAssignmentExpression::cast(parent) {
                        if let Ok(left_pattern) = expr.left() {
                            if left_pattern.as_js_object_assignment_pattern().is_none() {
                                return true;
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    false
}

fn check_const_violation(
    node: &SyntaxNode<biome_js_syntax::JsLanguage>,
    config: &NoMagicNumbersConfig,
) -> bool {
    if !config.enforce_const {
        return false;
    }

    node.ancestors()
        .find_map(JsVariableDeclarator::cast)
        .and_then(|decl| decl.syntax().parent())
        .and_then(JsVariableDeclarator::cast)
        .and_then(|parent| parent.id().ok())
        .map_or(false, |kind| kind.to_string() != "const")
}

fn is_valid_array_index(value: f64) -> bool {
    value.floor() == value && value >= 0.0 && value < MAX_ARRAY_LENGTH as f64
}
