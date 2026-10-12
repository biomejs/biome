/* should generate diagnostics */
import { readFile } from "node:fs/promises";
import { readFileSync as readSync } from "node:fs";
import { readFileSync } from "fs";

async function direct() {
	JSON.parse(await fs.readFile("./package.json"));
	JSON.parse(fs.readFileSync("./package.json"));
	JSON.parse(await fs.promises.readFile("./package.json"));
	JSON.parse(await fs["readFile"]("./package.json"));
	JSON.parse(await (fs.readFile("./package.json")));
	JSON.parse(await readFile("./package.json"));
	JSON.parse(readSync("./package.json"));
	JSON.parse(readFileSync("./package.json"));
	globalThis.JSON.parse(fs.readFileSync("./package.json"));
}

async function trailingComma() {
	JSON.parse(
		await fs.readFile(
			"./package.json",
		),
	);
}

async function throughVariables() {
	const promise = fs.readFile("./package.json");
	const packageJson = JSON.parse(await promise);

	const buffer = await fs.readFile("./package.json");
	JSON.parse(buffer);

	const first = fs.readFile("./package.json");
	const second = first;
	JSON.parse(await second);
}

async function nullEncoding() {
	JSON.parse(await fs.readFile("./package.json", null));
	JSON.parse(await fs.readFile("./package.json", undefined));
	JSON.parse(await fs.readFile("./package.json", { encoding: null }));
	JSON.parse(await fs.readFile("./package.json", { "encoding": undefined }));
	JSON.parse(await fs.readFile("./package.json", /* buffer */ null));
	// No fix: the comment would be lost.
	JSON.parse(await fs.readFile("./package.json", { /* buffer */ encoding: null }));
}
