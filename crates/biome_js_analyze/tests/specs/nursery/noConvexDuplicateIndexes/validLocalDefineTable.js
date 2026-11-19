/* should not generate diagnostics */
// `defineTable` isn't imported.
function defineTable() {}

defineTable({})
	.index("by_author", ["author"])
	.index("by_author_and_channel", ["author", "channel"]);
