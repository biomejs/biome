#!/usr/bin/env node

/**
 * Script to update versions of publishable Biome crates
 * Usage: node scripts/update-crates-versions.mjs <new-version> [--dry-run]
 * Example: node scripts/update-crates-versions.mjs 0.6.0
 * Example (dry-run): node scripts/update-crates-versions.mjs 0.6.0 --dry-run
 */

import fs from "node:fs";
import path from "node:path";
import { parseArgs } from "node:util";

const __filename = import.meta.filename;
const __dirname = import.meta.dirname;

// Matches a version field in Cargo.toml, capturing the field name with whitespace
// and the version value. Supports any whitespace/tabs around the equals sign.
// Example matches: version = "0.5.7", version      = "1.0.0"
// Capture groups: $1 = 'version\s*=\s*', $2 = version value
const CARGO_VERSION_FIELD_REGEX = /^(version\s*=\s*)"([^"]+)"$/gm;

// Matches a publish field in Cargo.toml to detect publish = true/false
// Example matches: publish = true, publish              = false
// Capture groups: $1 = full value (true or false)
const CARGO_PUBLISH_FIELD_REGEX = /^publish\s*=\s*(true|false)$/gm;

// Matches a workspace dependency line in root Cargo.toml for a specific crate.
// This is dynamically constructed for each crate name.
// Example: biome_analyze = { version = "0.5.7", path = "./crates/biome_analyze" }
// Template: `^(${crateName}\\s*=\\s*\\{\\s*version\\s*=\\s*)"([^"]+)"`
// Capture groups: $1 = everything before version value, $2 = version value
const WORKSPACE_DEPENDENCY_REGEX_TEMPLATE = (crateName) =>
	`^(${crateName}\\s*=\\s*\\{\\s*version\\s*=\\s*)"([^"]+)"`;

// Validates simple semantic version format (X.Y.Z only, no pre-release tags)
// Examples: 0.6.0, 1.0.0, 2.1.3
// Does NOT accept: 2.0.0-rc.2, 1.0.0-beta.1
const SEMVER_REGEX = /^\d+\.\d+\.\d+$/;

/**
 * Get the publish status of a crate from its Cargo.toml
 * @param {string} cargoTomlPath - Path to the Cargo.toml file
 * @returns {{hasPublishField: boolean, isPublishable: boolean, crateName: string}}
 */
function getCratePublishStatus(cargoTomlPath) {
	const content = fs.readFileSync(cargoTomlPath, "utf8");

	// Extract crate name for error messages
	const nameMatch = /^name\s*=\s*"([^"]+)"$/gm.exec(content);
	const crateName = nameMatch
		? nameMatch[1]
		: path.basename(path.dirname(cargoTomlPath));

	// Check if publish field exists
	CARGO_PUBLISH_FIELD_REGEX.lastIndex = 0;
	const publishMatch = CARGO_PUBLISH_FIELD_REGEX.exec(content);

	if (!publishMatch) {
		return {
			hasPublishField: false,
			isPublishable: false,
			crateName,
		};
	}

	return {
		hasPublishField: true,
		isPublishable: publishMatch[1] === "true",
		crateName,
	};
}

/**
 * Get all publishable crates by scanning the crates directory
 * @returns {Array<{name: string, path: string}>}
 */
function getPublishableCrates() {
	const cratesDir = path.join(__dirname, "..", "crates");
	const publishableCrates = [];
	const errors = [];

	const crateDirs = fs
		.readdirSync(cratesDir, { withFileTypes: true })
		.filter((dirent) => dirent.isDirectory())
		.map((dirent) => dirent.name);

	for (const crateName of crateDirs) {
		const cargoTomlPath = path.join(cratesDir, crateName, "Cargo.toml");

		if (!fs.existsSync(cargoTomlPath)) {
			console.warn(`WARNING: No Cargo.toml found in crates/${crateName}`);
			continue;
		}

		const status = getCratePublishStatus(cargoTomlPath);

		if (!status.hasPublishField) {
			errors.push(`crates/${crateName}/Cargo.toml`);
			continue;
		}

		if (status.isPublishable) {
			publishableCrates.push({
				name: status.crateName,
				path: cargoTomlPath,
			});
		}
	}

	if (errors.length > 0) {
		console.error(
			"\nError: The following crates are missing the 'publish' field:",
		);
		for (const errorPath of errors) {
			console.error(`  - ${errorPath}`);
		}
		console.error(
			"\nAll crates must have a 'publish = true' or 'publish = false' field.",
		);
		process.exit(1);
	}

	return publishableCrates;
}

/**
 * Update version in a Cargo.toml file preserving formatting
 * @param {string} filePath - Path to the Cargo.toml file
 * @param {string} newVersion - New version string
 * @param {boolean} dryRun - If true, don't write changes
 * @returns {{success: boolean, oldVersion?: string, newVersion?: string}}
 */
function updateCargoToml(filePath, newVersion, dryRun = false) {
	if (!fs.existsSync(filePath)) {
		console.warn(`WARNING: File not found: ${filePath}`);
		return { success: false };
	}

	const content = fs.readFileSync(filePath, "utf8");

	// Check if version field exists
	const match = CARGO_VERSION_FIELD_REGEX.exec(content);
	if (!match) {
		console.warn(`WARNING: No version field found in: ${filePath}`);
		return { success: false };
	}

	const oldVersion = match[2];

	// Reset regex lastIndex
	CARGO_VERSION_FIELD_REGEX.lastIndex = 0;

	// Replace version while preserving exact formatting
	const updatedContent = content.replace(
		CARGO_VERSION_FIELD_REGEX,
		`$1"${newVersion}"`,
	);

	if (content === updatedContent) {
		console.warn(`WARNING: No changes made to: ${filePath}`);
		return { success: false };
	}

	if (!dryRun) {
		fs.writeFileSync(filePath, updatedContent, "utf8");
	}

	return { success: true, oldVersion, newVersion };
}

/**
 * Update workspace dependencies in root Cargo.toml
 * @param {Array<{name: string, path: string}>} publishableCrates - List of publishable crates
 * @param {string} newVersion - New version string
 * @param {boolean} dryRun - If true, don't write changes
 */
function updateWorkspaceDependencies(
	publishableCrates,
	newVersion,
	dryRun = false,
) {
	const rootCargoPath = path.join(__dirname, "..", "Cargo.toml");
	const content = fs.readFileSync(rootCargoPath, "utf8");

	// For each publishable crate, update its version in workspace dependencies
	let updatedContent = content;

	for (const crate of publishableCrates) {
		const regex = new RegExp(
			WORKSPACE_DEPENDENCY_REGEX_TEMPLATE(crate.name),
			"gm",
		);

		const match = regex.exec(updatedContent);
		if (match) {
			// Reset regex for replacement
			regex.lastIndex = 0;
			updatedContent = updatedContent.replace(regex, `$1"${newVersion}"`);
		}
	}

	if (content !== updatedContent && !dryRun) {
		fs.writeFileSync(rootCargoPath, updatedContent, "utf8");
	}
}

/**
 * Main function
 */
function main() {
	let parsedArgs;

	try {
		parsedArgs = parseArgs({
			options: {
				"dry-run": {
					type: "boolean",
					default: false,
				},
				help: {
					type: "boolean",
					default: false,
				},
			},
			allowPositionals: true,
		});
	} catch (error) {
		console.error(`Error: ${error.message}`);
		process.exit(1);
	}

	const { values, positionals } = parsedArgs;

	if (values.help) {
		console.info(
			"Usage: node scripts/update-crates-versions.mjs <version> [--dry-run]",
		);
		console.info("\nOptions:");
		console.info(
			"  --dry-run    Show what would be updated without making changes",
		);
		console.info("  --help       Show this help message");
		console.info("\nExamples:");
		console.info("  node scripts/update-crates-versions.mjs 0.6.0");
		console.info("  node scripts/update-crates-versions.mjs 0.6.0 --dry-run");
		process.exit(0);
	}

	if (positionals.length === 0) {
		console.error("Error: Missing version argument");
		console.error(
			"Usage: node scripts/update-crates-versions.mjs <version> [--dry-run]",
		);
		console.error("Run with --help for more information");
		process.exit(1);
	}

	const newVersion = positionals[0];
	const dryRun = values["dry-run"];

	// Validate version format (basic semver check)
	if (!SEMVER_REGEX.test(newVersion)) {
		console.error(`Error: Invalid version format: ${newVersion}`);
		console.error("Expected format: X.Y.Z (e.g., 0.6.0, 1.0.0, 2.1.3)");
		process.exit(1);
	}

	if (dryRun) {
		console.info(
			`[DRY RUN] Checking what would be updated to version ${newVersion}...\n`,
		);
	} else {
		console.info(`Updating crate versions to ${newVersion}...\n`);
	}

	// Get all publishable crates by scanning crates directory
	const publishableCrates = getPublishableCrates();

	console.info(`Found ${publishableCrates.length} publishable crate(s)\n`);

	// Update workspace dependencies in root Cargo.toml
	updateWorkspaceDependencies(publishableCrates, newVersion, dryRun);

	// Update individual crate Cargo.toml files
	let updatedCount = 0;

	for (const crate of publishableCrates) {
		const result = updateCargoToml(crate.path, newVersion, dryRun);
		if (result.success) {
			if (dryRun) {
				console.info(
					`  ${crate.name}: ${result.oldVersion} -> ${result.newVersion}`,
				);
			}
			updatedCount++;
		}
	}

	console.info(
		`\n${dryRun ? "Would update" : "Updated"} ${updatedCount} crate(s) to version ${newVersion}`,
	);

	if (dryRun) {
		console.info("\nRun without --dry-run to apply these changes:");
		console.info(`   node scripts/update-crates-versions.mjs ${newVersion}`);
	}
}

main();
