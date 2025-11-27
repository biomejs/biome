const { extractPrettierTests } = require("../../../../biome_formatter_test/src/prettier/prepare_tests");
import fs from "fs/promises";

async function main() {
	await extractPrettierTests("html", {
		parser: "html",
	});
	// These tests don't contain real HTML, and its specific to prettier's testing infra
	await fs.rmdir("html/cursor", { recursive: true });

	await extractPrettierTests("vue", {
		parser: "vue",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
