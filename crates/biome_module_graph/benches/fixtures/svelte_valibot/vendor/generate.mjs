import { readFile, writeFile } from "node:fs/promises";
import ts from "typescript";
import { rollup } from "rollup";
import { dts } from "rollup-plugin-dts";

const output = process.argv[2];
const printer = ts.createPrinter({ removeComments: true });
const svelte = ts.createSourceFile(
	"index.d.ts",
	await readFile("node_modules/svelte/types/index.d.ts", "utf8"),
	ts.ScriptTarget.Latest,
	true,
);
const store = svelte.statements.find(
	(statement) =>
		ts.isModuleDeclaration(statement) && statement.name.text === "svelte/store",
);
if (!store || !ts.isModuleBlock(store.body))
	throw new Error("Missing svelte/store declarations");
await writeFile(
	`${output}/svelte-store.d.ts`,
	store.body.statements
		.map((statement) =>
			printer.printNode(ts.EmitHint.Unspecified, statement, svelte),
		)
		.join("\n") + "\n",
);
await writeFile(
	"valibot-entry.d.ts",
	`export { object, strictObject, string, number, boolean, literal, picklist, variant, union, array, tuple, record, optional, nullable, pipe, minLength, minValue, maxValue, integer, email, uuid, url, trim, toLowerCase, transform, check, brand, partial, pick, safeParse, parse, flatten, type InferInput, type InferOutput } from "valibot";\n`,
);
const bundle = await rollup({
	input: "valibot-entry.d.ts",
	plugins: [dts({ respectExternal: true })],
});
const { output: chunks } = await bundle.generate({ format: "es" });
const source = ts.createSourceFile(
	"valibot.d.ts",
	chunks[0].code,
	ts.ScriptTarget.Latest,
	true,
);
await writeFile(`${output}/valibot.d.ts`, printer.printFile(source));
await bundle.close();
