use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    JsAssignmentExpression, JsAssignmentOperator, JsBinaryExpression, JsBinaryOperator,
    JsSyntaxToken, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, declare_node_union};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Disallow bitwise operators.
    ///
    /// The use of bitwise operators in JavaScript is very rare and often `&` or `|` is simply a mistyped `&&` or `||`,
    /// which will lead to unexpected behavior.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let x = y | z;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// x |= y;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let x = y || z;
    /// ```
    ///
    /// ```js
    /// let x = y && z;
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides the options described below.
    ///
    /// ### allow
    ///
    /// Allows a list of bitwise operators to be used as exceptions.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allow": ["&", "|", "^", "~", "<<", ">>", ">>>"]
    ///     }
    /// }
    /// ```
    ///
    pub NoBitwiseOperators {
        version: "next",
        name: "noBitwiseOperators",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-bitwise"),
        ],
        recommended: false,
    }
}

declare_node_union! {
    pub AnyExpressionWithBitwise = JsBinaryExpression | JsUnaryExpression | JsAssignmentExpression
}

impl AnyExpressionWithBitwise {
    fn operator_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyExpressionWithBitwise::JsBinaryExpression(node) => node.operator_token().ok(),
            AnyExpressionWithBitwise::JsUnaryExpression(node) => node.operator_token().ok(),
            AnyExpressionWithBitwise::JsAssignmentExpression(node) => node.operator_token().ok(),
        }
    }
    fn exist_bitwise_op(&self) -> bool {
        match self {
            AnyExpressionWithBitwise::JsBinaryExpression(binary_expr) => {
                binary_expr.operator().ok().is_some_and(|op| {
                    matches!(
                        op,
                        JsBinaryOperator::BitwiseAnd
                            | JsBinaryOperator::BitwiseOr
                            | JsBinaryOperator::BitwiseXor
                            | JsBinaryOperator::LeftShift
                            | JsBinaryOperator::RightShift
                            | JsBinaryOperator::UnsignedRightShift
                    )
                })
            }
            AnyExpressionWithBitwise::JsUnaryExpression(unary_expr) => unary_expr
                .operator()
                .ok()
                .is_some_and(|op| op == JsUnaryOperator::BitwiseNot),
            AnyExpressionWithBitwise::JsAssignmentExpression(assign_expr) => {
                assign_expr.operator().ok().is_some_and(|op| {
                    matches!(
                        op,
                        JsAssignmentOperator::BitwiseAndAssign
                            | JsAssignmentOperator::BitwiseOrAssign
                            | JsAssignmentOperator::BitwiseXorAssign
                            | JsAssignmentOperator::LeftShiftAssign
                            | JsAssignmentOperator::RightShiftAssign
                            | JsAssignmentOperator::UnsignedRightShiftAssign
                    )
                })
            }
        }
    }
}

impl Rule for NoBitwiseOperators {
    type Query = Ast<AnyExpressionWithBitwise>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoBitwiseOperatorsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();
        if expr.exist_bitwise_op() {
            let op = expr.operator_token()?;
            if ctx
                .options()
                .allow
                .iter()
                .any(|op_str| op_str.as_ref() == op.text_trimmed())
            {
                return None;
            }
            return Some(op);
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let op = state.text_trimmed();
        let suggested_op = match op {
            "&" => Some("&&"),
            "|" => Some("||"),
            "^" => Some("**"),
            _ => None,
        };
        let note_msg = match suggested_op {
            Some(value) => format!("Did you mean {value} instead? If you want to use the bitwise operator, consider suppressing this diagnostic."),
            None => "Bitwise operators are prohibited because their use can be confusing or unintended. If you did want to use the bitwise operator, consider suppressing this diagnostic.".to_string(),
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of '"<Emphasis>{op}</Emphasis>"'."
                },
            )
            .note(note_msg),
        )
    }
}

/// Rule's options
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoBitwiseOperatorsOptions {
    /// Allows a list of bitwise operators to be used as exceptions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub allow: Box<[Box<str>]>,
}
