/* should not generate diagnostics */
import { defineTable } from "convex/server";

// Type assertions don't hide that an index is staged.
defineTable({})
	.index("by_author", ["author"])
	.index("by_author_and_channel", { fields: ["author", "channel"], staged: true as const });
