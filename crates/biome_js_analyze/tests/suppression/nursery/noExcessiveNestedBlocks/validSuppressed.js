/* should not generate diagnostics */
function foo() {
    if (a) {
        if (b) {
            if (c) {
                if (d) {
                    // biome-ignore lint/nursery/noExcessiveNestedBlocks: The nesting mirrors the spec.
                    if (e) {
                        if (f) {}
                    }
                }
            }
        }
    }
}
