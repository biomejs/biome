import * as fs from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";

const EXTENSION_ROOT = resolve(fileURLToPath(import.meta.url), "../..");
const GRADLE_PROPERTIES_PATH = resolve(EXTENSION_ROOT, "gradle.properties");
const semverRegex =
	/(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(-(0|[1-9A-Za-z-][0-9A-Za-z-]*)(\.[0-9A-Za-z-]+)*)?(\+[0-9A-Za-z-]+(\.[0-9A-Za-z-]+)*)?/g;
const gradleProperties = fs
	.readFileSync(GRADLE_PROPERTIES_PATH)
	.toString("utf-8");
const versionLineRegex = /^(pluginVersion =) (.+)$/gm;
const versionLine = gradleProperties.match(versionLineRegex)[0];
let version = versionLine.match(semverRegex);

if (
	typeof process.env.GITHUB_SHA !== "string" ||
	process.env.GITHUB_SHA === ""
) {
	throw new Error("GITHUB_SHA environment variable is undefined");
}

version += `-nightly.${process.env.GITHUB_SHA.substring(0, 7)}`;

const content = gradleProperties.replace(versionLineRegex, `$1 ${version}`);
fs.writeFileSync(GRADLE_PROPERTIES_PATH, content);

console.log(version);
