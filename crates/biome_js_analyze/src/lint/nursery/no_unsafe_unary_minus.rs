use crate::services::typed::Typed;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsUnaryExpression, JsUnaryOperator};
use biome_rowan::AstNode;
use biome_rule_options::no_unsafe_unary_minus::NoUnsafeUnaryMinusOptions;

declare_lint_rule! {
    /// Require the operand of unary `-` to be a `number` or a `bigint`.
    ///
    /// Unary negation converts any other operand to a number first. Negating a
    /// string, boolean, nullish value, or object usually produces `NaN` or an
    /// unexpected number, and TypeScript accepts many of these cases without
    /// reporting an error.
    ///
    /// Operands typed as `any` or `never` are allowed. Explicit `unknown`
    /// operands are reported.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic,file=invalid-string.ts
    /// declare const value: string;
    /// -value;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-object.ts
    /// declare const value: { x: number };
    /// -value;
    /// ```
    ///
    /// ```ts,expect_diagnostic,file=invalid-unknown.ts
    /// declare const value: unknown;
    /// -value;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts,file=valid-number.ts
    /// declare const value: number;
    /// -value;
    /// ```
    ///
    /// ```ts,file=valid-numeric-union.ts
    /// declare const value: number | bigint;
    /// -value;
    /// ```
    ///
    /// ```ts,file=valid-member.ts
    /// declare const value: { x: number };
    /// -value.x;
    /// ```
    pub NoUnsafeUnaryMinus {
        version: "2.6.0",
        name: "noUnsafeUnaryMinus",
        language: "js",
        sources: &[RuleSource::EslintTypeScript("no-unsafe-unary-minus").same()],
        recommended: true,
        domains: &[RuleDomain::Types],
    }
}

impl Rule for NoUnsafeUnaryMinus {
    type Query = Typed<JsUnaryExpression>;
    /// Description of the first operand type that isn't a `number` or `bigint`.
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeUnaryMinusOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node.operator().ok()? != JsUnaryOperator::Minus {
            return None;
        }

        let argument = node.argument().ok()?;
        // Inference treats `values[index]` as possibly `undefined`, but
        // TypeScript doesn't by default.
        let allow_undefined = matches!(
            argument.clone().omit_parentheses(),
            AnyJsExpression::JsComputedMemberExpression(_)
        );
        let ty = ctx.type_of_expression(&argument)?;
        if ty.is_unknown_keyword() {
            return Some(ty.type_description());
        }

        let variant = ty
            .try_find_variant(|variant| {
                if variant.is_primitive() {
                    !(variant.is_number_or_number_literal()
                        || variant.is_bigint_or_bigint_literal()
                        || (allow_undefined && variant.is_undefined()))
                } else {
                    variant.is_object_like()
                }
            })
            .ok()??;

        Some(if variant.is_function() {
            "function"
        } else if variant.is_primitive() {
            variant.type_description()
        } else {
            "object"
        })
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "The operand of unary "<Emphasis>"-"</Emphasis>" may be "<Emphasis>{*state}</Emphasis>" instead of "<Emphasis>"number"</Emphasis>" or "<Emphasis>"bigint"</Emphasis>"."
                },
            )
            .note(markup! {
                "Unary "<Emphasis>"-"</Emphasis>" converts other values to a number first, which usually produces "<Emphasis>"NaN"</Emphasis>" or an unexpected result."
            })
            .note(markup! {
                "Convert the operand to a number explicitly, or negate a numeric value instead."
            }),
        )
    }
}
