use crate::frameworks::vue::vue_call::is_vue_api_reference;
use crate::frameworks::vue::vue_component::{
    AnyPotentialVueComponent, AnyVueComponent, VueComponent, VueComponentQuery,
    VueOptionsApiBasedComponent,
};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsBinding, AnyJsBindingPattern,
    AnyJsExpression, AnyJsFormalParameter, AnyJsObjectBindingPatternMember, AnyJsObjectMember,
    AnyJsObjectMemberName, AnyJsParameter, AnyTsType, AnyTsTypeMember, JsCallExpression,
    JsIdentifierBinding, JsIdentifierExpression, JsParameters, JsStaticMemberExpression,
    JsVariableDeclarator, TsIdentifierBinding, TsInterfaceDeclaration, TsTypeAliasDeclaration,
    TsTypeMemberList,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, AstSeparatedList, TextRange, WalkEvent};
use biome_rule_options::no_vue_shadow_native_events::NoVueShadowNativeEventsOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow Vue component events named after built-in browser events.
    ///
    /// A component can send its own events to its parent with `emit`. When such an event has the
    /// same name as a browser event, like `click` or `keydown`, it looks like the browser event but
    /// works differently:
    ///
    /// - The listener receives whatever value the component passes to `emit`, instead of the
    ///   browser's `Event` object.
    /// - The event only reaches the parent that listens to the component. Most browser events also
    ///   reach every element that contains the element where they happened.
    /// - Modifiers that act on the browser's `Event` object, such as `@click.stop` and
    ///   `@click.prevent`, do nothing unless the component passes that object along.
    ///
    /// Give the event a name that describes what happened in the component, such as `save` or
    /// `update:modelValue`.
    ///
    /// This rule checks the event names listed in the `emits` option and in `defineEmits()`. It
    /// also checks the event names passed to `this.$emit()`, to the `emit` function that `setup()`
    /// receives, to the function returned by `defineEmits()`, and to `$emit()` in templates.
    /// Uppercase and lowercase letters are treated the same, so `Click` is reported too.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineEmits(["click"]);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineEmits<{ keydown: [key: string] }>();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   methods: {
    ///     onClick() {
    ///       this.$emit("change", 42);
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// const emit = defineEmits(["save"]);
    /// emit("save");
    /// </script>
    /// ```
    ///
    pub NoVueShadowNativeEvents {
        version: "next",
        name: "noVueShadowNativeEvents",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-shadow-native-events").inspired()],
    }
}

/// A component event name that matches a native DOM event.
pub struct ShadowedNativeEvent {
    /// The range of the event name, either in an emits declaration or as the first argument of
    /// an emit call.
    range: TextRange,
    native_event: &'static str,
}

impl Rule for NoVueShadowNativeEvents {
    type Query = VueComponentQuery;
    type State = ShadowedNativeEvent;
    type Signals = Box<[Self::State]>;
    type Options = NoVueShadowNativeEventsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let source = ctx.source_type::<JsFileSource>();
        let mut result = Vec::new();

        if source.as_embedding_kind().is_vue() && !source.is_embedded_source() {
            // Template expressions, such as `@click="$emit('click')"`.
            if let AnyPotentialVueComponent::JsCallExpression(call) = ctx.query()
                && is_template_emit_call(call, model)
            {
                check_emit_call(call, &mut result);
            }
            return result.into_boxed_slice();
        }

        let Some(component) =
            VueComponent::from_potential_component(ctx.query(), model, source, ctx.file_path())
        else {
            return result.into_boxed_slice();
        };

        match component.kind() {
            AnyVueComponent::Setup(setup) => {
                for define_emits in setup.define_emits_calls() {
                    check_define_emits_call(define_emits.call(), model, &mut result);
                    if let Some(binding) = define_emits.binding() {
                        for call in calls_through_binding(binding, model) {
                            check_emit_call(&call, &mut result);
                        }
                    }
                }
            }
            AnyVueComponent::OptionsApi(component) => {
                check_options_based_component(component, model, &mut result);
            }
            AnyVueComponent::CreateApp(component) => {
                check_options_based_component(component, model, &mut result);
            }
            AnyVueComponent::DefineComponent(component) => {
                check_options_based_component(component, model, &mut result);
            }
        }

        result.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let native_event = state.native_event;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This component event has the same name as the browser event "<Emphasis>{native_event}</Emphasis>"."
                },
            )
            .note(markup! {
                "It is easy to mistake for the browser event, but it works differently: listeners receive the value passed to "<Emphasis>"emit"</Emphasis>" instead of an "<Emphasis>"Event"</Emphasis>" object, the event doesn't reach the elements around the component, and modifiers such as "<Emphasis>".stop"</Emphasis>" don't work on it."
            })
            .note(markup! {
                "Rename the event to describe what happened in the component."
            }),
        )
    }
}

/// Native DOM event names, sorted case-insensitively for binary search.
const NATIVE_EVENTS: &[&str] = &[
    "abort",
    "animationcancel",
    "animationend",
    "animationiteration",
    "animationstart",
    "auxclick",
    "beforeinput",
    "beforetoggle",
    "blur",
    "canplay",
    "canplaythrough",
    "change",
    "click",
    "compositionend",
    "compositionstart",
    "compositionupdate",
    "contextmenu",
    "copy",
    "cut",
    "dblclick",
    "drag",
    "dragend",
    "dragenter",
    "dragexit",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    "durationchange",
    "emptied",
    "encrypted",
    "ended",
    "error",
    "focus",
    "focusin",
    "focusout",
    "formdata",
    "fullscreenchange",
    "fullscreenerror",
    "gotpointercapture",
    "input",
    "invalid",
    "keydown",
    "keypress",
    "keyup",
    "load",
    "loadeddata",
    "loadedmetadata",
    "loadstart",
    "lostpointercapture",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "paste",
    "pause",
    "play",
    "playing",
    "pointercancel",
    "pointerdown",
    "pointerenter",
    "pointerleave",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "progress",
    "ratechange",
    "reset",
    "scroll",
    "scrollend",
    "securitypolicyviolation",
    "seeked",
    "seeking",
    "select",
    "stalled",
    "submit",
    "suspend",
    "timeupdate",
    "toggle",
    "touchcancel",
    "touchend",
    "touchmove",
    "touchstart",
    "transitioncancel",
    "transitionend",
    "transitionrun",
    "transitionstart",
    "volumechange",
    "waiting",
    "wheel",
];

fn find_native_event(name: &str) -> Option<&'static str> {
    NATIVE_EVENTS
        .binary_search_by(|probe| probe.cmp_ignore_ascii_case(name))
        .ok()
        .map(|index| NATIVE_EVENTS[index])
}

fn push_if_native(name: &str, range: TextRange, result: &mut Vec<ShadowedNativeEvent>) {
    if let Some(native_event) = find_native_event(name) {
        result.push(ShadowedNativeEvent {
            range,
            native_event,
        });
    }
}

/// Checks the event name passed as the first argument of an emit call.
fn check_emit_call(call: &JsCallExpression, result: &mut Vec<ShadowedNativeEvent>) {
    let Some(Ok(argument)) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().first())
    else {
        return;
    };
    let Some(argument) = argument.as_any_js_expression() else {
        return;
    };
    if let Some(value) = argument.as_static_value()
        && let Some(name) = value.as_string_constant()
    {
        push_if_native(name, argument.range(), result);
    }
}

/// Checks whether `call` calls the global `$emit` that Vue exposes to templates.
fn is_template_emit_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    call.callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
        .and_then(|callee| callee.as_js_reference_identifier())
        .is_some_and(|reference| {
            reference.name().is_ok_and(|name| name == "$emit")
                && model.binding(&reference).is_none()
        })
}

fn check_define_emits_call(
    call: &JsCallExpression,
    model: &SemanticModel,
    result: &mut Vec<ShadowedNativeEvent>,
) {
    if let Some(Ok(argument)) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().first())
        && let Some(expression) = argument.as_any_js_expression()
    {
        check_runtime_emits(expression, result);
    }
    if let Some(type_arguments) = call.type_arguments()
        && let Some(Ok(emits_type)) = type_arguments.ts_type_argument_list().first()
    {
        check_type_emits(&emits_type, model, true, result);
    }
}

/// Checks the value of the `emits` option or the argument of `defineEmits()`, as in
/// `['change']` or `{ change: null }`.
fn check_runtime_emits(expression: &AnyJsExpression, result: &mut Vec<ShadowedNativeEvent>) {
    match expression.inner_expression() {
        Some(AnyJsExpression::JsArrayExpression(array)) => {
            for element in array.elements().iter().flatten() {
                if let AnyJsArrayElement::AnyJsExpression(element) = element
                    && let Some(value) = element.as_static_value()
                    && let Some(name) = value.as_string_constant()
                {
                    push_if_native(name, element.range(), result);
                }
            }
        }
        Some(AnyJsExpression::JsObjectExpression(object)) => {
            for member in object.members().iter().flatten() {
                let name_range = match &member {
                    AnyJsObjectMember::JsPropertyObjectMember(property) => {
                        property.name().ok().map(|name| name.range())
                    }
                    AnyJsObjectMember::JsMethodObjectMember(method) => {
                        method.name().ok().map(|name| name.range())
                    }
                    AnyJsObjectMember::JsShorthandPropertyObjectMember(shorthand) => {
                        shorthand.name().ok().map(|name| name.range())
                    }
                    _ => None,
                };
                if let Some(name_range) = name_range
                    && let Some(name) = member.name()
                {
                    push_if_native(name.text(), name_range, result);
                }
            }
        }
        _ => {}
    }
}

/// Checks the type argument of `defineEmits()`.
///
/// References to local type aliases and interfaces are resolved only when `resolve_references`
/// is set, which prevents endless recursion through circular type aliases.
fn check_type_emits(
    emits_type: &AnyTsType,
    model: &SemanticModel,
    resolve_references: bool,
    result: &mut Vec<ShadowedNativeEvent>,
) {
    match emits_type {
        // `defineEmits<{ (e: 'change'): void; update: [value: string] }>()`
        AnyTsType::TsObjectType(object_type) => {
            check_type_members(&object_type.members(), result);
        }
        // `defineEmits<(e: 'change') => void>()`
        AnyTsType::TsFunctionType(function_type) => {
            if let Ok(parameters) = function_type.parameters() {
                check_event_parameter(&parameters, result);
            }
        }
        AnyTsType::TsParenthesizedType(parenthesized) => {
            if let Ok(ty) = parenthesized.ty() {
                check_type_emits(&ty, model, resolve_references, result);
            }
        }
        AnyTsType::TsReferenceType(reference_type) if resolve_references => {
            let Some(binding) = reference_type
                .name()
                .ok()
                .and_then(|name| name.as_js_reference_identifier().cloned())
                .and_then(|reference| model.binding(&reference))
            else {
                return;
            };
            let Some(binding) = TsIdentifierBinding::cast(binding.syntax().clone()) else {
                return;
            };
            if let Some(type_alias) = binding.parent::<TsTypeAliasDeclaration>() {
                if let Ok(ty) = type_alias.ty() {
                    check_type_emits(&ty, model, false, result);
                }
            } else if let Some(interface) = binding.parent::<TsInterfaceDeclaration>() {
                check_type_members(&interface.members(), result);
            }
        }
        _ => {}
    }
}

fn check_type_members(members: &TsTypeMemberList, result: &mut Vec<ShadowedNativeEvent>) {
    for member in members {
        let name = match member {
            AnyTsTypeMember::TsCallSignatureTypeMember(signature) => {
                if let Ok(parameters) = signature.parameters() {
                    check_event_parameter(&parameters, result);
                }
                continue;
            }
            AnyTsTypeMember::TsPropertySignatureTypeMember(property) => property.name(),
            AnyTsTypeMember::TsMethodSignatureTypeMember(method) => method.name(),
            _ => continue,
        };
        if let Ok(name) = name {
            check_member_name(&name, result);
        }
    }
}

fn check_member_name(name: &AnyJsObjectMemberName, result: &mut Vec<ShadowedNativeEvent>) {
    if let Some(text) = name.name() {
        push_if_native(text.text(), name.range(), result);
    }
}

/// Checks the string literal types of the event parameter in an emit signature, as in
/// `(e: 'change' | 'update', value: string) => void`.
fn check_event_parameter(parameters: &JsParameters, result: &mut Vec<ShadowedNativeEvent>) {
    let Some(Ok(AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
        parameter,
    )))) = parameters.items().first()
    else {
        return;
    };
    let Some(Ok(event_type)) = parameter
        .type_annotation()
        .map(|annotation| annotation.ty())
    else {
        return;
    };
    match event_type {
        AnyTsType::TsStringLiteralType(literal) => {
            if let Ok(name) = literal.inner_string_text() {
                push_if_native(name.text(), literal.range(), result);
            }
        }
        AnyTsType::TsUnionType(union) => {
            for variant in union.types().iter().flatten() {
                if let AnyTsType::TsStringLiteralType(literal) = variant
                    && let Ok(name) = literal.inner_string_text()
                {
                    push_if_native(name.text(), literal.range(), result);
                }
            }
        }
        _ => {}
    }
}

/// Checks a component defined with an options object: the `emits` option, emit calls through
/// the `setup()` context, and `this.$emit()` calls.
fn check_options_based_component(
    component: &impl VueOptionsApiBasedComponent,
    model: &SemanticModel,
    result: &mut Vec<ShadowedNativeEvent>,
) {
    for (name, member) in component.iter_declaration_groups() {
        match (name.text(), member) {
            ("emits", AnyJsObjectMember::JsPropertyObjectMember(property)) => {
                if let Ok(value) = property.value() {
                    check_runtime_emits(&value, result);
                }
            }
            ("setup", AnyJsObjectMember::JsMethodObjectMember(method)) => {
                if let Ok(parameters) = method.parameters() {
                    check_setup_context(&parameters, model, result);
                }
            }
            ("setup", AnyJsObjectMember::JsPropertyObjectMember(property)) => {
                if let Ok(value) = property.value() {
                    check_setup_function(&value, model, result);
                }
            }
            _ => {}
        }
    }

    // `defineComponent((props, context) => { ... }, { emits: [...] })`
    if let Some(setup) = component.setup_func() {
        check_setup_function(&setup, model, result);
    }

    if let Some(definition) = component.definition_expression() {
        let mut iter = definition.syntax().preorder();
        while let Some(event) = iter.next() {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            let Some(call) = JsCallExpression::cast(node) else {
                continue;
            };
            // Nested `defineComponent()` and `createApp()` calls are checked as components of
            // their own, and `this` inside them refers to the nested component.
            if call
                .callee()
                .ok()
                .and_then(|callee| callee.inner_expression())
                .is_some_and(|callee| {
                    is_vue_api_reference(&callee, model, "defineComponent")
                        || is_vue_api_reference(&callee, model, "createApp")
                })
            {
                iter.skip_subtree();
                continue;
            }
            if is_this_emit_call(&call, model) {
                check_emit_call(&call, result);
            }
        }
    }
}

fn check_setup_function(
    function: &AnyJsExpression,
    model: &SemanticModel,
    result: &mut Vec<ShadowedNativeEvent>,
) {
    let parameters = match function.inner_expression() {
        Some(AnyJsExpression::JsFunctionExpression(function)) => function.parameters().ok(),
        Some(AnyJsExpression::JsArrowFunctionExpression(arrow)) => match arrow.parameters() {
            Ok(AnyJsArrowFunctionParameters::JsParameters(parameters)) => Some(parameters),
            _ => None,
        },
        _ => None,
    };
    if let Some(parameters) = parameters {
        check_setup_context(&parameters, model, result);
    }
}

/// Checks emit calls made through the second parameter of `setup()`, either
/// `setup(props, context) { context.emit(...) }` or `setup(props, { emit }) { emit(...) }`.
fn check_setup_context(
    parameters: &JsParameters,
    model: &SemanticModel,
    result: &mut Vec<ShadowedNativeEvent>,
) {
    let Some(Ok(AnyJsParameter::AnyJsFormalParameter(AnyJsFormalParameter::JsFormalParameter(
        context,
    )))) = parameters.items().iter().nth(1)
    else {
        return;
    };
    match context.binding() {
        Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(context))) => {
            let calls = model
                .as_binding(&context)
                .all_reads()
                .filter_map(|reference| {
                    let context = JsIdentifierExpression::cast(reference.syntax().parent()?)?;
                    context_emit_call(&context.into())
                });
            for call in calls {
                check_emit_call(&call, result);
            }
        }
        Ok(AnyJsBindingPattern::JsObjectBindingPattern(pattern)) => {
            let Some(emit) =
                pattern
                    .properties()
                    .iter()
                    .flatten()
                    .find_map(|property| {
                        match property {
                    // `{ emit }`
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        property,
                    ) => {
                        let binding = property.identifier().ok()?;
                        let binding = binding.as_js_identifier_binding()?;
                        (binding.name_token().ok()?.text_trimmed() == "emit")
                            .then(|| binding.clone())
                    }
                    // `{ emit: fire }`
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                        if property.member().ok()?.name()?.text() != "emit" {
                            return None;
                        }
                        property
                            .pattern()
                            .ok()?
                            .as_any_js_binding()?
                            .as_js_identifier_binding()
                            .cloned()
                    }
                    _ => None,
                }
                    })
            else {
                return;
            };
            for call in calls_through_binding(&emit, model) {
                check_emit_call(&call, result);
            }
        }
        _ => {}
    }
}

/// Returns `object.emit(...)` given `object`.
fn context_emit_call(object: &AnyJsExpression) -> Option<JsCallExpression> {
    let object = object.outer_expression()?;
    let member = JsStaticMemberExpression::cast(object.syntax().parent()?)?;
    if member.object().ok()?.syntax() != object.syntax()
        || member
            .member()
            .ok()?
            .as_js_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "emit"
    {
        return None;
    }
    call_with_callee(&member.into())
}

/// Returns the calls whose callee is a read of `binding`.
fn calls_through_binding(
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> impl Iterator<Item = JsCallExpression> {
    model
        .as_binding(binding)
        .all_reads()
        .filter_map(|reference| {
            let callee = JsIdentifierExpression::cast(reference.syntax().parent()?)?;
            call_with_callee(&callee.into())
        })
}

/// Returns the call expression whose callee is `callee`, ignoring parentheses and TypeScript
/// wrappers around it.
fn call_with_callee(callee: &AnyJsExpression) -> Option<JsCallExpression> {
    let callee = callee.outer_expression()?;
    let call = JsCallExpression::cast(callee.syntax().parent()?)?;
    (call.callee().ok()?.syntax() == callee.syntax()).then_some(call)
}

/// Checks whether `call` is `this.$emit(...)`, or `vm.$emit(...)` after `const vm = this`.
fn is_this_emit_call(call: &JsCallExpression, model: &SemanticModel) -> bool {
    let Some(AnyJsExpression::JsStaticMemberExpression(callee)) = call
        .callee()
        .ok()
        .and_then(|callee| callee.inner_expression())
    else {
        return false;
    };
    let is_emit_member = callee
        .member()
        .ok()
        .and_then(|member| member.as_js_name()?.value_token().ok())
        .is_some_and(|token| token.text_trimmed() == "$emit");
    if !is_emit_member {
        return false;
    }
    match callee
        .object()
        .ok()
        .and_then(|object| object.inner_expression())
    {
        Some(AnyJsExpression::JsThisExpression(_)) => true,
        Some(AnyJsExpression::JsIdentifierExpression(identifier)) => identifier
            .name()
            .ok()
            .and_then(|reference| model.binding(&reference))
            .and_then(|binding| binding.syntax().parent())
            .and_then(JsVariableDeclarator::cast)
            .and_then(|declarator| declarator.initializer()?.expression().ok())
            .and_then(|initializer| initializer.inner_expression())
            .is_some_and(|initializer| matches!(initializer, AnyJsExpression::JsThisExpression(_))),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_events_are_sorted() {
        for pair in NATIVE_EVENTS.windows(2) {
            assert_eq!(
                pair[0].cmp_ignore_ascii_case(pair[1]),
                std::cmp::Ordering::Less,
                "{} must come before {}",
                pair[0],
                pair[1]
            );
        }
    }
}
