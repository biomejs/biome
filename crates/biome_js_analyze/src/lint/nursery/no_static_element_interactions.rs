use crate::services::aria::Aria;
use biome_analyze::context::RuleContext;
use biome_analyze::{declare_rule, Rule, RuleDiagnostic, RuleSource};
use biome_console::markup;
use biome_js_syntax::jsx_ext::AnyJsxElement;
use biome_rowan::AstNode;

declare_rule! {
    /// Enforce that non-interactive, visible elements (such as `<div>`) that have click handlers use the role attribute.
    ///
    /// Static HTML elements do not have semantic meaning. This is clear in the case of `<div>` and `<span>`. It is less so clear in the case of elements that seem semantic, but that do not have a semantic mapping in the accessibility layer. For example `<a>`, `<big>`, `<blockquote>`, `<footer>`, `<picture>`, `<strike>`, and `<time>` -- to name a few -- have no semantic layer mapping. They are as void of meaning as `<div>`.
    ///
    /// The [WAI-ARIA role attribute](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) confers a semantic mapping to an element. The semantic value can then be expressed to a user via assistive technology.
    /// In order to add interactivity such as a mouse or key event listener to a static element, that element must be given a role value as well.
    ///
    /// Source: [jsx-a11y/no-static-element-interactions](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-static-element-interactions.md)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}}></div>;
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <span onClick={() => {}}></span>;
    /// ```
    ///
    /// When `<a>` does not have "href" attribute, that is non-interactive.
    /// ```jsx,expect_diagnostic
    /// <a onClick={() => {}}></a>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <div role="button" onClick={() => {}}></div>
    ///     <span role="link" onClick={() => {}}></span>
    ///     <a href="http://example.com" onClick={() => {}}></a>
    /// </>
    /// ```
    ///
    pub NoStaticElementInteractions {
        version: "next",
        name: "noStaticElementInteractions",
        language: "js",
        sources: &[RuleSource::EslintJsxA11y("no-static-element-interactions")],
        recommended: false,
    }
}

// EVENT_TO_HANDLERS is a mapping of event categories to their corresponding event handlers.
// For example, the "keyboard" category includes handlers like "onKeyDown", "onKeyPress", and "onKeyUp".
const EVENT_TO_HANDLERS: &[(&str, &[&str])] = &[
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/ClipboardEvent
    //     "clipboard",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/copy_event
    //         "onCopy",
    //         //ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/cut_event
    //         "onCut",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/paste_event
    //         "onPaste",
    //     ],
    // ),
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/CompositionEvent
    //     "composition",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionstart_event
    //         "onCompositionStart",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionend_event
    //         "onCompositionEnd",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/compositionupdate_event
    //         "onCompositionUpdate",
    //     ],
    // ),
    (
        // ref https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent
        "keyboard",
        &[
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/keydown_event
            "onKeyDown",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/keyup_event
            "onKeyUp",
            //ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/keypress_event
            "onKeyPress",
        ],
    ),
    (
        // ref: https://developer.mozilla.org/en-US/docs/Web/API/FocusEvent
        "focus",
        &[
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/focus_event
            "onFocus",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/blur_event
            "onBlur",
        ],
    ),
    // (
    //     "form",
    //     &[
    //         /// ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/change_event
    //         "onChange",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/input_event
    //         "onInput",
    //         // https://developer.mozilla.org/en-US/docs/Web/API/HTMLFormElement/submit_event
    //         "onSubmit",
    //     ],
    // ),
    (
        // ref: https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent
        "mouse",
        &[
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/click_event
            "onClick",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/contextmenu_event
            "onContextMenu",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/dblclick_event
            "onDblClick",
            "onDoubleClick",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drag_event
            "onDrag",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragend_event
            "onDragEnd",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragenter_event
            "onDragEnter",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragleave_event
            "onDragLeave",
            "onDragExit",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragover_event
            "onDragOver",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/dragstart_event
            "onDragStart",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/drop_event
            "onDrop",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousedown_event
            "onMouseDown",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseenter_event
            "onMouseEnter",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseleave_event
            "onMouseLeave",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mousemove_event
            "onMouseMove",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseout_event
            "onMouseOut",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseover_event
            "onMouseOver",
            // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/mouseup_event
            "onMouseUp",
        ],
    ),
    // (
    //     "selection",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLInputElement/select_event
    //         "onSelect",
    //     ],
    // ),
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/TouchEvent
    //     "touch",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/touchcancel_event
    //         "onTouchCancel",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/touchend_event
    //         "onTouchEnd",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/touchmove_event
    //         "onTouchMove",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/touchstart_event
    //         "onTouchStart",
    //     ],
    // ),
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/UIEvent
    //     "ui",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/scroll_event
    //         "onScroll",
    //     ],
    // ),
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/WheelEvent
    //     "wheel",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/wheel_event
    //         "onWheel",
    //     ],
    // ),
    // (
    //     "media",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/abort_event
    //         "onAbort",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplay_event
    //         "onCanPlay",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/canplaythrough_event
    //         "onCanPlayThrough",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/durationchange_event
    //         "onDurationChange",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/emptied_event
    //         "onEmptied",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/encrypted_event
    //         "onEncrypted",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ended_event
    //         "onEnded",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/ErrorEvent
    //         "onError",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadeddata_event
    //         "onLoadedData",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadedmetadata_event
    //         "onLoadedMetadata",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/loadstart_event
    //         "onLoadStart",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/pause_event
    //         "onPause",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play_event
    //         "onPlay",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/playing_event
    //         "onPlaying",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/progress_event
    //         "onProgress",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/ratechange_event
    //         "onRateChange",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeked_event
    //         "onSeeked",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/seeking_event
    //         "onSeeking",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/stalled_event
    //         "onStalled",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/suspend_event
    //         "onSuspend",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/timeupdate_event
    //         "onTimeUpdate",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/volumechange_event
    //         "onVolumeChange",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/waiting_event
    //         "onWaiting",
    //     ],
    // ),
    // (
    //     "image",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/load_event
    //         "onLoad",
    //     ],
    // ),
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/AnimationEvent
    //     "animation",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/animationstart_event
    //         "onAnimationStart",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/animationend_event
    //         "onAnimationEnd",
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/animationiteration_event
    //         "onAnimationIteration",
    //     ],
    // ),
    // (
    //     // ref: https://developer.mozilla.org/en-US/docs/Web/API/TransitionEvent
    //     "transition",
    //     &[
    //         // ref: https://developer.mozilla.org/en-US/docs/Web/API/Element/transitionend_event
    //         "onTransitionEnd",
    //     ],
    // ),
];

// no-static-element-interactions rule checks only focus, keyboard and mouse categories.
const CATEGORIES_TO_CHECK: &[&str] = &["focus", "keyboard", "mouse"];

impl Rule for NoStaticElementInteractions {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let aria_roles = ctx.aria_roles();
        let attributes = ctx.extract_attributes(&node.attributes());
        let element_name = element_name.text_trimmed();

        // Check if the element is hidden from screen readers.
        if is_hidden_from_screen_reader(node, element_name) {
            return None;
        }

        if let Some(attributes) = &attributes {
            if aria_roles.is_not_static_element(element_name, attributes) {
                return None;
            }

            // Check if the element has any interactive event handlers.
            if !CATEGORIES_TO_CHECK.iter().any(|&category| {
                if let Some(handlers) = EVENT_TO_HANDLERS
                    .iter()
                    .find(|&&(cat, _)| cat == category)
                    .map(|&(_, handlers)| handlers)
                {
                    handlers.iter().any(|&handler| {
                        if let Some(values) = &attributes.get(handler) {
                            values.iter().any(|value| value != "null")
                        } else {
                            false
                        }
                    })
                } else {
                    false
                }
            }) {
                return None;
            }
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {{"Static Elements should not be interactive."}},
        ).note(
            markup! {{"To add interactivity such as a mouse or key event listener to a static element, give the element an appropriate role value."}}
        ))
    }
}

/**
 * Returns boolean indicating that the aria-hidden prop
 * is present or the value is true. Will also return true if
 * there is an input with type='hidden'.
 *
 * <div aria-hidden /> is equivalent to the DOM as <div aria-hidden=true />.
 * ref: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/src/util/isHiddenFromScreenReader.js
 */
fn is_hidden_from_screen_reader(node: &AnyJsxElement, element_name: &str) -> bool {
    node.find_attribute_by_name("aria-hidden")
        .map_or(false, |attr| {
            attr.as_static_value()
                .map_or(true, |val| val.text() == "true")
        })// <div aria-hidden />
        || (element_name == "input"
            && node.find_attribute_by_name("type").map_or(false, |attr| {
                attr.as_static_value()
                    .map_or(false, |val| val.text() == "hidden")
            })) // <input type="hidden" />
}
