use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_js_syntax::{AnyJsExpression, JsCallExpression};
use biome_languages::JsFileSource;
use biome_rowan::AstNode;
use biome_rule_options::use_svelte_event_dispatcher_types::UseSvelteEventDispatcherTypesOptions;

use crate::frameworks::is_framework_api_reference;
use crate::services::semantic::Semantic;

declare_lint_rule! {
    /// Require `createEventDispatcher` calls to declare the types of their events.
    ///
    /// `createEventDispatcher` returns a function that a Svelte component uses to send
    /// events to its parent. In TypeScript, the events are described between angle
    /// brackets: `createEventDispatcher<{ save: string }>()` declares one event named
    /// `save` that carries a `string`.
    ///
    /// When that description is missing, TypeScript accepts any event name with any
    /// data. A misspelled event name or the wrong kind of data is not reported, and the
    /// parent component doesn't know what its event handlers receive.
    ///
    /// This rule only checks Svelte components whose script uses TypeScript
    /// (`<script lang="ts">`), and only calls to the `createEventDispatcher` function
    /// imported from `svelte`.
    ///
    /// `createEventDispatcher` is how components send events in Svelte 3 and Svelte 4.
    /// Svelte 5 deprecates it: there, a component receives functions from its parent
    /// and calls them instead of sending events.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```svelte,expect_diagnostic
    /// <script lang="ts">
    /// import { createEventDispatcher } from "svelte";
    ///
    /// const dispatch = createEventDispatcher();
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```svelte
    /// <script lang="ts">
    /// import { createEventDispatcher } from "svelte";
    ///
    /// const dispatch = createEventDispatcher<{ save: string; close: null }>();
    /// </script>
    /// ```
    ///
    /// ### References
    ///
    /// - [Svelte `createEventDispatcher`](https://svelte.dev/docs/svelte/svelte#createEventDispatcher)
    pub UseSvelteEventDispatcherTypes {
        version: "2.6.4",
        name: "useSvelteEventDispatcherTypes",
        language: "js",
        domains: &[RuleDomain::Svelte],
        sources: &[RuleSource::EslintSvelte("require-event-dispatcher-types").same()],
        recommended: true,
    }
}

impl Rule for UseSvelteEventDispatcherTypes {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSvelteEventDispatcherTypesOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let source_type = ctx.source_type::<JsFileSource>();
        if !source_type.is_typescript() || !source_type.as_embedding_kind().is_svelte_component() {
            return None;
        }

        let call = ctx.query();
        if call.type_arguments().is_some() {
            return None;
        }

        // `is_framework_api_reference` looks through type syntax around the callee, but a
        // callee such as `createEventDispatcher<Events>` or a cast already types the call.
        let callee = call.callee().ok()?.omit_parentheses();
        if !matches!(
            callee,
            AnyJsExpression::JsIdentifierExpression(_)
                | AnyJsExpression::JsStaticMemberExpression(_)
                | AnyJsExpression::JsComputedMemberExpression(_)
        ) {
            return None;
        }

        is_framework_api_reference(
            &callee,
            ctx.model(),
            CREATE_EVENT_DISPATCHER,
            &[SVELTE_MODULE],
            None,
        )
        .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This "<Emphasis>"createEventDispatcher"</Emphasis>" call doesn't declare the types of its events."
                },
            )
            .note(markup! {
                "Without them, TypeScript accepts any event name with any data, so mistakes in the events this component sends aren't reported."
            })
            .note(markup! {
                "Describe the events between angle brackets, for example "<Emphasis>"createEventDispatcher<{ save: string }>()"</Emphasis>"."
            }),
        )
    }
}

const CREATE_EVENT_DISPATCHER: &str = "createEventDispatcher";
const SVELTE_MODULE: &str = "svelte";
