#!/usr/bin/env node

import { execFileSync, spawnSync } from "node:child_process";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parseArgs } from "node:util";

const REPOSITORY_ROOT = path.resolve(
	path.dirname(fileURLToPath(import.meta.url)),
	"..",
);
const ROOT_MANIFEST = path.join(REPOSITORY_ROOT, "Cargo.toml");
const CRATES_DIRECTORY = path.join(REPOSITORY_ROOT, "crates");
const VERSION_PATTERN = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/;

type BumpType = "minor" | "patch";
type PublishSetting = "true" | "false";

interface CrateInfo {
	description?: string;
	manifest: string;
	manifestPath: string;
	name: string;
	publish?: PublishSetting;
	versionWorkspace: boolean;
}

interface ManifestSection {
	content: string;
	end: number;
	start: number;
}

interface ValidationResult {
	publicCrates: CrateInfo[];
	version: string;
}

export function bumpVersion(version: string, bump: BumpType): string {
	const match = VERSION_PATTERN.exec(version);
	if (!match) {
		throw new Error(`Invalid version: ${version}`);
	}

	const major = Number(match[1]);
	const minor = Number(match[2]);
	const patch = Number(match[3]);

	switch (bump) {
		case "minor":
			return `${major}.${minor + 1}.0`;
		case "patch":
			return `${major}.${minor}.${patch + 1}`;
	}
}

export function getWorkspaceVersion(manifest: string): string {
	const section = getSection(manifest, "workspace.package");
	const match = /^version\s*=\s*"([^"]+)"$/m.exec(section.content);
	if (!match) {
		throw new Error("Missing version in [workspace.package]");
	}
	return match[1];
}

export function updateRootManifest(
	manifest: string,
	crateNames: readonly string[],
	newVersion: string,
): string {
	const section = getSection(manifest, "workspace.package");
	const updatedSection = section.content.replace(
		/^(version\s*=\s*)"[^"]+"$/m,
		`$1"${newVersion}"`,
	);
	if (updatedSection === section.content) {
		throw new Error("Could not update [workspace.package].version");
	}

	let updated =
		manifest.slice(0, section.start) +
		updatedSection +
		manifest.slice(section.end);

	for (const crateName of crateNames) {
		const dependency = new RegExp(
			`^(${escapeRegExp(crateName)}\\s*=\\s*\\{[^}\\n]*\\bversion\\s*=\\s*)"[^"]+"`,
			"m",
		);
		if (!dependency.test(updated)) {
			throw new Error(
				`Missing versioned workspace dependency for ${crateName}`,
			);
		}
		updated = updated.replace(dependency, `$1"${newVersion}"`);
	}

	return updated;
}

export function detectVersionChange(
	previousManifest: string,
	currentManifest: string,
): string | null {
	const previous = getWorkspaceVersion(previousManifest);
	const current = getWorkspaceVersion(currentManifest);
	if (previous === current) {
		return null;
	}

	if (
		current !== bumpVersion(previous, "minor") &&
		current !== bumpVersion(previous, "patch")
	) {
		throw new Error(
			`Crate version must be a single minor or patch bump: ${previous} -> ${current}`,
		);
	}

	return current;
}

function createReadmeImage(): string {
	return `<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-dark-transparent.svg">
    <source media="(prefers-color-scheme: light)" srcset="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg">
    <img alt="Shows the banner of Biome, with its logo and the phrase 'Biome - Toolchain of the web'." src="https://raw.githubusercontent.com/biomejs/resources/main/svg/slogan-light-transparent.svg" width="400">
  </picture>
</p>
`;
}

function getSection(manifest: string, name: string): ManifestSection {
	const header = `[${name}]`;
	const headerStart = manifest.indexOf(header);
	if (headerStart === -1) {
		throw new Error(`Missing ${header}`);
	}

	const start = headerStart + header.length;
	const nextSection = /^\s*\[/m.exec(manifest.slice(start));
	const end = nextSection ? start + nextSection.index : manifest.length;
	return { content: manifest.slice(start, end), start, end };
}

function escapeRegExp(value: string): string {
	return value.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

function readCrates(): CrateInfo[] {
	return fs
		.readdirSync(CRATES_DIRECTORY, { withFileTypes: true })
		.filter((entry) => entry.isDirectory())
		.map((entry) => path.join(CRATES_DIRECTORY, entry.name, "Cargo.toml"))
		.filter((manifestPath) => fs.existsSync(manifestPath))
		.map((manifestPath) => {
			const manifest = fs.readFileSync(manifestPath, "utf8");
			const name = /^name\s*=\s*"([^"]+)"$/m.exec(manifest)?.[1];
			const publish = /^publish\s*=\s*(true|false)$/m.exec(manifest)?.[1] as
				| PublishSetting
				| undefined;
			const description = /^description\s*=\s*"([^"]+)"$/m.exec(manifest)?.[1];
			if (!name) {
				throw new Error(`Missing package name in ${manifestPath}`);
			}
			return {
				description,
				manifest,
				manifestPath,
				name,
				publish,
				versionWorkspace: /^version\.workspace\s*=\s*true$/m.test(manifest),
			};
		});
}

/**
 * Validates the workspace invariants required to publish the public crates.
 *
 * - Checks that every crate explicitly opts in or out of publication.
 * - Checks that private crates do not inherit the shared release version.
 * - Checks that public crates inherit the shared release version.
 * - Checks that public crates provide a description and a README with the shared header.
 * - Checks that public workspace dependencies use the shared release version.
 *
 * All violations are reported together in a single error.
 */
function validateRepository(): ValidationResult {
	const rootManifest = fs.readFileSync(ROOT_MANIFEST, "utf8");
	const version = getWorkspaceVersion(rootManifest);
	const crates = readCrates();
	const publicCrates = crates.filter((crate) => crate.publish === "true");
	const errors: string[] = [];

	for (const crate of crates) {
		const relativeManifest = path.relative(REPOSITORY_ROOT, crate.manifestPath);
		if (!crate.publish) {
			errors.push(`${relativeManifest}: missing explicit publish field`);
			continue;
		}
		if (crate.publish === "false" && crate.versionWorkspace) {
			errors.push(
				`${relativeManifest}: private crate inherits release version`,
			);
		}
	}

	for (const crate of publicCrates) {
		const relativeManifest = path.relative(REPOSITORY_ROOT, crate.manifestPath);
		if (!crate.versionWorkspace) {
			errors.push(`${relativeManifest}: public crate must inherit its version`);
		}
		if (!crate.description) {
			errors.push(`${relativeManifest}: public crate is missing a description`);
		}
		const readmePath = path.join(path.dirname(crate.manifestPath), "README.md");
		const readmeImage = createReadmeImage();
		if (!fs.existsSync(readmePath)) {
			errors.push(`${relativeManifest}: missing README.md`);
		} else {
			const readme = fs.readFileSync(readmePath, "utf8");
			if (!readme.startsWith(readmeImage)) {
				errors.push(
					`${relativeManifest}: README.md is missing the shared image`,
				);
			}
		}

		const dependency = new RegExp(
			`^${escapeRegExp(crate.name)}\\s*=\\s*\\{[^}\\n]*\\bversion\\s*=\\s*"${escapeRegExp(version)}"`,
			"m",
		);
		if (!dependency.test(rootManifest)) {
			errors.push(
				`Cargo.toml: ${crate.name} workspace dependency must use ${version}`,
			);
		}
	}

	if (errors.length > 0) {
		throw new Error(errors.join("\n"));
	}
	console.log(
		`Validated ${publicCrates.length} public crates at version ${version}`,
	);
	return { publicCrates, version };
}

function runCargo(args: string[]): void {
	const result = spawnSync("cargo", args, {
		cwd: REPOSITORY_ROOT,
		stdio: "inherit",
	});
	if (result.error) {
		throw result.error;
	}
	if (result.status !== 0) {
		throw new Error(`cargo ${args[0]} failed with status ${result.status}`);
	}
}

function packageArguments(publicCrates: readonly CrateInfo[]): string[] {
	return publicCrates.flatMap((crate) => ["--package", crate.name]);
}

function bump(bumpType: BumpType, dryRun: boolean): void {
	const rootManifest = fs.readFileSync(ROOT_MANIFEST, "utf8");
	const crates = readCrates();
	const publicCrates = crates.filter((crate) => crate.publish === "true");
	const currentVersion = getWorkspaceVersion(rootManifest);
	const newVersion = bumpVersion(currentVersion, bumpType);
	const updatedManifest = updateRootManifest(
		rootManifest,
		publicCrates.map((crate) => crate.name),
		newVersion,
	);

	if (!dryRun) {
		fs.writeFileSync(ROOT_MANIFEST, updatedManifest);
	}
	console.log(newVersion);
}

function detect(reference: string): void {
	const currentManifest = fs.readFileSync(ROOT_MANIFEST, "utf8");
	let previousManifest: string;
	try {
		previousManifest = execFileSync(
			"git",
			["show", `${reference}:Cargo.toml`],
			{
				cwd: REPOSITORY_ROOT,
				encoding: "utf8",
			},
		);
	} catch {
		return;
	}

	let version: string | null;
	try {
		version = detectVersionChange(previousManifest, currentManifest);
	} catch (error) {
		if (
			error instanceof Error &&
			error.message === "Missing version in [workspace.package]"
		) {
			return;
		}
		throw error;
	}
	if (version) {
		console.log(version);
	}
}

function verify(allowDirty = false): void {
	const { publicCrates } = validateRepository();
	const args = ["publish", "--dry-run", "--locked"];
	if (allowDirty) {
		args.push("--allow-dirty");
	}
	runCargo([...args, ...packageArguments(publicCrates)]);
}

function usage(): void {
	console.error(`Usage:
  node --experimental-strip-types scripts/update-crates-versions.ts bump <minor|patch> [--dry-run]
  node --experimental-strip-types scripts/update-crates-versions.ts check
  node --experimental-strip-types scripts/update-crates-versions.ts detect <git-ref>
  node --experimental-strip-types scripts/update-crates-versions.ts verify [--allow-dirty]`);
}

function isBumpType(value: string | undefined): value is BumpType {
	return value === "minor" || value === "patch";
}

function main(): void {
	const { positionals, values } = parseArgs({
		allowPositionals: true,
		options: {
			"allow-dirty": {
				default: false,
				type: "boolean",
			},
			"dry-run": {
				default: false,
				type: "boolean",
			},
		},
	});
	const [command, ...args] = positionals;
	switch (command) {
		case "bump": {
			const [bumpType] = args;
			if (args.length !== 1 || !isBumpType(bumpType) || values["allow-dirty"]) {
				usage();
				process.exitCode = 1;
				return;
			}
			bump(bumpType, values["dry-run"]);
			break;
		}
		case "check":
			if (args.length > 0 || values["allow-dirty"] || values["dry-run"]) {
				usage();
				process.exitCode = 1;
				return;
			}
			validateRepository();
			break;
		case "detect":
			if (
				args.length !== 1 ||
				!args[0] ||
				values["allow-dirty"] ||
				values["dry-run"]
			) {
				usage();
				process.exitCode = 1;
				return;
			}
			detect(args[0]);
			break;
		case "verify":
			if (args.length > 0 || values["dry-run"]) {
				usage();
				process.exitCode = 1;
				return;
			}
			verify(values["allow-dirty"]);
			break;
		default:
			usage();
			process.exitCode = 1;
	}
}

if (
	process.argv[1] &&
	pathToFileURL(path.resolve(process.argv[1])).href === import.meta.url
) {
	try {
		main();
	} catch (error) {
		console.error(error instanceof Error ? error.message : String(error));
		process.exitCode = 1;
	}
}
