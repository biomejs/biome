//! This module includes implementations for loading and deserializing a Stylelint configuration.
//!
//! See [super::stylelint_stylelint] for the data representation of a Stylelint configuration.

use crate::CliDiagnostic;
use crate::diagnostics::MigrationDiagnostic;
use biome_console::{Console, ConsoleExt, markup};
use biome_deserialize::Merge;
use biome_deserialize::json::deserialize_from_json_str;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_fs::{FileSystem, OpenOptions};
use biome_json_parser::JsonParserOptions;
use camino::Utf8Path;
use std::path::{Path, PathBuf};

use super::node;
use super::stylelint_stylelint;

/// List of Stylelint configuration filenames.
///
/// See <https://stylelint.io/user-guide/configure>
///
/// Order is important. It translates the priority of the files, matching the
/// lookup order used by Stylelint's `cosmiconfig` search.
const CONFIG_FILES: [&str; 8] = [
    ".stylelintrc",
    ".stylelintrc.json",
    // Prefixed with `./` to ensure that it is loadable via Node.js's `import()`.
    "./.stylelintrc.js",
    "./.stylelintrc.cjs",
    "./.stylelintrc.mjs",
    "./stylelint.config.js",
    "./stylelint.config.cjs",
    "./stylelint.config.mjs",
];

/// A Stylelint config can be embedded in `package.json`.
const PACKAGE_JSON: &str = "package.json";

/// Stylelint ignore file. Uses the same syntax as gitignore.
pub(crate) const IGNORE_FILE: &str = ".stylelintignore";

/// Returns the Stylelint configuration file in the working directory with the highest priority.
///
/// Unlike Stylelint, it doesn't look for a configuration file in parent directories
/// when no configuration file is found in the working directory.
///
/// Deserialization errors are reported using `console`.
/// Other errors (file not found, unsupported config format, ...) are directly returned.
///
/// The `extends` field is recursively resolved.
pub(crate) fn read_stylelint_config(
    fs: &dyn FileSystem,
    console: &mut dyn Console,
) -> Result<Config, CliDiagnostic> {
    for config_path_str in CONFIG_FILES {
        let path = Utf8Path::new(config_path_str);
        if fs.path_exists(path) {
            return load_config_data(fs, path, console).map(|data| Config {
                path: config_path_str,
                data,
            });
        }
    }
    // We don't report an error if the Stylelint config is not embedded in `PACKAGE_JSON`.
    if let Ok(data) = load_config_data(fs, Utf8Path::new(PACKAGE_JSON), console) {
        return Ok(Config {
            path: PACKAGE_JSON,
            data,
        });
    }
    Err(CliDiagnostic::MigrateError(MigrationDiagnostic { reason: "The default Stylelint configuration file `.stylelintrc[.*]` was not found in the working directory.".to_string()}))
}

#[derive(Debug)]
pub(crate) struct Config {
    /// Path of the Stylelint config file.
    pub(crate) path: &'static str,
    /// Resolved Stylelint config.
    pub(crate) data: stylelint_stylelint::StylelintConfigData,
}

/// Load a Stylelint config file and recursively resolve its `extends` field.
fn load_config_data(
    fs: &dyn FileSystem,
    path: &Utf8Path,
    console: &mut dyn Console,
) -> Result<stylelint_stylelint::StylelintConfigData, CliDiagnostic> {
    let (deserialized, diagnostics) = match path.extension() {
        None | Some("json") => {
            let mut file = fs.open_with_options(path, OpenOptions::default().read(true))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            if path.file_name().is_some_and(|name| name == PACKAGE_JSON) {
                let (deserialized, diagnostics) =
                    deserialize_from_json_str::<stylelint_stylelint::StylelintPackageJson>(
                        &content,
                        JsonParserOptions::default()
                            .with_allow_trailing_commas()
                            .with_allow_comments(),
                        "",
                    )
                    .consume();
                (
                    deserialized.and_then(|packagejson| packagejson.stylelint),
                    diagnostics,
                )
            } else {
                deserialize_from_json_str::<stylelint_stylelint::StylelintConfigData>(
                    &content,
                    JsonParserOptions::default()
                        .with_allow_trailing_commas()
                        .with_allow_comments(),
                    "",
                )
                .consume()
            }
        }
        Some("js" | "cjs" | "mjs") => {
            let node::Resolution { content, .. } = node::load_config(path.as_ref())?;
            deserialize_from_json_str::<stylelint_stylelint::StylelintConfigData>(
                &content,
                JsonParserOptions::default(),
                "",
            )
            .consume()
        }
        Some(ext) => {
            return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: format!(
                    "Stylelint configuration ending with the extension `{ext}` are not supported."
                ),
            }));
        }
    };
    let path_str = path.to_string();
    for diagnostic in diagnostics.into_iter().filter(|diag| {
        matches!(
            diag.severity(),
            biome_diagnostics::Severity::Fatal
                | biome_diagnostics::Severity::Error
                | biome_diagnostics::Severity::Warning
        )
    }) {
        let diagnostic = diagnostic.with_file_path(path_str.clone());
        console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
    }
    if let Some(mut result) = deserialized {
        // recursively resolve the `extends` field.
        while !result.extends.is_empty() {
            resolve_extends(&mut result, console);
        }
        Ok(result)
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Could not deserialize the Stylelint configuration file".to_string(),
        }))
    }
}

/// Returns the configuration from an extended module or an error if the resolution failed.
///
/// Unlike ESLint, Stylelint doesn't add prefixes to the extended module names,
/// so the name is resolved as-is.
fn load_stylelint_extends_config(
    name: &str,
) -> Result<stylelint_stylelint::StylelintConfigData, CliDiagnostic> {
    let node::Resolution {
        content,
        resolved_path,
    } = node::load_config(name)?;
    let deserialized = deserialize_from_json_str::<stylelint_stylelint::StylelintConfigData>(
        &content,
        JsonParserOptions::default(),
        "",
    )
    .into_deserialized();
    let Some(mut deserialized) = deserialized else {
        return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: format!(
                "The Stylelint configuration of the module '{name}' cannot be extracted. This is likely an internal error."
            ),
        }));
    };
    // Resolve relative paths in `extends`.
    deserialized.extends.iter_mut().for_each(|extends_item| {
        if extends_item.starts_with('.') {
            let Some(resolved_path) = Path::new(&resolved_path).parent() else {
                return;
            };
            let mut path = PathBuf::new();
            path.push(resolved_path);
            path.push(Path::new(&extends_item));
            *extends_item = path.to_string_lossy().to_string();
        }
    });
    Ok(deserialized)
}

/// Load and merge included configuration via `self.extends`.
///
/// Unknown configurations are ignored.
/// `self.extends` is replaced by an empty array.
fn resolve_extends(
    config: &mut stylelint_stylelint::StylelintConfigData,
    console: &mut dyn Console,
) {
    let extensions: Vec<_> = config
        .extends
        .iter()
        .filter_map(|preset| match load_stylelint_extends_config(preset) {
            Ok(config) => Some(config),
            Err(diag) => {
                console.error(markup! {{PrintDiagnostic::simple(&diag)}});
                None
            }
        })
        .collect();
    config.extends.clear();
    for ext in extensions {
        config.merge_with(ext);
    }
}
