use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsxAttribute};
use biome_rowan::AstNode;
use biome_rule_options::no_jsx_props_bind::NoJsxPropsBindOptions;

declare_lint_rule! {
    /// Disallow .bind() or function declaration in JSX props
    ///
    /// Using `.bind()` on a function or declaring a function directly in props
    /// creates a new function on every render, which is treated as a completely different function.
    ///
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <Foo onClick={this._handleClick.bind(this)}></Foo>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <Foo onClick={() => console.log('Hello!')}></Foo>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <Foo onClick={function () { console.log('Hello!'); }}></Foo>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// <Foo onClick={this._handleClick}></Foo>
    /// ```
    ///
    pub NoJsxPropsBind {
        version: "next",
        name: "noJsxPropsBind",
        language: "jsx",
        recommended: false,
        sources: &[RuleSource::EslintReact("jsx-no-bind").inspired()],
        domains: &[RuleDomain::React],
    }
}

impl Rule for NoJsxPropsBind {
    type Query = Ast<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoJsxPropsBindOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expression = ctx
            .query()
            .initializer()?
            .value()
            .ok()?
            .as_jsx_expression_attribute_value()?
            .expression()
            .ok()?;
        match expression {
            AnyJsExpression::JsArrowFunctionExpression(_) => Some(()),
            AnyJsExpression::JsFunctionExpression(_) => Some(()),
            AnyJsExpression::JsCallExpression(_) => Some(()),
            _ => None,
        }
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
