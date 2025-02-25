use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleDomain, RuleSource,
    RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsArrowFunctionParameters, AnyJsExpression, AnyJsMemberExpression, JsCallExpression,
    JsSyntaxKind, JsxExpressionChild,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeOptionExt};

pub enum ReportType {
    PreferFor(JsCallExpression),
    PreferForOrIndex(JsCallExpression),
}

declare_lint_rule! {
    /// Enforce using Solid's `<For />` component for mapping an array to JSX elements.
    ///
    /// In Solid, `<For />` component for efficiently rendering lists. Array#map causes DOM elements to be recreated.
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
    pub UsePreferFor {
        version: "next",
        name: "usePreferFor",
        language: "js",
        domains: &[RuleDomain::Solid],
        recommended: false,
        sources: &[RuleSource::EslintSolid("perfer-for")],
        source_kind: RuleSourceKind::Inspired,
    }
}

impl Rule for UsePreferFor {
    type Query = Ast<JsCallExpression>;
    type State = ReportType;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if let Some(parent) = node.parent::<JsxExpressionChild>() {
            // judge the expression child under JSX_CHILD_LIST.
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

            if let Some(first_argument) = args.first() {
                let first_argument = first_argument.ok()?;

                match first_argument.as_any_js_expression()? {
                    AnyJsExpression::JsArrowFunctionExpression(arg) => {
                        let prefer_for_or_index = match arg.parameters().ok()? {
                            AnyJsArrowFunctionParameters::AnyJsBinding(_) => false,
                            AnyJsArrowFunctionParameters::JsParameters(params) => {
                                params.items().len() != 1
                                    || params
                                        .items()
                                        .first()
                                        .and_then(|item| item.ok())
                                        .is_some_and(|param| param.as_js_rest_parameter().is_some())
                            }
                        };

                        if prefer_for_or_index {
                            return Some(ReportType::PreferForOrIndex(node.clone()));
                        } else {
                            // The map fn doesn't take an index param, so it can't possibly be an index-keyed list. Use <For />.
                            // The returned JSX, if it's coming from React, will have an unnecessary `key` prop to be removed in
                            // the useless-keys rule.
                            return Some(ReportType::PreferFor(node.clone()));
                        }
                    }
                    AnyJsExpression::JsFunctionExpression(arg) => {
                        arg.parameters().ok().map(|params| {
                            let is_valid = params.items().len() == 1
                                && params
                                    .items()
                                    .first()
                                    .and_then(|item| item.ok())
                                    .is_some_and(|param| param.as_js_rest_parameter().is_none());

                            if is_valid {
                                ReportType::PreferFor(node.clone())
                            } else {
                                ReportType::PreferForOrIndex(node.clone())
                            }
                        });
                    }
                    _ => return None,
                }
            }

            return None;
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
            "Array#map causes DOM elements to be recreated."
            },
        );

        if matches!(state, ReportType::PreferForOrIndex(_)) {
            Some(diagnostic.note(markup! {
                "Use Solid's `<For />` component or `<Index />` component for rendering lists."
            }))
        } else {
            Some(diagnostic.note(markup! {
                "Use Solid's `<For />` component for efficiently rendering lists."
            }))
        }
    }
}
