import { rollup } from "rollup";
import { dts } from "rollup-plugin-dts";
const output = process.argv[2];
for (const [name, input] of [
	["zod", "node_modules/zod/lib/index.d.ts"],
	["tanstack-form", "node_modules/@tanstack/form-core/dist/esm/index.d.ts"],
]) {
	const bundle = await rollup({
		input,
		plugins: [dts({ respectExternal: true })],
	});
	await bundle.write({ file: `${output}/${name}.d.ts`, format: "es" });
	await bundle.close();
}
