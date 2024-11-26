use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{TsAnyType, TsTypeConstraintClause};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the `any` type usage.
    ///
    /// The `any` type in TypeScript is a dangerous "escape hatch" from the type system.
    /// Using `any` disables many type checking rules and is generally best used only as a last resort or when prototyping code.
    ///
    /// TypeScript's `--noImplicitAny` compiler option prevents an implied `any`,
    /// but doesn't prevent `any` from being explicitly used the way this rule does.
    ///
    /// Sometimes you can use the type `unknown` instead of the type `any`.
    /// It also accepts any value, however it requires to check that a property exists before calling it.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let variable: any = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class SomeClass {
    ///    message: Array<Array<any>>;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function fn(param: Array<any>): void {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let variable: number = 1;
    /// let variable2 = 1;
    /// ```
    ///
    /// ```ts
    /// class SomeClass<T extends any> {
    ///    message: Array<Array<unknown>>;
    /// }
    /// ```
    ///
    /// ```ts
    /// function fn(param: Array<Array<unknown>>): Array<unknown> {}
    /// ```
    ///
    pub NoExplicitAny {
        version: "1.0.0",
        name: "noExplicitAny",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("no-explicit-any")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoExplicitAny {
    type Query = Ast<TsAnyType>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if TsTypeConstraintClause::can_cast(node.syntax().parent()?.kind()) {
            // Ignore `<T extends any>`.
            // This use is inoffensive and already triggers the rule `noUselessTypeConstraint`.
            None
        } else {
            Some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"Unexpected "<Emphasis>"any"</Emphasis>". Specify a different type."}
                .to_owned(),
        ).note(markup! {
            <Emphasis>"any"</Emphasis>" disables many type checking rules. Its use should be avoided."
        });

        Some(diagnostic)
    }
}
