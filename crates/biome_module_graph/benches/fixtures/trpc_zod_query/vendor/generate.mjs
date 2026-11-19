import { writeFile } from "node:fs/promises";
import ts from "typescript";
import { rollup } from "rollup";
import { dts } from "rollup-plugin-dts";

const output = process.argv[2];
const printer = ts.createPrinter({ removeComments: true });

// Bundles `entry` into `${name}.d.ts`. Imports of packages listed in
// `external` are pointed at sibling bundles; a `null` target marks a package
// that must be tree-shaken away. Returns the names imported from each
// external specifier so the sibling bundles can export them.
async function bundle(name, entry, external = {}) {
	await writeFile(`${name}-entry.d.ts`, entry);
	const target = (id) =>
		Object.keys(external).find((pkg) => id === pkg || id.startsWith(`${pkg}/`));
	const bundle = await rollup({
		input: `${name}-entry.d.ts`,
		external: (id) => target(id) !== undefined,
		plugins: [
			dts({
				respectExternal: true,
				compilerOptions: {
					module: ts.ModuleKind.ESNext,
					moduleResolution: ts.ModuleResolutionKind.Bundler,
				},
			}),
		],
	});
	const { output: chunks } = await bundle.generate({ format: "es" });
	await bundle.close();
	const source = ts.createSourceFile(
		`${name}.d.ts`,
		chunks[0].code,
		ts.ScriptTarget.Latest,
		true,
	);
	const imports = {};
	const statements = source.statements.map((statement) => {
		if (
			(ts.isExportDeclaration(statement) && statement.moduleSpecifier) ||
			(ts.isModuleDeclaration(statement) && ts.isStringLiteral(statement.name))
		)
			throw new Error(`${name}: unexpected ${statement.getText()}`);
		if (!ts.isImportDeclaration(statement)) return statement;
		const id = statement.moduleSpecifier.text;
		const path = external[target(id)];
		const bindings = statement.importClause?.namedBindings;
		if (!path || !bindings || !ts.isNamedImports(bindings))
			throw new Error(`${name}: unexpected import from ${id}`);
		imports[id] ??= new Set();
		for (const element of bindings.elements)
			imports[id].add((element.propertyName ?? element.name).text);
		return ts.factory.updateImportDeclaration(
			statement,
			statement.modifiers,
			statement.importClause,
			ts.factory.createStringLiteral(path),
			statement.attributes,
		);
	});
	await writeFile(
		`${output}/${name}.d.ts`,
		printer.printFile(ts.factory.updateSourceFile(source, statements)),
	);
	return imports;
}

// Builds an entry that exports `exports` plus every name downstream bundles
// import from `pkg` or its subpaths. Subpaths re-export the same
// declarations, so the first specifier that provides a name wins.
function entry(pkg, exports, ...importMaps) {
	const seen = new Set();
	let code = "";
	for (const [id, names] of Object.entries(exports)) {
		names.forEach((n) => seen.add(n.replace(/^type /, "")));
		code += `export { ${names.join(", ")} } from "${id}";\n`;
	}
	for (const imports of importMaps)
		for (const [id, names] of Object.entries(imports)) {
			if (id !== pkg && !id.startsWith(`${pkg}/`)) continue;
			const fresh = [...names].filter((n) => !seen.has(n));
			fresh.forEach((n) => seen.add(n));
			if (fresh.length)
				code += `export type { ${fresh.join(", ")} } from "${id}";\n`;
		}
	return code;
}

// Flat exports, consumed as `import * as z`. Bundling zod's `z` namespace
// makes rollup-plugin-dts alias each member as `declare const x: typeof x`,
// which drops the type side of `z.ZodType`, `z.input`, and friends.
await bundle(
	"zod",
	entry("zod", {
		zod: [
			"array",
			"boolean",
			"coerce",
			"discriminatedUnion",
			"email",
			"enum",
			"flattenError",
			"input",
			"int",
			"iso",
			"literal",
			"number",
			"object",
			"output",
			"string",
			"union",
			"url",
			"uuid",
			"ZodError",
			"type infer",
			"type ZodType",
		],
	}),
);

// Only `createTRPCContext` references React; it and the React Query hooks are
// tree-shaken away.
const tanstackImports = await bundle(
	"trpc-tanstack-react-query",
	entry("@trpc/tanstack-react-query", {
		"@trpc/tanstack-react-query": [
			"createTRPCOptionsProxy",
			"type TRPCOptionsProxy",
			"type inferInput",
			"type inferOutput",
		],
	}),
	{
		"@trpc/server": "./trpc-server",
		"@trpc/client": "./trpc-client",
		"@tanstack/react-query": "./tanstack-query",
		react: null,
	},
);

await bundle(
	"tanstack-query",
	entry(
		"@tanstack/react-query",
		{
			"@tanstack/react-query": [
				"InfiniteQueryObserver",
				"MutationObserver",
				"QueryClient",
				"QueryObserver",
				"hashKey",
				"keepPreviousData",
				"skipToken",
				"type InfiniteData",
			],
		},
		tanstackImports,
	),
	{ react: null },
);

const clientImports = await bundle(
	"trpc-client",
	entry(
		"@trpc/client",
		{
			"@trpc/client": [
				"createTRPCClient",
				"httpBatchLink",
				"httpLink",
				"isTRPCClientError",
				"loggerLink",
				"splitLink",
				"TRPCClientError",
				"type TRPCLink",
			],
		},
		tanstackImports,
	),
	{ "@trpc/server": "./trpc-server" },
);

await bundle(
	"trpc-server",
	entry(
		"@trpc/server",
		{
			"@trpc/server": [
				"initTRPC",
				"TRPCError",
				"tracked",
				"type inferProcedureBuilderResolverOptions",
				"type inferRouterInputs",
				"type inferRouterOutputs",
			],
			"@trpc/server/observable": ["observable"],
		},
		tanstackImports,
		clientImports,
	),
);
