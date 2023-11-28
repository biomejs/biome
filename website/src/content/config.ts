import { docsSchema } from "@astrojs/starlight/schema";
// src/content/config.ts
import { defineCollection, z } from "astro:content";

const blogSchema = z.object({
	title: z.string(),
	subtitle: z.string().optional(),
	pubDate: z
		.string()
		.or(z.date())
		.transform((val) => new Date(val)),
	summary: z.string(),
	description: z.string(),
	authors: z.array(z.string()),
	coverImage: z
		.object({
			src: z.string(),
			caption: z.string().optional(),
			alt: z.string(),
		})
		.optional(),
	socialImage: z.string(),
});

const authorsSchema = z.object({
	name: z.string(),
	avatar: z.string(),
	url: z.string().optional(),
});

export const collections = {
	docs: defineCollection({ schema: docsSchema() }),
	blog: defineCollection({
		type: "content",
		schema: blogSchema,
	}),
	authors: defineCollection({
		type: "data",
		schema: authorsSchema,
	}),
};
