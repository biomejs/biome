import { docsSchema } from "@astrojs/starlight/schema";
// src/content/config.ts
import { defineCollection } from "astro:content";

export const collections = {
	docs: defineCollection({ schema: docsSchema() }),
};
