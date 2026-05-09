import {extractPrettierTests} from "../../../../biome_formatter_test/src/prettier/prepare_tests.js";

async function main() {
	await extractPrettierTests("yaml", {
		parser: "yaml",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
