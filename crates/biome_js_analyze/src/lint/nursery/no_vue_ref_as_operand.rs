use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsExpression, AnyJsName, JsCallArgumentList, JsCallExpression, JsConditionalExpression, JsIdentifierAssignment, JsIdentifierBinding, JsIdentifierExpression, JsLogicalExpression, JsMethodObjectMember, JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxKind, JsTemplateElement, JsTemplateExpression, JsVariableDeclaration, JsVariableDeclarator};
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_rowan::{AstNode,  SyntaxNodeCast, TextRange, declare_node_union};
use biome_rule_options::no_vue_ref_as_operand::NoVueRefAsOperandOptions;

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
        version: "next",
        name: "noVueRefAsOperand",
        language: "js",
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-ref-as-operand").same()],
    }
}

declare_node_union! {
    pub NoVueRefAsOperandQuery = JsIdentifierExpression | JsIdentifierAssignment
}

impl Rule for NoVueRefAsOperand {
    type Query = Semantic<NoVueRefAsOperandQuery>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = NoVueRefAsOperandOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let expr = ctx.query();
        let model = ctx.model();
        check_expression(expr, model)
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

fn check_expression(expr: &NoVueRefAsOperandQuery, model: &SemanticModel) -> Option<TextRange> {
    match expr {
        NoVueRefAsOperandQuery::JsIdentifierExpression(ident_expr) => {
            let reference = ident_expr.name().ok()?;
            let binding = model.binding(&reference)?.tree();
            let declarator = binding.syntax().ancestors().find_map(JsVariableDeclarator::cast)?;
            let init_clause = declarator.initializer()?;
            let init_expr = init_clause.expression().ok()?;
            let call_expr = init_expr.as_js_call_expression()?;

            let ident_binding = if let AnyJsIdentifierBinding::JsIdentifierBinding(binding) = &binding {
                binding
            } else {
                return None
            };

            if !is_calling_a_ref(call_expr, ident_binding, model) {
                return None;
            }

            if let Some(parent) = ident_expr.syntax().parent() {
                match parent.kind() {
                    // if (refValue)
                    JsSyntaxKind::JS_IF_STATEMENT |
                    // switch (refValue)
                    JsSyntaxKind::JS_SWITCH_STATEMENT |
                    // -refValue, +refValue, !refValue, ~refValue, typeof refValue
                    JsSyntaxKind::JS_UNARY_EXPRESSION |
                    // refValue+1, refValue-1
                    JsSyntaxKind::JS_BINARY_EXPRESSION |
                    // bar+=refValue, bar-=refValue
                    JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                        return Some(ident_expr.range())
                    }
                    // refValue || other, refValue && other. ignore: other || refValue
                    JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                        // Report only left
                        if let Some(logical_expr) = parent.cast::<JsLogicalExpression>()
                        && let Ok(left)= logical_expr.left()
                        && let Some(left) = left.as_js_identifier_expression()
                        && left != ident_expr {
                            return None
                        }

                        // Report only refs which are constants
                        if let Some(declaration) = declarator.syntax().ancestors().find_map(JsVariableDeclaration::cast)
                        && (declaration.is_const()) {
                            return Some(ident_expr.range())
                        }
                    }
                    // refValue ? x : y
                    JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                        // Report only test
                        if let Some(conditional_expr) = parent.cast::<JsConditionalExpression>()
                        && let Ok(test)= conditional_expr.test()
                        && let Some(expr) = test.as_js_identifier_expression()
                        && expr != ident_expr {
                            return None
                        }

                        return Some(ident_expr.range())
                    }
                    // `${refValue}`
                    JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
                        // Ignore tagged template
                        if let Some(template_ele) = parent.cast::<JsTemplateElement>()
                        && let Some(grand_parent) = template_ele.syntax().grand_parent()
                        && let Some(template_expr) = grand_parent.cast::<JsTemplateExpression>()
                        && template_expr.tag().is_some() {
                            return None
                        }

                        return Some(ident_expr.range())
                    }
                    // refValue.x
                    JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                        if let Some(static_member_expr) = parent.cast::<AnyJsStaticMemberLike>() {
                            return check_static_member_access(ident_expr, ident_binding, &static_member_expr, call_expr)
                        }
                    }
                    // refValue in emit() in setup contexts: emit('event', refValue) or context.emit('event', refValue)
                    // refValue in emit() with defineEmits
                    JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                        if let Some(call_argument_list) = parent.cast::<JsCallArgumentList>()
                            && let Some(grand_parent) = call_argument_list.syntax().grand_parent()
                            && let Some(call_expr) = grand_parent.cast::<JsCallExpression>()
                            && let Ok(callee) = call_expr.callee()
                            && (is_emit_call_in_setup(&callee, model) || is_emit_call_by_macro(&callee, model))
                        {
                            return Some(ident_expr.range());
                        }
                    }
                    _ => {}
                }
            }

            None
        }
        NoVueRefAsOperandQuery::JsIdentifierAssignment(ident_assignment) => {
            let binding = model.binding(ident_assignment)?.tree();
            let declarator = binding.syntax().ancestors().find_map(JsVariableDeclarator::cast)?;
            let init_clause = declarator.initializer()?;
            let init_expr = init_clause.expression().ok()?;
            let call_expr = init_expr.as_js_call_expression()?;
            let ident_binding = if let AnyJsIdentifierBinding::JsIdentifierBinding(binding) = &binding {
                binding
            } else {
                return None
            };

            if !is_calling_a_ref(call_expr, ident_binding, model) {
                return None;
            }

            Some(ident_assignment.range())
        }
    }
}

declare_node_union! {
    pub AnyJsStaticMemberLike = JsStaticMemberExpression | JsStaticMemberAssignment
}

const REF_VALUE_APIS: &[&str] = &[
    "ref",
    "computed",
    "toRef",
    "customRef",
    "shallowRef",
    "toRefs"
];

fn is_calling_a_ref(
    call_expr: &JsCallExpression,
    ident_binding: &JsIdentifierBinding, 
    model: &SemanticModel) -> bool {
    if let Ok(callee) = call_expr.callee() {
        if ident_binding.is_under_object_pattern_binding().is_some_and(|v|v) {
            return is_valid_destructured_ref(&callee, call_expr, model);
        }
    
        REF_VALUE_APIS
            .iter()
            .any(|ref_name| is_vue_api_reference(&callee, model, ref_name))
        || is_vue_compiler_macro_call(call_expr, model, "defineModel")
    } else {
        false
    }
}

/// Check if a destructured binding is a valid ref.
/// Object destructuring may lose reactivity, but there are some cases allowed:
/// 1. Reactive objects wrapped by toRefs()
/// 2. Model destructured with defineModel()
fn is_valid_destructured_ref(
    callee: &AnyJsExpression,
    call_expr: &JsCallExpression,
    model: &SemanticModel,
) -> bool {
    is_vue_api_reference(callee, model, "toRefs")
        || is_vue_compiler_macro_call(call_expr, model, "defineModel")
}

/// Check if a static member access on a ref is valid
fn check_static_member_access(
    ident_expr: &JsIdentifierExpression,
    ident_binding: &JsIdentifierBinding,
    static_member_expr: &AnyJsStaticMemberLike,
    ref_call_expr: &JsCallExpression,
) -> Option<TextRange> {
    let member = match static_member_expr {
        AnyJsStaticMemberLike::JsStaticMemberExpression(expr) => expr.member().ok()?,
        AnyJsStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.member().ok()?,
    };

    let ref_callee_expr = ref_call_expr.callee().ok()?;
    let ref_callee_name = ref_callee_expr.get_callee_member_name()?;
    if ref_callee_name.text() == "toRefs" {
        if is_valid_static_member_wrapped_in_to_refs(ident_binding, &member) {
            return None
        }
        return Some(static_member_expr.range())
    }

    if !is_value_static_member(&member) {
        return Some(ident_expr.range())
    }

    None
}

/// Check if a static member is `.value` 
fn is_value_static_member(member: &AnyJsName) -> bool {
    member
        .as_js_name()
        .and_then(|m| m.value_token().ok())
        .is_some_and(|m| m.text_trimmed() == "value")
}

/// Check if a static member is accessing a valid property on a ref created by `toRefs()`.
fn is_valid_static_member_wrapped_in_to_refs(
    ident_binding: &JsIdentifierBinding,
    member: &AnyJsName,
) -> bool {
    // Destructured refs: `const { foo } = toRefs(obj); foo.value`
    if ident_binding.is_under_pattern_binding().is_some_and(|v| v) {
        return is_value_static_member(member)
    }

    // Direct refs: `const refs = toRefs(obj); refs.foo.value`
    member
        .syntax()
        .grand_parent()
        .and_then(|grand_parent| grand_parent.cast::<JsStaticMemberExpression>())
        .and_then(|grand_parent_expr| grand_parent_expr.member().ok())
        .is_some_and(|m| is_value_static_member(&m))
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
        _ => false
    }
}

/// Check if this binding is defined inside a setup() method
fn is_emit_in_setup_method(binding: &AnyJsIdentifierBinding) -> bool {
    binding
        .syntax()
        .ancestors()
        .find_map(JsMethodObjectMember::cast)
        .and_then(|method| {
            method
                .name()
                .ok()?
                .as_js_literal_member_name()?
                .name()
                .ok()
        })
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
    && let Some(call_expr) = expr.as_js_call_expression() {
        is_vue_compiler_macro_call(call_expr, model, "defineEmits")
    } else {
        false
    }
}
