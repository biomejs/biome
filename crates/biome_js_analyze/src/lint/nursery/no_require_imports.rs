use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_js_syntax::JsCallExpression;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Use the newer ES6-style imports over require().
    ///
    /// ES6-style `import`s are modern alternative to CommonJS `require` imports. Supported by all modern browsers and Node.js versions.
    /// Tooling can more easily statically analyze and tree-shake ES6-style `import`s compared in comparison to `require`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// require('node:fs');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import fs from 'node:fs';
    /// ```
    /// ```js
    /// import('node:fs')
    /// ```
    ///
    pub NoRequireImports {
        version: "next",
        name: "noRequireImports",
        language: "js",
        sources: &[
            RuleSource::EslintTypeScript("no-require-imports"),
        ],
        recommended: false,
    }
}

impl Rule for NoRequireImports {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx.query();
        let callee = expression.callee().ok()?;
        let name = callee.as_js_reference_identifier()?.value_token().ok()?;
        if name.text_trimmed() == "require" {
            Some(())
        } else {
            None
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Use ES6-style `import`s instead of `require`."
                },
            )
            .note(markup! {
                "ES6-style `import` statements are more easily statically analyzable and tree-shakable compared to `require` imports."
            }),
        )
    }
}
