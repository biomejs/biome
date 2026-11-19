// should not generate diagnostics
function run(value?: number) {
    // biome-ignore lint/nursery/useStrictBooleanExpressions: Deliberately treat zero as absent.
    if (value) {}
}
