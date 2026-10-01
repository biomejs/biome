use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::binding_ext::AnyJsIdentifierBinding;
use biome_js_syntax::{
    AnyFunctionLike, AnyJsAssignment, AnyJsExpression, AnyJsMemberExpression, AnyJsObjectMember,
    AnyPossibleGlobalIdentifier, JsArrowFunctionExpression, JsAssignmentExpression,
    JsCallExpression, JsFunctionExpression, JsGetterClassMember, JsGetterObjectMember,
    JsIdentifierExpression, JsMethodObjectMember, JsPostUpdateExpression, JsPreUpdateExpression,
    JsReferenceIdentifier, JsSetterClassMember, JsSetterObjectMember, JsSyntaxKind, JsSyntaxNode,
    JsThisExpression, JsUnaryExpression, JsUnaryOperator, global_identifier,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText, declare_node_union};
use biome_rule_options::no_vue_side_effects_in_computed::NoVueSideEffectsInComputedOptions;

use crate::frameworks::vue::vue_call::is_vue_api_reference;
use crate::frameworks::vue::vue_component::{
    AnyVueMethod, VueComponent, VueComponentDeclarations, VueComponentQuery, VueDeclaration,
    VueDeclarationCollectionFilter, VueDeclarationName,
};

declare_lint_rule! {
    /// Disallow side effects in computed properties.
    ///
    /// A computed property derives a value from other state. Mutating state inside
    /// it makes the result depend on how many times the getter ran, which Vue does
    /// not promise: the value is cached, recomputed only when a dependency changes,
    /// and the mutation can itself invalidate that cache.
    ///
    /// This rule reports mutations of `this` inside an Options API `computed`
    /// getter, and mutations of setup state inside a `computed()` getter. A
    /// mutation is an assignment, an update (`++`, `--`), a `delete`, a call to one
    /// of the array methods that reorder or resize in place (`push`, `pop`,
    /// `shift`, `unshift`, `reverse`, `splice`, `sort`, `copyWithin`, `fill`), or an
    /// `Object.assign()` that writes into the tracked object. A `computed()` getter
    /// that writes to a setup binding directly — `count++` rather than
    /// `state.count++` — is reported too.
    ///
    /// Values created inside the getter are not tracked, so building up a local
    /// array or object and returning it is allowed.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     fullName() {
    ///       this.firstName = 'lorem'
    ///       return `${this.firstName} ${this.lastName}`
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     reversed() {
    ///       return this.items.reverse()
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { computed } from 'vue'
    ///
    /// const state = useState()
    /// const reversed = computed(() => state.items.reverse())
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   computed: {
    ///     fullName() {
    ///       return `${this.firstName} ${this.lastName}`
    ///     },
    ///     reversed() {
    ///       // `slice` copies, so the original array is left alone.
    ///       return this.items.slice(0).reverse()
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue
    /// <script setup>
    /// import { computed } from 'vue'
    ///
    /// const state = useState()
    /// const grouped = computed(() => {
    ///   // `categories` belongs to the getter, so mutating it is not a side effect.
    ///   const categories = {}
    ///   for (const item of state.items) {
    ///     categories[item.category] ??= []
    ///     categories[item.category].push(item)
    ///   }
    ///   return categories
    /// })
    /// </script>
    /// ```
    ///
    /// A computed property that has to write state does it in a setter, which is
    /// not checked:
    ///
    /// ```vue
    /// <script>
    /// export default {
    ///   computed: {
    ///     fullName: {
    ///       get() {
    ///         return `${this.firstName} ${this.lastName}`
    ///       },
    ///       set(value) {
    ///         const names = value.split(' ')
    ///         this.firstName = names[0]
    ///         this.lastName = names[names.length - 1]
    ///       }
    ///     }
    ///   }
    /// }
    /// </script>
    /// ```
    ///
    pub NoVueSideEffectsInComputed {
        version: "next",
        name: "noVueSideEffectsInComputed",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-side-effects-in-computed-properties").same()],
    }
}

declare_node_union! {
    /// A function that can serve as a computed getter.
    ///
    /// An arrow function only appears on the `computed()` side: in the Options API
    /// it would capture the surrounding `this` rather than the component instance.
    pub AnyJsComputedGetter =
        JsFunctionExpression | JsArrowFunctionExpression | JsMethodObjectMember
}

declare_node_union! {
    /// The expressions that write through an assignment target.
    pub AnyJsWriteExpression =
        JsAssignmentExpression | JsPreUpdateExpression | JsPostUpdateExpression
}

declare_node_union! {
    /// Everything that owns a `this` and a body of its own.
    ///
    /// [AnyFunctionLike] leaves out accessors, which do bound `this` like a method.
    pub AnyJsFunctionScope = AnyFunctionLike
        | JsGetterObjectMember
        | JsSetterObjectMember
        | JsGetterClassMember
        | JsSetterClassMember
}

/// A mutation found inside a computed getter.
pub struct SideEffect {
    /// The mutating expression.
    range: TextRange,
    /// The name of the Options API computed property the getter belongs to.
    /// A `computed()` getter has no name of its own, so this is `None` for one.
    property_name: Option<TokenText>,
}

impl Rule for NoVueSideEffectsInComputed {
    type Query = VueComponentQuery;
    type State = SideEffect;
    type Signals = Box<[Self::State]>;
    type Options = NoVueSideEffectsInComputedOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let model = ctx.model();
        let Some(component) = VueComponent::from_potential_component(
            ctx.query(),
            model,
            ctx.source_type(),
            ctx.file_path(),
        ) else {
            return Box::new([]);
        };

        let mut side_effects = Vec::new();

        // Options API: `computed: { fullName() { ... } }`.
        for declaration in component.declarations(VueDeclarationCollectionFilter::Computed.into()) {
            let VueDeclaration::Computed(ref method) = declaration else {
                continue;
            };
            let Some(getter) = options_api_getter(method) else {
                continue;
            };
            collect_this_mutations(
                &getter,
                declaration.declaration_name().as_ref(),
                model,
                &mut side_effects,
            );
        }

        // Composition API: `computed(() => ...)`, wherever it sits in the component.
        for call in ctx
            .query()
            .syntax()
            .descendants()
            .filter_map(JsCallExpression::cast)
        {
            let Some(callee) = call.callee().ok().and_then(|callee| callee.inner_expression())
            else {
                continue;
            };
            if !is_vue_api_reference(&callee, model, "computed") {
                continue;
            }
            let Some(getter) = computed_call_getter(&call) else {
                continue;
            };
            collect_setup_mutations(&call, &getter, model, &mut side_effects);
        }

        // Bindings do not come back in source order, so sort to keep the output
        // stable.
        side_effects.sort_by_key(|side_effect| side_effect.range.start());
        side_effects.into_boxed_slice()
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = match &state.property_name {
            Some(name) => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This mutates state inside the "<Emphasis>{name.text()}</Emphasis>" computed property."
                },
            ),
            None => RuleDiagnostic::new(
                rule_category!(),
                state.range,
                markup! {
                    "This mutates state inside a computed getter."
                },
            ),
        };

        Some(
            diagnostic
                .note(markup! {
                    "A computed value is cached and recomputed only when a dependency changes, so the getter does not run a predictable number of times — and neither does this mutation."
                })
                .note(markup! {
                    "Derive the value instead of mutating: copy before reordering, with "<Emphasis>"slice()"</Emphasis>" or "<Emphasis>"toReversed()"</Emphasis>". If the write is the point, move it to a watcher, a method, or the computed property's own setter."
                }),
        )
    }
}

/// Returns the getter of an Options API computed property.
///
/// ```js
/// computed: {
///   shorthand() { ... },           // the method itself
///   assigned: function () { ... }, // the function expression
///   pair: { get() { ... } },       // the `get` member
/// }
/// ```
fn options_api_getter(method: &AnyVueMethod) -> Option<AnyJsComputedGetter> {
    match method {
        AnyVueMethod::JsMethodObjectMember(method) => Some(method.clone().into()),
        AnyVueMethod::JsPropertyObjectMember(property) => {
            match property.value().ok()?.inner_expression()? {
                AnyJsExpression::JsFunctionExpression(function) => Some(function.into()),
                AnyJsExpression::JsObjectExpression(object) => object
                    .members()
                    .into_iter()
                    .flatten()
                    .find_map(|member| getter_of_object_member(&member, false)),
                _ => None,
            }
        }
    }
}

/// Returns the getter passed to `computed()`.
///
/// ```js
/// computed(() => ...)             // the arrow function
/// computed(function () { ... })   // the function expression
/// computed({ get: () => ... })    // the `get` member
/// ```
///
/// A getter passed by name (`computed(getFullName)`) is not inspected: the
/// function it points at is not necessarily a getter everywhere else it is used.
fn computed_call_getter(call: &JsCallExpression) -> Option<AnyJsComputedGetter> {
    let first_argument = call
        .arguments()
        .ok()?
        .args()
        .first()?
        .ok()?
        .as_any_js_expression()?
        .clone()
        .inner_expression()?;

    match first_argument {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => Some(arrow.into()),
        AnyJsExpression::JsFunctionExpression(function) => Some(function.into()),
        AnyJsExpression::JsObjectExpression(object) => object
            .members()
            .into_iter()
            .flatten()
            .find_map(|member| getter_of_object_member(&member, true)),
        _ => None,
    }
}

/// Returns the function of a `get` member of a computed property object.
fn getter_of_object_member(
    member: &AnyJsObjectMember,
    allow_arrow: bool,
) -> Option<AnyJsComputedGetter> {
    if member.name()?.text() != "get" {
        return None;
    }
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => Some(method.clone().into()),
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            match property.value().ok()?.inner_expression()? {
                AnyJsExpression::JsFunctionExpression(function) => Some(function.into()),
                AnyJsExpression::JsArrowFunctionExpression(arrow) if allow_arrow => {
                    Some(arrow.into())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Reports mutations of the component instance inside an Options API getter.
fn collect_this_mutations(
    getter: &AnyJsComputedGetter,
    property_name: Option<&TokenText>,
    model: &SemanticModel,
    side_effects: &mut Vec<SideEffect>,
) {
    for this_expression in getter
        .syntax()
        .descendants()
        .filter_map(JsThisExpression::cast)
    {
        if !runs_directly_in(this_expression.syntax(), getter) {
            continue;
        }
        let Some(parent) = this_expression.syntax().parent() else {
            continue;
        };
        let range = if let Some(assignment) = AnyJsAssignment::cast_ref(&parent) {
            // `this.count = 1`, `this.count++`.
            Some(write_range(&assignment))
        } else if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
            // The walk starts at the member rather than at `this`, because a call
            // on the component itself is one of its own methods: `this.sort()` is
            // not an array mutation, only `this.items.sort()` is. The source rule
            // draws the line in the same place.
            find_mutation(&member.into(), model)
        } else {
            // The component can also be written into whole: `Object.assign(this, ...)`.
            find_mutation(&this_expression.clone().into(), model)
        };
        if let Some(range) = range {
            side_effects.push(SideEffect {
                range,
                property_name: property_name.cloned(),
            });
        }
    }
}

/// Reports mutations of setup state inside a `computed()` getter.
///
/// Only bindings the setup scope owns are tracked. That scope is the function the
/// `computed()` call sits in — `setup()` in the Options API — or the whole module
/// in `<script setup>`, where the module body *is* the setup body. Module state
/// used by an Options API component is not setup state, and neither is an import.
fn collect_setup_mutations(
    call: &JsCallExpression,
    getter: &AnyJsComputedGetter,
    model: &SemanticModel,
    side_effects: &mut Vec<SideEffect>,
) {
    let setup_range = match call
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(AnyJsFunctionScope::cast)
    {
        Some(function) => function.range(),
        None => call
            .syntax()
            .ancestors()
            .last()
            .map_or_else(|| call.range(), |root| root.text_trimmed_range()),
    };
    let getter_range = getter.range();

    // The model already maps every binding to its references and says which of
    // them write, so walk the bindings the setup scope owns instead of resolving
    // each identifier in the getter again. A binding declared inside the getter
    // belongs to a child scope, so it is never reached from here.
    for scope in model.scope(call.syntax()).ancestors() {
        for binding in scope.bindings() {
            let declaration = binding.tree();
            if !setup_range.contains_range(declaration.range()) || is_import_binding(&declaration) {
                continue;
            }
            for reference in binding.all_references() {
                let node = reference.syntax();
                if !getter_range.contains_range(node.text_trimmed_range())
                    || !runs_directly_in(&node, getter)
                {
                    continue;
                }
                let range = if reference.is_write() {
                    // The binding itself is written to: `count = 1`, `count++`.
                    AnyJsAssignment::cast_ref(&node).map(|assignment| write_range(&assignment))
                } else {
                    // A read. The write, if any, is further up the member chain.
                    JsReferenceIdentifier::cast_ref(&node)
                        .and_then(|reference| reference.parent::<JsIdentifierExpression>())
                        .and_then(|expression| find_mutation(&expression.into(), model))
                };
                if let Some(range) = range {
                    side_effects.push(SideEffect {
                        range,
                        property_name: None,
                    });
                }
            }
        }
    }
}

/// Walks up from `expression` looking for a write to it or to one of its members.
///
/// The walk follows the member chain, so `this.a.b.c = 1` is found from `this.a`,
/// and it stops as soon as the chain does: a value handed to a call or copied into
/// a new object is no longer the tracked one. That is what keeps
/// `[...this.items].reverse()` and `Object.keys(this.a).sort()` off the report.
fn find_mutation(expression: &AnyJsExpression, model: &SemanticModel) -> Option<TextRange> {
    let mut current = expression.outer_expression()?;
    // The last member of the chain, whose name a mutating call is read from.
    let mut last_member: Option<AnyJsMemberExpression> = None;

    loop {
        let parent = current.syntax().parent()?;

        if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
            if member.object().ok()?.syntax() != current.syntax() {
                // `foo[this.key]` reads a member name, it does not write to `this`.
                return None;
            }
            current = AnyJsExpression::from(member.clone()).outer_expression()?;
            last_member = Some(member);
            continue;
        }

        // A member assignment only ever appears in a write position, so reaching
        // one means the chain is being assigned to or updated.
        if let Some(member) = AnyJsMemberAssignment::cast_ref(&parent) {
            if member.object().ok()?.syntax() != current.syntax() {
                return None;
            }
            return AnyJsAssignment::cast_ref(&parent).map(|assignment| write_range(&assignment));
        }

        if let Some(unary) = JsUnaryExpression::cast_ref(&parent) {
            return matches!(unary.operator(), Ok(JsUnaryOperator::Delete)).then(|| unary.range());
        }

        // The chain is the callee: `this.items.reverse()`.
        if let Some(call) = JsCallExpression::cast_ref(&parent) {
            let member_name = last_member.as_ref()?.member_name()?;
            return MUTATING_ARRAY_METHODS
                .binary_search(&member_name.text())
                .is_ok()
                .then(|| call.range());
        }

        // The chain is an argument. Only `Object.assign()` writes into one.
        if parent.kind() == JsSyntaxKind::JS_CALL_ARGUMENT_LIST {
            let call = parent.grand_parent().and_then(JsCallExpression::cast)?;
            return is_object_assign_into(&call, &current, model).then(|| call.range());
        }

        return None;
    }
}

/// Returns the range of the write an assignment target belongs to: the whole
/// assignment or update expression, or the target itself when it is neither (a
/// destructuring pattern, or a `for ... of` binding).
fn write_range(assignment: &AnyJsAssignment) -> TextRange {
    assignment
        .syntax()
        .parent()
        .and_then(AnyJsWriteExpression::cast)
        .map_or_else(|| assignment.range(), |write| write.range())
}

/// Returns true when the innermost function around `node` is the getter.
///
/// A mutation in a nested function is not a side effect of the getter: it runs
/// when that function is called, which may be never.
fn runs_directly_in(node: &JsSyntaxNode, getter: &AnyJsComputedGetter) -> bool {
    node.ancestors()
        .skip(1)
        .find_map(AnyJsFunctionScope::cast)
        .is_some_and(|function| function.syntax() == getter.syntax())
}

fn is_import_binding(binding: &AnyJsIdentifierBinding) -> bool {
    binding
        .syntax()
        .ancestors()
        .any(|ancestor| ancestor.kind() == JsSyntaxKind::JS_IMPORT)
}

/// Matches `Object.assign(target, ...)`, the one call that writes into an argument.
fn is_object_assign_into(
    call: &JsCallExpression,
    target: &AnyJsExpression,
    model: &SemanticModel,
) -> bool {
    let Some(first_argument) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().first()?.ok())
        .and_then(|argument| argument.as_any_js_expression().cloned())
    else {
        return false;
    };
    if first_argument.syntax() != target.syntax() {
        return false;
    }
    let Some(callee) = call.callee().ok().and_then(|callee| callee.inner_expression()) else {
        return false;
    };
    let Some(member) = AnyJsMemberExpression::cast_ref(callee.syntax()) else {
        return false;
    };
    if member
        .member_name()
        .is_none_or(|name| name.text() != "assign")
    {
        return false;
    }
    // Only the global `Object` assigns; a binding that shadows it is someone else's.
    let Some(object) = member.object().ok().map(AnyJsExpression::omit_parentheses) else {
        return false;
    };
    let Some(object) = AnyPossibleGlobalIdentifier::cast_ref(object.syntax()) else {
        return false;
    };
    global_identifier(&object).is_some_and(|(reference, name)| {
        name.text() == "Object" && model.binding(&reference).is_none()
    })
}

/// The `Array.prototype` methods that reorder or resize the array in place.
/// Sorted for binary search.
const MUTATING_ARRAY_METHODS: &[&str] = &[
    "copyWithin",
    "fill",
    "pop",
    "push",
    "reverse",
    "shift",
    "sort",
    "splice",
    "unshift",
];

#[cfg(test)]
mod tests {
    use super::MUTATING_ARRAY_METHODS;

    #[test]
    fn mutating_array_methods_should_be_sorted() {
        assert!(
            MUTATING_ARRAY_METHODS.is_sorted(),
            "MUTATING_ARRAY_METHODS should be sorted for binary search."
        );
    }
}
