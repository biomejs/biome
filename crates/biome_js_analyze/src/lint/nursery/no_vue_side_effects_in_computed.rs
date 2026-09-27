use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsIdentifierReference, AnyJsObjectMember, JsCallExpression,
    JsComputedMemberAssignment, JsComputedMemberExpression, JsIdentifierExpression, JsName,
    JsStaticMemberAssignment, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode,
    JsThisExpression, JsUnaryExpression, JsUnaryOperator,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, TokenText};
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
        recommended: false,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-side-effects-in-computed-properties").same()],
    }
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
///
/// An arrow function is not a getter here: it captures the surrounding `this`
/// rather than the component instance, so `this` inside it is not component state.
fn options_api_getter(method: &AnyVueMethod) -> Option<JsSyntaxNode> {
    match method {
        AnyVueMethod::JsMethodObjectMember(method) => Some(method.syntax().clone()),
        AnyVueMethod::JsPropertyObjectMember(property) => {
            match property.value().ok()?.inner_expression()? {
                AnyJsExpression::JsFunctionExpression(function) => Some(function.into_syntax()),
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
fn computed_call_getter(call: &JsCallExpression) -> Option<JsSyntaxNode> {
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
        AnyJsExpression::JsArrowFunctionExpression(arrow) => Some(arrow.into_syntax()),
        AnyJsExpression::JsFunctionExpression(function) => Some(function.into_syntax()),
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
) -> Option<JsSyntaxNode> {
    if member.name()?.text() != "get" {
        return None;
    }
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => Some(method.syntax().clone()),
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            match property.value().ok()?.inner_expression()? {
                AnyJsExpression::JsFunctionExpression(function) => Some(function.into_syntax()),
                AnyJsExpression::JsArrowFunctionExpression(arrow) if allow_arrow => {
                    Some(arrow.into_syntax())
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Reports mutations of the component instance inside an Options API getter.
fn collect_this_mutations(
    getter: &JsSyntaxNode,
    property_name: Option<&TokenText>,
    model: &SemanticModel,
    side_effects: &mut Vec<SideEffect>,
) {
    for this_expression in getter.descendants().filter_map(JsThisExpression::cast) {
        if !runs_directly_in(this_expression.syntax(), getter) {
            continue;
        }
        // `this.$set(...)` is the Vue 2 spelling of a reactive write.
        if let Some(range) = find_mutation(this_expression.syntax())
            .or_else(|| find_this_set_call(this_expression.syntax()))
        {
            side_effects.push(SideEffect {
                range,
                property_name: property_name.cloned(),
            });
        }
    }

    // `Vue.set(target, 'foo', value)` writes through Vue itself instead.
    for call in getter.descendants().filter_map(JsCallExpression::cast) {
        if !runs_directly_in(call.syntax(), getter) {
            continue;
        }
        if let Some(range) = find_vue_set_call(&call, model) {
            side_effects.push(SideEffect {
                range,
                property_name: property_name.cloned(),
            });
        }
    }
}

/// Reports mutations of setup state inside a `computed()` getter.
///
/// Only bindings from the setup scope are tracked. That scope is the function the
/// `computed()` call sits in — `setup()` in the Options API — or the whole module
/// in `<script setup>`, where the module body *is* the setup body. A binding
/// declared inside the getter belongs to the getter, so mutating it is not a side
/// effect, and an import or a global is not setup state at all.
fn collect_setup_mutations(
    call: &JsCallExpression,
    getter: &JsSyntaxNode,
    model: &SemanticModel,
    side_effects: &mut Vec<SideEffect>,
) {
    let setup_scope_range = match enclosing_function(call.syntax()) {
        Some(function) => function.text_trimmed_range(),
        None => call
            .syntax()
            .ancestors()
            .last()
            .map_or_else(|| call.range(), |root| root.text_trimmed_range()),
    };
    let getter_range = getter.text_trimmed_range();

    for reference in getter.descendants().filter_map(AnyJsIdentifierReference::cast) {
        if !runs_directly_in(reference.syntax(), getter) {
            continue;
        }
        // No binding means a global, which is not setup state.
        let Some(binding) = model.binding(&reference) else {
            continue;
        };
        let binding_node = binding.syntax();
        let binding_range = binding_node.text_trimmed_range();
        if !setup_scope_range.contains_range(binding_range)
            || getter_range.contains_range(binding_range)
            || is_import_binding(&binding_node)
        {
            continue;
        }
        let range = match reference {
            // The binding itself is written to: `count = 1`, `count++`.
            AnyJsIdentifierReference::JsIdentifierAssignment(assignment) => {
                Some(write_range(assignment.syntax()))
            }
            // Otherwise the write, if any, is further up the member chain.
            AnyJsIdentifierReference::JsReferenceIdentifier(reference) => reference
                .parent::<JsIdentifierExpression>()
                .and_then(|expression| find_mutation(expression.syntax())),
            AnyJsIdentifierReference::JsxReferenceIdentifier(_) => None,
        };
        if let Some(range) = range {
            side_effects.push(SideEffect {
                range,
                property_name: None,
            });
        }
    }
}

/// Walks up from `node` looking for a write to it or to one of its members.
///
/// The walk follows the member chain, so `this.a.b.c = 1` is found from `this`,
/// and it stops as soon as the chain does: a value handed to a call or copied into
/// a new object is no longer the tracked one. That is what keeps
/// `[...this.items].reverse()` and `Object.keys(this.a).sort()` off the report.
fn find_mutation(node: &JsSyntaxNode) -> Option<TextRange> {
    let mut current = outermost_parenthesized(node.clone());
    // The last member expression of the chain, which a mutating call is read from.
    let mut last_member: Option<JsSyntaxNode> = None;

    loop {
        let parent = current.parent()?;
        match parent.kind() {
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                if member_object(&parent)? != current {
                    // `foo[this.key]` reads a member name, it does not write to `this`.
                    return None;
                }
                last_member = Some(parent.clone());
                current = outermost_parenthesized(parent);
            }
            // A member assignment node only ever appears in a write position, so
            // reaching one means the chain is being assigned to or updated.
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                if member_object(&parent)? != current {
                    return None;
                }
                return Some(write_range(&parent));
            }
            JsSyntaxKind::JS_UNARY_EXPRESSION => {
                let unary = JsUnaryExpression::cast(parent)?;
                return matches!(unary.operator(), Ok(JsUnaryOperator::Delete))
                    .then(|| unary.range());
            }
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                // The chain is the callee: `this.items.reverse()`.
                let call = JsCallExpression::cast(parent)?;
                let member_name = static_member_name(last_member.as_ref()?)?;
                return MUTATING_ARRAY_METHODS
                    .contains(&member_name.text_trimmed())
                    .then(|| call.range());
            }
            JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                // The chain is an argument. Only `Object.assign()` writes into one.
                let call = parent.grand_parent().and_then(JsCallExpression::cast)?;
                return is_object_assign_into(&call, &current).then(|| call.range());
            }
            _ => return None,
        }
    }
}

/// Returns the range of the write an assignment target belongs to: the whole
/// assignment or update expression, or the target itself when it is neither (a
/// destructuring pattern, or a `for ... of` binding).
fn write_range(assignment: &JsSyntaxNode) -> TextRange {
    assignment
        .parent()
        .filter(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION
                    | JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
                    | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            )
        })
        .map_or_else(
            || assignment.text_trimmed_range(),
            |parent| parent.text_trimmed_range(),
        )
}

/// Matches `this.$set(...)`, the Vue 2 instance helper for a reactive write, and
/// returns the range of its name.
fn find_this_set_call(this_expression: &JsSyntaxNode) -> Option<TextRange> {
    let member = outermost_parenthesized(this_expression.clone()).parent()?;
    if member.kind() != JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION {
        return None;
    }
    let name = static_member_name(&member)?;
    if name.text_trimmed() != "$set" {
        return None;
    }
    let call = outermost_parenthesized(member).parent()?;
    (call.kind() == JsSyntaxKind::JS_CALL_EXPRESSION).then(|| name.text_trimmed_range())
}

/// Matches Vue 2's `set()` helper — `Vue.set(...)` on the global, or the named
/// export — and returns the range of the name that identifies it.
///
/// The receiver is resolved through the semantic model, so a local object that
/// happens to own a `set` method is not mistaken for Vue.
fn find_vue_set_call(call: &JsCallExpression, model: &SemanticModel) -> Option<TextRange> {
    let callee = call.callee().ok()?.inner_expression()?;
    if !is_vue_api_reference(&callee, model, "set") {
        return None;
    }
    Some(match static_member_name(callee.syntax()) {
        Some(name) => name.text_trimmed_range(),
        None => callee.range(),
    })
}

/// Returns true when the innermost function around `node` is `function_node`.
///
/// A mutation in a nested function is not a side effect of the getter: it runs
/// when that function is called, which may be never.
fn runs_directly_in(node: &JsSyntaxNode, function_node: &JsSyntaxNode) -> bool {
    enclosing_function(node).is_some_and(|function| &function == function_node)
}

fn enclosing_function(node: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    node.ancestors()
        .skip(1)
        .find(|ancestor| is_function_like(ancestor.kind()))
}

fn is_function_like(kind: JsSyntaxKind) -> bool {
    matches!(
        kind,
        JsSyntaxKind::JS_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_FUNCTION_DECLARATION
            | JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_METHOD_OBJECT_MEMBER
            | JsSyntaxKind::JS_METHOD_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_SETTER_OBJECT_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
    )
}

fn is_import_binding(binding: &JsSyntaxNode) -> bool {
    binding
        .ancestors()
        .any(|ancestor| ancestor.kind() == JsSyntaxKind::JS_IMPORT)
}

/// Returns the object of a member expression or a member assignment.
fn member_object(member: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    let object = match member.kind() {
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
            JsStaticMemberExpression::cast_ref(member)?.object().ok()?
        }
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            JsComputedMemberExpression::cast_ref(member)?.object().ok()?
        }
        JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
            JsStaticMemberAssignment::cast_ref(member)?.object().ok()?
        }
        JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
            JsComputedMemberAssignment::cast_ref(member)?.object().ok()?
        }
        _ => return None,
    };
    Some(object.into_syntax())
}

/// Returns the name token of a static member expression.
fn static_member_name(member: &JsSyntaxNode) -> Option<biome_js_syntax::JsSyntaxToken> {
    JsStaticMemberExpression::cast_ref(member)?
        .member()
        .ok()?
        .as_js_name()
        .and_then(|name: &JsName| name.value_token().ok())
}

fn is_object_assign_into(call: &JsCallExpression, target: &JsSyntaxNode) -> bool {
    let Some(first_argument) = call
        .arguments()
        .ok()
        .and_then(|arguments| arguments.args().first()?.ok())
        .and_then(|argument| argument.as_any_js_expression().cloned())
    else {
        return false;
    };
    if first_argument.syntax() != target {
        return false;
    }
    let Some(callee) = call.callee().ok().and_then(|callee| callee.inner_expression()) else {
        return false;
    };
    let Some(member) = callee.as_js_static_member_expression() else {
        return false;
    };
    let is_assign = member
        .member()
        .ok()
        .and_then(|name| name.as_js_name()?.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "assign");
    let is_object = member
        .object()
        .ok()
        .and_then(|object| object.as_js_identifier_expression()?.name().ok())
        .and_then(|reference| reference.value_token().ok())
        .is_some_and(|name| name.text_trimmed() == "Object");
    is_assign && is_object
}

/// Returns the outermost expression that wraps `node` in parentheses, so that
/// `(this.items).reverse()` does not stop the walk up the member chain.
fn outermost_parenthesized(node: JsSyntaxNode) -> JsSyntaxNode {
    let mut current = node;
    while let Some(parent) = current.parent() {
        if parent.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION {
            break;
        }
        current = parent;
    }
    current
}

/// The `Array.prototype` methods that reorder or resize the array in place.
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
