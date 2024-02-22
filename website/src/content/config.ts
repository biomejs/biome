// src/content/config.ts
import { type ImageFunction, defineCollection, z } from "astro:content";
import { docsSchema } from "@astrojs/starlight/schema";

const blogSchema = (image: ImageFunction) =>
	z.object({
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
				lightSrc: image(),
				darkSrc: image(),
				caption: z.string().optional(),
				alt: z.string(),
			})
			.optional(),
		socialImage: image(),
	});

const authorsSchema = z.object({
	name: z.string(),
	avatar: z.string(),
	url: z.string().url().optional(),
});

export const collections = {
	docs: defineCollection({ schema: docsSchema() }),
	blog: defineCollection({
		type: "content",
		schema: ({ image }) => blogSchema(image),
	}),
	authors: defineCollection({
		type: "data",
		schema: authorsSchema,
	}),
};
