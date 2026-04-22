use biome_string_case::StrOnlyExtension;

// https://github.com/jsx-eslint/jsx-ast-utils/blob/v3.3.5/src/eventHandlers.js
const EVENT_HANDLERS: &[(&str, &[&str])] = &[
    (
        "animation",
        &["onanimationend", "onanimationiteration", "onanimationstart"],
    ),
    ("clipboard", &["oncopy", "oncut", "onpaste"]),
    (
        "composition",
        &[
            "oncompositionend",
            "oncompositionstart",
            "oncompositionupdate",
        ],
    ),
    ("focus", &["onblur", "onfocus"]),
    ("form", &["onchange", "oninput", "onsubmit"]),
    ("image", &["onerror", "onload"]),
    ("keyboard", &["onkeydown", "onkeypress", "onkeyup"]),
    (
        "media",
        &[
            "onabort",
            "oncanplay",
            "oncanplaythrough",
            "ondurationchange",
            "onemptied",
            "onencrypted",
            "onended",
            "onerror",
            "onloadeddata",
            "onloadedmetadata",
            "onloadstart",
            "onpause",
            "onplay",
            "onplaying",
            "onprogress",
            "onratechange",
            "onseeked",
            "onseeking",
            "onstalled",
            "onsuspend",
            "ontimeupdate",
            "onvolumechange",
            "onwaiting",
        ],
    ),
    (
        "mouse",
        &[
            "onclick",
            "oncontextmenu",
            "ondblclick",
            "ondoubleclick",
            "ondrag",
            "ondragend",
            "ondragenter",
            "ondragexit",
            "ondragleave",
            "ondragover",
            "ondragstart",
            "ondrop",
            "onmousedown",
            "onmouseenter",
            "onmouseleave",
            "onmousemove",
            "onmouseout",
            "onmouseover",
            "onmouseup",
        ],
    ),
    ("selection", &["onselect"]),
    (
        "touch",
        &["ontouchcancel", "ontouchend", "ontouchmove", "ontouchstart"],
    ),
    ("transition", &["ontransitionend"]),
    ("ui", &["onscroll"]),
    ("wheel", &["onwheel"]),
];

pub fn matches_event_handler(handler_types: &[&str], event_handler: &str) -> bool {
    let event_handler_lower = event_handler.to_lowercase_cow();
    handler_types.iter().any(|handler_type| {
        EVENT_HANDLERS
            .binary_search_by_key(handler_type, |&(a, _)| a)
            .is_ok_and(|idx| {
                EVENT_HANDLERS[idx]
                    .1
                    .binary_search(&event_handler_lower.as_ref())
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
