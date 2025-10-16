#!/usr/bin/env node

/**
 * Script to update versions of publishable Biome crates
 * Usage: node scripts/update-crates-versions.mjs <new-version> [--dry-run]
 * Example: node scripts/update-crates-versions.mjs 0.6.0
 * Example (dry-run): node scripts/update-crates-versions.mjs 0.6.0 --dry-run
 */

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { parseArgs } from "node:util";

const __filename = import.meta.filename;
const __dirname = import.meta.dirname;

// Matches a version field in Cargo.toml, capturing the field name with whitespace
// and the version value. Supports any whitespace/tabs around the equals sign.
// Example matches: version = "0.5.7", version      = "1.0.0"
// Capture groups: $1 = 'version\s*=\s*', $2 = version value
const CARGO_VERSION_FIELD_REGEX = /^(version\s*=\s*)"([^"]+)"$/gm;

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

// List of publishable crates (those that have a version in workspace dependencies)
const PUBLISHABLE_CRATES = [
	"biome_analyze",
	"biome_aria",
	"biome_aria_metadata",
	"biome_console",
	"biome_control_flow",
	"biome_css_analyze",
	"biome_css_factory",
	"biome_css_formatter",
	"biome_css_parser",
	"biome_css_syntax",
	"biome_deserialize",
	"biome_deserialize_macros",
	"biome_diagnostics",
	"biome_diagnostics_categories",
	"biome_diagnostics_macros",
	"biome_formatter",
	"biome_fs",
	"biome_graphql_analyze",
	"biome_graphql_factory",
	"biome_graphql_formatter",
	"biome_graphql_parser",
	"biome_graphql_syntax",
	"biome_grit_factory",
	"biome_grit_parser",
	"biome_grit_syntax",
	"biome_html_factory",
	"biome_html_parser",
	"biome_html_syntax",
	"biome_js_analyze",
	"biome_js_factory",
	"biome_js_formatter",
	"biome_js_parser",
	"biome_js_semantic",
	"biome_js_syntax",
	"biome_js_type_info",
	"biome_js_type_info_macros",
	"biome_jsdoc_comment",
	"biome_json_analyze",
	"biome_json_factory",
	"biome_json_formatter",
	"biome_json_parser",
	"biome_json_syntax",
	"biome_markup",
	"biome_package",
	"biome_parser",
	"biome_project_layout",
	"biome_rowan",
	"biome_rule_options",
	"biome_string_case",
	"biome_suppression",
	"biome_text_edit",
	"biome_text_size",
	"biome_unicode_table",
];

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
 * @param {string} newVersion - New version string
 * @param {boolean} dryRun - If true, don't write changes
 * @returns {{success: boolean, changes: Array<{crate: string, oldVersion: string, newVersion: string}>}}
 */
function updateWorkspaceDependencies(newVersion, dryRun = false) {
	const rootCargoPath = path.join(__dirname, "..", "Cargo.toml");
	const content = fs.readFileSync(rootCargoPath, "utf8");

	// For each publishable crate, update its version in workspace dependencies
	let updatedContent = content;
	const changes = [];

	for (const crateName of PUBLISHABLE_CRATES) {
		const regex = new RegExp(
			WORKSPACE_DEPENDENCY_REGEX_TEMPLATE(crateName),
			"gm",
		);

		const match = regex.exec(updatedContent);
		if (match) {
			const oldVersion = match[2];
			changes.push({ crate: crateName, oldVersion, newVersion });

			// Reset regex for replacement
			regex.lastIndex = 0;
			updatedContent = updatedContent.replace(regex, `$1"${newVersion}"`);
		}
	}

	if (content !== updatedContent) {
		if (!dryRun) {
			fs.writeFileSync(rootCargoPath, updatedContent, "utf8");
		}
		return { success: true, changes };
	}

	return { success: false, changes: [] };
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
		console.info("Usage: node scripts/update-crates-versions.mjs <version> [--dry-run]");
		console.info("\nOptions:");
		console.info("  --dry-run    Show what would be updated without making changes");
		console.info("  --help       Show this help message");
		console.info("\nExamples:");
		console.info("  node scripts/update-crates-versions.mjs 0.6.0");
		console.info("  node scripts/update-crates-versions.mjs 0.6.0 --dry-run");
		process.exit(0);
	}

	if (positionals.length === 0) {
		console.error("Error: Missing version argument");
		console.error("Usage: node scripts/update-crates-versions.mjs <version> [--dry-run]");
		console.error("Run with --help for more information");
		process.exit(1);
	}

	const newVersion = positionals[0];
	const dryRun = values["dry-run"];

	// Validate version format (basic semver check)
	if (!SEMVER_REGEX.test(newVersion)) {
		console.error(`Error: Invalid version format: ${newVersion}`);
		console.error(
			"Expected format: X.Y.Z (e.g., 0.6.0, 1.0.0, 2.1.3)",
		);
		process.exit(1);
	}

	if (dryRun) {
		console.info(
			`[DRY RUN] Checking what would be updated to version ${newVersion}...\n`,
		);
	} else {
		console.info(`Updating crate versions to ${newVersion}...\n`);
	}

	// Update workspace dependencies
	const workspaceResult = updateWorkspaceDependencies(newVersion, dryRun);

	// Update individual crate Cargo.toml files
	let updatedCount = 0;

	for (const crateName of PUBLISHABLE_CRATES) {
		const cargoTomlPath = path.join(
			__dirname,
			"..",
			"crates",
			crateName,
			"Cargo.toml",
		);

		const result = updateCargoToml(cargoTomlPath, newVersion, dryRun);
		if (result.success) {
			if (dryRun) {
				console.info(
					`${crateName}: ${result.oldVersion} -> ${result.newVersion}`,
				);
			}
			updatedCount++;
		}
	}

	console.info(
		`\n${dryRun ? "Would update" : "Updated"} ${updatedCount} crate(s) to version ${newVersion}`,
	);

	if (dryRun) {
		console.info(
			"\nRun without --dry-run to apply these changes:",
		);
		console.info(`   node scripts/update-crates-versions.mjs ${newVersion}`);
	}
}

main();
