// should generate diagnostics
function check(value: { count?: number, label?: string, enabled?: boolean }, array: unknown[] | undefined) {
    if (value.count) {}
    if (value.label) {}
    if (value.enabled) {}
    if (array?.length) {}
}
