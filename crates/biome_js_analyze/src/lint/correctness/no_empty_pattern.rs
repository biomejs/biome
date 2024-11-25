use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsArrayBindingPattern, JsObjectBindingPattern};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList};

declare_lint_rule! {
    /// Disallows empty destructuring patterns.
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var {} = foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var {a: {}} = foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo({}) {}
    /// ```
    ///
    /// ### Valid
    /// The following cases are valid because they create new bindings.
    ///
    /// ```js
    /// var {a = {}} = foo;
    /// var {a, b = {}} = foo;
    /// var {a = []} = foo;
    /// function foo({a = {}}) {}
    /// function foo({a = []}) {}
    /// var [a] = foo;
    /// ```
    pub NoEmptyPattern {
        version: "1.0.0",
        name: "noEmptyPattern",
        language: "js",
        sources: &[RuleSource::Eslint("no-empty-pattern")],
        recommended: true,
        severity: Severity::Error,
    }
}

impl Rule for NoEmptyPattern {
    type Query = Ast<JsAnyBindPatternLike>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        use JsAnyBindPatternLike::*;
        match ctx.query() {
            JsArrayBindingPattern(array) => {
                if array.elements().len() == 0 {
                    Some(())
                } else {
                    None
                }
            }
            JsObjectBindingPattern(object) => {
                if object.properties().len() == 0 {
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let node_type = match node {
            JsAnyBindPatternLike::JsArrayBindingPattern(_) => "array",
            JsAnyBindPatternLike::JsObjectBindingPattern(_) => "object",
        };

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unexpected empty "{node_type}" pattern."
            },
        ))
    }
}

declare_node_union! {
    /// enum of `JsObjectBindingPattern` and `JsArrayBindingPattern`
    pub JsAnyBindPatternLike = JsArrayBindingPattern | JsObjectBindingPattern
}
