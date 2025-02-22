use std::str::FromStr;

use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, FixKind, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, JsBinaryExpression, JsBinaryOperator,
    JsLogicalOperator, JsUnaryOperator,
};
use biome_rowan::{AstNode, BatchMutationExt};
use biome_string_case::StrLikeExtension;

use crate::JsRuleAction;

declare_lint_rule! {
    /// This rule checks that the result of a `typeof` expression is compared to a valid value.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// typeof foo === "strnig";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo == "undefimed";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof bar != "nunber";
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo === undefined;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// typeof foo == 0;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// typeof foo === "string";
    /// ```
    ///
    /// ```js
    /// typeof bar == "undefined";
    /// ```
    ///
    /// ```js
    /// typeof bar === typeof qux;
    /// ```
    ///
    /// ```js
    /// typeof foo === bar
    /// ```
    pub UseValidTypeof {
        version: "1.0.0",
        name: "useValidTypeof",
        language: "js",
        sources: &[RuleSource::Eslint("valid-typeof")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

impl Rule for UseValidTypeof {
    type Query = Ast<JsBinaryExpression>;
    type State = AnyJsExpression;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();
        let left = n.left().ok()?.omit_parentheses();
        let right = n.right().ok()?.omit_parentheses();
        if !matches!(
            n.operator().ok()?,
            JsBinaryOperator::Equality
                | JsBinaryOperator::StrictEquality
                | JsBinaryOperator::Inequality
                | JsBinaryOperator::StrictInequality
        ) {
            return None;
        }

        // Test if one side is a `typeof` expression and set `other` to the other side.
        let other = match (left, right) {
            (AnyJsExpression::JsUnaryExpression(unary), other)
                if unary.operator().ok()? == JsUnaryOperator::Typeof =>
            {
                other
            }
            (other, AnyJsExpression::JsUnaryExpression(unary))
                if unary.operator().ok()? == JsUnaryOperator::Typeof =>
            {
                other
            }
            _ => {
                return None;
            }
        };

        if let Some(literal) = other.as_static_value() {
            let Some(literal_str) = literal.as_string_constant() else {
                // The literal is not a string
                return Some(other);
            };
            if JsTypeofValue::from_str(literal_str).is_ok() {
                return None;
            }
            return Some(other);
        }

        match other {
            // `typeof foo == ident`
            AnyJsExpression::JsIdentifierExpression(_)
            | AnyJsExpression::JsCallExpression(_)
            | AnyJsExpression::JsComputedMemberExpression(_)
            | AnyJsExpression::JsConditionalExpression(_)
            | AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsSuperExpression(_)
            | AnyJsExpression::JsThisExpression(_) => None,
            // `typeof foo == typeof bar`
            AnyJsExpression::JsUnaryExpression(unary)
                if unary.operator() == Ok(JsUnaryOperator::Typeof) =>
            {
                None
            }
            // `typeof foo == f() ?? g()`
            AnyJsExpression::JsLogicalExpression(expr)
                if expr.operator() == Ok(JsLogicalOperator::NullishCoalescing) =>
            {
                None
            }
            other => Some(other),
        }
    }

    fn diagnostic(_: &RuleContext<Self>, expr: &Self::State) -> Option<RuleDiagnostic> {
        if let Some(literal) = expr.as_static_value() {
            if let Some(literal) = literal.as_string_constant() {
                return Some(RuleDiagnostic::new(
                    rule_category!(),
                    expr.range(),
                    markup! {
                        "\""{literal}"\" is not a valid "<Emphasis>"typeof"</Emphasis>" value."
                    },
                ));
            }
        }
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                expr.range(),
                markup! { "Invalid "<Emphasis>"typeof"</Emphasis>" comparison." },
            )
            .footer_list(
                "Compare with one of the following string literals:",
                [
                    format_args!("\"{}\"", JsTypeofValue::BigInt),
                    format_args!("\"{}\"", JsTypeofValue::Boolean),
                    format_args!("\"{}\"", JsTypeofValue::Function),
                    format_args!("\"{}\"", JsTypeofValue::Number),
                    format_args!("\"{}\"", JsTypeofValue::Object),
                    format_args!("\"{}\"", JsTypeofValue::String),
                    format_args!("\"{}\"", JsTypeofValue::Symbol),
                    format_args!("\"{}\"", JsTypeofValue::Undefined),
                ],
            ),
        )
    }

    fn action(ctx: &RuleContext<Self>, other: &Self::State) -> Option<JsRuleAction> {
        let literal = other.as_static_value()?;
        let literal = literal.as_string_constant()?;

        // Try to fix the casing of the literal eg. "String" -> "string"
        let suggestion = literal.to_ascii_lowercase_cow();
        let suggestion = JsTypeofValue::from_str(&suggestion).ok()?;
        let suggestion = suggestion.as_str();

        let mut mutation = ctx.root().begin();
        mutation.replace_node(
            other.clone(),
            AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(
                make::js_string_literal_expression(if ctx.as_preferred_quote().is_double() {
                    make::js_string_literal(suggestion)
                } else {
                    make::js_string_literal_single_quotes(suggestion)
                }),
            )),
        );

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            markup! { "Use a valid "<Emphasis>"typeof"</Emphasis>" value." }.to_owned(),
            mutation,
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum JsTypeofValue {
    Undefined,
    Object,
    Boolean,
    Number,
    String,
    Function,
    Symbol,
    BigInt,
}
impl JsTypeofValue {
    /// Convert a [JsTypeName] to a JS string literal
    const fn as_str(self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Object => "object",
            Self::Boolean => "boolean",
            Self::Number => "number",
            Self::String => "string",
            Self::Function => "function",
            Self::Symbol => "symbol",
            Self::BigInt => "bigint",
        }
    }
}
impl FromStr for JsTypeofValue {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "undefined" => Ok(Self::Undefined),
            "object" => Ok(Self::Object),
            "boolean" => Ok(Self::Boolean),
            "number" => Ok(Self::Number),
            "string" => Ok(Self::String),
            "function" => Ok(Self::Function),
            "symbol" => Ok(Self::Symbol),
            "bigint" => Ok(Self::BigInt),
            _ => Err(()),
        }
    }
}
impl std::fmt::Display for JsTypeofValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
