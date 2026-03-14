use crate::react::ReactCreateElementCall;
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsxAttributeName, JsCallExpression, JsxAttribute};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_inline_styles::NoInlineStylesOptions;

declare_lint_rule! {
    /// Disallow the use of inline styles in JSX.
    ///
    /// Inline styles via the `style` attribute make code harder to maintain and override,
    /// prevent reusability of styling, and can be a security concern when implementing
    /// a strict Content Security Policy (CSP).
    ///
    /// Instead of inline styles, use CSS classes, CSS modules, or a styling library.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div style={{ color: "red" }}>Error</div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button style={{ background: "blue" }}>Click</button>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement("div", { style: { color: "red" } });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div className="text-red">Error</div>
    /// ```
    ///
    /// ```jsx
    /// <button className="btn-primary">Click</button>
    /// ```
    ///
    /// ```js
    /// React.createElement("div", { className: "container" });
    /// ```
    ///
    pub NoInlineStyles {
        version: "next",
        name: "noInlineStyles",
        language: "jsx",
        sources: &[
            RuleSource::HtmlEslint("no-inline-styles").inspired(),
        ],
        domains: &[RuleDomain::React],
        recommended: false,
    }
}

declare_node_union! {
    pub AnyJsElementWithStyle = JsxAttribute | JsCallExpression
}

pub enum NoInlineStylesState {
    JsxAttribute(TextRange),
    ReactProp(TextRange),
}

impl NoInlineStylesState {
    fn range(&self) -> TextRange {
        match self {
            Self::JsxAttribute(range) | Self::ReactProp(range) => *range,
        }
    }
}

impl Rule for NoInlineStyles {
    type Query = Semantic<AnyJsElementWithStyle>;
    type State = NoInlineStylesState;
    type Signals = Option<Self::State>;
    type Options = NoInlineStylesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            AnyJsElementWithStyle::JsxAttribute(jsx_attribute) => {
                let name = jsx_attribute.name().ok()?;
                if let AnyJsxAttributeName::JsxName(jsx_name) = name {
                    if jsx_name.syntax().text_trimmed() == "style" {
                        return Some(NoInlineStylesState::JsxAttribute(jsx_attribute.range()));
                    }
                }
            }
            AnyJsElementWithStyle::JsCallExpression(call_expression) => {
                if let Some(react_create_element) =
                    ReactCreateElementCall::from_call_expression(call_expression, model)
                {
                    let ReactCreateElementCall { props, .. } = react_create_element;

                    if let Some(props) = props {
                        for member in props.members() {
                            let Ok(member) = member else { continue };
                            let Some(property_member) = member.as_js_property_object_member()
                            else {
                                continue;
                            };
                            let Ok(property_name) = property_member.name() else {
                                continue;
                            };
                            let Some(name) = property_name.as_js_literal_member_name() else {
                                continue;
                            };

                            if name.syntax().text_trimmed() == "style" {
                                return Some(NoInlineStylesState::ReactProp(
                                    property_member.range(),
                                ));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Avoid using the "<Emphasis>"style"</Emphasis>" prop."
                },
            )
            .note(markup! {
                "Inline styles make code harder to maintain, override, and can interfere with Content Security Policy."
            })
            .note(markup! {
                "Use a CSS class or a styling library instead."
            }),
        )
    }
}
