/// Sorted array of Node builtin
pub const BUILTIN: [&str; 29] = [
    "AbortController",
    "AbortSignal",
    "Buffer",
    "DOMException",
    "Event",
    "EventTarget",
    "Intl",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "TextDecoder",
    "TextEncoder",
    "URL",
    "URLSearchParams",
    "atob",
    "btoa",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "fetch",
    "global",
    "performance",
    "process",
    "queueMicrotask",
    "setImmediate",
    "setInterval",
    "setTimeout",
    "structuredClone",
];

pub const NODE: [&str; 34] = [
    "AbortController",
    "AbortSignal",
    "Buffer",
    "DOMException",
    "Event",
    "EventTarget",
    "Intl",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "TextDecoder",
    "TextEncoder",
    "URL",
    "URLSearchParams",
    "__dirname",
    "__filename",
    "atob",
    "btoa",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "exports",
    "fetch",
    "global",
    "module",
    "performance",
    "process",
    "queueMicrotask",
    "require",
    "setImmediate",
    "setInterval",
    "setTimeout",
    "structuredClone",
];

/// Sorted array of CommonJs builtin
pub const COMMON_JS: [&str; 4] = ["exports", "global", "module", "require"];

#[test]
fn test_order() {
    for items in BUILTIN.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in COMMON_JS.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
