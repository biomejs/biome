/// Sorted array of TypeScript Node globals
///
/// Source: <https://github.com/DefinitelyTyped/DefinitelyTyped/blob/8279710799f3c39d5cd129cac5705b809b66022a/types/node/globals.d.ts#L17-L411>
pub const NODE: &[&str] = &[
    "AbortController",
    "AbortSignal",
    "Array",
    "AsyncDisposable",
    "BigInt64Array",
    "BigUint64Array",
    "Disposable",
    "ErrorConstructor",
    "File",
    "Float32Array",
    "Float64Array",
    "FormData",
    "Headers",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "NodeJS",
    "NodeModule",
    "NodeRequire",
    "ReadonlyArray",
    "RelativeIndexable",
    "Request",
    "RequestInit",
    "RequireResolve",
    "Response",
    "ResponseInit",
    "String",
    "SymbolConstructor",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "__dirname",
    "__filename",
    "console",
    "exports",
    "fetch",
    "gc",
    "module",
    "process",
    "require",
    "structuredClone",
];

/// Returns `true` if `name` is a TypeScript node global
///
/// ```
/// use biome_js_analyze::globals::typescript::node::is_global;
///
/// assert!(is_global(&"RelativeIndexable"));
/// ```
pub fn is_global(name: &str) -> bool {
    NODE.binary_search(&name).is_ok()
}

#[test]
fn test_order() {
    for items in NODE.windows(2) {
        assert!(items[0] < items[1], "{} < {}", items[0], items[1]);
    }
}
