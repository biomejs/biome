// should not generate diagnostics
function check(key: keyof { a: number }) {
    if (key) {}
}
