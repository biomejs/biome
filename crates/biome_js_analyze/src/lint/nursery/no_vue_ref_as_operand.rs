use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{ReferencesExtensions, SemanticModel};
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyJsBindingPattern, AnyJsExpression, AnyJsIdentifierReference, AnyJsName,
    AnyJsObjectBindingPatternMember, JsAssignmentExpression, JsAssignmentOperator,
    JsCallArgumentList, JsCallExpression, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsConditionalExpression, JsIdentifierBinding, JsIdentifierExpression, JsInitializerClause,
    JsLogicalExpression, JsMethodObjectMember, JsStaticMemberAssignment, JsStaticMemberExpression,
    JsSyntaxKind, JsTemplateElement, JsTemplateExpression, JsVariableDeclaration,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, SyntaxNodeCast, TextRange, declare_node_union};
use biome_rule_options::no_vue_ref_as_operand::NoVueRefAsOperandOptions;
use smallvec::{SmallVec, smallvec};

use crate::frameworks::vue::vue_call::is_vue_compiler_macro_call;
use crate::{frameworks::vue::vue_call::is_vue_api_reference, services::semantic::Semantic};

declare_lint_rule! {
    /// Disallow the use of value wrapped by `ref()`(Composition API) as operand
    ///
    /// To access value wrapped by `ref()`, you must use `.value`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { ref } from "vue"
    ///
    /// const count = ref(0)
    /// count++
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { ref } from "vue"
    ///
    /// const ok = ref(false)
    /// const msg = ok ? "yes" : "no"
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { ref } from "vue"
    ///
    /// const ok = ref(false)
    /// if (ok) {
    ///   //
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { ref } from "vue"
    ///
    /// export default {
    ///   setup(_props, { emit }) {
    ///     const count = ref(0)
    ///     emit('increment', count)
    ///   }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { ref } from "vue"
    ///
    /// const count = ref(0)
    /// count.value++
    /// ```
    ///
    /// ```js
    /// import { ref } from "vue"
    ///
    /// const ok = ref(true)
    /// const msg = ok.value ? "yes" : "no"
    /// if (ok.value) {
    ///   //
    /// }
    /// ```
    ///
    /// ```js
    /// import { ref } from "vue"
    ///
    /// export default {
    ///   setup(_props, { emit }) {
    ///     const count = ref(0)
    ///     emit('increment', count.value)
    ///   }
    /// }
    /// ```
    ///
    pub NoVueRefAsOperand {
        version: "2.4.5",
        name: "noVueRefAsOperand",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-ref-as-operand").same()],
    }
}

impl Rule for NoVueRefAsOperand {
    type Query = Semantic<JsVariableDeclarator>;
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueRefAsOperandOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let declarator = ctx.query();
        let model = ctx.model();
        let Some(producer) = ref_producer(declarator, model) else {
            return Vec::new().into_boxed_slice();
        };
        let Ok(pattern) = declarator.id() else {
            return Vec::new().into_boxed_slice();
        };

        let mut pending = seed_bindings(&pattern, &producer);
        let mut violations = Vec::new();

        while let Some(tracked) = pending.pop() {
            match tracked {
                TrackedBinding::Ref(binding) => {
                    track_ref_references(&binding, model, &mut pending, &mut violations)
                }
                TrackedBinding::ToRefsObject(binding) => {
                    track_to_refs_references(&binding, model, &mut pending, &mut violations)
                }
            }
        }

        violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state,
                markup! {
                    "Ref value is accessed without "<Emphasis>"`.value`"</Emphasis>"."
                },
            )
            .note(markup! {
                "Without "<Emphasis>"`.value`"</Emphasis>", Vue cannot track changes to the ref, which may break reactivity."
            })
            .note(markup! {
                "Use "<Emphasis>"`.value`"</Emphasis>" to access ref value."
            }),
        )
    }
}

/// The shape returned by a Vue API that creates refs.
enum RefProducer {
    /// The call returns a ref directly.
    ///
    /// ```js
    /// const count = ref(0)
    /// const doubled = computed(() => count.value * 2)
    /// const property = toRef(state, "property")
    /// const custom = customRef(factory)
    /// const shallow = shallowRef({ value: 0 })
    /// const element = useTemplateRef("element")
    /// ```
    Ref,
    /// The call returns a plain object whose properties are refs.
    ///
    /// ```js
    /// const refs = toRefs(state)
    /// refs.count // Ref<number>
    /// refs // { count: Ref<number> }
    /// ```
    ToRefs,
    /// The macro returns a ref directly or as the first element of a tuple.
    /// The second tuple element contains the model modifiers and is not a ref.
    ///
    /// ```js
    /// const model = defineModel()
    /// const [modelValue, modifiers] = defineModel()
    /// ```
    DefineModel,
}

/// Distinguishes a ref binding from a binding that contains multiple refs.
///
/// These states require different member-access semantics. A member such as
/// `count.other` uses a [`TrackedBinding::Ref`] as an operand, while
/// `refs.count` selects a ref from a [`TrackedBinding::ToRefsObject`].
///
/// ```js
/// const count = ref(0)
/// count.value++
///
/// const refs = toRefs(state)
/// const refsAlias = refs // ToRefsObject
/// const countAlias = refsAlias.count // Ref
/// countAlias.value++
/// ```
enum TrackedBinding {
    /// A binding whose value is a ref, including aliases and `toRefs()` properties.
    Ref(JsIdentifierBinding),
    /// A binding whose value is the plain object returned by `toRefs()`.
    ToRefsObject(JsIdentifierBinding),
}

fn ref_producer(declarator: &JsVariableDeclarator, model: &SemanticModel) -> Option<RefProducer> {
    let initializer = declarator
        .initializer()?
        .expression()
        .ok()?
        .inner_expression()?;
    let call = initializer.as_js_call_expression()?;
    let callee = call.callee().ok()?;

    if is_vue_api_reference(&callee, model, "toRefs") {
        return Some(RefProducer::ToRefs);
    }
    if REF_VALUE_APIS
        .iter()
        .any(|ref_name| is_vue_api_reference(&callee, model, ref_name))
    {
        return Some(RefProducer::Ref);
    }
    if is_vue_compiler_macro_call(call, model, "defineModel") {
        return Some(RefProducer::DefineModel);
    }
    None
}

/// Collect the initial set of bindings that produce Vue refs.
fn seed_bindings(
    pattern: &AnyJsBindingPattern,
    producer: &RefProducer,
) -> SmallVec<[TrackedBinding; 1]> {
    match producer {
        RefProducer::Ref => identifier_binding(pattern)
            .map(TrackedBinding::Ref)
            .into_iter()
            .collect(),
        RefProducer::ToRefs => {
            if let Some(binding) = identifier_binding(pattern) {
                // ```
                // const foo = toRefs();
                //       ^^^
                // ```
                smallvec![TrackedBinding::ToRefsObject(binding)]
            } else if pattern.as_js_object_binding_pattern().is_some() {
                object_pattern_bindings(pattern)
            } else {
                SmallVec::new()
            }
        }
        RefProducer::DefineModel => {
            let binding = identifier_binding(pattern).or_else(|| {
                let array = pattern.as_js_array_binding_pattern()?;
                let first = array.elements().first()?.ok()?;
                let element = first.as_js_array_binding_pattern_element()?;
                identifier_binding(&element.pattern().ok()?)
            });
            binding.map(TrackedBinding::Ref).into_iter().collect()
        }
    }
}

#[inline]
fn identifier_binding(pattern: &AnyJsBindingPattern) -> Option<JsIdentifierBinding> {
    pattern
        .as_any_js_binding()?
        .as_js_identifier_binding()
        .cloned()
}

fn object_pattern_bindings(pattern: &AnyJsBindingPattern) -> SmallVec<[TrackedBinding; 1]> {
    let Some(pattern) = pattern.as_js_object_binding_pattern() else {
        return SmallVec::new();
    };
    pattern
        .properties()
        .into_iter()
        .filter_map(Result::ok)
        .filter_map(|member| match member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                identifier_binding(&property.pattern().ok()?).map(TrackedBinding::Ref)
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => rest
                .binding()
                .ok()?
                .as_js_identifier_binding()
                .cloned()
                .map(TrackedBinding::ToRefsObject),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                // For destructured assignments, treat each created binding as a produced ref.
                // ```
                // const { foo, bar } = toRefs();
                //         ^^^  ^^^
                // ```
                property
                    .identifier()
                    .ok()?
                    .as_js_identifier_binding()
                    .cloned()
                    .map(TrackedBinding::Ref)
            }
            AnyJsObjectBindingPatternMember::JsBogusBinding(_)
            | AnyJsObjectBindingPatternMember::JsMetavariable(_) => None,
        })
        .collect()
}

fn track_ref_references(
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
    pending: &mut SmallVec<[TrackedBinding; 1]>,
    violations: &mut Vec<TextRange>,
) {
    for reference in binding.all_references(model) {
        let Some(reference) = AnyJsIdentifierReference::cast(reference.syntax()) else {
            continue;
        };
        match reference {
            AnyJsIdentifierReference::JsReferenceIdentifier(reference) => {
                let Some(expression) = reference.parent::<JsIdentifierExpression>() else {
                    continue;
                };
                if let Some(alias) = identifier_initializer_binding(&expression) {
                    pending.push(TrackedBinding::Ref(alias));
                } else if let Some(range) =
                    check_ref_expression(&expression.clone().into(), binding, model)
                {
                    violations.push(range);
                }
            }
            AnyJsIdentifierReference::JsIdentifierAssignment(assignment) => {
                violations.push(assignment.range());
            }
            AnyJsIdentifierReference::JsxReferenceIdentifier(_) => {}
        }
    }
}

fn track_to_refs_references(
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
    pending: &mut SmallVec<[TrackedBinding; 1]>,
    violations: &mut Vec<TextRange>,
) {
    for reference in binding.all_references(model) {
        let Some(reference) = AnyJsIdentifierReference::cast(reference.syntax()) else {
            continue;
        };
        let expression: AnyJsExpression = match reference {
            AnyJsIdentifierReference::JsReferenceIdentifier(reference) => {
                let Some(expression) = reference.parent::<JsIdentifierExpression>() else {
                    continue;
                };
                expression.into()
            }
            AnyJsIdentifierReference::JsIdentifierAssignment(_) => continue,
            AnyJsIdentifierReference::JsxReferenceIdentifier(_) => continue,
        };

        if let Some(pattern) = initializer_pattern(&expression) {
            if let Some(alias) = identifier_binding(&pattern) {
                pending.push(TrackedBinding::ToRefsObject(alias));
            } else if pattern.as_js_object_binding_pattern().is_some() {
                pending.extend(object_pattern_bindings(&pattern));
            }
            continue;
        }

        let Some(object_expression) = expression.outer_expression() else {
            continue;
        };
        let Some(member) = object_expression
            .syntax()
            .parent()
            .and_then(AnyJsRefMember::cast)
        else {
            continue;
        };
        let Ok(object) = member.object() else {
            continue;
        };
        if object != object_expression {
            continue;
        }
        if let Some(member_expression) = member.expression() {
            if let Some(pattern) = initializer_pattern(&member_expression)
                && let Some(alias) = identifier_binding(&pattern)
            {
                pending.push(TrackedBinding::Ref(alias));
                continue;
            }
            if let Some(range) = check_ref_expression(&member_expression, binding, model) {
                violations.push(range);
            }
        } else {
            violations.push(member.range());
        }
    }
}

fn identifier_initializer_binding(
    expression: &JsIdentifierExpression,
) -> Option<JsIdentifierBinding> {
    identifier_binding(&initializer_pattern(&expression.clone().into())?)
}

fn initializer_pattern(expression: &AnyJsExpression) -> Option<AnyJsBindingPattern> {
    let expression = expression.outer_expression()?;
    let initializer = expression
        .syntax()
        .parent()
        .and_then(JsInitializerClause::cast)?;
    if initializer.expression().ok()? != expression {
        return None;
    }
    initializer.parent::<JsVariableDeclarator>()?.id().ok()
}

fn is_plain_assignment_right(expression: &AnyJsExpression) -> bool {
    let Some(expression) = expression.outer_expression() else {
        return false;
    };
    let Some(assignment) = expression
        .syntax()
        .parent()
        .and_then(JsAssignmentExpression::cast)
    else {
        return false;
    };
    assignment.operator().ok() == Some(JsAssignmentOperator::Assign)
        && assignment.right().is_ok_and(|right| right == expression)
}

fn check_ref_expression(
    expression: &AnyJsExpression,
    binding: &JsIdentifierBinding,
    model: &SemanticModel,
) -> Option<TextRange> {
    if is_plain_assignment_right(expression) {
        return None;
    }
    let operand = expression.outer_expression()?;
    let parent = operand.syntax().parent()?;
    match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT
        | JsSyntaxKind::JS_SWITCH_STATEMENT
        | JsSyntaxKind::JS_UNARY_EXPRESSION
        | JsSyntaxKind::JS_BINARY_EXPRESSION
        | JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => Some(expression.range()),
        JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
            let logical_expr = parent.cast::<JsLogicalExpression>()?;
            let left = logical_expr.left().ok()?;
            if left == operand && is_const_binding(binding) {
                Some(expression.range())
            } else {
                None
            }
        }
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
            let conditional_expr = parent.cast::<JsConditionalExpression>()?;
            (conditional_expr.test().ok()? == operand).then_some(expression.range())
        }
        JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
            let template_element = parent.cast::<JsTemplateElement>()?;
            let template = template_element
                .syntax()
                .grand_parent()
                .and_then(JsTemplateExpression::cast)?;
            template.tag().is_none().then_some(expression.range())
        }
        JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
            let static_member = parent.cast::<AnyJsStaticMemberLike>()?;
            check_static_member_access(expression, &static_member)
        }
        JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
            let call_argument_list = parent.cast::<JsCallArgumentList>()?;
            let call_expr = call_argument_list
                .syntax()
                .grand_parent()
                .and_then(JsCallExpression::cast)?;
            let callee = call_expr.callee().ok()?;
            (is_emit_call_in_setup(&callee, model) || is_emit_call_by_macro(&callee, model))
                .then_some(expression.range())
        }
        _ => None,
    }
}

fn is_const_binding(binding: &JsIdentifierBinding) -> bool {
    binding
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsVariableDeclaration::cast)
        .is_some_and(|declaration| declaration.is_const())
}

declare_node_union! {
    pub AnyJsStaticMemberLike = JsStaticMemberExpression | JsStaticMemberAssignment
}

declare_node_union! {
    pub AnyJsRefMember = JsStaticMemberExpression | JsComputedMemberExpression | JsStaticMemberAssignment | JsComputedMemberAssignment
}

impl AnyJsRefMember {
    fn object(&self) -> biome_rowan::SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsStaticMemberExpression(member) => member.object(),
            Self::JsComputedMemberExpression(member) => member.object(),
            Self::JsStaticMemberAssignment(member) => member.object(),
            Self::JsComputedMemberAssignment(member) => member.object(),
        }
    }

    fn expression(&self) -> Option<AnyJsExpression> {
        match self {
            Self::JsStaticMemberExpression(member) => Some(member.clone().into()),
            Self::JsComputedMemberExpression(member) => Some(member.clone().into()),
            Self::JsStaticMemberAssignment(_) | Self::JsComputedMemberAssignment(_) => None,
        }
    }
}

/// Vue APIs that return a `Ref<T>` of some kind
const REF_VALUE_APIS: &[&str] = &[
    "ref",
    "computed",
    "toRef",
    "customRef",
    "shallowRef",
    "useTemplateRef",
];

fn check_static_member_access(
    expression: &AnyJsExpression,
    static_member_expr: &AnyJsStaticMemberLike,
) -> Option<TextRange> {
    let member = match static_member_expr {
        AnyJsStaticMemberLike::JsStaticMemberExpression(expr) => expr.member().ok()?,
        AnyJsStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.member().ok()?,
    };

    if !is_allowed_ref_member(&member) {
        return Some(expression.range());
    }

    None
}

fn is_allowed_ref_member(member: &AnyJsName) -> bool {
    member
        .as_js_name()
        .and_then(|m| m.value_token().ok())
        .is_some_and(|name| matches!(name.text_trimmed(), "value" | "effect"))
}

/// Check if emit is used in setup context
fn is_emit_call_in_setup(callee_expr: &AnyJsExpression, model: &SemanticModel) -> bool {
    match callee_expr {
        // Direct emit call: emit('event', refValue)
        AnyJsExpression::JsIdentifierExpression(ident_expr) => {
            if let Ok(reference) = ident_expr.name()
                && let Some(ident_name) = reference.name().ok()
                && ident_name == "emit"
                && let Some(binding) = model.binding(&reference)
            {
                is_emit_in_setup_method(&binding.tree())
            } else {
                false
            }
        }
        // Member access: context.emit('event', refValue)
        AnyJsExpression::JsStaticMemberExpression(static_member_expr) => {
            if let Ok(member) = static_member_expr.member()
                && let Some(member_name) = member.as_js_name()
                && let Ok(member_name) = member_name.value_token()
                && member_name.text_trimmed() == "emit"
                && let Ok(object) = static_member_expr.object()
                && let Some(ident) = object.as_js_identifier_expression()
                && let Ok(reference) = ident.name()
                && let Some(binding) = model.binding(&reference)
            {
                is_emit_in_setup_method(&binding.tree())
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Check if this binding is defined inside a setup() method
fn is_emit_in_setup_method(binding: &AnyJsIdentifierBinding) -> bool {
    binding
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(JsMethodObjectMember::cast)
        .and_then(|method| method.name().ok()?.as_js_literal_member_name()?.name().ok())
        .is_some_and(|name| name == "setup")
}

/// Check if emit is defined by a macro (defineEmits)
fn is_emit_call_by_macro(callee: &AnyJsExpression, model: &SemanticModel) -> bool {
    if let Some(ident_expr) = callee.as_js_identifier_expression()
        && let Ok(reference) = ident_expr.name()
        && let Some(binding) = model.binding(&reference)
        && let Some(parent) = binding.syntax().parent()
        && let Some(decl) = parent.cast::<JsVariableDeclarator>()
        && let Some(init) = decl.initializer()
        && let Some(expr) = init.expression().ok()
        && let Some(call_expr) = expr.as_js_call_expression()
    {
        is_vue_compiler_macro_call(call_expr, model, "defineEmits")
    } else {
        false
    }
}
