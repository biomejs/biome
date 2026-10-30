use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsExpression, JsBinaryExpression, JsSyntaxKind, T};
use biome_parser::{TokenSet, token_set};
use biome_rowan::{AstNode, AstSeparatedList};
use biome_rule_options::no_object_literal_comparison::NoObjectLiteralComparisonOptions;

declare_lint_rule! {
    /// Disallow comparing a value with an empty object `{}` or an empty array `[]`.
    ///
    /// Code like `value === {}` or `list === []` looks like it checks whether a value is empty, but it doesn't.
    /// Each `{}` and `[]` creates a new object, and JavaScript compares objects and arrays by reference,
    /// not by their contents. With `===`, the comparison is always `false`, and with `!==`, it's always `true`.
    /// With `==` and `!=`, JavaScript first converts `{}` to the string `"[object Object]"` and `[]` to the
    /// empty string `""` when the other value isn't an object, so `"" == []` and `0 == []` are both `true`,
    /// even though neither value is an empty array.
    ///
    /// To check whether an object has no properties, use `Object.keys(value).length === 0`.
    /// To check whether an array is empty, use `value.length === 0`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (value === {}) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (list !== []) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// if (value == {}) {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// if (value !== null && typeof value === "object" && Object.keys(value).length === 0) {}
    /// if (Array.isArray(list) && list.length === 0) {}
    /// if (value === null) {}
    /// ```
    ///
    /// ## See Also
    ///
    /// - To report more comparisons that always give the same result, see [`noConstantBinaryExpressions`](https://biomejs.dev/linter/rules/no-constant-binary-expressions/).
    ///
    pub NoObjectLiteralComparison {
        version: "next",
        name: "noObjectLiteralComparison",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        sources: &[RuleSource::Oxc("bad-object-literal-comparison").same()],
    }
}

impl Rule for NoObjectLiteralComparison {
    type Query = Ast<JsBinaryExpression>;
    /// The kind of empty literal found on one side of the comparison.
    type State = EmptyLiteralKind;
    type Signals = Option<Self::State>;
    type Options = NoObjectLiteralComparisonOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !EQUALITY_OPERATORS.contains(node.operator_token().ok()?.kind()) {
            return None;
        }
        EmptyLiteralKind::from_expression(&node.left().ok()?)
            .or_else(|| EmptyLiteralKind::from_expression(&node.right().ok()?))
    }

    fn diagnostic(ctx: &RuleContext<Self>, kind: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This compares a value with an empty "{kind}" literal."
                },
            )
            .note(markup! {
                "Each "{kind}" literal creates a new "{kind}". Comparing a value with it never checks whether the value is empty: objects and arrays are compared by reference, not by their contents."
            })
            .note(match kind {
                EmptyLiteralKind::Object => markup! {
                    "To check whether an object has no properties, use "<Emphasis>"Object.keys(value).length === 0"</Emphasis>"."
                },
                EmptyLiteralKind::Array => markup! {
                    "To check whether an array is empty, use "<Emphasis>"value.length === 0"</Emphasis>"."
                },
            }),
        )
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EmptyLiteralKind {
    /// An empty object literal: `{}`.
    Object,
    /// An empty array literal: `[]`.
    Array,
}

impl EmptyLiteralKind {
    /// Returns the kind of `expression` if it evaluates to an empty object or array literal,
    /// looking through parentheses, TypeScript type wrappers such as `[] as string[]`, and
    /// sequence expressions, whose value is their last expression, such as `(a, {})`.
    fn from_expression(expression: &AnyJsExpression) -> Option<Self> {
        match expression.inner_expression()? {
            AnyJsExpression::JsObjectExpression(object) if object.members().is_empty() => {
                Some(Self::Object)
            }
            AnyJsExpression::JsArrayExpression(array) if array.elements().is_empty() => {
                Some(Self::Array)
            }
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for EmptyLiteralKind {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::Object => "object",
            Self::Array => "array",
        })
    }
}

const EQUALITY_OPERATORS: TokenSet<JsSyntaxKind> =
    token_set![T![==], T![!=], T![===], T![!==]];
