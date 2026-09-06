// should generate diagnostics

// `x` is a floating promise with or without narrowing. This case exists to
// catch narrowing suppressing the diagnostic, not to prove that narrowing
// happens -- the inferred types are pinned by
// `biome_module_graph/tests/spec_tests/narrowing.test.rs`.
function narrowedToPromise(x: Promise<void> | undefined) {
	if (x) {
		x;
	}
}
