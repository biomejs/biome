//! Field order constants for various package.json object types

/// https://docs.npmjs.com/cli/v10/configuring-npm/package-json#people-fields-author-contributors
/// People field order (author, contributors, maintainers)
pub const PEOPLE_FIELD_ORDER: &[&str] = &["name", "email", "url"];

/// URL object field order (repository, bugs when object type)
pub const URL_FIELD_ORDER: &[&str] = &["type", "url"];

/// Bugs object field order
pub const BUGS_FIELD_ORDER: &[&str] = &["url", "email"];

/// Directories field order
pub const DIRECTORIES_FIELD_ORDER: &[&str] = &["lib", "bin", "man", "doc", "example", "test"];

/// Volta field order
pub const VOLTA_FIELD_ORDER: &[&str] = &["node", "npm", "yarn"];

/// Binary field order
pub const BINARY_FIELD_ORDER: &[&str] = &[
    "module_name",
    "module_path",
    "remote_path",
    "package_name",
    "host",
];

/// Git hooks in execution order (from git-hooks-list package)
pub const GIT_HOOKS_ORDER: &[&str] = &[
    "applypatch-msg",
    "pre-applypatch",
    "post-applypatch",
    "pre-commit",
    "pre-merge-commit",
    "prepare-commit-msg",
    "commit-msg",
    "post-commit",
    "pre-rebase",
    "post-checkout",
    "post-merge",
    "pre-push",
    "pre-receive",
    "update",
    "proc-receive",
    "post-receive",
    "post-update",
    "reference-transaction",
    "push-to-checkout",
    "pre-auto-gc",
    "post-rewrite",
    "sendemail-validate",
    "fsmonitor-watchman",
    "p4-changelist",
    "p4-prepare-changelist",
    "p4-post-changelist",
    "p4-pre-submit",
    "post-index-change",
];

/// VS Code badge field order
pub const VSCODE_BADGE_FIELD_ORDER: &[&str] = &["description", "url", "href"];

/// Workspaces field order
pub const WORKSPACES_FIELD_ORDER: &[&str] = &["packages", "catalog"];

/// devEngines.packageManager field order
pub const DEV_ENGINES_PM_ORDER: &[&str] = &["name", "version", "onFail"];

/// pnpm config base properties order
pub const PNPM_BASE_CONFIG_PROPERTIES: &[&str] = &[
    "neverBuiltDependencies",
    "onlyBuiltDependencies",
    "onlyBuiltDependenciesFile",
    "overrides",
    "packageExtensions",
    "patchedDependencies",
    "peerDependencyRules",
    "allowedDeprecatedVersions",
    "allowNonAppliedPatches",
    "auditConfig",
    "ignoredOptionalDependencies",
    "updateConfig",
];

/// ESLint config base properties order
pub const ESLINT_BASE_CONFIG_PROPERTIES: &[&str] = &[
    "env",
    "parser",
    "parserOptions",
    "settings",
    "plugins",
    "extends",
    "rules",
    "overrides",
    "globals",
    "processor",
    "noInlineConfig",
    "reportUnusedDisableDirectives",
];
