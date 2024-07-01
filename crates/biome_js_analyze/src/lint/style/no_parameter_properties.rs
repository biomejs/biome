use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_syntax::TsPropertyParameter;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow the use of parameter properties in class constructors.
    ///
    /// TypeScript includes a "parameter properties" shorthand for declaring a class constructor parameter and class property in one location.
    /// Parameter properties can confuse those new to TypeScript as they are less explicit than other ways of declaring and initializing class members.
    /// Moreover, private class properties, starting with `#`, cannot be turned into "parameter properties".
    /// This questions the future of this feature.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// class A {
    ///     constructor(readonly name: string) {}
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// class A {
    ///     constructor(name: string) {}
    /// }
    /// ```
    ///
    pub NoParameterProperties {
        version: "1.0.0",
        name: "noParameterProperties",
        language: "ts",
        sources: &[RuleSource::EslintTypeScript("parameter-properties")],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

impl Rule for NoParameterProperties {
    type Query = Ast<TsPropertyParameter>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let param_prop = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            param_prop.range(),
            markup! {
                "Use a more explicit "<Emphasis>"class property"</Emphasis>" instead of a "<Emphasis>"parameter property"</Emphasis>"."
            },
        ).note(
            markup! {
                <Emphasis>"Parameter properties"</Emphasis>" are less explicit than other ways of declaring and initializing "<Emphasis>"class properties"</Emphasis>"."
            }
        ))
    }
}
