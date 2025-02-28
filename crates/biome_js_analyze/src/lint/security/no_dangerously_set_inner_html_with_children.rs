use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::services::semantic::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    JsCallExpression, JsPropertyObjectMember, JsSyntaxNode, JsxAttribute, JsxElement,
    JsxSelfClosingElement,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, TextRange};

declare_lint_rule! {
    /// Report when a DOM element or a component uses both `children` and `dangerouslySetInnerHTML` prop.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function createMarkup() {
    ///     return { __html: 'child' }
    /// }
    /// <Component dangerouslySetInnerHTML={createMarkup()}>"child1"</Component>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// function createMarkup() {
    ///     return { __html: 'child' }
    /// }
    /// <Component dangerouslySetInnerHTML={createMarkup()} children="child1" />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('div', { dangerouslySetInnerHTML: { __html: 'HTML' } }, 'children')
    /// ```
    pub NoDangerouslySetInnerHtmlWithChildren {
        version: "1.0.0",
        name: "noDangerouslySetInnerHtmlWithChildren",
        language: "jsx",
        sources: &[RuleSource::EslintReact("no-danger-with-children")],
        recommended: true,
        severity: Severity::Error,
    }
}

declare_node_union! {
    pub DangerousProp = JsxAttribute | JsPropertyObjectMember
}
/// The kind of children
enum ChildrenKind {
    /// As prop, e.g.
    /// ```jsx
    /// <Component children="child" />
    /// ```
    Prop(TextRange),
    /// As direct descendent, e.g.
    /// ```jsx
    /// <ComponentA><ComponentB /> </ComponentA>
    /// ```
    Direct(TextRange),
}

impl ChildrenKind {
    fn range(&self) -> &TextRange {
        match self {
            ChildrenKind::Prop(range) | ChildrenKind::Direct(range) => range,
        }
    }
}

pub struct RuleState {
    /// The `dangerouslySetInnerHTML` prop range
    dangerous_prop: TextRange,

    /// The kind of `children` found
    children_kind: ChildrenKind,
}

declare_node_union! {
    pub AnyJsCreateElement = JsxElement | JsxSelfClosingElement | JsCallExpression
}

impl AnyJsCreateElement {
    /// If checks if the element has direct children (no children prop)
    fn has_children(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        match self {
            AnyJsCreateElement::JsxElement(element) => {
                if !element.children().is_empty() {
                    Some(element.children().syntax().clone())
                } else {
                    None
                }
            }
            AnyJsCreateElement::JsxSelfClosingElement(_) => None,
            AnyJsCreateElement::JsCallExpression(expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(expression, model)?;

                react_create_element
                    .children
                    .map(|children| children.syntax().clone())
            }
        }
    }

    fn find_dangerous_prop(&self, model: &SemanticModel) -> Option<DangerousProp> {
        match self {
            AnyJsCreateElement::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;

                opening_element
                    .find_attribute_by_name("dangerouslySetInnerHTML")
                    .map(DangerousProp::from)
            }
            AnyJsCreateElement::JsxSelfClosingElement(element) => element
                .find_attribute_by_name("dangerouslySetInnerHTML")
                .map(DangerousProp::from),
            AnyJsCreateElement::JsCallExpression(call_expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;

                react_create_element
                    .find_prop_by_name("dangerouslySetInnerHTML")
                    .map(DangerousProp::from)
            }
        }
    }

    fn find_children_prop(&self, model: &SemanticModel) -> Option<DangerousProp> {
        match self {
            AnyJsCreateElement::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;

                opening_element
                    .find_attribute_by_name("children")
                    .map(DangerousProp::from)
            }
            AnyJsCreateElement::JsxSelfClosingElement(element) => element
                .find_attribute_by_name("children")
                .map(DangerousProp::from),
            AnyJsCreateElement::JsCallExpression(call_expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;

                react_create_element
                    .find_prop_by_name("children")
                    .map(DangerousProp::from)
            }
        }
    }
}

impl Rule for NoDangerouslySetInnerHtmlWithChildren {
    type Query = Semantic<AnyJsCreateElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();
        if let Some(dangerous_prop) = node.find_dangerous_prop(model) {
            let dangerous_prop = dangerous_prop.range();
            if let Some(children_node) = node.has_children(model) {
                return Some(RuleState {
                    children_kind: ChildrenKind::Direct(children_node.text_trimmed_range()),
                    dangerous_prop,
                });
            } else if let Some(children_prop) = node.find_children_prop(model) {
                return Some(RuleState {
                    children_kind: ChildrenKind::Prop(children_prop.range()),
                    dangerous_prop,
                });
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.dangerous_prop,
            markup! {
                "Avoid passing both "<Emphasis>"children"</Emphasis>" and the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
            },
        ).detail(state.children_kind.range(), markup! {
            "This is the source of the children prop"
        }).note(
            markup! {
                "Setting HTML content will inadvertently override any passed children in React"
            }
        ))
    }
}
