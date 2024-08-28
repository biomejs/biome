/// Sorted array of Node builtin
///
/// Source: <https://github.com/sindresorhus/globals/blob/9e2e2598dabdb845ff76c0c3acf5c52c812a64de/globals.json#L1801-L1869>
pub const BUILTIN: &[&str] = &[
    "AbortController",
    "AbortSignal",
    "Blob",
    "BroadcastChannel",
    "Buffer",
    "ByteLengthQueuingStrategy",
    "CompressionStream",
    "CountQueuingStrategy",
    "Crypto",
    "CryptoKey",
    "CustomEvent",
    "DOMException",
    "DecompressionStream",
    "Event",
    "EventTarget",
    "File",
    "FormData",
    "Headers",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "Navigator",
    "Performance",
    "PerformanceEntry",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformanceResourceTiming",
    "ReadableByteStreamController",
    "ReadableStream",
    "ReadableStreamBYOBReader",
    "ReadableStreamBYOBRequest",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "Request",
    "Response",
    "SubtleCrypto",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TransformStream",
    "TransformStreamDefaultController",
    "URL",
    "URLSearchParams",
    "WebAssembly",
    "WritableStream",
    "WritableStreamDefaultController",
    "WritableStreamDefaultWriter",
    "atob",
    "btoa",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "crypto",
    "fetch",
    "global",
    "navigator",
    "performance",
    "process",
    "queueMicrotask",
    "setImmediate",
    "setInterval",
    "setTimeout",
    "structuredClone",
];

/// Sorted array of Node
///
/// Source: <https://github.com/sindresorhus/globals/blob/9e2e2598dabdb845ff76c0c3acf5c52c812a64de/globals.json#L1727-L1800>
pub const NODE: &[&str] = &[
    "AbortController",
    "AbortSignal",
    "Blob",
    "BroadcastChannel",
    "Buffer",
    "ByteLengthQueuingStrategy",
    "CompressionStream",
    "CountQueuingStrategy",
    "Crypto",
    "CryptoKey",
    "CustomEvent",
    "DOMException",
    "DecompressionStream",
    "Event",
    "EventTarget",
    "File",
    "FormData",
    "Headers",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "Navigator",
    "Performance",
    "PerformanceEntry",
    "PerformanceMark",
    "PerformanceMeasure",
    "PerformanceObserver",
    "PerformanceObserverEntryList",
    "PerformanceResourceTiming",
    "ReadableByteStreamController",
    "ReadableStream",
    "ReadableStreamBYOBReader",
    "ReadableStreamBYOBRequest",
    "ReadableStreamDefaultController",
    "ReadableStreamDefaultReader",
    "Request",
    "Response",
    "SubtleCrypto",
    "TextDecoder",
    "TextDecoderStream",
    "TextEncoder",
    "TextEncoderStream",
    "TransformStream",
    "TransformStreamDefaultController",
    "URL",
    "URLSearchParams",
    "WebAssembly",
    "WritableStream",
    "WritableStreamDefaultController",
    "WritableStreamDefaultWriter",
    "__dirname",
    "__filename",
    "atob",
    "btoa",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "crypto",
    "exports",
    "fetch",
    "global",
    "module",
    "navigator",
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
///
/// Source: <https://github.com/sindresorhus/globals/blob/9e2e2598dabdb845ff76c0c3acf5c52c812a64de/globals.json#L1870-L1875>
pub const COMMON_JS: &[&str] = &["exports", "global", "module", "require"];

/// Returns `true` if `name` is a node global
///
/// ```
/// use biome_js_analyze::globals::javascript::node::is_global;
///
/// assert!(is_global(&"__dirname"));
/// ```
pub fn is_global(name: &str) -> bool {
    NODE.binary_search(&name).is_ok()
}

#[test]
fn test_order() {
    for items in BUILTIN.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in NODE.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
    for items in COMMON_JS.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
