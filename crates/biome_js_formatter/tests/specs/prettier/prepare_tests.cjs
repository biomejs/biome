const { promises: fs } = require("fs");
const {
	extractPrettierTests,
} = require("../../../../biome_formatter_test/src/prettier/prepare_tests.cjs");

async function main() {
	const DIRS = ["./js", "./jsx", "./typescript"];
	for (const path of DIRS) {
		await fs.rm(path, { recursive: true, force: true });
	}

	await extractPrettierTests("js", {
		parser: "babel",
	});

	await extractPrettierTests("jsx", {
		parser: "babel",
	});

	await extractPrettierTests("typescript", {
		parser: "typescript",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
