use crate::react::{is_react_call_api, ReactLibrary};
use crate::services::semantic::Semantic;
use biome_analyze::{context::RuleContext, declare_lint_rule, Rule, RuleDiagnostic, RuleDomain};
use biome_analyze::{RuleSource, RuleSourceKind};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression, AnyJsObjectMember, AnyJsxAttribute,
    AnyJsxChild, JsArrayExpression, JsCallExpression, JsFunctionBody, JsObjectExpression,
    JsxAttributeList, JsxExpressionChild, JsxTagExpression,
};
use biome_rowan::{declare_node_union, AstNode, AstNodeList, AstSeparatedList, TextRange};

declare_lint_rule! {
    /// Disallow missing key props in iterators/collection literals.
    ///
    /// Warn if an element that likely requires a key prop--namely, one present in an array literal or an arrow function expression.
    /// Check out React documentation for [explanation on the why does React need keys.](https://react.dev/learn/rendering-lists#why-does-react-need-keys)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// [<Hello />];
    /// ```
    /// ```jsx,expect_diagnostic
    /// data.map((x) => <Hello>{x}</Hello>);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// [<Hello key="first" />, <Hello key="second" />, <Hello key="third" />];
    /// data.map((x) => <Hello key={x.id}>{x}</Hello>);
    /// ```
    ///
    pub UseJsxKeyInIterable {
        version: "1.6.0",
        name: "useJsxKeyInIterable",
        language: "jsx",
        sources: &[RuleSource::EslintReact("jsx-key")],
        source_kind: RuleSourceKind::SameLogic,
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::React],
    }
}

declare_node_union! {
    pub UseJsxKeyInIterableQuery = JsArrayExpression | JsCallExpression
}

declare_node_union! {
    pub ReactComponentExpression = JsxTagExpression | JsCallExpression
}

impl Rule for UseJsxKeyInIterable {
    type Query = Semantic<UseJsxKeyInIterableQuery>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
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
        .into_boxed_slice()
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
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
}

/// Handle collections of components
///
/// Examples
///
/// ```jsx
/// [<h1></h1>, <h1></h1>]
/// ```
fn handle_collections(node: &JsArrayExpression, model: &SemanticModel) -> Vec<TextRange> {
    let is_inside_jsx = node.parent::<JsxExpressionChild>().is_some();
    node.elements()
        .iter()
        .filter_map(|node| {
            let node = node.ok()?;
            // no need to handle spread case, if the spread argument is itself a list it
            // will be handled during list declaration
            let node = AnyJsExpression::cast(node.into_syntax())?;
            handle_potential_react_component(node, model, is_inside_jsx)
        })
        .flatten()
        .collect()
}

/// Handle iterators return components
///
/// Examples
///
/// ```jsx
/// data.map(x => <h1>{x}</h1>)
/// ```
fn handle_iterators(node: &JsCallExpression, model: &SemanticModel) -> Option<Vec<TextRange>> {
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
            Some(handle_function_body(&body, model, is_inside_jsx))
        }
        AnyJsExpression::JsArrowFunctionExpression(callback) => {
            let body = callback.body().ok()?;
            match body {
                AnyJsFunctionBody::AnyJsExpression(expr) => {
                    handle_potential_react_component(expr, model, is_inside_jsx)
                }
                AnyJsFunctionBody::JsFunctionBody(body) => {
                    Some(handle_function_body(&body, model, is_inside_jsx))
                }
            }
        }
        _ => None,
    }
}

/// Inspects each statement for variable declarations and return statements to find potential React components.
fn handle_function_body(
    node: &JsFunctionBody,
    model: &SemanticModel,
    is_inside_jsx: bool,
) -> Vec<TextRange> {
    // if the return statement definitely has a key prop, don't need to check the rest of the function
    let return_statement = node
        .statements()
        .iter()
        .find_map(|statement| statement.as_js_return_statement().cloned());
    let is_return_component = return_statement
        .as_ref()
        .and_then(|ret| {
            let returned_value = ret.argument()?;
            let returned_value = unwrap_parenthesis(returned_value)?;
            Some(ReactComponentExpression::can_cast(
                returned_value.syntax().kind(),
            ))
        })
        .unwrap_or_default();
    let ranges = return_statement.and_then(|ret| {
        let returned_value = ret.argument()?;
        handle_potential_react_component(returned_value, model, is_inside_jsx)
    });
    if ranges.is_none() && is_return_component {
        return vec![];
    }

    node.statements()
        .iter()
        .filter_map(|statement| {
            if let Some(statement) = statement.as_js_variable_statement() {
                let declaration = statement.declaration().ok()?;
                Some(
                    declaration
                        .declarators()
                        .iter()
                        .filter_map(|declarator| {
                            let decl = declarator.ok()?;
                            let init = decl.initializer()?.expression().ok()?;
                            handle_potential_react_component(init, model, is_inside_jsx)
                        })
                        .flatten()
                        .collect(),
                )
            } else if let Some(statement) = statement.as_js_return_statement() {
                let returned_value = statement.argument()?;
                handle_potential_react_component(returned_value, model, is_inside_jsx)
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

fn handle_potential_react_component(
    node: AnyJsExpression,
    model: &SemanticModel,
    is_inside_jsx: bool,
) -> Option<Vec<TextRange>> {
    let node = unwrap_parenthesis(node)?;

    if let AnyJsExpression::JsConditionalExpression(node) = node {
        let consequent =
            handle_potential_react_component(node.consequent().ok()?, model, is_inside_jsx);
        let alternate =
            handle_potential_react_component(node.alternate().ok()?, model, is_inside_jsx);

        return match (consequent, alternate) {
            (Some(consequent), Some(alternate)) => Some([consequent, alternate].concat()),
            (Some(consequent), None) => Some(consequent),
            (None, Some(alternate)) => Some(alternate),
            (None, None) => None,
        };
    }

    if is_inside_jsx {
        if let Some(node) = ReactComponentExpression::cast(node.into_syntax()) {
            let range = handle_react_component(node, model)?;
            Some(range)
        } else {
            None
        }
    } else {
        let range =
            handle_react_component(ReactComponentExpression::cast(node.into_syntax())?, model)?;
        Some(range)
    }
}

fn handle_react_component(
    node: ReactComponentExpression,
    model: &SemanticModel,
) -> Option<Vec<TextRange>> {
    match node {
        ReactComponentExpression::JsxTagExpression(node) => handle_jsx_tag(&node, model),
        ReactComponentExpression::JsCallExpression(node) => {
            handle_react_non_jsx(&node, model).map(|r| vec![r])
        }
    }
}

/// Handle normal jsx tag
///
/// Examples
///
/// ```jsx
/// <Hello></Hello>
/// ```
fn handle_jsx_tag(node: &JsxTagExpression, model: &SemanticModel) -> Option<Vec<TextRange>> {
    let tag = node.tag().ok()?;
    let tag = AnyJsxChild::cast(tag.into_syntax())?;
    handle_jsx_child(&tag, model)
}

fn handle_jsx_child(node: &AnyJsxChild, model: &SemanticModel) -> Option<Vec<TextRange>> {
    let mut stack: Vec<AnyJsxChild> = vec![node.clone()];
    let mut ranges: Vec<TextRange> = vec![];

    while let Some(current) = stack.pop() {
        match current {
            AnyJsxChild::JsxElement(node) => {
                let open_node = node.opening_element().ok()?;
                if !has_key_attribute(&open_node.attributes()) {
                    ranges.push(open_node.range());
                }
            }
            AnyJsxChild::JsxSelfClosingElement(node) => {
                if !has_key_attribute(&node.attributes()) {
                    ranges.push(node.range());
                }
            }
            AnyJsxChild::JsxExpressionChild(node) => {
                let expr = node.expression()?;
                if let Some(child_ranges) = handle_potential_react_component(expr, model, true) {
                    ranges.extend(child_ranges);
                }
            }
            AnyJsxChild::JsxFragment(node) => {
                let has_any_tags = node.children().iter().any(|child| match &child {
                    AnyJsxChild::JsxElement(_) | AnyJsxChild::JsxSelfClosingElement(_) => true,
                    // HACK: don't flag the entire fragment if there's a conditional expression
                    AnyJsxChild::JsxExpressionChild(node) => node
                        .expression()
                        .is_some_and(|n| n.as_js_conditional_expression().is_some()),
                    _ => false,
                });

                if !has_any_tags {
                    ranges.push(node.range());
                    break;
                }

                stack.extend(node.children());
            }
            _ => {}
        }
    }

    if ranges.is_empty() {
        None
    } else {
        Some(ranges)
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
                name.to_trimmed_string() == "key"
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

// unwrap parenthesized expression
fn unwrap_parenthesis(expr: AnyJsExpression) -> Option<AnyJsExpression> {
    let mut inner_expr = expr;
    while let AnyJsExpression::JsParenthesizedExpression(parenthesized_expr) = inner_expr {
        inner_expr = parenthesized_expr.expression().ok()?;
    }
    Some(inner_expr)
}
