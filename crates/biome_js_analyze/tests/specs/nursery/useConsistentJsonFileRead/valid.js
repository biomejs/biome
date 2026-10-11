/* should not generate diagnostics */
import { readFile as read } from "some-module";
import readFileDefault from "node:fs";

async function strings() {
	JSON.parse(await fs.readFile("./package.json", "utf8"));
	JSON.parse(fs.readFileSync("./package.json", "utf-8"));
	JSON.parse(await fs.readFile("./package.json", { encoding: "utf8" }));
	JSON.parse(await fs.readFile("./package.json", encoding));
}

async function otherOptions() {
	const promise = fs.readFile("./package.json", { encoding: null, signal });
	JSON.parse(await promise);
	JSON.parse(await fs.readFile("./package.json", { flag: "r" }));
	JSON.parse(await fs.readFile("./package.json", { ...options }));
}

async function notReadFile() {
	JSON.parse(await fs.readJson("./package.json"));
	JSON.parse(await readFile("./package.json"));
	JSON.parse(await read("./package.json"));
	JSON.parse(await readFileDefault("./package.json"));
	JSON.parse(await fs.readFile());
	JSON.parse(await fs.readFile(...args));
	JSON.parse(await fs.readFile("./package.json", null, extra));
	JSON.parse(await fs?.readFile("./package.json"));
	JSON.parse(await fs.readFile?.("./package.json"));
	JSON.parse(text);
}

async function notJsonParse() {
	JSON.parse(await fs.readFile("./package.json"), reviver);
	JSON?.parse(await fs.readFile("./package.json"));
	JSON.stringify(await fs.readFile("./package.json"));
	foo.parse(await fs.readFile("./package.json"));
}

function shadowedJson(JSON) {
	JSON.parse(fs.readFileSync("./package.json"));
}

function shadowedUndefined(undefined) {
	JSON.parse(fs.readFileSync("./package.json", undefined));
}

async function variableUsedElsewhere() {
	const promise = fs.readFile("./package.json");
	console.log(promise);
	JSON.parse(await promise);

	let buffer = await fs.readFile("./package.json");
	buffer = await fs.readFile("./other.json", "utf8");
	JSON.parse(buffer);

	const { data } = await fs.readFile("./package.json");
	JSON.parse(data);
}
