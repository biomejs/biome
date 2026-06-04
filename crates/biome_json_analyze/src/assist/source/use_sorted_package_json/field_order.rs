#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldTransformer {
    None,
    SortObject,
    SortPeopleObject,
    SortURLObject,
    SortBugsObject,
    SortDirectories,
    SortVolta,
    SortBinary,
    SortGitHooks,
    SortDependencies,
    SortDependenciesMeta,
    SortScripts,
    SortExports,
    SortEslintConfig,
    SortPrettierConfig,
    SortPeopleArray,
    SortBadgesArray,
    SortObjectDeep,
    SortHusky,
    SortDevEngines,
    SortWorkspaces,
    SortPnpmConfig,
    UniqArray,
    UniqAndSortArray,
}

#[derive(Debug, Clone, Copy)]
pub struct FieldMetadata {
    pub key: &'static str,
    pub transformer: FieldTransformer,
}

/// VSCode extension manifest fields: https://code.visualstudio.com/api/references/extension-manifest
/// Based on https://github.com/keithamus/sort-package-json/blob/main/defaultRules.md
pub const PACKAGE_JSON_FIELDS: &[FieldMetadata] = &[
    FieldMetadata {
        key: "$schema",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "name",
        transformer: FieldTransformer::None,
    },
    /* vscode */
    FieldMetadata {
        key: "displayName",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "version",
        transformer: FieldTransformer::None,
    },
    /* yarn */
    FieldMetadata {
        key: "stableVersion",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "private",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "description",
        transformer: FieldTransformer::None,
    },
    /* vscode */
    FieldMetadata {
        key: "categories",
        transformer: FieldTransformer::UniqArray,
    },
    FieldMetadata {
        key: "keywords",
        transformer: FieldTransformer::UniqArray,
    },
    FieldMetadata {
        key: "homepage",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "bugs",
        transformer: FieldTransformer::SortBugsObject,
    },
    FieldMetadata {
        key: "repository",
        transformer: FieldTransformer::SortURLObject,
    },
    FieldMetadata {
        key: "funding",
        transformer: FieldTransformer::SortURLObject,
    },
    FieldMetadata {
        key: "license",
        transformer: FieldTransformer::SortURLObject,
    },
    /* vscode */
    FieldMetadata {
        key: "qna",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "author",
        transformer: FieldTransformer::SortPeopleObject,
    },
    FieldMetadata {
        key: "maintainers",
        transformer: FieldTransformer::SortPeopleArray,
    },
    FieldMetadata {
        key: "contributors",
        transformer: FieldTransformer::SortPeopleArray,
    },
    /* vscode */
    FieldMetadata {
        key: "publisher",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "sideEffects",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "type",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "imports",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "exports",
        transformer: FieldTransformer::SortExports,
    },
    FieldMetadata {
        key: "main",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "svelte",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "umd:main",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "jsdelivr",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "unpkg",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "module",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "source",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "jsnext:main",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "browser",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "react-native",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "types",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "typesVersions",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "typings",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "style",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "example",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "examplestyle",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "assets",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "bin",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "man",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "directories",
        transformer: FieldTransformer::SortDirectories,
    },
    FieldMetadata {
        key: "files",
        transformer: FieldTransformer::UniqArray,
    },
    FieldMetadata {
        key: "workspaces",
        transformer: FieldTransformer::SortWorkspaces,
    },
    // node-pre-gyp https://www.npmjs.com/package/node-pre-gyp#1-add-new-entries-to-your-packagejson
    FieldMetadata {
        key: "binary",
        transformer: FieldTransformer::SortBinary,
    },
    FieldMetadata {
        key: "scripts",
        transformer: FieldTransformer::SortScripts,
    },
    FieldMetadata {
        key: "betterScripts",
        transformer: FieldTransformer::SortScripts,
    },
    /* vscode */
    FieldMetadata {
        key: "l10n",
        transformer: FieldTransformer::None,
    },
    /* vscode */
    FieldMetadata {
        key: "contributes",
        transformer: FieldTransformer::SortObject,
    },
    /* vscode */
    FieldMetadata {
        key: "activationEvents",
        transformer: FieldTransformer::UniqArray,
    },
    FieldMetadata {
        key: "husky",
        transformer: FieldTransformer::SortHusky,
    },
    FieldMetadata {
        key: "simple-git-hooks",
        transformer: FieldTransformer::SortGitHooks,
    },
    FieldMetadata {
        key: "pre-commit",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "commitlint",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "lint-staged",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "nano-staged",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "config",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "nodemonConfig",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "browserify",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "babel",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "browserslist",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "xo",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "prettier",
        transformer: FieldTransformer::SortPrettierConfig,
    },
    FieldMetadata {
        key: "eslintConfig",
        transformer: FieldTransformer::SortEslintConfig,
    },
    FieldMetadata {
        key: "eslintIgnore",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "npmpkgjsonlint",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "npmPackageJsonLintConfig",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "npmpackagejsonlint",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "release",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "remarkConfig",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "stylelint",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "ava",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "jest",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "jest-junit",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "jest-stare",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "mocha",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "nyc",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "c8",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "tap",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "oclif",
        transformer: FieldTransformer::SortObjectDeep,
    },
    FieldMetadata {
        key: "resolutions",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "overrides",
        transformer: FieldTransformer::SortDependencies,
    },
    FieldMetadata {
        key: "dependencies",
        transformer: FieldTransformer::SortDependencies,
    },
    FieldMetadata {
        key: "devDependencies",
        transformer: FieldTransformer::SortDependencies,
    },
    FieldMetadata {
        key: "dependenciesMeta",
        transformer: FieldTransformer::SortDependenciesMeta,
    },
    FieldMetadata {
        key: "peerDependencies",
        transformer: FieldTransformer::SortDependencies,
    },
    FieldMetadata {
        key: "peerDependenciesMeta",
        transformer: FieldTransformer::SortObjectDeep,
    },
    FieldMetadata {
        key: "optionalDependencies",
        transformer: FieldTransformer::SortDependencies,
    },
    FieldMetadata {
        key: "bundledDependencies",
        transformer: FieldTransformer::UniqAndSortArray,
    },
    FieldMetadata {
        key: "bundleDependencies",
        transformer: FieldTransformer::UniqAndSortArray,
    },
    /* vscode */
    FieldMetadata {
        key: "extensionPack",
        transformer: FieldTransformer::UniqAndSortArray,
    },
    /* vscode */
    FieldMetadata {
        key: "extensionDependencies",
        transformer: FieldTransformer::UniqAndSortArray,
    },
    FieldMetadata {
        key: "flat",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "packageManager",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "engines",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "engineStrict",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "devEngines",
        transformer: FieldTransformer::SortDevEngines,
    },
    FieldMetadata {
        key: "volta",
        transformer: FieldTransformer::SortVolta,
    },
    FieldMetadata {
        key: "languageName",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "os",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "cpu",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "preferGlobal",
        transformer: FieldTransformer::SortObject,
    },
    FieldMetadata {
        key: "publishConfig",
        transformer: FieldTransformer::SortObject,
    },
    /* vscode */
    FieldMetadata {
        key: "icon",
        transformer: FieldTransformer::None,
    },
    /* vscode */
    FieldMetadata {
        key: "badges",
        transformer: FieldTransformer::SortBadgesArray,
    },
    /* vscode */
    FieldMetadata {
        key: "galleryBanner",
        transformer: FieldTransformer::SortObject,
    },
    /* vscode */
    FieldMetadata {
        key: "preview",
        transformer: FieldTransformer::None,
    },
    /* vscode */
    FieldMetadata {
        key: "markdown",
        transformer: FieldTransformer::None,
    },
    FieldMetadata {
        key: "pnpm",
        transformer: FieldTransformer::SortPnpmConfig,
    },
];

pub fn get_field_index(field_name: &str) -> Option<usize> {
    PACKAGE_JSON_FIELDS
        .iter()
        .position(|metadata| metadata.key == field_name)
}

pub fn get_field_transformer(field_name: &str) -> FieldTransformer {
    PACKAGE_JSON_FIELDS
        .iter()
        .find(|metadata| metadata.key == field_name)
        .map_or(FieldTransformer::None, |metadata| metadata.transformer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_ordering() {
        let schema_idx = get_field_index("$schema").unwrap();
        let name_idx = get_field_index("name").unwrap();
        let version_idx = get_field_index("version").unwrap();
        let description_idx = get_field_index("description").unwrap();
        let scripts_idx = get_field_index("scripts").unwrap();
        let dependencies_idx = get_field_index("dependencies").unwrap();
        let dev_dependencies_idx = get_field_index("devDependencies").unwrap();
        let engines_idx = get_field_index("engines").unwrap();

        assert!(schema_idx < name_idx);
        assert!(name_idx < version_idx);
        assert!(version_idx < description_idx);
        assert!(scripts_idx < dependencies_idx);
        assert!(dependencies_idx < dev_dependencies_idx);
        assert!(dev_dependencies_idx < engines_idx);
        assert_eq!(get_field_index("unknown-field"), None);
        assert_eq!(get_field_index("custom-field"), None);
    }
}
