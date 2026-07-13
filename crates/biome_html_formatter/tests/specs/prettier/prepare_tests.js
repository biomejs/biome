import { extractPrettierTests } from "../../../../biome_formatter_test/src/prettier/prepare_tests.js";
import fs from "fs/promises";

async function main() {
	await extractPrettierTests("html", {
		parser: "html",
	});

	await extractPrettierTests("vue", {
		parser: "vue",
	});

	await extractPrettierTests("angular", {
		parser: "angular",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
