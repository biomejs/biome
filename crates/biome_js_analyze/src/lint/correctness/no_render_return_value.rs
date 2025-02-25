use crate::react::{is_react_call_api, ReactLibrary};
use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Rule, RuleDiagnostic, RuleDomain};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{JsCallExpression, JsExpressionStatement};
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Prevent the usage of the return value of `React.render`.
    ///
    /// > `ReactDOM.render()` currently returns a reference to the root `ReactComponent` instance. However, using this return value is legacy
    /// and should be avoided because future versions of React may render components asynchronously in some cases.
    /// If you need a reference to the root `ReactComponent` instance, the preferred solution is to attach a [callback ref](https://reactjs.org/docs/refs-and-the-dom.html#callback-refs)
    /// to the root element.
    ///
    /// Source: [ReactDOM documentation](https://facebook.github.io/react/docs/react-dom.html#render)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const foo = ReactDOM.render(<div />, document.body);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// ReactDOM.render(<div />, document.body);
    /// ```
    pub NoRenderReturnValue {
        version: "1.0.0",
        name: "noRenderReturnValue",
        language: "jsx",
        recommended: true,
        domains: &[RuleDomain::React],
        severity: Severity::Error,
    }
}

impl Rule for NoRenderReturnValue {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?.omit_parentheses();
        let model = ctx.model();
        if is_react_call_api(&callee, model, ReactLibrary::ReactDOM, "render") {
            let parent = node.syntax().parent()?;

            if !JsExpressionStatement::can_cast(parent.kind()) {
                return Some(());
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Do not depend on the value returned by the function "<Emphasis>"ReactDOM.render()"</Emphasis>"."
            },
        ).note(markup! {
"The returned value is legacy and future versions of React might return that value asynchronously."
"
Check the "<Hyperlink href="https://facebook.github.io/react/docs/react-dom.html#render">"React documentation"</Hyperlink>" for more information."

        })
        )
    }
}
