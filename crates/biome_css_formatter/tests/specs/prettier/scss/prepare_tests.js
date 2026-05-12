const path = require("path");
const { extractPrettierTests } = require("../../../../../biome_formatter_test/src/prettier/prepare_tests");

async function main() {
	process.chdir(path.resolve(__dirname, ".."));
	await extractPrettierTests("scss", {
		parser: "scss",
	});
}

main().catch((err) => {
	console.error(err);
	process.exit(1);
});
