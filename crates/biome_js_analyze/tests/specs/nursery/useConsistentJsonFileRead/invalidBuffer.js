/* should generate diagnostics */
import { readFile } from "node:fs/promises";

async function direct() {
	JSON.parse(await fs.readFile("./package.json", "utf8"));
	JSON.parse(fs.readFileSync("./package.json", "utf-8"));
	JSON.parse(fs.readFileSync("./package.json", "UTF8"));
	JSON.parse(await fs.readFile("./package.json", `utf8`));
	JSON.parse(await fs.readFile("./package.json", { encoding: "utf8" }));
	JSON.parse(await readFile("./package.json", "utf8"));
}

async function throughVariables() {
	const promise = fs.readFile("./package.json", "utf8");
	JSON.parse(await promise);
}

async function trailingComma() {
	JSON.parse(
		await fs.readFile(
			"./package.json",
			"utf8",
		),
	);
}

async function noFix() {
	// The comments would be lost.
	JSON.parse(await fs.readFile("./package.json", /* string */ "utf8"));
	JSON.parse(await fs.readFile("./package.json", { /* string */ encoding: "utf8" }));
	JSON.parse(await fs.readFile("./package.json", // string
		"utf8"));
}
