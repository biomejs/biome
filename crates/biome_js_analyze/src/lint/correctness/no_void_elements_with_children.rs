use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::services::semantic::Semantic;
use crate::JsRuleAction;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_lint_rule, FixKind, Rule, RuleDiagnostic, RuleSource};
use biome_console::{markup, MarkupBuf};
use biome_diagnostics::Severity;
use biome_js_factory::make::{jsx_attribute_list, jsx_self_closing_element};
use biome_js_syntax::{
    AnyJsxAttribute, JsCallExpression, JsPropertyObjectMember, JsxAttribute, JsxElement,
    JsxSelfClosingElement,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt};

declare_lint_rule! {
    /// This rules prevents void elements (AKA self-closing elements) from having children.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <br>invalid child</br>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img alt="some text" children={"some child"} />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('img', {}, 'child')
    /// ```
    pub NoVoidElementsWithChildren {
        version: "1.0.0",
        name: "noVoidElementsWithChildren",
        language: "jsx",
        sources: &[RuleSource::EslintReact("void-dom-elements-no-children")],
        recommended: true,
        severity: Severity::Error,
        fix_kind: FixKind::Unsafe,
    }
}

declare_node_union! {
    pub NoVoidElementsWithChildrenQuery = JsxElement | JsCallExpression | JsxSelfClosingElement
}

const VOID_ELEMENTS: [&str; 16] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "menuitem",
    "meta", "param", "source", "track", "wbr",
];
/// Returns true if the name of the element belong to a self-closing element
fn is_void_dom_element(element_name: &str) -> bool {
    VOID_ELEMENTS.contains(&element_name)
}

pub enum NoVoidElementsWithChildrenCause {
    /// The cause affects React using JSX code
    Jsx {
        /// If the current element has children props in style
        ///
        /// ```jsx
        /// <img>
        ///     Some child
        /// </img>
        /// ```
        children_cause: bool,
        /// If the current element has the prop `dangerouslySetInnerHTML`
        dangerous_prop_cause: Option<JsxAttribute>,
        /// If the current element has the prop `children`
        children_prop: Option<JsxAttribute>,
    },
    /// The cause affects React using `React` object APIs
    ReactCreateElement {
        /// If the current element has children props in style:
        ///
        /// ```js
        /// React.createElement('img', {}, 'child')
        /// ```
        children_cause: bool,
        /// If the current element has the prop `dangerouslySetInnerHTML`
        dangerous_prop_cause: Option<JsPropertyObjectMember>,
        /// If the current element has the prop `children`
        children_prop: Option<JsPropertyObjectMember>,
        react_create_element: ReactCreateElementCall,
    },
}

pub struct NoVoidElementsWithChildrenState {
    /// The name of the element that triggered the rule
    element_name: String,
    /// It tracks the causes that triggers the rule
    cause: NoVoidElementsWithChildrenCause,
}

impl NoVoidElementsWithChildrenState {
    fn new(element_name: impl Into<String>, cause: NoVoidElementsWithChildrenCause) -> Self {
        Self {
            element_name: element_name.into(),
            cause,
        }
    }

    fn has_children_cause(&self) -> bool {
        match &self.cause {
            NoVoidElementsWithChildrenCause::Jsx {
                children_prop,
                children_cause,
                ..
            } => *children_cause || children_prop.is_some(),
            NoVoidElementsWithChildrenCause::ReactCreateElement {
                children_prop,
                children_cause,
                ..
            } => *children_cause || children_prop.is_some(),
        }
    }

    fn has_dangerous_prop_cause(&self) -> bool {
        match &self.cause {
            NoVoidElementsWithChildrenCause::Jsx {
                dangerous_prop_cause,
                ..
            } => dangerous_prop_cause.is_some(),
            NoVoidElementsWithChildrenCause::ReactCreateElement {
                dangerous_prop_cause,
                ..
            } => dangerous_prop_cause.is_some(),
        }
    }

    fn diagnostic_message(&self) -> MarkupBuf {
        let has_children_cause = self.has_children_cause();
        let has_dangerous_cause = self.has_dangerous_prop_cause();
        match (has_children_cause, has_dangerous_cause) {
            (true, true) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have "<Emphasis>"children"</Emphasis>
                    ", or the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            }
            (true, false) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have "<Emphasis>"children"</Emphasis>"."
                }).to_owned()
            }
            (false, true) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            },
            _ => unreachable!("At least a cause must be set")

        }
    }

    fn action_message(&self) -> MarkupBuf {
        let has_children_cause = self.has_children_cause();
        let has_dangerous_cause = self.has_dangerous_prop_cause();
        match (has_children_cause, has_dangerous_cause) {
            (true, true) => {
                (markup! {
                    "Remove the "<Emphasis>"children"</Emphasis>" and the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            }
            (true, false) => {
                (markup! {
                   "Remove the "<Emphasis>"children"</Emphasis>"."
                }).to_owned()
            }
            (false, true) => {
                (markup! {
                  "Remove the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            },
            _ => unreachable!("At least a cause must be set")

        }
    }
}

impl Rule for NoVoidElementsWithChildren {
    type Query = Semantic<NoVoidElementsWithChildrenQuery>;
    type State = NoVoidElementsWithChildrenState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            NoVoidElementsWithChildrenQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let name = opening_element.name().ok()?;
                let name = name.as_jsx_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                if is_void_dom_element(name) {
                    let dangerous_prop =
                        opening_element.find_attribute_by_name("dangerouslySetInnerHTML");
                    let has_children = !element.children().is_empty();
                    let children_prop = opening_element.find_attribute_by_name("children");
                    if dangerous_prop.is_some() || has_children || children_prop.is_some() {
                        let cause = NoVoidElementsWithChildrenCause::Jsx {
                            children_prop,
                            dangerous_prop_cause: dangerous_prop,
                            children_cause: has_children,
                        };

                        return Some(NoVoidElementsWithChildrenState::new(name, cause));
                    }
                }
            }
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(element) => {
                let name = element.name().ok()?;
                let name = name.as_jsx_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                if is_void_dom_element(name) {
                    let dangerous_prop = element.find_attribute_by_name("dangerouslySetInnerHTML");
                    let children_prop = element.find_attribute_by_name("children");
                    if dangerous_prop.is_some() || children_prop.is_some() {
                        let cause = NoVoidElementsWithChildrenCause::Jsx {
                            children_prop,
                            dangerous_prop_cause: dangerous_prop,
                            children_cause: false,
                        };

                        return Some(NoVoidElementsWithChildrenState::new(name, cause));
                    }
                }
            }
            NoVoidElementsWithChildrenQuery::JsCallExpression(call_expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;
                let element_type = react_create_element
                    .element_type
                    .as_any_js_expression()?
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?;

                let element_name = element_type.inner_string_text().ok()?;
                let element_name = element_name.text();
                if is_void_dom_element(element_name) {
                    let has_children = react_create_element.children.is_some();
                    let dangerous_prop =
                        react_create_element.find_prop_by_name("dangerouslySetInnerHTML");
                    let children_prop = react_create_element.find_prop_by_name("children");

                    if dangerous_prop.is_some() || has_children || children_prop.is_some() {
                        let cause = NoVoidElementsWithChildrenCause::ReactCreateElement {
                            children_prop,
                            dangerous_prop_cause: dangerous_prop,
                            children_cause: has_children,
                            react_create_element,
                        };

                        return Some(NoVoidElementsWithChildrenState::new(element_name, cause));
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = match node {
            NoVoidElementsWithChildrenQuery::JsxElement(element) => {
                element.syntax().text_trimmed_range()
            }
            NoVoidElementsWithChildrenQuery::JsCallExpression(expression) => {
                expression.syntax().text_trimmed_range()
            }
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(element) => {
                element.syntax().text_trimmed_range()
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            state.diagnostic_message(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            NoVoidElementsWithChildrenQuery::JsxElement(element) => {
                if let NoVoidElementsWithChildrenCause::Jsx {
                    children_prop,
                    dangerous_prop_cause,
                    ..
                } = &state.cause
                {
                    let opening_element = element.opening_element().ok()?;
                    let closing_element = element.closing_element().ok()?;

                    // here we create a new list of attributes, ignoring the ones that needs to be
                    // removed
                    let new_attribute_list: Vec<_> = opening_element
                        .attributes()
                        .into_iter()
                        .filter_map(|attribute| {
                            if let AnyJsxAttribute::JsxAttribute(attribute) = &attribute {
                                if let Some(children_prop) = children_prop {
                                    if children_prop == attribute {
                                        return None;
                                    }
                                }

                                if let Some(dangerous_prop_cause) = dangerous_prop_cause {
                                    if dangerous_prop_cause == attribute {
                                        return None;
                                    }
                                }
                            }
                            Some(attribute)
                        })
                        .collect();

                    let new_attribute_list = jsx_attribute_list(new_attribute_list);

                    let new_node = jsx_self_closing_element(
                        opening_element.l_angle_token().ok()?,
                        opening_element.name().ok()?,
                        new_attribute_list,
                        closing_element.slash_token().ok()?,
                        opening_element.r_angle_token().ok()?,
                    )
                    .build();
                    mutation.replace_element(
                        element.clone().into_syntax().into(),
                        new_node.into_syntax().into(),
                    );
                }
            }
            NoVoidElementsWithChildrenQuery::JsCallExpression(_) => {
                if let NoVoidElementsWithChildrenCause::ReactCreateElement {
                    children_prop,
                    dangerous_prop_cause,
                    react_create_element,
                    children_cause,
                } = &state.cause
                {
                    if *children_cause {
                        if let Some(children) = react_create_element.children.as_ref() {
                            mutation.remove_node(children.clone());
                        }
                    }
                    if let Some(children_prop) = children_prop.as_ref() {
                        mutation.remove_node(children_prop.clone());
                    }
                    if let Some(dangerous_prop_case) = dangerous_prop_cause.as_ref() {
                        mutation.remove_node(dangerous_prop_case.clone());
                    }
                }
            }
            // self closing elements don't have inner children so we can safely remove the props
            // that we don't need
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(_) => {
                if let NoVoidElementsWithChildrenCause::Jsx {
                    children_prop,
                    dangerous_prop_cause,
                    ..
                } = &state.cause
                {
                    if let Some(children_prop) = children_prop.as_ref() {
                        mutation.remove_node(children_prop.clone());
                    }
                    if let Some(dangerous_prop_case) = dangerous_prop_cause.as_ref() {
                        mutation.remove_node(dangerous_prop_case.clone());
                    }
                }
            }
        }

        Some(JsRuleAction::new(
            ctx.metadata().action_category(ctx.category(), ctx.group()),
            ctx.metadata().applicability(),
            state.action_message(),
            mutation,
        ))
    }
}
