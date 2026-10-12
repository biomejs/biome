/* should not generate diagnostics */
// biome-ignore lint/nursery/noEagerArrayMethodsInLoops: test
for (const item of items.map(callback)) {}

// biome-ignore lint/nursery/noEagerArrayMethodsInLoops: test
for (const item of items
	.filter((item) => item.isEnabled)
) {}

for (
	// biome-ignore lint/nursery/noEagerArrayMethodsInLoops: test
	const item of items.map(callback)
) {}
