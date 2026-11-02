use crate::frameworks::vue::vue_call::vue_api_name;
use crate::services::vue::{VueComponent, node_at_range};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::assign_ext::AnyJsMemberAssignment;
use biome_js_syntax::{
    AnyJsExpression, AnyJsFunctionBody, AnyJsMemberExpression, AnyJsObjectMember,
    AnyJsStatement, JsArrowFunctionExpression, JsCallArgumentList, JsCallExpression,
    JsClassDeclaration, JsClassExpression, JsFunctionDeclaration, JsFunctionExpression,
    JsGetterObjectMember, JsIdentifierExpression, JsLanguage, JsMethodObjectMember, JsModule,
    JsObjectExpression, JsSetterObjectMember, JsThisExpression, JsUnaryExpression, JsUnaryOperator,
    JsVariableDeclarator, global_identifier,
};
use biome_rowan::{
    AstNode, AstSeparatedList, SyntaxKindSet, TextRange, TokenText, WalkEvent,
    declare_node_union,
};
use biome_rule_options::no_vue_side_effects_in_computed::NoVueSideEffectsInComputedOptions;
use biome_vue_semantic::{Computed, SymbolKind};

declare_lint_rule! {
    /// Disallow changing state inside Vue computed properties.
    ///
    /// A computed property should only calculate a value from other values and return it. Vue
    /// decides when to run it: it may run it many times, or reuse the last result without running
    /// it at all. When a computed property also changes state, such as assigning to a variable or
    /// sorting an array, those changes happen at times that are hard to predict, and the
    /// component becomes hard to understand.
    ///
    /// This rule reports code in the function that calculates a computed property when that code:
    ///
    /// - assigns to, increments, or deletes a property of the component (`this.count++`), or of
    ///   a variable declared in `setup()` or in `<script setup>` (`user.value.name = ""`);
    /// - calls an array method that changes the array itself instead of returning a new one:
    ///   `copyWithin()`, `fill()`, `pop()`, `push()`, `reverse()`, `shift()`, `sort()`,
    ///   `splice()` or `unshift()`;
    /// - passes such a value as the first argument of `Object.assign()`, which changes it.
    ///
    /// Changes to variables declared inside the computed property are allowed, because no other
    /// code can see them. A computed property can also have a `set` function, which runs when a
    /// new value is assigned to the property. That function isn't checked, because changing state
    /// is its purpose.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   computed: {
    ///     reversedItems() {
    ///       return this.items.reverse();
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// import { computed, ref } from "vue";
    ///
    /// const count = ref(0);
    /// const doubled = computed(() => {
    ///   count.value++;
    ///   return count.value * 2;
    /// });
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { computed, ref } from "vue";
    ///
    /// const items = ref([]);
    /// const reversedItems = computed(() => [...items.value].reverse());
    /// const total = computed(() => {
    ///   let sum = 0;
    ///   for (const item of items.value) {
    ///     sum += item.price;
    ///   }
    ///   return sum;
    /// });
    /// </script>
    /// ```
    ///
    pub NoVueSideEffectsInComputed {
        version: "2.6.4",
        name: "noVueSideEffectsInComputed",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-side-effects-in-computed-properties").inspired()],
    }
}

impl Rule for NoVueSideEffectsInComputed {
    type Query = VueComponent;
    type State = SideEffect;
    type Signals = Box<[Self::State]>;
    type Options = NoVueSideEffectsInComputedOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let component = ctx.query();
        let model = ctx.model();
        let root = ctx.root();
        let mut effects = Vec::new();

        // Computed properties of the `computed` option, which reach the
        // component's state through `this`.
        for computed in component
            .computed()
            .filter(|computed| computed.kind() == SymbolKind::Computed)
        {
            let Some(body) = computed
                .getter_range()
                .and_then(|range| node_at_range::<AnyVueComputedGetter>(&root, range))
                .and_then(|getter| options_getter_body(&getter))
            else {
                continue;
            };
            for start in getter_nodes::<JsThisExpression>(&body) {
                if let Some(range) = find_mutation(start.into(), model) {
                    effects.push(SideEffect {
                        range,
                        owner: ComputedOwner::Option(computed.clone()),
                    });
                }
            }
        }

        // `computed()` calls, which reach the component's state through the
        // variables `setup()` or `<script setup>` declares.
        let definition_range = component.range();
        let Some(definition) = node_at_range::<AnyVueComponentDefinition>(&root, definition_range)
        else {
            return effects.into_boxed_slice();
        };
        for call in component_calls(&definition, model) {
            let is_computed_call = call.callee().is_ok_and(|callee| {
                vue_api_name(&callee, model).is_some_and(|name| name.text() == "computed")
            });
            if !is_computed_call {
                continue;
            }
            let Some(body) = composition_getter_body(&call) else {
                continue;
            };
            // The whole function, so that its parameters count as its own.
            let Some(getter_range) = body
                .syntax()
                .parent()
                .map(|function| function.text_trimmed_range())
            else {
                continue;
            };
            for identifier in getter_nodes::<JsIdentifierExpression>(&body) {
                if !is_shared_variable(&identifier, model, definition_range, getter_range) {
                    continue;
                }
                if let Some(range) = find_mutation(identifier.into(), model) {
                    effects.push(SideEffect {
                        range,
                        owner: ComputedOwner::Call(call.clone()),
                    });
                }
            }
        }

        // Each target of a destructuring assignment such as
        // `[this.a, this.b] = pair` reports the same assignment.
        effects.sort_unstable_by_key(|effect| effect.range.start());
        effects.dedup_by_key(|effect| effect.range);
        effects.into_boxed_slice()
    }

    fn text_range(_ctx: &RuleContext<Self>, effect: &Self::State) -> Option<TextRange> {
        Some(effect.range)
    }

    fn diagnostic(_ctx: &RuleContext<Self>, effect: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                effect.range,
                markup! { {effect.owner}" changes state." },
            )
            .note(markup! {
                "Vue decides when to run a computed property, and skips running it again while the values it reads stay the same. Changes made inside it happen at unpredictable times."
            })
            .note(markup! {
                "Make the change in a method or a watcher instead. To sort or reverse an array, work on a copy, as in "<Emphasis>"[...items].reverse()"</Emphasis>"."
            }),
        )
    }
}

/// A change to state written in the getter of a computed property.
pub struct SideEffect {
    /// The range of the expression that makes the change.
    range: TextRange,
    owner: ComputedOwner,
}

/// The computed property a side effect is written in.
enum ComputedOwner {
    /// A property of the `computed` option.
    Option(Computed),
    /// A `computed()` call.
    Call(JsCallExpression),
}

impl biome_console::fmt::Display for ComputedOwner {
    /// Writes the subject of the diagnostic message: the computed property,
    /// named when it has a name.
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        match self {
            Self::Option(computed) => fmt.write_markup(markup! {
                "The computed property "<Emphasis>{computed.name()}</Emphasis>
            }),
            Self::Call(call) => match assigned_variable_name(call) {
                Some(name) => fmt.write_markup(markup! {
                    "The computed property "<Emphasis>{name.text()}</Emphasis>
                }),
                None => fmt.write_str("This computed property"),
            },
        }
    }
}

/// Returns the name of the variable a `computed()` call is assigned to, as in
/// `const total = computed(() => {})`.
fn assigned_variable_name(call: &JsCallExpression) -> Option<TokenText> {
    let declarator = call
        .syntax()
        .parent()?
        .parent()
        .and_then(JsVariableDeclarator::cast)?;
    let binding = declarator.id().ok()?;
    let binding = binding.as_any_js_binding()?.as_js_identifier_binding()?;
    Some(binding.name_token().ok()?.token_text_trimmed())
}

declare_node_union! {
    /// The definition of a Vue component: the module of a `<script setup>`
    /// block, the options object exported by a plain `<script>` block, or a
    /// `defineComponent()` or `createApp()` call.
    AnyVueComponentDefinition = JsModule | AnyJsExpression
}

/// Returns the calls written in the definition of a component, without those
/// written in the definition of another component nested in it, which is
/// checked on its own.
fn component_calls<'a>(
    definition: &'a AnyVueComponentDefinition,
    model: &'a SemanticModel,
) -> impl Iterator<Item = JsCallExpression> + 'a {
    let mut preorder = definition
        .syntax()
        .preorder_of_kinds(JsCallExpression::KIND_SET);
    std::iter::from_fn(move || {
        loop {
            let WalkEvent::Enter(node) = preorder.next()? else {
                continue;
            };
            let Some(call) = JsCallExpression::cast(node) else {
                continue;
            };
            let is_nested_component = call.syntax() != definition.syntax()
                && call.callee().is_ok_and(|callee| {
                    vue_api_name(&callee, model)
                        .is_some_and(|name| COMPONENT_FACTORIES.contains(&name.text()))
                });
            if is_nested_component {
                preorder.skip_subtree();
                continue;
            }
            return Some(call);
        }
    })
}

declare_node_union! {
    /// The getter of a property of the `computed` option: a method such as
    /// `total() {}` or `get() {}`, a property whose value is a function, or a
    /// function expression.
    AnyVueComputedGetter = AnyJsObjectMember | JsFunctionExpression
}

/// Returns the body of a getter declared in the `computed` option.
///
/// Arrow functions are skipped, because `this` inside them is not the
/// component.
fn options_getter_body(getter: &AnyVueComputedGetter) -> Option<AnyJsFunctionBody> {
    match getter {
        AnyVueComputedGetter::AnyJsObjectMember(AnyJsObjectMember::JsMethodObjectMember(
            method,
        )) => method.body().ok().map(AnyJsFunctionBody::from),
        AnyVueComputedGetter::AnyJsObjectMember(AnyJsObjectMember::JsPropertyObjectMember(
            property,
        )) => match property.value().ok()?.omit_parentheses() {
            AnyJsExpression::JsFunctionExpression(function) => {
                function.body().ok().map(AnyJsFunctionBody::from)
            }
            _ => None,
        },
        AnyVueComputedGetter::JsFunctionExpression(function) => {
            function.body().ok().map(AnyJsFunctionBody::from)
        }
        AnyVueComputedGetter::AnyJsObjectMember(_) => None,
    }
}

/// Returns the body of the getter passed to `computed()`: a function, or the
/// `get` member of an object that also declares a setter.
fn composition_getter_body(call: &JsCallExpression) -> Option<AnyJsFunctionBody> {
    let argument = call.arguments().ok()?.args().iter().next()?.ok()?;
    match argument.as_any_js_expression()?.clone().omit_parentheses() {
        AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok(),
        AnyJsExpression::JsFunctionExpression(function) => {
            function.body().ok().map(AnyJsFunctionBody::from)
        }
        AnyJsExpression::JsObjectExpression(object) => object_get_body(&object),
        _ => None,
    }
}

/// Returns the body of the `get` member of `{ get() {}, set(value) {} }`.
fn object_get_body(object: &JsObjectExpression) -> Option<AnyJsFunctionBody> {
    let member = object
        .members()
        .iter()
        .flatten()
        .find(|member| member.name().is_some_and(|name| name.text() == "get"))?;
    match member {
        AnyJsObjectMember::JsMethodObjectMember(method) => {
            method.body().ok().map(AnyJsFunctionBody::from)
        }
        AnyJsObjectMember::JsPropertyObjectMember(property) => {
            match property.value().ok()?.omit_parentheses() {
                AnyJsExpression::JsArrowFunctionExpression(arrow) => arrow.body().ok(),
                AnyJsExpression::JsFunctionExpression(function) => {
                    function.body().ok().map(AnyJsFunctionBody::from)
                }
                _ => None,
            }
        }
        _ => None,
    }
}

/// Returns the nodes of type `N` in a getter body, without those in the
/// bodies of the functions and classes declared in it.
///
/// Code in a nested function may never run while the getter runs, so it is
/// not checked. The exception is an arrow function passed to an array method
/// such as `forEach()` or `map()`, which calls it before returning.
fn getter_nodes<N: AstNode<Language = JsLanguage>>(
    body: &AnyJsFunctionBody,
) -> impl Iterator<Item = N> + '_ {
    let kinds = N::KIND_SET
        .union(NESTED_SCOPE_KINDS)
        .union(JsArrowFunctionExpression::KIND_SET);
    let mut preorder = body.syntax().preorder_of_kinds(kinds);
    std::iter::from_fn(move || {
        loop {
            let WalkEvent::Enter(node) = preorder.next()? else {
                continue;
            };
            let is_skipped = match JsArrowFunctionExpression::cast_ref(&node) {
                Some(arrow) => !is_array_callback(&arrow),
                None => NESTED_SCOPE_KINDS.matches(node.kind()),
            };
            if is_skipped {
                preorder.skip_subtree();
                continue;
            }
            if let Some(node) = N::cast(node) {
                return Some(node);
            }
        }
    })
}

/// Returns `true` when `arrow` is passed to an array method that calls it
/// right away, as in `items.forEach((item) => {})`.
fn is_array_callback(arrow: &JsArrowFunctionExpression) -> bool {
    let Some(call) = arrow
        .syntax()
        .parent()
        .and_then(JsCallArgumentList::cast)
        .and_then(|list| list.syntax().grand_parent())
        .and_then(JsCallExpression::cast)
    else {
        return false;
    };
    call.callee()
        .ok()
        .and_then(|callee| AnyJsMemberExpression::cast(callee.omit_parentheses().into_syntax()))
        .and_then(|callee| callee.member_name())
        .is_some_and(|name| ARRAY_CALLBACK_METHODS.binary_search(&name.text()).is_ok())
}

/// Returns `true` when `identifier` refers to a variable that other code of
/// the component can see: one declared in the component definition, outside
/// the getter, and not imported.
fn is_shared_variable(
    identifier: &JsIdentifierExpression,
    model: &SemanticModel,
    definition: TextRange,
    getter: TextRange,
) -> bool {
    let Ok(reference) = identifier.name() else {
        return false;
    };
    let Some(binding) = model.binding(&reference) else {
        return false;
    };
    let declared_at = binding.syntax().text_trimmed_range();
    !binding.is_imported()
        && definition.contains_range(declared_at)
        && !getter.contains_range(declared_at)
}

/// Follows the member chain that starts at `start` and returns the range of
/// the expression that changes it, if any: an assignment to one of its
/// members, an update or `delete` of one, a call to an array method that
/// changes the array in place, or `Object.assign()` with the value as target.
fn find_mutation(start: AnyJsExpression, model: &SemanticModel) -> Option<TextRange> {
    let mut current = start;
    let mut has_member = false;
    loop {
        current = current.outer_expression()?;
        let parent = current.syntax().parent()?;
        if let Some(member) = AnyJsMemberExpression::cast_ref(&parent) {
            if member.object().ok()? != current {
                return None;
            }
            has_member = true;
            current = member.into();
            continue;
        }
        if let Some(target) = AnyJsMemberAssignment::cast_ref(&parent) {
            if target.object().ok()? != current {
                return None;
            }
            // A member of the value is written. Report the assignment or
            // update that holds the target, or the target itself when no
            // expression does, as in `for (this.item of items)`.
            let changed = target
                .syntax()
                .ancestors()
                .take_while(|node| !AnyJsStatement::can_cast(node.kind()))
                .find(|node| AnyJsExpression::can_cast(node.kind()))
                .unwrap_or_else(|| target.syntax().clone());
            return Some(changed.text_trimmed_range());
        }
        if let Some(call) = JsCallExpression::cast_ref(&parent) {
            let callee = AnyJsMemberExpression::cast(current.inner_expression()?.into_syntax())?;
            let name = callee.member_name()?;
            if MUTATING_METHODS.binary_search(&name.text()).is_ok() {
                return Some(call.syntax().text_trimmed_range());
            }
            return None;
        }
        if let Some(unary) = JsUnaryExpression::cast_ref(&parent) {
            if has_member && unary.operator().ok()? == JsUnaryOperator::Delete {
                return Some(unary.syntax().text_trimmed_range());
            }
            return None;
        }
        if let Some(list) = JsCallArgumentList::cast(parent) {
            return object_assign_call(&current, &list, model);
        }
        return None;
    }
}

/// Returns the range of the `Object.assign()` call that `argument` is the
/// first argument of.
fn object_assign_call(
    argument: &AnyJsExpression,
    list: &JsCallArgumentList,
    model: &SemanticModel,
) -> Option<TextRange> {
    let first = list.iter().next()?.ok()?;
    if first.as_any_js_expression() != Some(argument) {
        return None;
    }
    let call = list
        .syntax()
        .grand_parent()
        .and_then(JsCallExpression::cast)?;
    let callee = AnyJsMemberExpression::cast(call.callee().ok()?.omit_parentheses().into_syntax())?;
    if callee.member_name()?.text() != "assign" {
        return None;
    }
    let object = callee.object().ok()?;
    let (reference, name) = global_identifier(&object.as_any_global_identifier_expression()?)?;
    (name.text() == "Object" && model.binding(&reference).is_none())
        .then(|| call.syntax().text_trimmed_range())
}

/// Functions and classes, whose bodies don't run when they are declared.
const NESTED_SCOPE_KINDS: SyntaxKindSet<JsLanguage> = JsFunctionExpression::KIND_SET
    .union(JsFunctionDeclaration::KIND_SET)
    .union(JsMethodObjectMember::KIND_SET)
    .union(JsGetterObjectMember::KIND_SET)
    .union(JsSetterObjectMember::KIND_SET)
    .union(JsClassExpression::KIND_SET)
    .union(JsClassDeclaration::KIND_SET);

/// Vue functions that define a component.
const COMPONENT_FACTORIES: &[&str] = &["createApp", "defineComponent"];

/// Array methods that change the array they are called on.
const MUTATING_METHODS: &[&str] = &[
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

/// Array methods that call the function they are given before returning.
const ARRAY_CALLBACK_METHODS: &[&str] = &[
    "every",
    "filter",
    "find",
    "findIndex",
    "findLast",
    "findLastIndex",
    "flatMap",
    "forEach",
    "map",
    "reduce",
    "reduceRight",
    "some",
    "sort",
    "toSorted",
];

#[cfg(test)]
mod tests {
    use super::{ARRAY_CALLBACK_METHODS, MUTATING_METHODS};

    fn assert_sorted(name: &str, list: &[&str]) {
        for pair in list.windows(2) {
            assert!(
                pair[0] < pair[1],
                "{} must come before {} in {name}",
                pair[1],
                pair[0]
            );
        }
    }

    #[test]
    fn mutating_methods_are_sorted() {
        assert_sorted("MUTATING_METHODS", MUTATING_METHODS);
    }

    #[test]
    fn array_callback_methods_are_sorted() {
        assert_sorted("ARRAY_CALLBACK_METHODS", ARRAY_CALLBACK_METHODS);
    }
}
