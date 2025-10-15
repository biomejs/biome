use biome_analyze::{
    Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression, JsSyntaxKind, JsxExpressionChild};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeOptionExt};
use biome_rule_options::use_solid_for_component::UseSolidForComponentOptions;

declare_lint_rule! {
    /// Enforce using Solid's `<For />` component for mapping an array to JSX elements.
    ///
    /// In Solid, `<For />` component for efficiently rendering lists. Array#map causes DOM elements to be recreated.
    ///
    /// For details on `<For />` Component, see the [Solid docs about Components](https://docs.solidjs.com/reference/components/for).
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// let Component = (props) => <ol>{props.data.map(d => <li>{d.text}</li>)}</ol>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// let Component = (props) => <>{props.data.map(d => <li>{d.text}</li>)}</>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// let Component = (props) => (
    ///   <ol>
    ///     {props.data.map((d) => (
    ///       <li key={d.id}>{d.text}</li>
    ///     ))}
    ///   </ol>
    /// );
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// let Component = (props) => <ol><For each={props.data}>{d => <li>{d.text}</li>}</For></ol>;
    /// ```
    ///
    /// ```jsx
    /// let abc = x.map(y => y + z);
    /// ```
    ///
    /// ```jsx
    /// let Component = (props) => {
    ///  let abc = x.map(y => y + z);
    ///  return <div>Hello, world!</div>;
    /// }
    /// ```
    ///
    pub UseSolidForComponent {
        version: "2.0.0",
        name: "useSolidForComponent",
        language: "js",
        domains: &[RuleDomain::Solid],
        recommended: false,
        sources: &[RuleSource::EslintSolid("prefer-for").inspired()],
    }
}

impl Rule for UseSolidForComponent {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSolidForComponentOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let Some(parent) = node.parent::<JsxExpressionChild>() {
            // Only judge the expression child under JSX_CHILD_LIST
            // all jsxexpression with case can be covered here like:
            // <ol>{props.data.map(d => <li>{d.text}</li>)}</ol>
            if !matches!(
                parent.syntax().parent().kind(),
                Some(JsSyntaxKind::JSX_CHILD_LIST)
            ) {
                return None;
            }

            // check for Array.prototype.map in JSX
            let member_expression =
                AnyJsMemberExpression::cast(node.callee().ok()?.omit_parentheses().into_syntax())?;
            let args = node.arguments().ok()?.args();

            if args.len() != 1 || member_expression.member_name()?.text() != "map" {
                return None;
            }

            return Some(());
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
            "Array.prototype.map will cause DOM elements to be recreated, it is not recommended to use it in Solid here."
            },
        ).note(markup! {
            "Use Solid's "<Emphasis>"<For />"</Emphasis>" component for efficiently rendering lists. See \
            "<Hyperlink href="https://docs.solidjs.com/reference/components/for">"Solid docs"</Hyperlink>" for more details."
        }))
    }
}
