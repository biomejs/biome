/* should generate diagnostics */
import { defineTable } from "convex/server";
import { defineTable as table } from "./helpers";
import * as server from "convex/server";
import { v } from "convex/values";

// A prefix of a later index.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", ["author"])
	.index("by_author_and_channel", ["author", "channel"]);

// A prefix of an earlier index.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author_and_channel", ["author", "channel"])
	.index("by_author", ["author"]);

// Same fields: only the later index is reported.
defineTable({ author: v.string() })
	.index("by_author", ["author"])
	.index("by_writer", ["author"]);

// Each redundant index is reported once, against the first index that covers it.
defineTable({ a: v.string(), b: v.string(), c: v.string() })
	.index("by_a", ["a"])
	.index("by_a_b", ["a", "b"])
	.index("by_a_b_c", ["a", "b", "c"]);

// Renamed import.
table({ author: v.string(), channel: v.string() })
	.index("by_author", ["author"])
	.index("by_author_and_channel", ["author", "channel"]);

// Namespace import.
server
	.defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", ["author"])
	.index("by_author_and_channel", ["author", "channel"]);

// Other calls in the chain are followed.
defineTable({ author: v.string(), body: v.string() })
	.index("by_author", ["author"])
	.searchIndex("search_body", { searchField: "body" })
	.index("by_author_and_body", ["author", "body"]);

// Config objects, `staged: false`, template literals and quoted keys.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", { fields: [`author`], staged: false })
	.index(`by_author_and_channel`, { "fields": ["author", "channel"] });

// Parentheses around the table.
(defineTable({ author: v.string(), channel: v.string() }))
	.index("by_author", ["author"])
	.index("by_author_and_channel", ["author", "channel"]);

// Tables inside `defineSchema`.
export default defineSchema({
	messages: defineTable({ author: v.string(), channel: v.string() })
		.index("by_author", ["author"])
		.index("by_author_and_channel", ["author", "channel"]),
});

// Computed keys with a constant name are read like plain keys.
defineTable({ author: v.string(), channel: v.string() })
	.index("by_author", { ["fields"]: ["author"], [`staged`]: false })
	.index("by_author_and_channel", ["author", "channel"]);
