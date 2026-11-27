/* should generate diagnostics */
import { defineTable } from "convex/server";
import { v } from "convex/values";

// Type assertions are ignored.
(defineTable({ author: v.string(), channel: v.string() }) as any)
	.index("by_author" as const, ["author"] as const)
	.index("by_author_and_channel", { fields: ["author", "channel"] } satisfies object)!
	.index("by_author_and_channel_and_body", ["author", "channel", "body"]);
