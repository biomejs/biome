use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{global_identifier, AnyJsExpression};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallows the use of `process` global.
    ///
    /// NodeJS and Deno expose `process` global but they are hard to statically analyze by tools, 
    /// so code should not assume they are available. Instead, `import process from "node:process"`.
    ///
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = process.env.FOO;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import process from "node:process";
    ///
    /// const foo = process.env.FOO;
    /// ```
    ///
    /// The rule is not able to detect cases where the global object is aliased:
    ///
    /// ```js
    /// const foo = globalThis;
    /// const bar = foo.process;
    /// ```
    ///
    pub NoProcessGlobal {
        version: "next",
        name: "noProcessGlobal",
        language: "js",
        sources: &[RuleSource::DenoLint("no-process-global")],
        recommended: false,
    }
}

impl Rule for NoProcessGlobal {
    type Query = Semantic<AnyJsExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        let (reference, name) = global_identifier(node)?;
        if name.text() != "process" {
            return None;
        }
        model.binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "process global is discouraged."
                },
            )
            .note(markup! {
                "Add `import process from \"node:procee\";`"
            })
        )
    }
}
