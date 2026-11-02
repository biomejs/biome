use crate::services::vue::{VueEvent, VueEventSite};
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{
    AnyJsFunction, AnyTsType, JsCallArguments, JsMethodObjectMember, JsParameters, JsSyntaxNode,
    TsCallSignatureTypeMember, TsPropertySignatureTypeMember,
};
use biome_rowan::{AstNode, AstSeparatedList, TextRange, declare_node_union};
use biome_rule_options::use_vue_single_event_payload::UseVueSingleEventPayloadOptions;
use biome_vue_semantic::{Emit, EmitForm};

declare_lint_rule! {
    /// Require Vue component events to carry at most one value.
    ///
    /// A component can send a value along with an event, as in `emit('change', value)`, and it
    /// can send several, as in `emit('change', value, extra)`. A parent that listens to the event
    /// with a handler written inline in its template, such as `@change="onChange($event)"`, can
    /// only read the first value through `$event`. The other values are silently lost.
    ///
    /// Put all the values in one object instead, as in `emit('change', { value, extra })`, so every
    /// listener receives all of them.
    ///
    /// This rule checks the calls that emit an event: `$emit()` in templates, `this.$emit()`, the
    /// `emit` function that `setup()` receives, and the function returned by `defineEmits()`.
    /// It also checks the events declared in the `emits` option and in `defineEmits()`, when the
    /// declaration lists the values the event carries.
    ///
    /// Calls whose event name is not written as a string, as in `emit(eventName, a, b)`, are not
    /// checked.
    ///
    /// :::note
    /// In `.vue` files, this rule needs to read the template to check the calls written there,
    /// which requires
    /// [`html.experimentalFullSupportEnabled`](https://biomejs.dev/reference/configuration/#htmlexperimentalfullsupportenabled)
    /// to be `true`.
    /// :::
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// const emit = defineEmits(["change"]);
    ///
    /// function onInput(value, extra) {
    ///   emit("change", value, extra);
    /// }
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   emits: {
    ///     change: (value, extra) => value !== null,
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineEmits<{
    ///   change: [value: string, extra: number];
    /// }>();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup lang="ts">
    /// const emit = defineEmits<{
    ///   change: [payload: { value: string; extra: number }];
    ///   close: [];
    /// }>();
    ///
    /// function onInput(value: string, extra: number) {
    ///   emit("change", { value, extra });
    /// }
    ///
    /// function onClose() {
    ///   emit("close");
    /// }
    /// </script>
    /// ```
    ///
    pub UseVueSingleEventPayload {
        version: "next",
        name: "useVueSingleEventPayload",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("prefer-single-event-payload").inspired()],
    }
}

impl Rule for UseVueSingleEventPayload {
    type Query = VueEvent;
    /// The range of the values after the first one.
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = UseVueSingleEventPayloadOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let root = ctx.root();
        match ctx.query() {
            VueEventSite::Declaration(emit) => {
                if emit.payload_params()? < 2 {
                    return None;
                }
                extra_declared_payloads(root.syntax(), emit)
            }
            VueEventSite::Call(reference) => {
                if reference.call_args()? < 2 {
                    return None;
                }
                let arguments = root
                    .syntax()
                    .covering_element(reference.range())
                    .ancestors()
                    .find_map(JsCallArguments::cast)?;
                // Skips the event name and the first value.
                range_from(&arguments.args(), 2)
            }
        }
    }

    fn text_range(_ctx: &RuleContext<Self>, range: &Self::State) -> Option<TextRange> {
        Some(*range)
    }

    fn diagnostic(ctx: &RuleContext<Self>, range: &Self::State) -> Option<RuleDiagnostic> {
        let site = ctx.query();
        let action = match site {
            VueEventSite::Declaration(_) => "declared",
            VueEventSite::Call(_) => "emitted",
        };
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                range,
                markup! {
                    "The event "<Emphasis>{site.name()}</Emphasis>" is "{action}" with more than one value."
                },
            )
            .note(markup! {
                "A listener written inline in a template, such as "<Emphasis>"@change=\"onChange($event)\""</Emphasis>", can only read the first value through "<Emphasis>"$event"</Emphasis>". The other values are lost."
            })
            .note(markup! {
                "Put all the values in one object, and send that object instead."
            }),
        )
    }
}

declare_node_union! {
    /// A function that validates the values of an event in the `emits` option.
    pub AnyVueEmitValidator = AnyJsFunction | JsMethodObjectMember
}

impl AnyVueEmitValidator {
    fn parameters(&self) -> Option<JsParameters> {
        match self {
            Self::AnyJsFunction(function) => function.parenthesized_parameters(),
            Self::JsMethodObjectMember(method) => method.parameters().ok(),
        }
    }
}

/// Returns the range of the values after the first one in the declaration of
/// `emit`, found in the tree rooted at `root`.
fn extra_declared_payloads(root: &JsSyntaxNode, emit: &Emit) -> Option<TextRange> {
    match emit.form()? {
        // `change: (value, extra) => true`
        EmitForm::Object => {
            let validator = root
                .covering_element(emit.validator_range()?)
                .ancestors()
                .find_map(AnyVueEmitValidator::cast)?;
            range_from(&validator.parameters()?.items(), 1)
        }
        // `change: [value: string, extra: number]`
        EmitForm::TypeMember => {
            let property = root
                .covering_element(emit.declaration_range())
                .ancestors()
                .find_map(TsPropertySignatureTypeMember::cast)?;
            let AnyTsType::TsTupleType(tuple) = property.type_annotation()?.ty().ok()? else {
                return None;
            };
            range_from(&tuple.elements(), 1)
        }
        // `(e: 'change', value: string, extra: number): void`
        EmitForm::CallSignature => {
            let signature = root
                .covering_element(emit.declaration_range())
                .ancestors()
                .find_map(TsCallSignatureTypeMember::cast)?;
            range_from(&signature.parameters().ok()?.items(), 2)
        }
        EmitForm::ArrayString => None,
    }
}

/// Returns the range from the item at index `start` of `list` to its last
/// item, or `None` when the list has no item at that index.
fn range_from<L: AstSeparatedList>(list: &L, start: usize) -> Option<TextRange> {
    let mut items = list.iter().skip(start).flatten();
    let first = items.next()?.range();
    Some(items.last().map_or(first, |last| first.cover(last.range())))
}
