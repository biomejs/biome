import { docsSchema } from "@astrojs/starlight/schema";
// src/content/config.ts
import { defineCollection } from "astro:content";

export const collections = {
	// @ts-expect-error
	docs: defineCollection({ schema: docsSchema() }),
};
