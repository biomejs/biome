use crate::frameworks::vue::vue_instance::{
    is_in_component_definition_object, is_this_or_direct_const_alias,
};
use crate::services::semantic::Semantic;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_js_syntax::{AnyJsMemberExpression, JsCallExpression};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, TextRange};
use biome_rule_options::no_vue_deprecated_events_api::NoVueDeprecatedEventsApiOptions;

declare_lint_rule! {
    /// Disallow the Vue `$on`, `$off`, and `$once` instance methods.
    ///
    /// Vue 3 removed the `$on`, `$off`, and `$once` methods from component instances, so a component can no longer
    /// listen to its own events with them. Calling one of these methods throws an error, because the method doesn't exist.
    ///
    /// To react to an event that a child component emits, listen to it with `v-on` (or `@`) in the parent's template.
    /// To send events between components that aren't parent and child, use a small event library such as
    /// [mitt](https://github.com/developit/mitt) or [tiny-emitter](https://github.com/scottcorgan/tiny-emitter).
    ///
    /// This rule checks calls on `this`, and on variables declared with `const vm = this`, inside the options object
    /// of a component: the default export of a `.vue` file, or the object passed to `defineComponent()` or `createApp()`.
    /// Calls written as `this.$on?.()` are allowed, because they only run when the method exists.
    ///
    /// See the [Vue 3 migration guide](https://v3-migration.vuejs.org/breaking-changes/events-api.html) for more information.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   mounted() {
    ///     this.$on("start", () => {
    ///       console.log("start");
    ///     });
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// import { defineComponent } from "vue";
    ///
    /// export default defineComponent({
    ///   beforeUnmount() {
    ///     this.$off("start");
    ///   },
    /// });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script>
    /// import mitt from "mitt";
    ///
    /// const emitter = mitt();
    ///
    /// export default {
    ///   mounted() {
    ///     emitter.on("start", () => {
    ///       console.log("start");
    ///     });
    ///     this.$emit("ready");
    ///   },
    /// };
    /// </script>
    /// ```
    pub NoVueDeprecatedEventsApi {
        version: "next",
        name: "noVueDeprecatedEventsApi",
        language: "js",
        sources: &[RuleSource::EslintVueJs("no-deprecated-events-api").inspired()],
        recommended: true,
        severity: Severity::Error,
        domains: &[RuleDomain::Vue],
    }
}

impl Rule for NoVueDeprecatedEventsApi {
    type Query = Semantic<JsCallExpression>;
    type State = EventsApiMethod;
    type Signals = Option<Self::State>;
    type Options = NoVueDeprecatedEventsApiOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let call = ctx.query();
        if call.is_optional() {
            return None;
        }
        let member = deprecated_events_api_callee(call)?;
        let method = EventsApiMethod::from_name(member.member_name()?.text())?;
        let object = member.object().ok()?;
        if !is_this_or_direct_const_alias(object.clone(), ctx.model())
            || !is_in_component_definition_object(
                &object,
                ctx.model(),
                ctx.source_type::<JsFileSource>(),
                ctx.file_path(),
            )
        {
            return None;
        }

        Some(method)
    }

    fn diagnostic(ctx: &RuleContext<Self>, method: &Self::State) -> Option<RuleDiagnostic> {
        let member = deprecated_events_api_callee(ctx.query())?;
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                member_name_range(&member)?,
                markup! {
                    "The Vue "<Emphasis>{method}</Emphasis>" instance method was removed in Vue 3."
                },
            )
            .note(markup! {
                "Component instances no longer have this method, so calling it throws an error."
            })
            .note(markup! {
                "Listen to a child component's events with "<Emphasis>"v-on"</Emphasis>" in the parent's template, or use an event library such as "<Emphasis>"mitt"</Emphasis>". See the "<Hyperlink href="https://v3-migration.vuejs.org/breaking-changes/events-api.html">"Vue 3 migration guide"</Hyperlink>" for more information."
            }),
        )
    }
}

/// A Vue 2 instance method for listening to the component's own events.
#[derive(Clone, Copy)]
pub enum EventsApiMethod {
    On,
    Off,
    Once,
}

impl EventsApiMethod {
    fn from_name(name: &str) -> Option<Self> {
        match name {
            "$on" => Some(Self::On),
            "$off" => Some(Self::Off),
            "$once" => Some(Self::Once),
            _ => None,
        }
    }
}

impl biome_console::fmt::Display for EventsApiMethod {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::On => "$on",
            Self::Off => "$off",
            Self::Once => "$once",
        })
    }
}

/// Returns the member expression that `call` calls, looking through
/// parentheses and TypeScript wrappers such as `this.$on!`.
fn deprecated_events_api_callee(call: &JsCallExpression) -> Option<AnyJsMemberExpression> {
    AnyJsMemberExpression::cast(call.callee().ok()?.inner_expression()?.into_syntax())
}

/// Returns the range of the member name: `$on` in `this.$on`, or `"$on"` in
/// `this["$on"]`.
fn member_name_range(member: &AnyJsMemberExpression) -> Option<TextRange> {
    Some(match member {
        AnyJsMemberExpression::JsStaticMemberExpression(member) => member.member().ok()?.range(),
        AnyJsMemberExpression::JsComputedMemberExpression(member) => {
            member.member().ok()?.range()
        }
    })
}
