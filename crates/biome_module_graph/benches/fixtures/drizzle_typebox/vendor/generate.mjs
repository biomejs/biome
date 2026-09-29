import { writeFile } from "node:fs/promises";
import ts from "typescript";
import { rollup } from "rollup";
import { dts } from "rollup-plugin-dts";
import { generateDtsBundle } from "dts-bundle-generator";

const output = process.argv[2];
await writeFile(
	"entry.d.ts",
	`
export { relations, eq, and, or, desc, asc, count, sum, sql, inArray, gte, getTableColumns } from "drizzle-orm";
export { pgTable, pgEnum, uuid, text, integer, numeric, boolean, timestamp, jsonb, primaryKey, index, alias, type PgDatabase, type PgQueryResultHKT } from "drizzle-orm/pg-core";
export { createSelectSchema, createInsertSchema, createUpdateSchema } from "drizzle-typebox";
export { Type, type Static, type TSchema } from "@sinclair/typebox";
`,
);
await writeFile(
	"tsconfig.json",
	JSON.stringify({
		compilerOptions: {
			strict: true,
			skipLibCheck: true,
			target: "ES2022",
			moduleResolution: "bundler",
			module: "ESNext",
		},
	}),
);
await writeFile("typebox-entry.d.ts", 'export * from "@sinclair/typebox";\n');
const [typebox] = generateDtsBundle(
	[
		{
			filePath: "typebox-entry.d.ts",
			libraries: { inlinedLibraries: ["@sinclair/typebox"] },
			output: { noBanner: true, exportReferencedTypes: false },
		},
	],
	{ preferredConfigPath: "tsconfig.json" },
);
const printer = ts.createPrinter({ removeComments: true });
await writeFile(
	`${output}/typebox.d.ts`,
	printer.printFile(
		ts.createSourceFile("typebox.d.ts", typebox, ts.ScriptTarget.Latest, true),
	),
);
const bundle = await rollup({
	input: "entry.d.ts",
	external: ["@sinclair/typebox"],
	plugins: [dts({ respectExternal: true })],
});
const { output: chunks } = await bundle.generate({
	format: "es",
	paths: { "@sinclair/typebox": "./typebox" },
});
const source = ts.createSourceFile(
	"drizzle-typebox.d.ts",
	chunks[0].code,
	ts.ScriptTarget.Latest,
	true,
);
await writeFile(
	`${output}/drizzle-typebox.d.ts`,
	ts.createPrinter({ removeComments: true }).printFile(source),
);
await bundle.close();
