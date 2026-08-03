use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyTsType, TsAsAssignment, TsAsExpression, TsTypeAssertionAssignment,
    TsTypeAssertionExpression,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_unsafe_type_assertion::NoUnsafeTypeAssertionOptions;

declare_lint_rule! {
    /// Disallow TypeScript type assertions other than const assertions.
    ///
    /// Type assertions override TypeScript's inferred type without performing any runtime checks.
    /// This can hide invalid assumptions about a value and lead to runtime errors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface SomeType {
    ///     value: string;
    /// }
    /// declare const value: unknown;
    /// const asserted = value as SomeType;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface SomeType {
    ///     value: string;
    /// }
    /// declare const value: unknown;
    /// const asserted = <SomeType>value;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface SomeType {
    ///     value: string;
    /// }
    /// declare const asserted: unknown;
    /// (asserted as SomeType).value = "foo";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// const tuple = ["value", 1] as const;
    /// const annotated: string = "value";
    /// const checked = { value: "value" } satisfies { value: string };
    /// ```
    pub NoUnsafeTypeAssertion {
        version: "next",
        name: "noUnsafeTypeAssertion",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("consistent-type-assertions").inspired()],
        recommended: false,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyTsTypeAssertionLike =
        TsAsAssignment
        | TsAsExpression
        | TsTypeAssertionAssignment
        | TsTypeAssertionExpression
}

impl AnyTsTypeAssertionLike {
    fn ty(&self) -> Option<AnyTsType> {
        match self {
            Self::TsAsAssignment(assertion) => assertion.ty().ok(),
            Self::TsAsExpression(assertion) => assertion.ty().ok(),
            Self::TsTypeAssertionAssignment(assertion) => assertion.ty().ok(),
            Self::TsTypeAssertionExpression(assertion) => assertion.ty().ok(),
        }
    }
}

impl Rule for NoUnsafeTypeAssertion {
    type Query = Ast<AnyTsTypeAssertionLike>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoUnsafeTypeAssertionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ty = ctx.query().ty()?;
        (!is_const_reference_type(&ty)).then_some(ty.range())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid unsafe type assertions."
                },
            )
            .note(markup! {
                "Type assertions bypass TypeScript's type checking and can cause runtime errors."
            })
            .note(markup! {
                "I suggest using a type annotation, the "<Emphasis>"satisfies"</Emphasis>" operator, a type guard, or control-flow narrowing instead."
            }),
        )
    }
}

// Copied from `biome_js_type_info::local_inference::is_const_reference_type`.
fn is_const_reference_type(type_annotation: &AnyTsType) -> bool {
    let Some(reference_type) = type_annotation.as_ts_reference_type() else {
        return false;
    };

    reference_type.type_arguments().is_none()
        && reference_type.name().ok().is_some_and(|name| {
            name.as_js_reference_identifier()
                .and_then(|identifier| identifier.value_token().ok())
                .is_some_and(|token| token.text_trimmed() == "const")
        })
}
