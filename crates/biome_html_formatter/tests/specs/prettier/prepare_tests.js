import { extractPrettierTests } from "../../../../biome_formatter_test/src/prettier/prepare_tests.js";
import fs from "fs/promises";

async function main() {
	await extractPrettierTests("html", {
		parser: "html",
	});
	// These tests don't contain real HTML, and its specific to prettier's testing infra
	await fs.rm("html/cursor", { recursive: true, force: true });

	await extractPrettierTests("vue", {
		parser: "vue",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
