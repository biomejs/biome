use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Require explicit return types on functions and class methods.
    ///
    /// Functions in TypeScript often don't need to be given an explicit return type annotation.
    /// Leaving off the return type is less code to read or write and allows the compiler to infer it from the contents of the function.
    ///
    /// However, explicit return types do make it visually more clear what type is returned by a function.
    /// They can also speed up TypeScript type checking performance in large codebases with many large functions.
    ///
    /// This rule enforces that functions do have an explicit return type annotation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that no value is returned (void)
    /// function test() {
    ///   return;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that a number is returned
    /// var fn = function () {
    ///    return 1;
    /// };
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// // Should indicate that a string is returned
    /// var arrowFn = () => 'test';
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class Test {
    ///   // Should indicate that no value is returned (void)
    ///   method() {
    ///     return;
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    /// ```ts
    /// // No return value should be expected (void)
    /// function test(): void {
    ///   return;
    /// }
    /// ```
    ///
    /// ```ts
    /// // A return value of type number
    /// var fn = function (): number {
    ///   return 1;
    /// }
    /// ```
    ///
    /// ```ts
    /// // A return value of type string
    /// var arrowFn = (): string => 'test';
    /// ```
    ///
    /// ```ts
    /// class Test {
    ///   // No return value should be expected (void)
    ///   method(): void {
    ///     return;
    ///   }
    /// }
    /// ```
    ///
    /// ## Options
    ///
    /// The rule provides several options that are detailed in the following subsections.
    ///
    /// ```json
    /// {
    ///     "//": "...",
    ///     "options": {
    ///         "allowExpressions": false,
    ///         "allowTypedFunctionExpressions": true,
    ///         "allowHigherOrderFunctions": true
    ///         "allowDirectConstAssertionInArrowFunctions": true
    ///         "allowConciseArrowFunctionExpressionsStartingWithVoid": false
    ///         "allowFunctionsWithoutTypeParameters": false
    ///         "allowedNames": [],
    ///         "allowIIFEs": false
    ///     }
    /// }
    /// ```
    ///
    pub UseExplicitFunctionReturnType {
        version: "next",
        name: "useExplicitFunctionReturnType",
        language: "ts",
        recommended: false,
        sources: &[RuleSource::EslintTypeScript("explicit-function-return-type")],
    }
}

impl Rule for UseExplicitFunctionReturnType {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
