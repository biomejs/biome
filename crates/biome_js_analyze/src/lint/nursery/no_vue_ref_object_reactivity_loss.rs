use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::{Binding, SemanticModel};
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{
    AnyJsArrayAssignmentPatternElement, AnyJsArrayBindingPatternElement, AnyJsAssignment,
    AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsExpression, AnyJsFunction,
    AnyJsMemberExpression, AnyJsObjectAssignmentPatternMember, AnyJsObjectBindingPatternMember,
    AnyJsRoot, JsAssignmentExpression, JsAssignmentOperator, JsCallExpression,
    JsConstructorClassMember, JsGetterClassMember, JsGetterObjectMember, JsIdentifierExpression,
    JsInitializerClause, JsLanguage, JsMethodClassMember, JsMethodObjectMember,
    JsObjectAssignmentPattern, JsObjectBindingPattern, JsSetterClassMember, JsSetterObjectMember,
    JsVariableDeclarator,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TextSize, WalkEvent, declare_node_union};
use biome_rule_options::no_vue_ref_object_reactivity_loss::NoVueRefObjectReactivityLossOptions;
use smallvec::SmallVec;

use crate::frameworks::vue::vue_call::{is_vue_compiler_macro_call, vue_api_name};
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Disallow reading the value of a Vue ref in a place where the read runs only once.
    ///
    /// A ref is a Vue object that holds a value that can change, such as the result of `ref()`,
    /// `shallowRef()`, `computed()`, `customRef()`, `toRef()`, `toRefs()`, `useTemplateRef()`, or
    /// `defineModel()`. Its current value is stored in its `.value` property.
    ///
    /// Reading `.value` gives you a copy of the value at that moment. When the read happens in the
    /// same function that created the ref, for example directly inside `<script setup>` or
    /// `setup()`, it runs only once. The copy keeps its first value and does not update when the
    /// ref changes later. This is called losing reactivity.
    ///
    /// The rule reports these reads of a ref's value:
    /// - reading `.value`, as in `const total = count.value`;
    /// - destructuring `value`, as in `const { value } = count`.
    ///
    /// Reads inside a nested function, such as a `computed()` getter, a `watch()` callback, or an
    /// event handler, are allowed, because Vue runs that function again when the ref changes.
    /// A function that is called right where it is defined, such as `(() => { ... })()`, runs
    /// right away, so reads inside it are reported too. Async functions and generator functions
    /// called this way are not reported.
    ///
    /// Writing to `.value`, as in `count.value = 1` or `count.value++`, is allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// import { ref } from "vue";
    ///
    /// const count = ref(0);
    /// const doubled = count.value * 2;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { ref } from "vue";
    ///
    /// const count = ref(0);
    /// const { value } = count;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { toRefs } from "vue";
    ///
    /// const { title } = toRefs(props);
    /// console.log(title.value);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// import { computed, ref, watch } from "vue";
    ///
    /// const count = ref(0);
    /// const doubled = computed(() => count.value * 2);
    ///
    /// watch(count, () => {
    ///     console.log(count.value);
    /// });
    ///
    /// function increment() {
    ///     count.value++;
    /// }
    ///
    /// count.value = 10;
    /// ```
    ///
    /// ## See Also
    ///
    /// - To find refs used without `.value`, see [`noVueRefAsOperand`](https://biomejs.dev/linter/rules/no-vue-ref-as-operand/).
    /// - To find destructured `props` in `setup()`, see [`noVueSetupPropsReactivityLoss`](https://biomejs.dev/linter/rules/no-vue-setup-props-reactivity-loss/).
    ///
    pub NoVueRefObjectReactivityLoss {
        version: "2.6.5",
        name: "noVueRefObjectReactivityLoss",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-ref-object-reactivity-loss").inspired()],
    }
}

impl Rule for NoVueRefObjectReactivityLoss {
    type Query = Semantic<AnyJsRoot>;
    /// The range of a read of a ref's value.
    type State = TextRange;
    type Signals = Box<[Self::State]>;
    type Options = NoVueRefObjectReactivityLossOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let mut tracker = RefTracker {
            model,
            scope: None,
            visited: SmallVec::new(),
            violations: Vec::new(),
        };

        for event in ctx
            .query()
            .syntax()
            .preorder_of_kinds(JsCallExpression::KIND_SET)
        {
            let WalkEvent::Enter(node) = event else {
                continue;
            };
            let Some(call) = JsCallExpression::cast(node) else {
                continue;
            };
            let Some(kind) = RefKind::from_call(&call, model) else {
                continue;
            };
            tracker.scope = execution_scope(&call);
            tracker.visit_expression(&call.into(), kind);
        }
        // A read can be reached from more than one call, as in
        // `let count = ref(0); count = ref(1);`.
        tracker
            .violations
            .sort_unstable_by_key(|range| range.start());
        tracker.violations.dedup();
        tracker.violations.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "This reads the value of a ref only once, so it won't update when the ref changes."
                },
            )
            .note(markup! {
                "This code runs only once, together with the code that creates the ref. The value read here is a copy, and Vue does not update it when the ref changes."
            })
            .note(markup! {
                "Read "<Emphasis>".value"</Emphasis>" inside a function, such as a "<Emphasis>"computed()"</Emphasis>" getter, or pass the ref itself instead of its value."
            }),
        )
    }
}

/// What a tracked expression holds.
#[derive(Clone, Copy)]
enum RefKind {
    /// A ref, such as the result of `ref()` or `computed()`.
    Ref,
    /// The result of `defineModel()`: a ref, or a tuple whose first element is the ref and whose
    /// second element holds the model modifiers.
    Model,
    /// The result of `toRefs()`: a plain object whose properties are refs.
    ToRefs,
}

impl RefKind {
    fn from_call(call: &JsCallExpression, model: &SemanticModel) -> Option<Self> {
        if is_vue_compiler_macro_call(call, model, "defineModel") {
            return Some(Self::Model);
        }
        let name = vue_api_name(&call.callee().ok()?, model)?;
        if name.text() == "toRefs" {
            Some(Self::ToRefs)
        } else if REF_APIS.binary_search(&name.text()).is_ok() {
            Some(Self::Ref)
        } else {
            None
        }
    }
}

/// Follows a ref from the call that creates it to every place that reads its value.
struct RefTracker<'a> {
    model: &'a SemanticModel,
    /// Where the ref currently being followed is created. Reads in the same scope are reported.
    scope: Option<AnyJsDeferredScope>,
    /// The variables already followed, as their start offsets, together with the range of
    /// [`Self::scope`] they were followed for. This stops assignments such as `a = a` from making
    /// the tracker follow the same variable forever.
    visited: SmallVec<[(TextSize, Option<TextRange>); 4]>,
    violations: Vec<TextRange>,
}

impl RefTracker<'_> {
    /// Follows `expression`, which holds a value of the given `kind`, to where it is used.
    fn visit_expression(&mut self, expression: &AnyJsExpression, kind: RefKind) {
        let Some(outer) = expression.outer_expression() else {
            return;
        };
        let Some(parent) = outer.syntax().parent() else {
            return;
        };

        if let Some(initializer) = JsInitializerClause::cast_ref(&parent) {
            if let Some(pattern) = initializer
                .parent::<JsVariableDeclarator>()
                .and_then(|declarator| declarator.id().ok())
            {
                self.visit_binding_pattern(&pattern, kind);
            }
        } else if let Some(assignment) = JsAssignmentExpression::cast_ref(&parent) {
            if assignment.operator().ok() == Some(JsAssignmentOperator::Assign)
                && assignment.right().is_ok_and(|right| right == outer)
                && let Ok(left) = assignment.left()
            {
                self.visit_assignment_pattern(&left, kind);
            }
        } else if let Some(member) = AnyJsMemberExpression::cast(parent)
            && member.object().is_ok_and(|object| object == outer)
        {
            match kind {
                RefKind::Ref | RefKind::Model => self.check_value_member(&member),
                RefKind::ToRefs => self.visit_expression(&member.into(), RefKind::Ref),
            }
        }
    }

    fn visit_binding_pattern(&mut self, pattern: &AnyJsBindingPattern, kind: RefKind) {
        match pattern {
            AnyJsBindingPattern::AnyJsBinding(AnyJsBinding::JsIdentifierBinding(binding)) => {
                self.track_binding(&self.model.as_binding(binding), kind);
            }
            AnyJsBindingPattern::JsObjectBindingPattern(object) => match kind {
                RefKind::Ref | RefKind::Model => {
                    if binding_pattern_reads_value(object) {
                        self.report(object);
                    }
                }
                RefKind::ToRefs => self.visit_to_refs_binding_pattern(object),
            },
            AnyJsBindingPattern::JsArrayBindingPattern(array) => {
                if matches!(kind, RefKind::Model)
                    && let Some(Ok(AnyJsArrayBindingPatternElement::JsArrayBindingPatternElement(
                        element,
                    ))) = array.elements().first()
                    && let Ok(pattern) = element.pattern()
                {
                    self.visit_binding_pattern(&pattern, RefKind::Ref);
                }
            }
            AnyJsBindingPattern::AnyJsBinding(_) => {}
        }
    }

    /// Visits the variables destructured from the result of `toRefs()`. Each property is a ref,
    /// and a rest element is again an object of refs.
    fn visit_to_refs_binding_pattern(&mut self, object: &JsObjectBindingPattern) {
        for member in object.properties().iter().flatten() {
            match member {
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                    if let Ok(pattern) = property.pattern() {
                        self.visit_binding_pattern(&pattern, RefKind::Ref);
                    }
                }
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                    property,
                ) => {
                    if let Ok(AnyJsBinding::JsIdentifierBinding(binding)) = property.identifier() {
                        self.track_binding(&self.model.as_binding(&binding), RefKind::Ref);
                    }
                }
                AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                    if let Ok(AnyJsBinding::JsIdentifierBinding(binding)) = rest.binding() {
                        self.track_binding(&self.model.as_binding(&binding), RefKind::ToRefs);
                    }
                }
                AnyJsObjectBindingPatternMember::JsBogusBinding(_)
                | AnyJsObjectBindingPatternMember::JsMetavariable(_) => {}
            }
        }
    }

    fn visit_assignment_pattern(&mut self, pattern: &AnyJsAssignmentPattern, kind: RefKind) {
        match pattern {
            AnyJsAssignmentPattern::AnyJsAssignment(AnyJsAssignment::JsIdentifierAssignment(
                assignment,
            )) => {
                if let Some(binding) = self.model.binding(assignment) {
                    self.track_binding(&binding, kind);
                }
            }
            AnyJsAssignmentPattern::JsObjectAssignmentPattern(object) => match kind {
                RefKind::Ref | RefKind::Model => {
                    if assignment_pattern_reads_value(object) {
                        self.report(object);
                    }
                }
                RefKind::ToRefs => self.visit_to_refs_assignment_pattern(object),
            },
            AnyJsAssignmentPattern::JsArrayAssignmentPattern(array) => {
                if matches!(kind, RefKind::Model)
                    && let Some(Ok(
                        AnyJsArrayAssignmentPatternElement::JsArrayAssignmentPatternElement(
                            element,
                        ),
                    )) = array.elements().first()
                    && let Ok(pattern) = element.pattern()
                {
                    self.visit_assignment_pattern(&pattern, RefKind::Ref);
                }
            }
            AnyJsAssignmentPattern::AnyJsAssignment(_) => {}
        }
    }

    /// Visits the variables assigned by destructuring the result of `toRefs()`, as in
    /// `({ title } = toRefs(props))`.
    fn visit_to_refs_assignment_pattern(&mut self, object: &JsObjectAssignmentPattern) {
        for member in object.properties().iter().flatten() {
            match member {
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
                    if let Ok(pattern) = property.pattern() {
                        self.visit_assignment_pattern(&pattern, RefKind::Ref);
                    }
                }
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                    property,
                ) => {
                    if let Ok(assignment) = property.identifier()
                        && let Some(binding) = self.model.binding(&assignment)
                    {
                        self.track_binding(&binding, RefKind::Ref);
                    }
                }
                AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(rest) => {
                    if let Ok(AnyJsAssignment::JsIdentifierAssignment(assignment)) = rest.target()
                        && let Some(binding) = self.model.binding(&assignment)
                    {
                        self.track_binding(&binding, RefKind::ToRefs);
                    }
                }
                AnyJsObjectAssignmentPatternMember::JsBogusAssignment(_) => {}
            }
        }
    }

    /// Visits every place where the variable `binding`, which holds a value of the given `kind`,
    /// is read.
    fn track_binding(&mut self, binding: &Binding, kind: RefKind) {
        let key = (
            binding.range().start(),
            self.scope.as_ref().map(AstNode::range),
        );
        if self.visited.contains(&key) {
            return;
        }
        self.visited.push(key);

        for reference in binding.all_reads() {
            if let Some(expression) = reference
                .syntax()
                .parent()
                .and_then(JsIdentifierExpression::cast)
            {
                self.visit_expression(&expression.into(), kind);
            }
        }
    }

    /// Reports `member` when it reads the `value` property of a ref.
    fn check_value_member(&mut self, member: &AnyJsMemberExpression) {
        if member
            .member_name()
            .is_some_and(|name| name.text() == "value")
            && !is_assignment_target_object(member)
        {
            self.report(member);
        }
    }

    fn report(&mut self, node: &impl AstNode<Language = JsLanguage>) {
        if execution_scope(node) == self.scope {
            self.violations.push(node.range());
        }
    }
}

declare_node_union! {
    /// Code whose body runs when it is called, not where it is written.
    pub AnyJsDeferredScope = AnyJsFunction
        | JsMethodObjectMember
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsMethodClassMember
        | JsGetterClassMember
        | JsSetterClassMember
        | JsConstructorClassMember
}

/// Vue APIs that return a ref. Sorted for binary search.
const REF_APIS: &[&str] = &[
    "computed",
    "customRef",
    "ref",
    "shallowRef",
    "toRef",
    "useTemplateRef",
];

/// Returns the innermost function that contains `node`, or `None` when `node` is at the top level
/// of the file.
///
/// A synchronous function that is called right where it is defined, such as
/// `(() => { ... })()`, runs immediately, so it is skipped and its caller's scope is returned.
fn execution_scope(node: &impl AstNode<Language = JsLanguage>) -> Option<AnyJsDeferredScope> {
    node.syntax()
        .ancestors()
        .skip(1)
        .filter_map(AnyJsDeferredScope::cast)
        .find(|scope| match scope {
            AnyJsDeferredScope::AnyJsFunction(function) => {
                !is_immediately_invoked_synchronously(function)
            }
            _ => true,
        })
}

/// Returns `true` for `(() => {})()` and `(function () {})()`. Async functions and generators
/// return `false`, because calling them does not run their body to completion.
fn is_immediately_invoked_synchronously(function: &AnyJsFunction) -> bool {
    if function.is_async() || function.is_generator() {
        return false;
    }
    let expression: AnyJsExpression = match function {
        AnyJsFunction::JsArrowFunctionExpression(function) => function.clone().into(),
        AnyJsFunction::JsFunctionExpression(function) => function.clone().into(),
        AnyJsFunction::JsFunctionDeclaration(_)
        | AnyJsFunction::JsFunctionExportDefaultDeclaration(_) => return false,
    };
    let Some(outer) = expression.outer_expression() else {
        return false;
    };
    outer
        .parent::<JsCallExpression>()
        .and_then(|call| call.callee().ok())
        .is_some_and(|callee| callee == outer)
}

/// Returns `true` when `member` is the object of an assigned member, as `count.value` in
/// `count.value.total = 1` or `count.value.items[0]++`. These change what the ref holds instead
/// of copying it.
fn is_assignment_target_object(member: &AnyJsMemberExpression) -> bool {
    let mut expression = AnyJsExpression::from(member.clone());
    loop {
        let Some(outer) = expression.outer_expression() else {
            return false;
        };
        let Some(parent) = outer.syntax().parent() else {
            return false;
        };
        if let Some(assignment) = AnyJsMemberAssignment::cast_ref(&parent) {
            return assignment.object().is_ok_and(|object| object == outer);
        }
        match AnyJsMemberExpression::cast(parent) {
            Some(parent) if parent.object().is_ok_and(|object| object == outer) => {
                expression = parent.into();
            }
            _ => return false,
        }
    }
}

/// Returns `true` for patterns such as `{ value }` and `{ value: current = 0 }`.
fn binding_pattern_reads_value(object: &JsObjectBindingPattern) -> bool {
    object
        .properties()
        .iter()
        .flatten()
        .any(|member| match member {
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => property
                .member()
                .ok()
                .and_then(|name| name.name())
                .is_some_and(|name| name == "value"),
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(property) => {
                property
                    .identifier()
                    .ok()
                    .and_then(|binding| binding.as_js_identifier_binding()?.name_token().ok())
                    .is_some_and(|name| name.text_trimmed() == "value")
            }
            AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(_)
            | AnyJsObjectBindingPatternMember::JsBogusBinding(_)
            | AnyJsObjectBindingPatternMember::JsMetavariable(_) => false,
        })
}

/// Returns `true` for patterns such as `({ value } = count)` and `({ value: current } = count)`.
fn assignment_pattern_reads_value(object: &JsObjectAssignmentPattern) -> bool {
    object
        .properties()
        .iter()
        .flatten()
        .any(|member| match member {
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternProperty(property) => {
                property
                    .member()
                    .ok()
                    .and_then(|name| name.name())
                    .is_some_and(|name| name == "value")
            }
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternShorthandProperty(
                property,
            ) => property
                .identifier()
                .ok()
                .and_then(|assignment| assignment.name_token().ok())
                .is_some_and(|name| name.text_trimmed() == "value"),
            AnyJsObjectAssignmentPatternMember::JsObjectAssignmentPatternRest(_)
            | AnyJsObjectAssignmentPatternMember::JsBogusAssignment(_) => false,
        })
}

#[cfg(test)]
mod tests {
    use super::REF_APIS;

    #[test]
    fn ref_apis_are_sorted() {
        for pair in REF_APIS.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must sort before {}",
                pair[0],
                pair[1]
            );
        }
    }
}
