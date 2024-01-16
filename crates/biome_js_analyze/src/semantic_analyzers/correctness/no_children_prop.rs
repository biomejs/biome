use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::{JsCallExpression, JsxAttribute};
use biome_rowan::{declare_node_union, AstNode, TextRange};
declare_rule! {
    /// Prevent passing of **children** as props.
    ///
    /// When using JSX, the children should be nested between the opening and closing tags.
    /// When not using JSX, the children should be passed as additional arguments to `React.createElement`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <FirstComponent children={'foo'} />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('div', { children: 'foo' });
    /// ```
    pub(crate) NoChildrenProp {
        version: "1.0.0",
        name: "noChildrenProp",
        source: RuleSource::EslintReact("no-children-prop"),
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) NoChildrenPropQuery = JsxAttribute | JsCallExpression
}

pub(crate) enum NoChildrenPropState {
    JsxProp(TextRange),
    MemberProp(TextRange),
}

impl Rule for NoChildrenProp {
    type Query = Semantic<NoChildrenPropQuery>;
    type State = NoChildrenPropState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            NoChildrenPropQuery::JsxAttribute(attribute) => {
                let name = attribute.name().ok()?;
                let name = name.as_jsx_name()?;
                if name.value_token().ok()?.text() == "children" {
                    return Some(NoChildrenPropState::JsxProp(name.range()));
                }

                None
            }
            NoChildrenPropQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                if let Some(react_create_element) =
                    ReactCreateElementCall::from_call_expression(call_expression, model)
                {
                    let children_prop = react_create_element.find_prop_by_name("children");

                    if let Some(children_prop) = children_prop {
                        return Some(NoChildrenPropState::MemberProp(
                            children_prop.name().ok()?.range(),
                        ));
                    }
                }
                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (range, footer_help) = match state {
            NoChildrenPropState::JsxProp(jsx_name_range) => {
                (
                    jsx_name_range,
                    (markup! {
                     "The canonical way to pass children in React is to use JSX elements"
                    }).to_owned()
                )
            }
            NoChildrenPropState::MemberProp(children_prop_range) => (
                children_prop_range,
                (markup! {
                     "The canonical way to pass children in React is to use additional arguments to React.createElement"
                }).to_owned()
            ),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "Avoid passing "<Emphasis>"children"</Emphasis>" using a prop"
                },
            )
            .note(footer_help),
        )
    }
}
