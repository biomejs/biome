use biome_analyze::{Ast, Rule, RuleDiagnostic, context::RuleContext, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyTsType, TsAsAssignment, TsAsExpression, TsTypeAssertionAssignment, TsTypeAssertionExpression,
};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_unsafe_type_assertion::NoUnsafeTypeAssertionOptions;

declare_lint_rule! {
    /// Disallow TypeScript type assertions other than const assertions.
    ///
    /// Type assertions override TypeScript's inferred type without performing any runtime checks.
    /// This can hide invalid assumptions about a value and lead to runtime errors.
    ///
    /// Safer alternatives include:
    ///
    /// - [Type annotations](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-annotations-on-variables)
    /// - The [`satisfies` operator](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-4-9.html#the-satisfies-operator)
    /// - [Type predicates](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#using-type-predicates)
    /// - [Assertion functions](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions)
    /// - [Control-flow narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#control-flow-analysis)
    /// - Validation libraries, like [Zod](https://zod.dev/), [Valibot](https://valibot.dev/), or [arktype](https://arktype.dev/)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// interface SomeType {
    ///     value: string;
    /// }
    /// declare const value;
    /// const asserted = value as SomeType;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface SomeType {
    ///     value: string;
    /// }
    /// declare const value;
    /// const asserted = <SomeType>value;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// interface SomeType {
    ///     value: string;
    /// }
    /// declare const asserted;
    /// (asserted as SomeType).value = "foo";
    /// ```
    ///
    /// ### Valid
    ///
    /// `const` assertions are allowed:
    ///
    /// ```ts
    /// const tuple = ["value", 1] as const;
    /// ```
    ///
    /// Use a [type annotation](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-annotations-on-variables):
    ///
    /// ```ts
    /// const annotated: string = "value";
    /// ```
    ///
    /// Use the [`satisfies` operator](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-4-9.html#the-satisfies-operator):
    ///
    /// ```ts
    /// const checked = { value: "value" } satisfies { value: string };
    /// ```
    ///
    /// Use a [type predicate](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#using-type-predicates):
    ///
    /// ```ts
    /// function isString(value: unknown): value is string {
    ///     return typeof value === "string";
    /// }
    /// ```
    ///
    /// Use an [assertion function](https://www.typescriptlang.org/docs/handbook/release-notes/typescript-3-7.html#assertion-functions):
    ///
    /// ```ts
    /// function assertIsString(value: unknown): asserts value is string {
    ///     if (!isString(value)) {
    ///         throw new TypeError("Expected a string");
    ///     }
    /// }
    /// ```
    ///
    /// Use [control-flow narrowing](https://www.typescriptlang.org/docs/handbook/2/narrowing.html#control-flow-analysis):
    ///
    /// ```ts
    /// function narrow(value: string | undefined) {
    ///     if (value !== undefined) {
    ///         return value.length;
    ///     }
    /// }
    /// ```
    pub NoUnsafeTypeAssertion {
        version: "2.5.9",
        name: "noUnsafeTypeAssertion",
        language: "ts",
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

impl Rule for NoUnsafeTypeAssertion {
    type Query = Ast<AnyTsTypeAssertionLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoUnsafeTypeAssertionOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ty = ctx.query().ty()?;
        (!is_const_reference_type(&ty)).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let query = ctx.query();
        let range = query.diagnostic_range()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid unsafe type assertions."
                },
            )
            .note(markup! {
                "Type assertions override the type for this expression, which can hide type errors and lead to runtime errors."
            })
            .note(markup! {
                "Use a "<Hyperlink href="https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#type-annotations-on-variables">"type annotation"</Hyperlink>", the "<Hyperlink href="https://www.typescriptlang.org/docs/handbook/release-notes/typescript-4-9.html#the-satisfies-operator">"satisfies operator"</Hyperlink>", a "<Hyperlink href="https://www.typescriptlang.org/docs/handbook/2/narrowing.html#using-type-predicates">"type predicate"</Hyperlink>", or "<Hyperlink href="https://www.typescriptlang.org/docs/handbook/2/narrowing.html#control-flow-analysis">"control-flow narrowing"</Hyperlink>" instead."
            }),
        )
    }
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

    fn diagnostic_range(&self) -> Option<TextRange> {
        Some(match self {
            Self::TsAsAssignment(assertion) => assertion
                .as_token()
                .ok()?
                .text_trimmed_range()
                .cover(assertion.ty().ok()?.range()),
            Self::TsAsExpression(assertion) => assertion
                .as_token()
                .ok()?
                .text_trimmed_range()
                .cover(assertion.ty().ok()?.range()),
            Self::TsTypeAssertionAssignment(assertion) => assertion
                .l_angle_token()
                .ok()?
                .text_trimmed_range()
                .cover(assertion.r_angle_token().ok()?.text_trimmed_range()),
            Self::TsTypeAssertionExpression(assertion) => assertion
                .l_angle_token()
                .ok()?
                .text_trimmed_range()
                .cover(assertion.r_angle_token().ok()?.text_trimmed_range()),
        })
    }
}

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
