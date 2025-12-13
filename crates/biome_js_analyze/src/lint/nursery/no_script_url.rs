use crate::react::ReactCreateElementCall;
use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{Rule, RuleDiagnostic, RuleSource, declare_lint_rule};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsxAttributeName, JsCallExpression, JsxAttribute};
use biome_rowan::{AstNode, TextRange, declare_node_union};
use biome_rule_options::no_script_url::NoScriptUrlOptions;
use biome_string_case::StrOnlyExtension;

declare_lint_rule! {
    /// Disallow `javascript:` URLs.
    ///
    /// Using `javascript:` URLs is considered a form of `eval` and can be a security risk.
    /// These URLs can execute arbitrary JavaScript code, which can lead to cross-site scripting (XSS) vulnerabilities.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="javascript:void(0)">Click me</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a href="javascript:alert('XSS')">Click me</a>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('a', { href: 'javascript:void(0)' });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <a href="https://example.com">Click me</a>
    /// ```
    ///
    /// ```jsx
    /// <a href="/path/to/page">Click me</a>
    /// ```
    ///
    /// ```jsx
    /// <a href="#section">Click me</a>
    /// ```
    ///
    pub NoScriptUrl {
        version: "2.3.9",
        name: "noScriptUrl",
        language: "js",
        sources: &[
            RuleSource::Eslint("no-script-url").same(),
            // Framework-specific equivalents
            RuleSource::EslintReact("jsx-no-script-url").same(),
            RuleSource::EslintQwik("jsx-no-script-url").same(),
            RuleSource::EslintSolid("jsx-no-script-url").same(),
            RuleSource::EslintReactXyz("dom-no-script-url").same(),
        ],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub AnyJsElementWithHref = JsxAttribute | JsCallExpression
}

pub enum NoScriptUrlState {
    JsxAttribute(TextRange),
    ReactProp(TextRange),
}

impl NoScriptUrlState {
    fn range(&self) -> TextRange {
        match self {
            Self::JsxAttribute(range) | Self::ReactProp(range) => *range,
        }
    }
}

impl Rule for NoScriptUrl {
    type Query = Semantic<AnyJsElementWithHref>;
    type State = NoScriptUrlState;
    type Signals = Option<Self::State>;
    type Options = NoScriptUrlOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            AnyJsElementWithHref::JsxAttribute(jsx_attribute) => {
                // Check if this is an href attribute
                let name = jsx_attribute.name().ok()?;
                if let AnyJsxAttributeName::JsxName(jsx_name) = name {
                    if jsx_name.syntax().text_trimmed() != "href" {
                        return None;
                    }
                } else {
                    return None;
                }

                // Check if the value contains javascript:
                let static_value = jsx_attribute.as_static_value()?;
                if let Some(const_str) = static_value.as_string_constant()
                    && const_str
                        .trim()
                        .to_lowercase_cow()
                        .starts_with("javascript:")
                {
                    return Some(NoScriptUrlState::JsxAttribute(
                        jsx_attribute.initializer()?.range(),
                    ));
                }
            }
            AnyJsElementWithHref::JsCallExpression(call_expression) => {
                // Check if this is a React.createElement call
                if let Some(react_create_element) =
                    ReactCreateElementCall::from_call_expression(call_expression, model)
                {
                    let ReactCreateElementCall { props, .. } = react_create_element;

                    // Look for href property in the props object
                    if let Some(props) = props {
                        let members = props.members();
                        for member in members {
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

                            if name.syntax().text_trimmed() == "href" {
                                let value = property_member.value().ok()?;

                                // Check if it's a string literal with javascript:
                                if let Some(string_literal) = value.as_any_js_literal_expression()
                                    && let Some(string_value) =
                                        string_literal.as_js_string_literal_expression()
                                {
                                    let text = string_value.inner_string_text().ok()?;
                                    if text.trim().to_lowercase_cow().starts_with("javascript:") {
                                        return Some(NoScriptUrlState::ReactProp(value.range()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.range(),
            markup! {
                "Avoid using "<Emphasis>"javascript:"</Emphasis>" URLs, as they can be a security risk."
            }
            .to_owned(),
        )
        .note(markup! {
            "Using "<Emphasis>"javascript:"</Emphasis>" URLs can lead to security vulnerabilities such as cross-site scripting (XSS)."
        })
        .note(markup! {
            "Consider using regular URLs, or if you need to handle click events, use event handlers instead."
        });

        Some(diagnostic)
    }
}
