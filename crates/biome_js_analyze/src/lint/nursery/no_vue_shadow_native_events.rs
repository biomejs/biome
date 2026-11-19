use crate::services::vue::VueEvent;
use biome_analyze::{
    Rule, RuleDiagnostic, RuleDomain, RuleSource, context::RuleContext, declare_lint_rule,
};
use biome_console::markup;
use biome_diagnostics::Severity;
use biome_rule_options::no_vue_shadow_native_events::NoVueShadowNativeEventsOptions;
use biome_string_case::StrLikeExtension;

declare_lint_rule! {
    /// Disallow Vue component events named after built-in browser events.
    ///
    /// A component can send its own events to its parent with `emit`. When such an event has the
    /// same name as a browser event, like `click` or `keydown`, it looks like the browser event but
    /// works differently:
    ///
    /// - The listener receives whatever value the component passes to `emit`, instead of the
    ///   browser's `Event` object.
    /// - The event only reaches the parent that listens to the component. Most browser events also
    ///   reach every element that contains the element where they happened.
    /// - Modifiers that act on the browser's `Event` object, such as `@click.stop` and
    ///   `@click.prevent`, do nothing unless the component passes that object along.
    ///
    /// Give the event a name that describes what happened in the component, such as `save` or
    /// `update:modelValue`.
    ///
    /// This rule checks the event names listed in the `emits` option and in `defineEmits()`. It
    /// also checks the event names passed to `this.$emit()`, to the `emit` function that `setup()`
    /// receives, to the function returned by `defineEmits()`, and to `$emit()` in templates.
    /// Uppercase and lowercase letters are treated the same, so `Click` is reported too.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```vue,expect_diagnostic
    /// <script setup>
    /// defineEmits(["click"]);
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script setup lang="ts">
    /// defineEmits<{ keydown: [key: string] }>();
    /// </script>
    /// ```
    ///
    /// ```vue,expect_diagnostic
    /// <script>
    /// export default {
    ///   methods: {
    ///     onClick() {
    ///       this.$emit("change", 42);
    ///     },
    ///   },
    /// };
    /// </script>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```vue
    /// <script setup>
    /// const emit = defineEmits(["save"]);
    /// emit("save");
    /// </script>
    /// ```
    ///
    pub NoVueShadowNativeEvents {
        version: "2.6.0",
        name: "noVueShadowNativeEvents",
        language: "js",
        recommended: false,
        severity: Severity::Warning,
        domains: &[RuleDomain::Vue],
        sources: &[RuleSource::EslintVueJs("no-shadow-native-events").inspired()],
    }
}

impl Rule for NoVueShadowNativeEvents {
    type Query = VueEvent;
    /// The native DOM event the event name matches.
    type State = &'static str;
    type Signals = Option<Self::State>;
    type Options = NoVueShadowNativeEventsOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        find_native_event(ctx.query().name())
    }

    fn diagnostic(ctx: &RuleContext<Self>, native_event: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ctx.query().range(),
                markup! {
                    "This component event has the same name as the browser event "<Emphasis>{native_event}</Emphasis>"."
                },
            )
            .note(markup! {
                "It is easy to mistake for the browser event, but it works differently: listeners receive the value passed to "<Emphasis>"emit"</Emphasis>" instead of an "<Emphasis>"Event"</Emphasis>" object, the event doesn't reach the elements around the component, and modifiers such as "<Emphasis>".stop"</Emphasis>" don't work on it."
            })
            .note(markup! {
                "Rename the event to describe what happened in the component."
            }),
        )
    }
}

/// Native DOM event names, sorted case-insensitively for binary search.
const NATIVE_EVENTS: &[&str] = &[
    "abort",
    "animationcancel",
    "animationend",
    "animationiteration",
    "animationstart",
    "auxclick",
    "beforeinput",
    "beforetoggle",
    "blur",
    "canplay",
    "canplaythrough",
    "change",
    "click",
    "compositionend",
    "compositionstart",
    "compositionupdate",
    "contextmenu",
    "copy",
    "cut",
    "dblclick",
    "drag",
    "dragend",
    "dragenter",
    "dragexit",
    "dragleave",
    "dragover",
    "dragstart",
    "drop",
    "durationchange",
    "emptied",
    "encrypted",
    "ended",
    "error",
    "focus",
    "focusin",
    "focusout",
    "formdata",
    "fullscreenchange",
    "fullscreenerror",
    "gotpointercapture",
    "input",
    "invalid",
    "keydown",
    "keypress",
    "keyup",
    "load",
    "loadeddata",
    "loadedmetadata",
    "loadstart",
    "lostpointercapture",
    "mousedown",
    "mouseenter",
    "mouseleave",
    "mousemove",
    "mouseout",
    "mouseover",
    "mouseup",
    "paste",
    "pause",
    "play",
    "playing",
    "pointercancel",
    "pointerdown",
    "pointerenter",
    "pointerleave",
    "pointermove",
    "pointerout",
    "pointerover",
    "pointerup",
    "progress",
    "ratechange",
    "reset",
    "scroll",
    "scrollend",
    "securitypolicyviolation",
    "seeked",
    "seeking",
    "select",
    "stalled",
    "submit",
    "suspend",
    "timeupdate",
    "toggle",
    "touchcancel",
    "touchend",
    "touchmove",
    "touchstart",
    "transitioncancel",
    "transitionend",
    "transitionrun",
    "transitionstart",
    "volumechange",
    "waiting",
    "wheel",
];

fn find_native_event(name: &str) -> Option<&'static str> {
    NATIVE_EVENTS
        .binary_search_by(|probe| probe.cmp_ignore_ascii_case(name))
        .ok()
        .map(|index| NATIVE_EVENTS[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_events_are_sorted() {
        for pair in NATIVE_EVENTS.windows(2) {
            assert_eq!(
                pair[0].cmp_ignore_ascii_case(pair[1]),
                std::cmp::Ordering::Less,
                "{} must come before {}",
                pair[0],
                pair[1]
            );
        }
    }
}
