import { writeFile } from "node:fs/promises";
import ts from "typescript";
import { rollup } from "rollup";
import { dts } from "rollup-plugin-dts";

const output = process.argv[2];
const printer = ts.createPrinter({ removeComments: true });

function replaceOnce(code, search, replacement) {
	const parts = code.split(search);
	if (parts.length !== 2)
		throw new Error(`Expected one occurrence of ${search}`);
	return parts.join(replacement);
}

async function bundle(name, entry, fix) {
	await writeFile(`${name}-entry.d.ts`, entry);
	const bundle = await rollup({
		input: `${name}-entry.d.ts`,
		plugins: [dts({ respectExternal: true })],
	});
	const { output: chunks } = await bundle.generate({ format: "es" });
	const source = ts.createSourceFile(
		`${name}.d.ts`,
		fix(chunks[0].code),
		ts.ScriptTarget.Latest,
		true,
	);
	// Augmentations of the original per-module specifiers match no module in
	// the bundle, so TypeScript ignores them.
	const statements = source.statements.filter(
		(statement) =>
			!(
				ts.isModuleDeclaration(statement) && ts.isStringLiteral(statement.name)
			),
	);
	await writeFile(
		`${output}/${name}.d.ts`,
		printer.printFile(ts.factory.updateSourceFile(source, statements)),
	);
	await bundle.close();
}

await bundle(
	"effect",
	`export { Cause, Context, Data, Duration, Effect, Either, Exit, Layer, Match, Option, Ref, Schedule } from "effect";
export { pipe } from "effect/Function";
`,
	// rollup-plugin-dts renames global ReadonlyArray references inside the
	// effect/Array namespace of the same name.
	(code) =>
		replaceOnce(
			replaceOnce(
				code,
				"S extends ReadonlyArray$1<infer A>",
				"S extends ReadonlyArray<infer A>",
			),
			"T extends ReadonlyArray$1<ReadonlyArray$1<any>>",
			"T extends ReadonlyArray<ReadonlyArray<any>>",
		),
);
await bundle(
	"arktype",
	`export { type, scope, match, ArkErrors, type Type } from "arktype";\n`,
	(code) => {
		// rollup-plugin-dts renames the global ReadonlyArray return type along
		// with the @ark/util constant of the same name.
		code = replaceOnce(code, "=> ReadonlyArray$1<T>;", "=> ReadonlyArray<T>;");
		// Avoid an unresolved Node import; File is also a global.
		code = replaceOnce(code, "import * as buffer from 'buffer';\n", "");
		return replaceOnce(code, "typeof buffer.File", "typeof File");
	},
);
