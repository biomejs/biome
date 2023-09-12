use crate::react::ReactCreateElementCall;
use crate::semantic_services::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic};
use biome_console::markup;
use biome_js_syntax::{AnyJsxAttributeName, JsCallExpression, JsxAttribute};
use biome_rowan::{declare_node_union, AstNode, TextRange};

declare_rule! {
    /// Prevent the usage of dangerous JSX props
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function createMarkup() {
    ///     return { __html: 'child' }
    /// }
    /// <div dangerouslySetInnerHTML={createMarkup()}></div>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('div', {
    ///     dangerouslySetInnerHTML: { __html: 'child' }
    /// });
    /// ```
    pub(crate) NoDangerouslySetInnerHtml {
        version: "1.0.0",
        name: "noDangerouslySetInnerHtml",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) AnyJsCreateElement = JsxAttribute | JsCallExpression
}

pub(crate) enum NoDangerState {
    Attribute(TextRange),
    Property(TextRange),
}

impl NoDangerState {
    fn range(&self) -> TextRange {
        match self {
            NoDangerState::Attribute(range) | NoDangerState::Property(range) => *range,
        }
    }
}

impl Rule for NoDangerouslySetInnerHtml {
    type Query = Semantic<AnyJsCreateElement>;
    type State = NoDangerState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        match node {
            AnyJsCreateElement::JsxAttribute(jsx_attribute) => {
                let name = jsx_attribute.name().ok()?;
                match name {
                    AnyJsxAttributeName::JsxName(jsx_name) => {
                        if jsx_name.syntax().text_trimmed() == "dangerouslySetInnerHTML" {
                            return Some(NoDangerState::Attribute(
                                jsx_attribute.name().ok()?.range(),
                            ));
                        }
                    }
                    AnyJsxAttributeName::JsxNamespaceName(_) => return None,
                }
            }
            AnyJsCreateElement::JsCallExpression(call_expression) => {
                if let Some(react_create_element) =
                    ReactCreateElementCall::from_call_expression(call_expression, model)
                {
                    let ReactCreateElementCall { props, .. } = react_create_element;
                    // if we are inside a create element call, we inspect the second argument, which
                    // should be an object expression. We look for a member that has as name
                    // "dangerouslySetInnerHTML"
                    if let Some(props) = props {
                        let members = props.members();
                        for member in members {
                            let member = member.ok()?;
                            let property_member =
                                member.as_js_property_object_member()?.name().ok()?;
                            let name = property_member.as_js_literal_member_name()?;

                            if name.syntax().text_trimmed() == "dangerouslySetInnerHTML" {
                                return Some(NoDangerState::Property(name.range()));
                            }
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(rule_category!(),
            state.range(),
            markup! {
                "Avoid passing content using the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
            }
                .to_owned(),
        ).warning(
            "Setting content using code can expose users to cross-site scripting (XSS) attacks",
        );
        Some(diagnostic)
    }
}
