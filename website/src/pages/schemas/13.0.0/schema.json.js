// Run `ROME_VERSION=<version number> cargo codegen-website
// to generate a new schema
import { readFileSync } from "fs";
import { join, resolve } from "path";

export function get() {
	const schemaPath = resolve(
		join("..", "npm", "rome", "configuration_schema.json"),
	);
	const schema = readFileSync(schemaPath, "utf8");

	return new Response(schema, {
		status: 200,
		headers: {
			"content-type": "application/json",
		},
	});
}
