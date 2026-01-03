/// The 110 predefined package.json fields in their canonical order.
/// Based on https://github.com/keithamus/sort-package-json/blob/main/defaultRules.md
pub const PACKAGE_JSON_FIELD_ORDER: &[&str] = &[
    // Core metadata
    "$schema",
    "name",
    "displayName", // VSCode
    "version",
    "stableVersion", // Yarn
    "private",
    "description",
    "categories", // VSCode
    "keywords",
    "homepage",
    // Contact & licensing
    "bugs",
    "repository",
    "funding",
    "license",
    "qna", // VSCode
    "author",
    "maintainers",
    "contributors",
    "publisher", // VSCode
    // Module configuration
    "sideEffects",
    "type",
    "imports",
    "exports",
    // Entry points
    "main",
    "svelte",
    "umd:main",
    "jsdelivr",
    "unpkg",
    "module",
    "source",
    "jsnext:main",
    "browser",
    "react-native",
    // TypeScript
    "types",
    "typesVersions",
    "typings",
    // Assets
    "style",
    "example",
    "examplestyle",
    "assets",
    // Binary & files
    "bin",
    "man",
    "directories",
    "files",
    "workspaces",
    "binary",
    // Scripts
    "scripts",
    "betterScripts",
    // VSCode extensions
    "l10n",             // VSCode
    "contributes",      // VSCode
    "activationEvents", // VSCode
    // Git hooks & linting
    "husky",
    "simple-git-hooks",
    "pre-commit",
    "commitlint",
    "lint-staged",
    "nano-staged",
    // Configuration tools
    "config",
    "nodemonConfig",
    "browserify",
    "babel",
    "browserslist",
    "xo",
    "prettier",
    "eslintConfig",
    "eslintIgnore",
    "npmpkgjsonlint",
    "npmPackageJsonLintConfig",
    "npmpackagejsonlint",
    // Testing & coverage
    "release",
    "remarkConfig",
    "stylelint",
    "ava",
    "jest",
    "jest-junit",
    "jest-stare",
    "mocha",
    "nyc",
    "c8",
    "tap",
    // CLI tools
    "oclif",
    // Dependencies
    "resolutions",
    "overrides",
    "dependencies",
    "devDependencies",
    "dependenciesMeta",
    "peerDependencies",
    "peerDependenciesMeta",
    "optionalDependencies",
    "bundledDependencies",
    "bundleDependencies",
    "extensionPack",         // VSCode
    "extensionDependencies", // VSCode
    // Package manager
    "flat",
    "packageManager",
    // Engines
    "engines",
    "engineStrict",
    "devEngines",
    "volta",
    // Platform
    "languageName",
    "os",
    "cpu",
    // Publishing
    "preferGlobal",
    "publishConfig",
    // VSCode metadata
    "icon",          // VSCode
    "badges",        // VSCode
    "galleryBanner", // VSCode
    "preview",       // VSCode
    "markdown",      // VSCode
    // pnpm
    "pnpm",
];

/// Get the index of a field in the canonical order.
/// Returns None if the field is not in the predefined list.
pub fn get_field_index(field_name: &str) -> Option<usize> {
    PACKAGE_JSON_FIELD_ORDER
        .iter()
        .position(|&name| name == field_name)
}

/// Check if a field is a private field (starts with underscore).
pub fn is_private_field(field_name: &str) -> bool {
    field_name.starts_with('_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_ordering() {
        // Test that commonly used fields are in the correct relative order
        // This test is resilient to adding/removing fields
        let schema_idx = get_field_index("$schema").unwrap();
        let name_idx = get_field_index("name").unwrap();
        let version_idx = get_field_index("version").unwrap();
        let description_idx = get_field_index("description").unwrap();
        let scripts_idx = get_field_index("scripts").unwrap();
        let dependencies_idx = get_field_index("dependencies").unwrap();
        let dev_dependencies_idx = get_field_index("devDependencies").unwrap();
        let engines_idx = get_field_index("engines").unwrap();

        // Core metadata comes first
        assert!(schema_idx < name_idx);
        assert!(name_idx < version_idx);
        assert!(version_idx < description_idx);

        // Scripts come before dependencies
        assert!(scripts_idx < dependencies_idx);

        // Dependencies are grouped together
        assert!(dependencies_idx < dev_dependencies_idx);

        // Engines come after dependencies
        assert!(dev_dependencies_idx < engines_idx);

        // Unknown fields should return None
        assert_eq!(get_field_index("unknown-field"), None);
        assert_eq!(get_field_index("custom-field"), None);
    }

    #[test]
    fn test_private_fields() {
        assert!(is_private_field("_internal"));
        assert!(is_private_field("_privateKey"));
        assert!(!is_private_field("name"));
        assert!(!is_private_field("version"));
    }
}
