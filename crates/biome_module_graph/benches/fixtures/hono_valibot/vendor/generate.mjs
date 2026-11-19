import { writeFile } from "node:fs/promises";
import ts from "typescript";
import { rollup } from "rollup";
import { dts } from "rollup-plugin-dts";

const output = process.argv[2];
const printer = ts.createPrinter({ removeComments: true });

async function bundle(name, entry, options = {}) {
	await writeFile(`${name}-entry.d.ts`, entry);
	const bundle = await rollup({
		input: `${name}-entry.d.ts`,
		external: options.external,
		plugins: [dts({ respectExternal: true })],
	});
	const { output: chunks } = await bundle.generate({
		format: "es",
		paths: options.paths,
	});
	const source = ts.createSourceFile(
		`${name}.d.ts`,
		chunks[0].code,
		ts.ScriptTarget.Latest,
		true,
	);
	await writeFile(`${output}/${name}.d.ts`, printer.printFile(source));
	await bundle.close();
}

await bundle(
	"valibot",
	`export { array, base64, boolean, brand, check, digits, email, flatten, hexColor, integer, isoDate, isoTimestamp, literal, maxLength, maxValue, minLength, minValue, nullable, number, object, omit, optional, partial, pick, picklist, pipe, regex, startsWith, string, toLowerCase, transform, trim, union, url, uuid, variant, type BaseIssue, type GenericSchema, type GenericSchemaAsync, type InferInput, type InferOutput, type SafeParseResult } from "valibot";\n`,
);
// The validator imports valibot types, which resolve to the sibling bundle.
await bundle(
	"hono",
	`export { Hono, type Context, type InferRequestType, type InferResponseType } from "hono";
export { hc, type ClientResponse } from "hono/client";
export { createMiddleware } from "hono/factory";
export { HTTPException } from "hono/http-exception";
export { vValidator } from "@hono/valibot-validator";
`,
	{ external: ["valibot"], paths: { valibot: "./valibot" } },
);
