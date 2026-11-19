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

async function bundle(name, entry, fix, compilerOptions) {
	await writeFile(`${name}-entry.d.ts`, entry);
	const bundle = await rollup({
		input: `${name}-entry.d.ts`,
		plugins: [dts({ respectExternal: true, compilerOptions })],
	});
	const { output: chunks } = await bundle.generate({ format: "es" });
	const source = ts.createSourceFile(
		`${name}.d.ts`,
		fix(chunks[0].code),
		ts.ScriptTarget.Latest,
		true,
	);
	await writeFile(`${output}/${name}.d.ts`, printer.printFile(source));
	await bundle.close();
}

await bundle(
	"kysely",
	`export { Kysely, sql, type ColumnType, type Generated, type GeneratedAlways, type JSONColumnType, type Selectable, type Insertable, type Updateable, type ExpressionBuilder, type Expression, type Transaction, type SqlBool, type NotNull } from "kysely";
export { jsonArrayFrom, jsonObjectFrom, jsonBuildObject } from "kysely/helpers/postgres";
`,
	(code) => code,
	// kysely/helpers/postgres is only reachable through package exports.
	{
		module: ts.ModuleKind.ESNext,
		moduleResolution: ts.ModuleResolutionKind.Bundler,
	},
);
await bundle(
	"ts-pattern",
	`export { match, isMatching, P, NonExhaustiveError } from "ts-pattern";\n`,
	(code) => {
		// P.infer is declared as a type named `infer`. Once hoisted out of its
		// namespace, a bare `infer<p>` parses as an infer type, so rename it.
		code = replaceOnce(
			code,
			"type infer<pattern> =",
			"type InferPattern<pattern> =",
		);
		code = replaceOnce(
			code,
			"type patterns_d_infer<pattern> = infer<pattern>;",
			"type patterns_d_infer<pattern> = InferPattern<pattern>;",
		);
		code = replaceOnce(code, "value is infer<p>;", "value is InferPattern<p>;");
		// is-matching.d.ts names both the patterns namespace and a type parameter
		// `P`; qualified names still resolve to the namespace. The bundle renames
		// the namespace to patterns_d but keeps these references.
		return replaceOnce(
			code,
			"WithDefault$1<P.narrow<T, P>, P.infer<P>>",
			"WithDefault$1<patterns_d.narrow<T, P>, patterns_d.infer<P>>",
		);
	},
);
