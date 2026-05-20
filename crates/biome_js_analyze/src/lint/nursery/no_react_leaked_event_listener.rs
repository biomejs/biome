use biome_analyze::{
    Ast, QueryMatch, Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext,
    declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, JsArrowFunctionExpression, JsCallExpression,
    JsForOfStatement, JsFunctionExpression, JsIdentifierBinding, JsObjectExpression,
    JsStaticMemberExpression, JsSyntaxKind, JsVariableDeclarator,
};
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_react_leaked_event_listener::NoReactLeakedEventListenerOptions;
use rustc_hash::FxHashMap;

use crate::{
    react::{effect_callback, is_effect_call},
    typescript::unwrap_typescript_expression,
    utils::is_node_equal,
};

declare_lint_rule! {
    /// Disallow forgetting to remove event listeners within `useEffect`.
    ///
    /// This rule detects `addEventListener` calls within `useEffect` hooks that don't have a corresponding
    /// `removeEventListener` call in the cleanup function. Forgetting to remove an event listener can lead to
    /// memory leaks and unexpected behavior when components unmount or dependencies change.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const handleClick = () => console.log("clicked");
    ///     window.addEventListener("click", handleClick);
    ///   }, []);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// import { useEffect } from "react";
    ///
    /// function MyComponent() {
    ///   useEffect(() => {
    ///     const handleClick = () => console.log("clicked");
    ///     window.addEventListener("click", handleClick);
    ///     return () => window.removeEventListener("click", handleClick);
    ///   }, []);
    /// }
    /// ```
    ///
    pub NoReactLeakedEventListener {
        version: "next",
        name: "noReactLeakedEventListener",
        language: "jsx",
        recommended: false,
        domains: &[RuleDomain::React],
        sources: &[RuleSource::EslintReactXyz("web-api-no-leaked-event-listener").same(), RuleSource::EslintReactWebApi("no-leaked-event-listener").same()],
    }
}

impl Rule for NoReactLeakedEventListener {
    type Query = Ast<JsCallExpression>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoReactLeakedEventListenerOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let query_call = ctx.query();
        if !is_effect_call(query_call) {
            return None;
        }

        let callback = effect_callback(query_call)?;
        let local_bindings = collect_bindings(&callback);
        let local_add_entries: Vec<_> = callback
            .syntax()
            .descendants()
            .filter_map(JsCallExpression::cast)
            .filter_map(|call| extract_event_listener_entry(&call, &local_bindings))
            .filter(|entry| entry.kind == EventListenerMethod::Add && !entry.has_signal)
            .collect();

        if local_add_entries.is_empty() {
            return None;
        }

        let root = query_call.syntax().ancestors().last()?;

        let mut remove_entries = Vec::new();
        let query_owner_scope = enclosing_owner_scope_range(query_call);

        for call in root.descendants().filter_map(JsCallExpression::cast) {
            if !is_effect_call(&call) {
                continue;
            }

            if enclosing_owner_scope_range(&call) != query_owner_scope {
                continue;
            }

            let Some(callback) = effect_callback(&call) else {
                continue;
            };

            let bindings = collect_bindings(&callback);
            for nested_call in callback
                .syntax()
                .descendants()
                .filter_map(JsCallExpression::cast)
            {
                let Some(entry) = extract_event_listener_entry(&nested_call, &bindings) else {
                    continue;
                };

                if entry.kind == EventListenerMethod::Remove {
                    remove_entries.push(entry);
                }
            }
        }

        local_add_entries
            .into_iter()
            .find(|add_entry| {
                !remove_entries
                    .iter()
                    .any(|remove_entry| is_inverse_entry(add_entry, remove_entry))
            })
            .map(|entry| entry.range)
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state,
            markup! {
                "An "<Emphasis>"addEventListener"</Emphasis>" in an effect should have a corresponding "<Emphasis>"removeEventListener"</Emphasis>" in cleanup."
            },
        )
        .note(markup! {
            "Not removing the listener can cause memory leaks and duplicate event handling after the component re-renders."
        })
        .note(markup! {
            "Return a cleanup function from the effect that calls "<Emphasis>"removeEventListener"</Emphasis>" with the same event type and listener."
        }))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventListenerMethod {
    Add,
    Remove,
}

#[derive(Debug, Clone)]
struct EventListenerEntry {
    kind: EventListenerMethod,
    range: TextRange,
    target: Option<AnyJsExpression>,
    event_type: AnyJsExpression,
    event_type_key: Option<EventTypeKey>,
    listener: AnyJsExpression,
    listener_is_inline_function: bool,
    capture: CaptureValue,
    has_signal: bool,
}

#[derive(Debug, Clone)]
enum EventTypeKey {
    ForEach(AnyJsExpression),
    ForOf(AnyJsExpression),
}

#[derive(Debug, Clone)]
enum CaptureValue {
    Known(bool),
    Expression(AnyJsExpression),
}

fn is_inverse_entry(add: &EventListenerEntry, remove: &EventListenerEntry) -> bool {
    if remove.kind != EventListenerMethod::Remove {
        return false;
    }

    if add.listener_is_inline_function {
        return false;
    }

    let has_same_event_type = match (&add.event_type_key, &remove.event_type_key) {
        (Some(left), Some(right)) => are_event_type_keys_equal(left, right),
        _ => are_expressions_equal(&add.event_type, &remove.event_type),
    };

    are_targets_equal(add.target.as_ref(), remove.target.as_ref())
        && has_same_event_type
        && are_expressions_equal(&add.listener, &remove.listener)
        && are_capture_values_equal(&add.capture, &remove.capture)
}

fn are_event_type_keys_equal(a: &EventTypeKey, b: &EventTypeKey) -> bool {
    match (a, b) {
        (EventTypeKey::ForEach(left), EventTypeKey::ForEach(right)) => {
            are_expressions_equal(left, right)
        }
        (EventTypeKey::ForOf(left), EventTypeKey::ForOf(right)) => {
            are_expressions_equal(left, right)
        }
        _ => false,
    }
}

fn are_targets_equal(a: Option<&AnyJsExpression>, b: Option<&AnyJsExpression>) -> bool {
    match (a, b) {
        (Some(left), Some(right)) => are_expressions_equal(left, right),
        (None, None) => true,
        _ => false,
    }
}

fn are_expressions_equal(a: &AnyJsExpression, b: &AnyJsExpression) -> bool {
    let left = unwrap_typescript_expression(a.clone().omit_parentheses());
    let right = unwrap_typescript_expression(b.clone().omit_parentheses());
    is_node_equal(left.syntax(), right.syntax())
}

fn are_capture_values_equal(a: &CaptureValue, b: &CaptureValue) -> bool {
    match (a, b) {
        (CaptureValue::Known(left), CaptureValue::Known(right)) => left == right,
        (CaptureValue::Expression(left), CaptureValue::Expression(right)) => {
            are_expressions_equal(left, right)
        }
        _ => false,
    }
}

fn extract_event_listener_entry(
    call: &JsCallExpression,
    bindings: &FxHashMap<String, AnyJsExpression>,
) -> Option<EventListenerEntry> {
    let callee = call.callee().ok()?;
    let unwrapped_callee = unwrap_typescript_expression(callee.omit_parentheses());

    let member = unwrapped_callee.as_js_static_member_expression().cloned()?;
    let member_name = member.member().ok()?.value_token().ok()?;

    let kind = match member_name.text_trimmed() {
        "addEventListener" => EventListenerMethod::Add,
        "removeEventListener" => EventListenerMethod::Remove,
        _ => return None,
    };

    if kind == EventListenerMethod::Add && is_react_native_back_handler_call(&member) {
        return None;
    }

    let Ok(target) = member.object() else {
        return None;
    };

    let [event_type, listener, options] = call.arguments().ok()?.get_arguments_by_index([0, 1, 2]);
    let event_type = event_type?.as_any_js_expression()?.clone();
    let listener = listener?.as_any_js_expression()?.clone();
    let listener_is_inline_function = is_inline_function_expression(&listener);
    let event_type_key = event_type_identifier_key(&event_type, call);

    let (capture, has_signal) = parse_options(options, bindings);

    Some(EventListenerEntry {
        kind,
        range: call.range(),
        target: Some(target),
        event_type,
        event_type_key,
        listener,
        listener_is_inline_function,
        capture,
        has_signal,
    })
}

fn is_inline_function_expression(expression: &AnyJsExpression) -> bool {
    let expression = unwrap_typescript_expression(expression.clone().omit_parentheses());
    matches!(
        expression,
        AnyJsExpression::JsArrowFunctionExpression(_) | AnyJsExpression::JsFunctionExpression(_)
    )
}

fn event_type_identifier_key(
    event_type: &AnyJsExpression,
    call: &JsCallExpression,
) -> Option<EventTypeKey> {
    let event_type = unwrap_typescript_expression(event_type.clone().omit_parentheses());
    let identifier = event_type.as_js_identifier_expression()?;
    let name_trimmed = identifier.name().ok()?.to_trimmed_text();
    let name = name_trimmed.text();

    if let Some(iterable) = for_each_iterable_key(call, name) {
        return Some(EventTypeKey::ForEach(iterable));
    }

    if let Some(iterable) = for_of_iterable_key(call, name) {
        return Some(EventTypeKey::ForOf(iterable));
    }

    None
}

fn for_each_iterable_key(
    call: &JsCallExpression,
    identifier_name: &str,
) -> Option<AnyJsExpression> {
    let function_node = call.syntax().ancestors().skip(1).find_map(|node| {
        if JsArrowFunctionExpression::can_cast(node.kind())
            || JsFunctionExpression::can_cast(node.kind())
        {
            Some(node)
        } else {
            None
        }
    })?;

    let has_parameter = function_node
        .descendants()
        .filter_map(JsIdentifierBinding::cast)
        .filter_map(|binding| binding.name_token().ok())
        .any(|token| token.text_trimmed() == identifier_name);
    if !has_parameter {
        return None;
    }

    let for_each_call = function_node
        .ancestors()
        .skip(1)
        .find_map(JsCallExpression::cast)?;
    let callee = for_each_call.callee().ok()?.omit_parentheses();
    let member = callee.as_js_static_member_expression()?;
    let method_name = member.member().ok()?.value_token().ok()?;
    if method_name.text_trimmed() != "forEach" {
        return None;
    }

    member.object().ok()
}

fn for_of_iterable_key(call: &JsCallExpression, identifier_name: &str) -> Option<AnyJsExpression> {
    let for_of = call.syntax().ancestors().find_map(JsForOfStatement::cast)?;
    let initializer = for_of.initializer().ok()?;

    let has_binding = initializer
        .syntax()
        .descendants()
        .filter_map(JsIdentifierBinding::cast)
        .filter_map(|binding| binding.name_token().ok())
        .any(|token| token.text_trimmed() == identifier_name);
    if !has_binding {
        return None;
    }

    for_of.expression().ok()
}

fn parse_options(
    options: Option<biome_js_syntax::AnyJsCallArgument>,
    bindings: &FxHashMap<String, AnyJsExpression>,
) -> (CaptureValue, bool) {
    let Some(options) = options.and_then(|arg| arg.as_any_js_expression().cloned()) else {
        return (CaptureValue::Known(false), false);
    };

    let options = unwrap_typescript_expression(options.omit_parentheses());

    if let Some(value) = resolve_boolean_expression(&options, bindings) {
        return (CaptureValue::Known(value), false);
    }

    if let Some(object) = resolve_object_expression(&options, bindings) {
        let capture = find_property_value(&object, "capture")
            .and_then(|capture_value| {
                let capture_expr = property_value_expression(&capture_value, bindings)?;
                let capture_expr = unwrap_typescript_expression(capture_expr.omit_parentheses());
                Some(
                    resolve_boolean_expression(&capture_expr, bindings)
                        .map(CaptureValue::Known)
                        .unwrap_or(CaptureValue::Expression(capture_expr)),
                )
            })
            .unwrap_or(CaptureValue::Known(false));

        let has_signal = find_property_value(&object, "signal")
            .and_then(|signal_value| property_value_expression(&signal_value, bindings))
            .is_some_and(|signal_expr| has_signal_expression(&signal_expr, bindings));

        return (capture, has_signal);
    }

    (CaptureValue::Known(false), false)
}

fn collect_bindings(callback: &AnyJsExpression) -> FxHashMap<String, AnyJsExpression> {
    let mut bindings = FxHashMap::default();

    for declarator in callback
        .syntax()
        .descendants()
        .filter_map(JsVariableDeclarator::cast)
    {
        let Ok(id) = declarator.id() else {
            continue;
        };

        let name = id.to_trimmed_string();
        if name.is_empty() {
            continue;
        }

        let Some(initializer) = declarator.initializer() else {
            continue;
        };
        let Ok(expression) = initializer.expression() else {
            continue;
        };

        bindings.insert(name, expression);
    }

    bindings
}

fn resolve_object_expression(
    expression: &AnyJsExpression,
    bindings: &FxHashMap<String, AnyJsExpression>,
) -> Option<JsObjectExpression> {
    let expression = unwrap_typescript_expression(expression.clone().omit_parentheses());

    if let Some(object) = expression.as_js_object_expression() {
        return Some(object.clone());
    }

    let identifier = expression.as_js_identifier_expression()?;
    let name = identifier
        .name()
        .ok()?
        .value_token()
        .ok()?
        .text_trimmed()
        .to_string();
    let init = bindings.get(name.as_str())?;
    resolve_object_expression(init, bindings)
}

enum PropertyValue {
    Expression(AnyJsExpression),
    ShorthandIdentifier(String),
}

fn property_value_expression(
    value: &PropertyValue,
    bindings: &FxHashMap<String, AnyJsExpression>,
) -> Option<AnyJsExpression> {
    match value {
        PropertyValue::Expression(expr) => Some(expr.clone()),
        PropertyValue::ShorthandIdentifier(name) => bindings.get(name.as_str()).cloned(),
    }
}

fn find_property_value(object: &JsObjectExpression, property_name: &str) -> Option<PropertyValue> {
    for member in object.members().into_iter().flatten() {
        match member {
            AnyJsObjectMember::JsPropertyObjectMember(property) => {
                if property.name().ok()?.name()? != property_name {
                    continue;
                }

                return property.value().ok().map(PropertyValue::Expression);
            }
            AnyJsObjectMember::JsShorthandPropertyObjectMember(property) => {
                let name = property
                    .name()
                    .ok()?
                    .value_token()
                    .ok()?
                    .text_trimmed()
                    .to_string();
                if name != property_name {
                    continue;
                }

                return Some(PropertyValue::ShorthandIdentifier(name));
            }
            _ => {}
        }
    }

    None
}

fn resolve_boolean_expression(
    expression: &AnyJsExpression,
    bindings: &FxHashMap<String, AnyJsExpression>,
) -> Option<bool> {
    let expression = unwrap_typescript_expression(expression.clone().omit_parentheses());

    if let Some(boolean_literal) = expression
        .as_any_js_literal_expression()
        .and_then(|literal| literal.as_js_boolean_literal_expression())
    {
        let token = boolean_literal.value_token().ok()?;
        return Some(token.text_trimmed() == "true");
    }

    if let Some(identifier) = expression.as_js_identifier_expression() {
        let name = identifier
            .name()
            .ok()?
            .value_token()
            .ok()?
            .text_trimmed()
            .to_string();
        let initializer = bindings.get(name.as_str())?;
        return resolve_boolean_expression(initializer, bindings);
    }

    if let Some(member) = expression.as_js_static_member_expression() {
        let object_identifier = member
            .object()
            .ok()?
            .as_js_identifier_expression()?
            .name()
            .ok()?
            .value_token()
            .ok()?;
        let object_initializer = bindings.get(object_identifier.text_trimmed())?;
        let object = resolve_object_expression(object_initializer, bindings)?;

        let property_name = member.member().ok()?.value_token().ok()?;
        let property_value = find_property_value(&object, property_name.text_trimmed())
            .and_then(|value| property_value_expression(&value, bindings))?;
        return resolve_boolean_expression(&property_value, bindings);
    }

    None
}

fn has_signal_expression(
    expression: &AnyJsExpression,
    bindings: &FxHashMap<String, AnyJsExpression>,
) -> bool {
    let expression = unwrap_typescript_expression(expression.clone().omit_parentheses());

    if expression.as_js_static_member_expression().is_some() {
        return true;
    }

    if let Some(identifier) = expression.as_js_identifier_expression() {
        let Ok(token) = identifier.name().and_then(|name| name.value_token()) else {
            return true;
        };

        let Some(initializer) = bindings.get(token.text_trimmed()) else {
            // Unresolved identifiers can be function params like `signal`.
            return true;
        };

        return has_signal_expression(initializer, bindings);
    }

    false
}

fn is_react_native_back_handler_call(member: &JsStaticMemberExpression) -> bool {
    member.object().ok().is_some_and(|object| {
        object
            .as_js_identifier_expression()
            .and_then(|id| id.name().ok())
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == "BackHandler")
    })
}

fn enclosing_owner_scope_range(call: &JsCallExpression) -> Option<TextRange> {
    call.syntax().ancestors().skip(1).find_map(|node| {
        matches!(
            node.kind(),
            JsSyntaxKind::JS_FUNCTION_DECLARATION
                | JsSyntaxKind::JS_FUNCTION_EXPRESSION
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
        )
        .then_some(node.text_range())
    })
}
