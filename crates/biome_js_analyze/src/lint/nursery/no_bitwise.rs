use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_deserialize_macros::Deserializable;
use biome_js_syntax::{
    JsAssignmentExpression, JsBinaryExpression, JsSyntaxToken, JsUnaryExpression, T,
};
use biome_rowan::{AstNode, declare_node_union};
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

declare_lint_rule! {
    /// Disallow bitwise operators
    ///
    /// The use of bitwise operators in JavaScript is very rare and often & or | is simply a mistyped && or ||,
    /// which will lead to unexpected behavior.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// let x = y | z;
    /// const x1 = y & z;
    /// const x2 = y ^ z;
    /// const x3 = ~ z;
    /// const x4 = y << z;
    /// const x5 = y >> z;
    /// const x6 = y >>> z;
    /// x |= y;
    /// x &= y;
    /// x ^= y;
    /// x <<= y;
    /// x >>= y;
    /// x >>>= y;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// let x = y || z;
    /// const x1 = y && z;
    /// const x2 = y > z;
    /// const x3 = y < z;
    /// x += y;
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides the option described below.
    ///
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "allow": ["&", "|", "^", "~", "<<", ">>", ">>>"],
    ///         "int32Hint": false,
    ///     }
    /// }
    /// ```
    /// ### allow
    ///
    /// Allows a list of bitwise operators to be used as exceptions.
    ///
    /// Examples of valid code for this rule with the `{ "allow": ["~"] }` option:
    ///
    /// ```js
    /// ~[1,2,3].indexOf(1) === -1;
    /// ```
    /// ### int32Hint
    ///
    /// Allows the use of bitwise OR in |0 pattern for type casting.
    ///
    /// Default: `false`
    ///
    /// Examples of valid code for this rule with the `{ "int32Hint": true }` option:
    ///
    /// ```js
    /// const b = a|0;
    /// ```
    ///
    pub NoBitwise {
        version: "next",
        name: "noBitwise",
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
            AnyExpressionWithBitwise::JsBinaryExpression(_) => {
                self.operator_token().is_some_and(|op| {
                    matches!(op.kind(), T![&] | T![|] | T![^] | T![<<] | T![>>] | T![>>>])
                })
            }
            AnyExpressionWithBitwise::JsUnaryExpression(_) => self
                .operator_token()
                .is_some_and(|op| matches!(op.kind(), T![~])),
            AnyExpressionWithBitwise::JsAssignmentExpression(_) => {
                self.operator_token().is_some_and(|op| {
                    matches!(
                        op.kind(),
                        T![&=] | T![|=] | T![^=] | T![<<=] | T![>>=] | T![>>>=]
                    )
                })
            }
        }
    }
}

impl Rule for NoBitwise {
    type Query = Ast<AnyExpressionWithBitwise>;
    type State = JsSyntaxToken;
    type Signals = Option<Self::State>;
    type Options = NoBitwiseOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();
        if expr.exist_bitwise_op() {
            let op = expr.operator_token().unwrap();
            if ctx
                .options()
                .allow
                .iter()
                .any(|op_str| op_str.as_ref() == op.text_trimmed())
            {
                return None;
            }
            // check if the operator is '|' and right is '0'
            if ctx.options().int32_hint && op.kind() == T![|] {
                if let AnyExpressionWithBitwise::JsBinaryExpression(node) = expr {
                    let right = node.right().ok()?.omit_parentheses();
                    if op.kind() == T![|]
                        && right
                            .as_any_js_literal_expression()
                            .is_some_and(|literal_expr| {
                                literal_expr.as_js_number_literal_expression().is_some_and(
                                    |number_literial| {
                                        number_literial
                                            .value_token()
                                            .ok()
                                            .is_some_and(|value| value.text_trimmed() == "0")
                                    },
                                )
                            })
                    {
                        return None;
                    }
                }
            }
            return Some(op);
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let op = state.text();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected use of '"<Emphasis>{op}</Emphasis>"'."
                },
            )
            .note(markup! {
                "The use of bitwise operators in JavaScript is very rare and often & or | is simply a mistyped && or ||."
            }),
        )
    }
}

/// Options for the `noBitwise` rule.
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoBitwiseOptions {
    // Allows a list of bitwise operators to be used as exceptions.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub allow: Box<[Box<str>]>,
    // Allows the use of bitwise OR in |0 pattern for type casting.
    #[serde(default)]
    pub int32_hint: bool,
}
