use crate::react::{is_react_call_api, ReactLibrary};
use crate::semantic_services::Semantic;
use biome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use biome_analyze::{RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression, AnyJsObjectMember, AnyJsxAttribute,
    AnyJsxTag, JsArrayExpression, JsCallExpression, JsObjectExpression, JsxAttributeList,
    JsxExpressionChild, JsxTagExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstSeparatedList, TextRange};

declare_rule! {
    /// Disallow missing key props in iterators/collection literals.
    ///
    /// Warn if an element that likely requires a key prop--namely, one present in an array literal or an arrow function expression.
    /// Check out React documentation for [explanation on the why does React need keys.](https://react.dev/learn/rendering-lists#why-does-react-need-keys)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// [<Hello />, <Hello />, <Hello />];
    /// data.map((x) => <Hello>{x}</Hello>);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// [<Hello key="first" />, <Hello key="second" />, <Hello key="third" />];
    /// data.map((x) => <Hello key={x.id}>{x}</Hello>);
    /// ```
    ///
    pub UseJsxKeyInIterable {
        version: "next",
        name: "useJsxKeyInIterable",
        source: RuleSource::EslintReact("jsx-key"),
        source_kind: RuleSourceKind::SameLogic,
        recommended: false,
    }
}

declare_node_union! {
    pub UseJsxKeyInIterableQuery = JsArrayExpression | JsCallExpression
}

declare_node_union! {
    pub ReactComponentExpression = JsxTagExpression | JsCallExpression
}

#[derive(Debug)]
pub enum UseJsxKeyInIterableState {
    MissingKeyProps(TextRange),
    CantDetermineJSXProp(TextRange),
}

impl Rule for UseJsxKeyInIterable {
    type Query = Semantic<UseJsxKeyInIterableQuery>;
    type State = UseJsxKeyInIterableState;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            UseJsxKeyInIterableQuery::JsArrayExpression(node) => handle_collections(node, model),
            UseJsxKeyInIterableQuery::JsCallExpression(node) => {
                handle_iterators(node, model).unwrap_or_default()
            }
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        match state {
            UseJsxKeyInIterableState::MissingKeyProps(state) => {
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    state,
                    markup! {
                        "Missing "<Emphasis>"key"</Emphasis>" property for this element in iterable."
                    },
                )
                .note(markup! {
                    "The order of the items may change, and having a key can help React identify which item was moved."
                }).note(markup! {
                    "Check the "<Hyperlink href="https://react.dev/learn/rendering-lists#why-does-react-need-keys">"React documentation"</Hyperlink>". "
                });
                Some(diagnostic)
            }
            UseJsxKeyInIterableState::CantDetermineJSXProp(state) => {
                let diagnostic = RuleDiagnostic::new(
                    rule_category!(),
                    state,
                    markup! {
                        "Cannot determine whether this child has the required "<Emphasis>"key"</Emphasis>" prop."
                    },
                )
                .note(markup! {
                    "Either return a JSX expression, or suppress this instance if you determine it is safe."
                }).note(markup! {
                    "Check the "<Hyperlink href="https://react.dev/learn/rendering-lists#why-does-react-need-keys">"React documentation for why a key prop is required"</Hyperlink>". "
                });
                Some(diagnostic)
            }
        }
    }
}

/// Handle collections of components
///
/// Examples
///
/// ```js
/// [<h1></h1>, <h1></h1>]
/// ```
fn handle_collections(
    node: &JsArrayExpression,
    model: &SemanticModel,
) -> Vec<UseJsxKeyInIterableState> {
    let is_inside_jsx = node.parent::<JsxExpressionChild>().is_some();
    node.elements()
        .iter()
        .filter_map(|node| {
            let node = node.ok()?;
            // no need to handle spread case, if the spread argument is itself a list it
            // will be handled during list declaration
            let node = node.as_any_js_expression()?;
            handle_potential_react_component(node, model, is_inside_jsx)
        })
        .collect()
}

/// Handle iterators return components
///
/// Examples
///
/// ```js
/// data.map(x => <h1>{x}</h1>)
/// ```
fn handle_iterators(
    node: &JsCallExpression,
    model: &SemanticModel,
) -> Option<Vec<UseJsxKeyInIterableState>> {
    let callee = node.callee().ok()?;
    let member_expression = AnyJsMemberExpression::cast(callee.into_syntax())?;
    let arguments = node.arguments().ok()?;

    if !matches!(
        member_expression.member_name()?.text(),
        "map"
            | "flatMap"
            | "from"
            | "forEach"
            | "filter"
            | "some"
            | "every"
            | "find"
            | "findIndex"
            | "reduce"
            | "reduceRight"
    ) {
        return None;
    }

    let caller_name = member_expression
        .object()
        .ok()
        .and_then(|o| o.as_js_identifier_expression()?.name().ok()?.name().ok());

    let callback_index = if caller_name.is_some_and(|name| name == "Array") {
        1
    } else {
        0
    };

    let callback_arguments = arguments.get_arguments_by_index([callback_index]);

    let callback_argument = callback_arguments
        .first()?
        .as_ref()?
        .as_any_js_expression()?;

    let is_inside_jsx = node.parent::<JsxExpressionChild>().is_some();
    match callback_argument {
        AnyJsExpression::JsFunctionExpression(callback) => {
            let body = callback.body().ok()?;
            let res = body
                .statements()
                .into_iter()
                .filter_map(|statement| {
                    let statement = statement.as_js_return_statement()?;
                    let returned_value = statement.argument()?;
                    handle_potential_react_component(&returned_value, model, is_inside_jsx)
                })
                .collect::<Vec<_>>();

            Some(res)
        }
        AnyJsExpression::JsArrowFunctionExpression(callback) => {
            let body = callback.body().ok()?;
            match body {
                AnyJsFunctionBody::AnyJsExpression(expr) => {
                    handle_potential_react_component(&expr, model, is_inside_jsx)
                        .map(|state| vec![state])
                }
                AnyJsFunctionBody::JsFunctionBody(body) => {
                    let res = body
                        .statements()
                        .into_iter()
                        .filter_map(|statement| {
                            let statement = statement.as_js_return_statement()?;
                            let returned_value = statement.argument()?;
                            handle_potential_react_component(&returned_value, model, is_inside_jsx)
                        })
                        .collect::<Vec<_>>();
                    Some(res)
                }
            }
        }
        _ => None,
    }
}

fn handle_potential_react_component(
    node: &AnyJsExpression,
    model: &SemanticModel,
    is_inside_jsx: bool,
) -> Option<UseJsxKeyInIterableState> {
    if is_inside_jsx {
        if let Some(node) = ReactComponentExpression::cast_ref(node.syntax()) {
            let range = handle_react_component(node, model)?;
            Some(UseJsxKeyInIterableState::MissingKeyProps(range))
        } else {
            Some(UseJsxKeyInIterableState::CantDetermineJSXProp(node.range()))
        }
    } else {
        let range =
            handle_react_component(ReactComponentExpression::cast_ref(node.syntax())?, model)?;
        Some(UseJsxKeyInIterableState::MissingKeyProps(range))
    }
}

fn handle_react_component(
    node: ReactComponentExpression,
    model: &SemanticModel,
) -> Option<TextRange> {
    match node {
        ReactComponentExpression::JsxTagExpression(node) => handle_jsx_tag(&node),
        ReactComponentExpression::JsCallExpression(node) => handle_react_non_jsx(&node, model),
    }
}

/// Handle normal jsx tag
///
/// Examples
///
/// ```js
/// <Hello></Hello>
/// ```
fn handle_jsx_tag(node: &JsxTagExpression) -> Option<TextRange> {
    let tag = node.tag().ok()?;
    match tag {
        AnyJsxTag::JsxElement(node) => {
            let open_node = node.opening_element().ok()?;
            if !has_key_attribute(&open_node.attributes()) {
                Some(open_node.range())
            } else {
                None
            }
        }
        AnyJsxTag::JsxSelfClosingElement(node) => {
            if !has_key_attribute(&node.attributes()) {
                Some(node.range())
            } else {
                None
            }
        }
        AnyJsxTag::JsxFragment(node) => Some(node.range()),
    }
}

// Handle components without JSX
//
// Examples
//
// ```js
// React.createElement("h1", {...})
// ```
fn handle_react_non_jsx(node: &JsCallExpression, model: &SemanticModel) -> Option<TextRange> {
    let callee = node.callee().ok()?;
    let arguments = node.arguments().ok()?;
    if !is_react_call_api(&callee, model, ReactLibrary::React, "cloneElement")
        && !is_react_call_api(&callee, model, ReactLibrary::React, "createElement")
    {
        return None;
    }
    let prop_arguments = arguments.get_arguments_by_index([1]);
    let prop_argument = prop_arguments.first();
    let Some(prop_argument) = prop_argument else {
        return Some(arguments.range());
    };
    let Some(prop_argument) = prop_argument.as_ref() else {
        return Some(arguments.range());
    };
    let prop_argument = prop_argument.as_any_js_expression()?;
    let props = prop_argument.as_js_object_expression()?;
    if has_key_prop(props) {
        return Some(prop_argument.range());
    }
    None
}

fn has_key_attribute(attributes: &JsxAttributeList) -> bool {
    attributes.into_iter().any(|attr| {
        // key must be statically provided, so no spread
        if let AnyJsxAttribute::JsxAttribute(attr) = attr {
            if let Ok(name) = attr.name() {
                name.text() == "key"
            } else {
                false
            }
        } else {
            false
        }
    })
}

fn has_key_prop(props: &JsObjectExpression) -> bool {
    props.members().into_iter().any(|prop| {
        let Ok(prop) = prop else { return false };
        // key must be statically provided, so no spread
        match prop {
            AnyJsObjectMember::JsPropertyObjectMember(prop) => {
                let Ok(name) = prop.name() else { return false };
                let Some(name) = name.name() else {
                    return false;
                };
                name == "text"
            }
            AnyJsObjectMember::JsShorthandPropertyObjectMember(prop) => {
                let Ok(name) = prop.name() else { return false };
                let Ok(name) = name.name() else { return false };
                name == "text"
            }
            _ => false,
        }
    })
}
