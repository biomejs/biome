import * as fs from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const BIOME_CLI_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const MANIFEST_PATH = resolve(BIOME_CLI_ROOT, "package.json");

const rootManifest = JSON.parse(
	fs.readFileSync(MANIFEST_PATH).toString("utf-8"),
);

let [major, minor, patch] = rootManifest.version
	.split(".")
	.map((num) => Number.parseInt(num, 10));
// increment patch version
patch += 1;
let version = `${major}.${minor}.${patch}`;

if (
	typeof process.env.GITHUB_SHA !== "string" ||
	process.env.GITHUB_SHA === ""
) {
	throw new Error("GITHUB_SHA environment variable is undefined");
}

version += `-preview.${process.env.GITHUB_SHA.substring(0, 7)}`;
rootManifest.version = version;

const content = JSON.stringify(rootManifest);
fs.writeFileSync(MANIFEST_PATH, content);

console.log(version);
