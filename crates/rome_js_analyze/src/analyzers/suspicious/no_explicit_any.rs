use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::TsAnyType;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the `any` type usage.
    ///
    /// The `any` type in TypeScript is a dangerous "escape hatch" from the type system.
    /// Using `any` disables many type checking rules and is generally best used only as a last resort or when prototyping code.
    ///
    /// TypeScript's `--noImplicitAny` compiler option prevents an implied `any`,
    /// but doesn't prevent `any` from being explicitly used the way this rule does.
    ///
    /// Source: https://typescript-eslint.io/rules/no-explicit-any
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
    ///   message: Array<Array<any>>;
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
    /// class SomeClass {
    ///   message: Array<Array<unknown>>;
    /// }
    /// ```
    ///
    /// ```ts
    /// function fn(param: Array<Array<unknown>>): Array<unknown> {}
    /// ```
    ///
    /// ```
    pub(crate) NoExplicitAny {
        version: "1.0.0",
        name: "noExplicitAny",
        recommended: true,
    }
}

impl Rule for NoExplicitAny {
    type Query = Ast<TsAnyType>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
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
