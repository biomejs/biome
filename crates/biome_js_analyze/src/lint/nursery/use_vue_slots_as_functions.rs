use crate::frameworks::vue::vue_instance::{
    is_in_component_definition_object, is_this_or_direct_const_alias,
};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression,
    AnyJsMemberExpression, JsAssignmentExpression, JsIdentifierExpression, JsInitializerClause,
    JsSyntaxKind, JsVariableDeclarator,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::use_vue_slots_as_functions::UseVueSlotsAsFunctionsOptions;
use rustc_hash::FxHashSet;

declare_lint_rule! {
    /// Require the slots in `this.$slots` to be called as functions.
    ///
    /// In Vue 3, each property of `$slots` is a function that returns the content of the slot.
    /// Code that treats a slot as the content itself, for example by reading its `length`,
    /// calling array methods on it, or spreading it into an array, doesn't work.
    /// Call the slot to get its content instead.
    ///
    /// See the [Vue documentation](https://vuejs.org/api/component-instance.html#slots) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { h } from "vue";
    ///
    /// export default {
    ///   render() {
    ///     return h("div", this.$slots.default.filter(isVisible));
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { h } from "vue";
    ///
    /// export default {
    ///   render() {
    ///     return h("div", [...this.$slots.default]);
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// import { h } from "vue";
    ///
    /// export default {
    ///   render() {
    ///     const children = this.$slots.default;
    ///     return h("div", children.length);
    ///   }
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import { h } from "vue";
    ///
    /// export default {
    ///   render() {
    ///     const children = this.$slots.default();
    ///     const header = this.$slots.header && this.$slots.header();
    ///     return h("div", [header, children]);
    ///   }
    /// };
    /// </script>
    /// ```
    pub UseVueSlotsAsFunctions {
        version: "next",
        name: "useVueSlotsAsFunctions",
        language: "js",
        sources: &[RuleSource::EslintVueJs("require-slots-as-functions").same()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for UseVueSlotsAsFunctions {
    type Query = Semantic<AnyJsMemberExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseVueSlotsAsFunctionsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let slot = ctx.query();
        let slots =
            AnyJsMemberExpression::cast(slot.object().ok()?.omit_parentheses().into_syntax())?;
        if slots.member_name()?.text() != "$slots"
            || !is_this_or_direct_const_alias(slots.object().ok()?, ctx.model())
            || !is_in_component_definition_object(
                slot,
                ctx.model(),
                ctx.source_type::<JsFileSource>(),
                ctx.file_path(),
            )
        {
            return None;
        }

        is_used_as_value(AnyJsExpression::from(slot.clone()), ctx.model()).then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let range = slot_name_range(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This slot is used as its content instead of being called."
                },
            )
            .note(markup! {
                "In Vue 3, each property of "<Emphasis>"$slots"</Emphasis>" is a function that returns the content of the slot, so reading or iterating it directly doesn't work."
            })
            .note(markup! {
                "Call the slot to get its content, for example "<Emphasis>"this.$slots.default()"</Emphasis>"."
            }),
        )
    }
}

/// Returns `true` when the value of `expression` is read as if it were the
/// content of the slot: a member of it is read or written, it is spread, it is put in
/// an array, or it is destructured.
///
/// Variables that hold the value are followed through their reads.
fn is_used_as_value(expression: AnyJsExpression, model: &SemanticModel) -> bool {
    let mut pending = vec![expression];
    let mut visited_bindings = FxHashSet::default();
    while let Some(expression) = pending.pop() {
        let Some(expression) = expression.outer_expression() else {
            continue;
        };
        let Some(parent) = expression.syntax().parent() else {
            continue;
        };
        let binding = match parent.kind() {
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                // `this.$slots.default.length`, but not `foo[this.$slots.default]`.
                if AnyJsMemberExpression::cast(parent)
                    .and_then(|member| member.object().ok())
                    .is_some_and(|object| object.syntax() == expression.syntax())
                {
                    return true;
                }
                continue;
            }
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                // `this.$slots.default.foo = value`
                if AnyJsMemberAssignment::cast(parent)
                    .and_then(|member| member.object().ok())
                    .is_some_and(|object| object.syntax() == expression.syntax())
                {
                    return true;
                }
                continue;
            }
            JsSyntaxKind::JS_SPREAD | JsSyntaxKind::JS_ARRAY_ELEMENT_LIST => return true,
            JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                let Some(declarator) = JsInitializerClause::cast(parent)
                    .and_then(|initializer| initializer.parent::<JsVariableDeclarator>())
                else {
                    continue;
                };
                match declarator.id() {
                    Ok(AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(
                        binding,
                    ))) => model.as_binding(&binding),
                    // `const [first] = this.$slots.default`
                    Ok(_) => return true,
                    Err(_) => continue,
                }
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let Some(assignment) = JsAssignmentExpression::cast(parent) else {
                    continue;
                };
                if assignment
                    .right()
                    .ok()
                    .is_none_or(|right| right.syntax() != expression.syntax())
                {
                    continue;
                }
                let Ok(AnyJsAssignmentPattern::AnyJsAssignment(
                    AnyJsAssignment::JsIdentifierAssignment(identifier),
                )) = assignment.left()
                else {
                    continue;
                };
                let Some(binding) = model.binding(&identifier) else {
                    continue;
                };
                binding
            }
            _ => continue,
        };

        if !visited_bindings.insert(binding.range()) {
            continue;
        }
        pending.extend(binding.all_reads().filter_map(|reference| {
            reference
                .syntax()
                .parent()
                .and_then(JsIdentifierExpression::cast)
                .map(AnyJsExpression::from)
        }));
    }
    false
}

/// Returns the range of the slot name: `default` in `this.$slots.default`.
fn slot_name_range(slot: &AnyJsMemberExpression) -> Option<TextRange> {
    match slot {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => {
            Some(member.member().ok()?.range())
        }
        AnyJsMemberExpression::JsComputedMemberExpression(member) => {
            Some(member.member().ok()?.range())
        }
    }
}
