/* should not generate diagnostics */

// biome-ignore lint/nursery/noUnreadableForOfExpression: test
for (const item of getItems(createArgument(seed))) {}

// biome-ignore lint/nursery/noUnreadableForOfExpression: test
for (const item of getItems(
	createArgument(seed),
)) {}

for (
	// biome-ignore lint/nursery/noUnreadableForOfExpression: test
	const item of getItems(createArgument(seed))
) {}
