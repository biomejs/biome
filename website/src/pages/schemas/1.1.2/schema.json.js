// Run `BIOME_VERSION=<version number> cargo codegen-website
// to generate a new schema
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";

export function get() {
	const schemaPath = resolve(
		join("..", "packages", "@biomejs", "biome", "configuration_schema.json"),
	);
	const schema = readFileSync(schemaPath, "utf8");

	return new Response(schema, {
		status: 200,
		headers: {
			"content-type": "application/json",
		},
	});
}
