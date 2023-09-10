const {extractPrettierTests} = require("../../../../biome_formatter_test/src/prettier/prepare_tests");

async function main() {
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
