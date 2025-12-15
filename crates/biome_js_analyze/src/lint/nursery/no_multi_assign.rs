use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, JsAssignmentExpression, JsPropertyClassMember, JsVariableDeclarator,
};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_multi_assign::NoMultiAssignOptions;

declare_lint_rule! {
    /// Disallow use of chained assignment expressions
    ///
    /// Chaining the assignment of variables can lead to unexpected results and
    /// be difficult to read.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// const foo = bar = "baz";
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const foo = "baz";
    /// const bar = "baz";
    /// ```
    ///
    pub NoMultiAssign {
        version: "next",
        name: "noMultiAssign",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("no-multi-assign").inspired()],
    }
}

impl Rule for NoMultiAssign {
    type Query = Ast<NoMultiAssignQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoMultiAssignOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query = ctx.query();
        match query {
            NoMultiAssignQuery::JsVariableDeclarator(node) => {
                let expr = node.initializer()?.expression().ok()?;
                if matches!(expr, AnyJsExpression::JsAssignmentExpression(_)) {
                    return Some(());
                }
            }
            NoMultiAssignQuery::JsPropertyClassMember(node) => {
                let expr = node.value()?.expression().ok()?;
                if matches!(expr, AnyJsExpression::JsAssignmentExpression(_)) {
                    return Some(());
                }
            }
            NoMultiAssignQuery::JsAssignmentExpression(node) => {
                let expr = node.right().ok()?;
                if matches!(expr, AnyJsExpression::JsAssignmentExpression(_)) {
                    return Some(());
                }
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Unexpected chained assignment."
                },
            )
            .note(markup! {
               "Variables with chained assignments in declarations may cause unintended implicit globals or unexpected scoping."
            }).note(markup!{
                "Split into separate assignments."
            }),
        )
    }
}

declare_node_union! {
    pub NoMultiAssignQuery = JsVariableDeclarator | JsPropertyClassMember | JsAssignmentExpression
}
