const {extractPrettierTests} = require("../../../../biome_formatter_test/src/prettier/prepare_tests");

async function main() {
	await extractPrettierTests("graphql", {
		parser: "graphql",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
