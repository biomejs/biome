use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsAssignment, AnyJsMemberExpression};
use biome_rowan::{AstNode, declare_node_union};
use biome_rule_options::no_proto::NoProtoOptions;

declare_lint_rule! {
    /// Disallow the use of the `__proto__` property.
    ///
    /// The use of `__proto__` for getting or setting the prototype of an object
    /// is deprecated. Use `Object.getPrototypeOf()` or
    /// `Object.setPrototypeOf()` instead.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// obj.__proto__ = a;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = obj.__proto__;
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// const a = Object.getPrototypeOf(obj);
    /// ```
    ///
    /// ```js
    /// Object.setPrototypeOf(obj, b);
    /// ```
    pub NoProto {
        version: "2.3.8",
        name: "noProto",
        language: "js",
        recommended: true,
        sources: &[RuleSource::Eslint("no-proto").same()],
    }
}

declare_node_union! {
    pub NoProtoQuery = AnyJsAssignment | AnyJsMemberExpression
}

impl Rule for NoProto {
    type Query = Ast<NoProtoQuery>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoProtoOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            NoProtoQuery::AnyJsAssignment(assignment) => match assignment {
                AnyJsAssignment::JsComputedMemberAssignment(assignment) => {
                    if assignment
                        .member()
                        .ok()?
                        .to_trimmed_text()
                        .trim_matches(['\'', '"', '`'])
                        == "__proto__"
                    {
                        return Some(());
                    }
                }
                AnyJsAssignment::JsStaticMemberAssignment(assignment) => {
                    if assignment.member().ok()?.to_trimmed_text() == "__proto__" {
                        return Some(());
                    }
                }
                _ => {}
            },
            NoProtoQuery::AnyJsMemberExpression(expr) => match expr {
                AnyJsMemberExpression::JsComputedMemberExpression(expr) => {
                    if expr
                        .member()
                        .ok()?
                        .to_trimmed_text()
                        .trim_matches(['\'', '"', '`'])
                        == "__proto__"
                    {
                        return Some(());
                    }
                }
                AnyJsMemberExpression::JsStaticMemberExpression(expr) => {
                    if expr.member().ok()?.to_trimmed_text() == "__proto__" {
                        return Some(());
                    }
                }
            },
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unexpected use of "<Emphasis>"__proto__"</Emphasis>"."
            },
        )
            .note(markup! {
                "The use of "<Emphasis>"__proto__"</Emphasis>" for getting or setting the prototype of an object is deprecated."
            })
            .note(markup! {
                "Use "<Emphasis>"Object.getPrototypeOf()"</Emphasis>" or "<Emphasis>"Object.setPrototypeOf()"</Emphasis>" instead."
            })
        )
    }
}
