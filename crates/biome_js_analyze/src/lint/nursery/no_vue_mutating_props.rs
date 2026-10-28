use crate::services::vue::VueReference;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsRoot, JsIdentifierAssignment};
use biome_rowan::AstNode;
use biome_rule_options::no_vue_mutating_props::NoVueMutatingPropsOptions;
use biome_vue_semantic::{Access, Namespace, Origin, RefBase, Reference, SnippetHost, SymbolKind};

declare_lint_rule! {
    /// Disallow changing the props of a Vue component.
    ///
    /// Props pass data from a parent component down to a child. The parent owns that data, so the
    /// child should treat its props as read-only. Vue refuses to assign a new value to a prop and
    /// warns about it. Changing an object or array prop in place, for example by setting one of
    /// its properties or calling `push()` on it, does work, but it changes the parent's data
    /// without the parent knowing, which makes the data flow of the application hard to follow.
    ///
    /// Instead, emit an event so the parent changes the value itself, or copy the prop into a
    /// local `ref()` or computed property when the child needs a value of its own.
    ///
    /// This rule reports props that are assigned, incremented, deleted, bound with `v-model`, or
    /// changed in place with an array method such as `push()` or `sort()`, in the script and in
    /// the template.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   props: {
    ///     count: Number,
    ///   },
    ///   methods: {
    ///     increment() {
    ///       this.count++;
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const props = defineProps({
    ///   items: Array,
    /// });
    ///
    /// function addItem() {
    ///   props.items.push("new item");
    /// }
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   props: {
    ///     user: Object,
    ///   },
    ///   setup(props) {
    ///     props.user.name = "Alice";
    ///   },
    /// });
    /// ```
    ///
    /// ```vue,expect_diagnostic,ignore
    /// <script setup>
    /// defineProps({
    ///   value: String,
    /// });
    /// </script>
    ///
    /// <template>
    ///   <input v-model="value" />
    /// </template>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// import { ref } from "vue";
    ///
    /// const props = defineProps({
    ///   count: Number,
    /// });
    /// const emit = defineEmits(["update:count"]);
    ///
    /// const localCount = ref(props.count);
    ///
    /// function increment() {
    ///   localCount.value++;
    ///   emit("update:count", props.count + 1);
    /// }
    /// </script>
    ///
    /// <template>
    ///   <input :value="count" @input="emit('update:count', $event.target.value)" />
    /// </template>
    /// ```
    ///
    pub NoVueMutatingProps {
        version: "2.6.4",
        name: "noVueMutatingProps",
        language: "js",
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-mutating-props").same()],
    }
}

impl Rule for NoVueMutatingProps {
    type Query = VueReference;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = NoVueMutatingPropsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let reference = ctx.query();
        let access = reference.access();
        if access != Access::Write && access != Access::Mutate {
            return None;
        }
        // A script only refers to a prop by a plain variable after
        // destructuring it, and assigning to that variable replaces the local
        // copy, not the prop.
        if access == Access::Write
            && !is_in_template(reference)
            && is_variable_assignment(&ctx.root(), reference)
        {
            return None;
        }
        let Some(symbol) = reference.symbol() else {
            // Every member of the props object is a prop, even when the
            // declarations of the props can't be read.
            return is_props_member(reference).then_some(());
        };
        match symbol.kind() {
            SymbolKind::Prop => Some(()),
            // A destructured prop whose declaration can't be read, and any
            // destructured prop used in the template, resolves to the variable
            // that holds it.
            SymbolKind::Local if symbol.origin() == Origin::PropsDestructure => Some(()),
            _ => None,
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let reference = ctx.query();
        // A destructured prop can be renamed, as in `{ modelValue: value }`.
        let symbol = reference
            .symbol()
            .filter(|symbol| symbol.kind() == SymbolKind::Prop);
        let name = symbol
            .as_ref()
            .map_or_else(|| reference.name(), |symbol| symbol.name());
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.range(),
                markup! {
                    "This changes the prop "<Emphasis>{name}</Emphasis>"."
                },
            )
            .note(markup! {
                "A prop belongs to the parent component. Changing it from the child either fails, or changes the parent's data without the parent knowing."
            })
            .note(markup! {
                "Emit an event so the parent can change the value, or copy the prop into a local "<Emphasis>"ref()"</Emphasis>" or computed property."
            }),
        )
    }
}

/// Returns `true` when `reference` names a member of the props object, as in
/// `props.title` or `$props.title`.
fn is_props_member(reference: &Reference) -> bool {
    if reference.namespace() != Namespace::Binding {
        return false;
    }
    match reference.base() {
        RefBase::Builtin => true,
        RefBase::Via(_) => reference.base_symbol().is_some_and(|symbol| {
            matches!(symbol.origin(), Origin::DefineProps | Origin::SetupProps)
        }),
        RefBase::Bare | RefBase::This => false,
    }
}

/// Returns `true` when `reference` is a variable being assigned to, as in
/// `count = 1`, rather than a member of an object.
fn is_variable_assignment(root: &AnyJsRoot, reference: &Reference) -> bool {
    root.syntax()
        .covering_element(reference.range())
        .parent()
        .is_some_and(|node| JsIdentifierAssignment::can_cast(node.kind()))
}

/// Returns `true` when `reference` is written in a template expression.
fn is_in_template(reference: &Reference) -> bool {
    reference
        .snippet()
        .is_some_and(|snippet| !matches!(snippet.host(), SnippetHost::Script(_)))
}
