/* should not generate diagnostics */
import { defineTable } from "convex/server";
import { v } from "convex/values";

// Neither index starts with all the fields of the other one.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_channel", ["channel"])
	.index("by_author_and_channel", ["author", "channel"]);

// Same fields in a different order.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author_and_channel", ["author", "channel"])
	.index("by_channel_and_author", ["channel", "author"]);

// A single index.
defineTable({ author: v.string() }).index("by_author", ["author"]);

// Staged indexes are skipped.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", ["author"])
	.index("by_author_and_channel", { fields: ["author", "channel"], staged: true });

defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", { fields: ["author"], staged: isStaged })
	.index("by_author_and_channel", ["author", "channel"]);

// Configs that might set `staged` or `fields` in a way that can't be read are skipped.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", { fields: ["author"], ...options })
	.index("by_author_and_channel", ["author", "channel"]);

defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", { fields: ["author"], [key]: true })
	.index("by_author_and_channel", ["author", "channel"]);

// Names and fields that aren't plain strings are skipped.
defineTable({ author: v.string(), channel: v.string() })
	.index(name, ["author"])
	.index("by_author_and_channel", ["author", "channel"]);

defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", [author])
	.index("by_author_and_channel", ["author", "channel"]);

defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", fields)
	.index("by_author_and_channel", ["author", "channel"]);

defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", [...fields])
	.index("by_author_and_channel", ["author", "channel"]);

// Search and vector indexes aren't database indexes.
defineTable({ author: v.string(), body: v.string() })
	.searchIndex("search_body", { searchField: "body", filterFields: ["author"] })
	.index("by_author_and_body", ["author", "body"]);

// Indexes added in a separate statement aren't followed.
const messages = defineTable({ author: v.string(), channel: v.string() }).index("by_author", ["author"]);
messages.index("by_author_and_channel", ["author", "channel"]);

// Different tables.
defineSchema({
	a: defineTable({ author: v.string() }).index("by_author", ["author"]),
	b: defineTable({ author: v.string() }).index("by_author", ["author"]),
});

// Computed member access isn't followed.
defineTable({ author: v.string(), channel: v.string() })
	["index"]("by_author", ["author"])
	.index("by_author_and_channel", ["author", "channel"]);

function shadowed(defineTable) {
	defineTable({ author: v.string(), channel: v.string() })
		.index("by_author", ["author"])
		.index("by_author_and_channel", ["author", "channel"]);
}
