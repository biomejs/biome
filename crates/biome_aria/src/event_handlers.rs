use biome_string_case::StrOnlyExtension;

// https://github.com/jsx-eslint/jsx-ast-utils/blob/v3.3.5/src/eventHandlers.js
// Removed "on" from event-handlers to prevent string allocations when adding leading "on" for vue event bindings that don't start with "on"
const EVENT_HANDLERS: &[(&str, &[&str])] = &[
    (
        "animation",
        &["animationend", "animationiteration", "animationstart"],
    ),
    ("clipboard", &["copy", "cut", "paste"]),
    (
        "composition",
        &["compositionend", "compositionstart", "compositionupdate"],
    ),
    ("focus", &["blur", "focus"]),
    ("form", &["change", "input", "submit"]),
    ("image", &["error", "load"]),
    ("keyboard", &["keydown", "keypress", "keyup"]),
    (
        "media",
        &[
            "abort",
            "canplay",
            "canplaythrough",
            "durationchange",
            "emptied",
            "encrypted",
            "ended",
            "error",
            "loadeddata",
            "loadedmetadata",
            "loadstart",
            "pause",
            "play",
            "playing",
            "progress",
            "ratechange",
            "seeked",
            "seeking",
            "stalled",
            "suspend",
            "timeupdate",
            "volumechange",
            "waiting",
        ],
    ),
    (
        "mouse",
        &[
            "click",
            "contextmenu",
            "dblclick",
            "doubleclick",
            "drag",
            "dragend",
            "dragenter",
            "dragexit",
            "dragleave",
            "dragover",
            "dragstart",
            "drop",
            "mousedown",
            "mouseenter",
            "mouseleave",
            "mousemove",
            "mouseout",
            "mouseover",
            "mouseup",
        ],
    ),
    ("selection", &["select"]),
    (
        "touch",
        &["touchcancel", "touchend", "touchmove", "touchstart"],
    ),
    ("transition", &["transitionend"]),
    ("ui", &["scroll"]),
    ("wheel", &["wheel"]),
];

pub fn matches_event_handler(handler_types: &[&str], event_handler: &str) -> bool {
    let binding = event_handler.to_lowercase_cow();
    let mut event_handler_lower = binding.as_ref();
    if event_handler_lower.starts_with("on") {
        event_handler_lower = &event_handler_lower[2..];
    }
    handler_types.iter().any(|handler_type| {
        EVENT_HANDLERS
            .binary_search_by_key(handler_type, |&(a, _)| a)
            .is_ok_and(|idx| {
                EVENT_HANDLERS[idx]
                    .1
                    .binary_search(&event_handler_lower)
                    .is_ok()
            })
    })
}

#[test]
fn test_order() {
    assert!(EVENT_HANDLERS.is_sorted_by_key(|(k, _)| *k));

    for (_, v) in EVENT_HANDLERS.iter() {
        assert!(v.is_sorted());
    }
}
