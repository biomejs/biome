use crate::{ast_utils::is_definitely_not_dom_node, services::semantic::Semantic};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsAssignment, AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression,
    AnyJsObjectAssignmentPatternMember, AnyJsObjectBindingPatternMember, AnyJsObjectMemberName,
    JsAssignmentExpression, JsAssignmentOperator, JsCallExpression, JsIdentifierAssignment,
    JsIdentifierExpression, JsLanguage, JsObjectAssignmentPattern, JsObjectBindingPattern,
    JsUnaryExpression, JsUnaryOperator, JsVariableDeclarator,
    assign_ext::AnyJsMemberAssignment,
    binding_ext::AnyJsBindingDeclaration, is_transparent_expression_wrapper,
    static_value::StaticValue,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, WalkEvent};
use biome_rule_options::use_observer_api::UseObserverApiOptions;

declare_lint_rule! {
    /// Reports `resize` and `scroll` listeners that read layout.
    ///
    /// `resize` and `scroll` events can fire frequently. Reading layout values in their
    /// listeners can make the browser recalculate layout for each event. `ResizeObserver`
    /// and `IntersectionObserver` report geometry changes without synchronously reading
    /// layout in an event listener.
    ///
    /// The rule checks inline listener functions, local function declarations, and local
    /// `const` function listeners. It ignores opaque listener references and layout reads
    /// inside functions nested within the listener. For element layout APIs, it excludes
    /// receivers whose syntax cannot produce a DOM node and treats unresolved expressions as DOM-like.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// window.addEventListener("resize", () => {
    ///     element.classList.toggle("is-small", element.offsetWidth < 500);
    /// });
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// window.addEventListener("scroll", () => {
    ///     const top = element.getBoundingClientRect().top;
    ///     element.classList.toggle("is-visible", top < window.innerHeight);
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// new ResizeObserver((entries) => {
    ///     for (const entry of entries) {
    ///         entry.target.classList.toggle("is-small", entry.contentRect.width < 500);
    ///     }
    /// }).observe(element);
    /// ```
    ///
    /// ```js
    /// new IntersectionObserver((entries) => {
    ///     for (const entry of entries) {
    ///         entry.target.classList.toggle("is-visible", entry.isIntersecting);
    ///     }
    /// }).observe(element);
    /// ```
    ///
    /// Plain scroll-position listeners are allowed because an observer API is not always
    /// a suitable replacement.
    ///
    /// ```js
    /// window.addEventListener("scroll", () => {
    ///     updateScrollPosition(window.scrollY);
    /// });
    /// ```
    ///
    /// ## Limitations
    ///
    /// This rule currently only uses syntax-based heuristics, and may trigger false positives in codebases that use `addEventListener` in their APIs frequently.
    pub UseObserverApi {
        version: "next",
        name: "useObserverApi",
        language: "js",
        sources: &[RuleSource::EslintUnicorn("prefer-observer-apis").inspired()],
        recommended: false,
        issue_number: Some("TODO"),
    }
}

/// The observer API recommended for the matched event and layout read.
#[derive(Clone, Copy)]
pub enum ObserverKind {
    Resize,
    Intersection,
}

/// Data required to emit a diagnostic for a matched listener.
pub struct UseObserverApiState {
    kind: ObserverKind,
    event_range: TextRange,
}

impl Rule for UseObserverApi {
    type Query = Semantic<JsCallExpression>;
    type State = UseObserverApiState;
    type Signals = Option<Self::State>;
    type Options = UseObserverApiOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        let model = ctx.model();

        if !is_add_event_listener_call(call, model) {
            return None;
        }

        let arguments = call.arguments().ok()?.args();
        if !(2..=3).contains(&arguments.len()) {
            return None;
        }

        let event_argument = arguments.first()?.ok()?.as_any_js_expression()?.clone();
        let kind = event_kind(&event_argument, model)?;
        let listener_argument = arguments
            .iter()
            .nth(1)?
            .ok()?
            .as_any_js_expression()?
            .clone();
        let listener_body = listener_body(&listener_argument, model)?;

        contains_layout_read(&listener_body, model).then_some(UseObserverApiState {
            kind,
            event_range: event_argument.range(),
        })
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (observer, event, replacement) = match state.kind {
            ObserverKind::Resize => ("ResizeObserver", "resize", "element size changes"),
            ObserverKind::Intersection => (
                "IntersectionObserver",
                "scroll",
                "element visibility changes",
            ),
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.event_range,
                markup! {
                    "This "<Emphasis>{event}</Emphasis>" listener reads layout."
                },
            )
            .note(markup! {
                "Reading layout in a frequently fired event listener can make the browser recalculate layout for each event."
            })
            .note(markup! {
                "Use "<Emphasis>{observer}</Emphasis>" to react to "{replacement}"."
            }),
        )
    }
}

const ELEMENT_LAYOUT_PROPERTIES: &[&str] = &[
    "clientHeight",
    "clientLeft",
    "clientTop",
    "clientWidth",
    "offsetHeight",
    "offsetLeft",
    "offsetParent",
    "offsetTop",
    "offsetWidth",
    "scrollHeight",
    "scrollWidth",
];
const LAYOUT_METHODS: &[&str] = &["getBoundingClientRect", "getClientRects"];
const VIEWPORT_PROPERTIES: &[&str] = &["innerHeight", "innerWidth"];
const VISUAL_VIEWPORT_PROPERTIES: &[&str] = &["height", "width"];

/// Returns whether the call targets an unshadowed global listener API or a receiver that may be a DOM object.
fn is_add_event_listener_call(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> bool {
    // Unfortunately, the query to figure out if something is a DOM element that can properly receive these `addEventListener`
    // calls are a bit brittle. The source rule uses type information to determine if something is a DOM element.
    //
    // Double unfortunately, we don't have all the types for the browser built in yet. So using our type inference for it requires more work. useDomQuerySelector is in a similar situation, but it can reasonably get away with not using type info.
    //
    // For now, this implementation uses more naive detection heuristics that leans slightly more towards the "false positive" side.
    if call.optional_chain_token().is_some() {
        return false;
    }

    let Some(callee) = call
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
    else {
        return false;
    };

    if let Some(identifier) = callee.as_js_identifier_expression() {
        return identifier.name().ok().is_some_and(|name| {
            name.has_name("addEventListener") && model.binding(&name).is_none()
        });
    }

    let Some(member) = AnyJsMemberExpression::cast(callee.into_syntax()) else {
        return false;
    };

    !member.is_optional_chain()
        && member_name(&member).is_some_and(|name| name.text() == "addEventListener")
        && member
            .object()
            .is_ok_and(|object| !is_definitely_not_dom_node(&object))
}

/// Classifies event names known to be exactly `resize` or `scroll`.
fn event_kind(expression: &AnyJsExpression, model: &SemanticModel) -> Option<ObserverKind> {
    let expression = expression.inner_expression()?;
    if let Some(value) = expression.as_static_value() {
        return match value.text() {
            "resize" => Some(ObserverKind::Resize),
            "scroll" => Some(ObserverKind::Intersection),
            _ => None,
        };
    }

    if let Some(identifier) = expression.as_js_identifier_expression()
        && let Some(binding) = model.binding(&identifier.name().ok()?)
        && let Some(AnyJsBindingDeclaration::JsVariableDeclarator(declarator)) =
            binding.tree().declaration()
        && declarator.declaration()?.is_const()
        && let Some(value) = declarator
            .initializer()
            .and_then(|initializer| initializer.expression().ok())
            .and_then(|initializer| initializer.inner_expression())
            .and_then(|initializer| initializer.as_static_value())
    {
        return match value.text() {
            "resize" => Some(ObserverKind::Resize),
            "scroll" => Some(ObserverKind::Intersection),
            _ => None,
        };
    }

    None
}

/// Resolves an inline or immutable local listener to its function body.
fn listener_body(expression: &AnyJsExpression, model: &SemanticModel) -> Option<AnyJsFunctionBody> {
    let expression = expression.inner_expression()?;
    match expression {
        AnyJsExpression::JsArrowFunctionExpression(function) => function.body().ok(),
        AnyJsExpression::JsFunctionExpression(function) => {
            Some(function.body().ok()?.into())
        }
        AnyJsExpression::JsIdentifierExpression(identifier) => {
            let binding = model.binding(&identifier.name().ok()?)?;
            match binding.tree().declaration()? {
                AnyJsBindingDeclaration::JsFunctionDeclaration(function) => {
                    if binding
                        .all_writes()
                        .any(|reference| reference.range_start() < identifier.range().start())
                    {
                        return None;
                    }
                    Some(function.body().ok()?.into())
                }
                AnyJsBindingDeclaration::JsVariableDeclarator(declarator) => {
                    if !declarator.declaration()?.is_const() {
                        return None;
                    }
                    let initializer = declarator.initializer()?.expression().ok()?;
                    let initializer = initializer.inner_expression()?;
                    match initializer {
                        AnyJsExpression::JsArrowFunctionExpression(function) => {
                            function.body().ok()
                        }
                        AnyJsExpression::JsFunctionExpression(function) => {
                            Some(function.body().ok()?.into())
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Searches a listener body for layout reads while skipping nested functions.
fn contains_layout_read(
    root: &AnyJsFunctionBody,
    model: &SemanticModel,
) -> bool {
    if matches!(
        root,
        AnyJsFunctionBody::AnyJsExpression(expression)
            if AnyFunctionLike::cast_ref(expression.syntax()).is_some()
    ) {
        return false;
    }

    let root = root.syntax();
    let mut preorder = root.preorder();

    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };

        if node != *root && AnyFunctionLike::cast_ref(&node).is_some() {
            preorder.skip_subtree();
            continue;
        }

        if JsCallExpression::cast(node.clone())
            .is_some_and(|call| is_layout_method_call(&call, model))
            || AnyJsMemberExpression::cast(node.clone())
                .is_some_and(|member| is_layout_member_expression_read(&member, model))
            || AnyJsMemberAssignment::cast(node.clone())
                .is_some_and(|member| is_layout_member_assignment_read(&member, model))
            || JsIdentifierExpression::cast(node.clone())
                .is_some_and(|identifier| is_global_viewport_identifier(&identifier, model))
            || JsIdentifierAssignment::cast(node.clone())
                .is_some_and(|identifier| is_global_viewport_assignment_read(&identifier, model))
            || JsVariableDeclarator::cast(node.clone()).is_some_and(|declarator| {
                is_layout_variable_destructuring_read(&declarator, model)
            })
            || JsAssignmentExpression::cast(node).is_some_and(|assignment| {
                is_layout_assignment_destructuring_read(&assignment, model)
            })
        {
            return true;
        }
    }

    false
}

/// Returns whether a call invokes a layout-reading method on a possible DOM element.
fn is_layout_method_call(
    call: &JsCallExpression,
    model: &SemanticModel,
) -> bool {
    let Some(callee) = call
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
    else {
        return false;
    };
    let Some(member) = AnyJsMemberExpression::cast(callee.into_syntax()) else {
        return false;
    };

    member_name(&member).is_some_and(|name| LAYOUT_METHODS.contains(&name.text()))
        && member.object().is_ok_and(|object| {
            !is_non_element_layout_object(&object, model)
                && !is_definitely_not_dom_node(&object)
        })
}

/// Returns whether a member expression reads a recognized layout value.
fn is_layout_member_expression_read(
    member: &AnyJsMemberExpression,
    model: &SemanticModel,
) -> bool {
    if is_deleted_expression(member) {
        return false;
    }
    member_name(member).is_some_and(|name| {
        member
            .object()
            .is_ok_and(|object| is_layout_member(&object, name.text(), model))
    })
}

/// Returns whether a member assignment reads a layout value as part of an update or compound assignment.
fn is_layout_member_assignment_read(
    member: &AnyJsMemberAssignment,
    model: &SemanticModel,
) -> bool {
    !is_simple_assignment(member) && assignment_member_is_layout(member, model)
}

/// Returns whether an assignment reads an unshadowed global viewport value.
fn is_global_viewport_assignment_read(
    identifier: &JsIdentifierAssignment,
    model: &SemanticModel,
) -> bool {
    if is_simple_assignment(identifier) {
        return false;
    }
    let Ok(name) = identifier.name_token() else {
        return false;
    };
    VIEWPORT_PROPERTIES.contains(&name.text_trimmed()) && model.binding(identifier).is_none()
}

/// Classifies a member name as a layout read using syntax and known global identities.
fn is_layout_member(
    object: &AnyJsExpression,
    name: &str,
    model: &SemanticModel,
) -> bool {
    (VIEWPORT_PROPERTIES.contains(&name) && is_global_object(object, model))
        || (VISUAL_VIEWPORT_PROPERTIES.contains(&name)
            && is_global_visual_viewport(object, model))
        || (ELEMENT_LAYOUT_PROPERTIES.binary_search(&name).is_ok()
            && !is_non_element_layout_object(object, model)
            && !is_definitely_not_dom_node(object))
}

/// Returns whether an assignment member names a recognized layout value.
fn assignment_member_is_layout(
    member: &AnyJsMemberAssignment,
    model: &SemanticModel,
) -> bool {
    let Ok(object) = member.object() else {
        return false;
    };
    match member {
        AnyJsMemberAssignment::JsStaticMemberAssignment(member) => member
            .member()
            .ok()
            .and_then(|member| member.as_js_name().cloned())
            .and_then(|name| name.value_token().ok())
            .is_some_and(|name| is_layout_member(&object, name.text_trimmed(), model)),
        AnyJsMemberAssignment::JsComputedMemberAssignment(member) => member
            .member()
            .ok()
            .and_then(|member| member.inner_expression())
            .and_then(|member| member.as_static_value())
            .is_some_and(|name| is_layout_member(&object, name.text(), model)),
    }
}

/// Extracts a static property name from dot or computed member syntax.
fn member_name(member: &AnyJsMemberExpression) -> Option<StaticValue> {
    match member {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => Some(StaticValue::String(
            member.member().ok()?.as_js_name()?.value_token().ok()?,
        )),
        AnyJsMemberExpression::JsComputedMemberExpression(member) => {
            member.member().ok()?.inner_expression()?.as_static_value()
        }
    }
}

/// Returns whether the node belongs to the left side of a plain assignment through assignment wrappers.
fn is_simple_assignment(node: &impl AstNode<Language = JsLanguage>) -> bool {
    let mut parent = node.syntax().parent();
    while let Some(node) = parent {
        if let Some(assignment) = JsAssignmentExpression::cast(node.clone()) {
            return assignment.operator().ok() == Some(JsAssignmentOperator::Assign);
        }
        if !AnyJsAssignment::cast(node.clone()).is_some_and(|assignment| {
            matches!(
                assignment,
                AnyJsAssignment::JsParenthesizedAssignment(_)
                    | AnyJsAssignment::TsAsAssignment(_)
                    | AnyJsAssignment::TsNonNullAssertionAssignment(_)
                    | AnyJsAssignment::TsSatisfiesAssignment(_)
                    | AnyJsAssignment::TsTypeAssertionAssignment(_)
            )
        }) {
            return false;
        }
        parent = node.parent();
    }

    false
}

/// Returns whether the member is the operand of `delete` through expression wrappers.
fn is_deleted_expression(member: &AnyJsMemberExpression) -> bool {
    member
        .syntax()
        .ancestors()
        .skip(1)
        .find(|ancestor| !is_transparent_expression_wrapper(ancestor))
        .and_then(JsUnaryExpression::cast)
        .and_then(|unary| unary.operator().ok())
        == Some(JsUnaryOperator::Delete)
}

/// Returns whether an identifier reads an unshadowed global viewport dimension.
fn is_global_viewport_identifier(
    identifier: &JsIdentifierExpression,
    model: &SemanticModel,
) -> bool {
    let Ok(name) = identifier.name() else {
        return false;
    };
    let Ok(token) = name.value_token() else {
        return false;
    };
    VIEWPORT_PROPERTIES.contains(&token.text_trimmed()) && model.binding(&name).is_none()
}

/// Returns whether a variable declaration destructures a recognized layout value from its initializer.
fn is_layout_variable_destructuring_read(
    declarator: &JsVariableDeclarator,
    model: &SemanticModel,
) -> bool {
    let Some(pattern) = declarator
        .id()
        .ok()
        .and_then(|pattern| pattern.as_js_object_binding_pattern().cloned())
    else {
        return false;
    };
    let Some(source) = declarator
        .initializer()
        .and_then(|initializer| initializer.expression().ok())
    else {
        return false;
    };
    binding_pattern_has_layout_read(&pattern, &source, model)
}

/// Returns whether an assignment pattern destructures a recognized layout value from its source.
fn is_layout_assignment_destructuring_read(
    assignment: &JsAssignmentExpression,
    model: &SemanticModel,
) -> bool {
    let Some(pattern) = assignment
        .left()
        .ok()
        .and_then(|pattern| pattern.as_js_object_assignment_pattern().cloned())
    else {
        return false;
    };
    let Ok(source) = assignment.right() else {
        return false;
    };

    assignment_pattern_has_layout_read(&pattern, &source, model)
}

/// Checks an object binding pattern for properties that read layout from the source expression.
fn binding_pattern_has_layout_read(
    pattern: &JsObjectBindingPattern,
    source: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    pattern.properties().iter().any(|property| {
        let Ok(property) = property else {
            return false;
        };
        match property {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => property
                .member()
                .is_ok_and(|member| object_member_is_layout(&member, source, model)),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                property
                    .identifier()
                    .ok()
                    .and_then(|identifier| identifier.as_js_identifier_binding().cloned())
                    .and_then(|identifier| identifier.name_token().ok())
                    .map(|token| token.token_text_trimmed())
                    .is_some_and(|name| is_layout_member(source, name.text(), model))
            }
            _ => false,
        }
    })
}

/// Checks an object assignment pattern for properties that read layout from the source expression.
fn assignment_pattern_has_layout_read(
    pattern: &JsObjectAssignmentPattern,
    source: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    pattern.properties().iter().any(|property| {
        let Ok(property) = property else {
            return false;
        };
        match property {
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
                property
                    .member()
                    .is_ok_and(|member| object_member_is_layout(&member, source, model))
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                property,
            ) => property
                .identifier()
                .ok()
                .and_then(|identifier| identifier.name_token().ok())
                .map(|token| token.token_text_trimmed())
                .is_some_and(|name| is_layout_member(source, name.text(), model)),
            _ => false,
        }
    })
}

/// Returns whether an object pattern member names a layout value on its source expression.
fn object_member_is_layout(
    member: &AnyJsObjectMemberName,
    source: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    match member {
        AnyJsObjectMemberName::JsComputedMemberName(member) => member
            .expression()
            .ok()
            .and_then(|expression| expression.inner_expression())
            .and_then(|expression| expression.as_static_value())
            .is_some_and(|name| is_layout_member(source, name.text(), model)),
        _ => member
            .name()
            .is_some_and(|name| is_layout_member(source, name.text(), model)),
    }
}

/// Returns whether a known global receiver cannot provide element layout properties or methods.
fn is_non_element_layout_object(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    is_global_object(expression, model)
        || is_global_document(expression, model)
        || is_global_visual_viewport(expression, model)
}

/// Returns whether an expression references an unshadowed global object alias.
fn is_global_object(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    is_global_identifier(expression, model, &["globalThis", "self", "window"])
}

/// Returns whether an expression references the unshadowed global document.
fn is_global_document(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    is_global_identifier(expression, model, &["document"])
        || is_global_object_member(expression, model, "document")
}

/// Returns whether an expression references the unshadowed global visual viewport.
fn is_global_visual_viewport(expression: &AnyJsExpression, model: &SemanticModel) -> bool {
    is_global_identifier(expression, model, &["visualViewport"])
        || is_global_object_member(expression, model, "visualViewport")
}

/// Matches an unshadowed identifier against a fixed set of global names.
fn is_global_identifier(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    names: &[&str],
) -> bool {
    let Some(identifier) = expression
        .inner_expression()
        .and_then(|expression| expression.as_js_identifier_expression().cloned())
    else {
        return false;
    };
    let Ok(name) = identifier.name() else {
        return false;
    };
    let Ok(token) = name.value_token() else {
        return false;
    };
    names.contains(&token.text_trimmed()) && model.binding(&name).is_none()
}

/// Matches a property accessed through an unshadowed global object name, such as
/// `window.document` or `globalThis.visualViewport`.
fn is_global_object_member(
    expression: &AnyJsExpression,
    model: &SemanticModel,
    expected_member: &str,
) -> bool {
    let Some(expression) = expression.inner_expression() else {
        return false;
    };
    let Some(member) = AnyJsMemberExpression::cast(expression.into_syntax()) else {
        return false;
    };
    member_name(&member).is_some_and(|name| name.text() == expected_member)
        && member
            .object()
            .is_ok_and(|object| is_global_object(&object, model))
}

#[cfg(test)]
mod tests {
    use super::ELEMENT_LAYOUT_PROPERTIES;

    #[test]
    fn element_layout_properties_are_sorted() {
        assert!(ELEMENT_LAYOUT_PROPERTIES.is_sorted());
    }
}
