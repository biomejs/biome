use crate::diagnostics::MigrationDiagnostic;
use crate::CliDiagnostic;
use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Merge;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_fs::{FileSystem, OpenOptions};
use biome_json_parser::JsonParserOptions;
use camino::Utf8Path;
use std::borrow::Cow;
use std::path::{Path, PathBuf};

use super::eslint_eslint;
use super::node;

/// This modules includes implementations for loading and deserializing an eslint configuration.
///
/// See [super::eslint_eslint] for the data representation of an ESLint configuration.
///
/// Each ESLint plugin has its own module in which rule options are defined.
/// For example, the ESLint TypeScript plugin is defined in [super::eslint_typescript].
/// Note that we don't need to deserialise every existing rule option.
/// We only need to deserialise options that have equivalent biome options.
/// This greatly reduces the amount of work involved.
///
/// ESLint flat configuration filenames.
///
/// See https://eslint.org/docs/latest/use/configure/configuration-files-new
const FLAT_CONFIG_FILES: [&str; 3] = [
    // Prefixed with `./` to ensure that it is loadable via Node.js's `import()`
    "./eslint.config.js",
    "./eslint.config.mjs",
    "./eslint.config.cjs",
];

/// List of ESLint **legacy** configuration filenames.
///
/// See https://eslint.org/docs/latest/use/configure/configuration-files
///
/// Order is important.
/// It translates the priority of the files.
/// For example, ESLint looks for `./.eslintrc.js` before looking for `./.eslintrc.json`.
const LEGACY_CONFIG_FILES: [&str; 6] = [
    // Prefixed with `./` to ensure that it is loadable via Node.js's `import()`
    "./.eslintrc.js",
    // Prefixed with `./` to ensure that it is loadable via Node.js's `import()`
    "./.eslintrc.cjs",
    ".eslintrc.yaml",
    ".eslintrc.yml",
    ".eslintrc.json",
    ".eslintrc",
];

/// An ESLint config can be embedded in `package.json`
const PACKAGE_JSON: &str = "package.json";

/// ESLint Ignore file. Use the same syntax as gitignore.
pub(crate) const IGNORE_FILE: &str = ".eslintignore";

/// Returns the ESLint configuration file in the working directory with the highest priority.
///
/// This function respects the priority between ESLint configuration files.
/// For example, it looks for `./.eslintrc.js` before looking for `./.eslintrc.json`.
/// It first ries to load a flat configuration file.
///
/// Unlike ESLint, it doesn't look for a configuration file in parent directories
/// when no configuration file is found in the working directory.
///
/// Deserialization errors are reported using `console`.
/// Other errors (File Not found, unsupported config format, ...) are directly returned.
///
/// We extract the ESLint configuration from a JavaScript file, by invoking `node`.
///
/// The `extends` field is recursively resolved.
pub(crate) fn read_eslint_config(
    fs: &dyn FileSystem,
    console: &mut dyn Console,
) -> Result<Config, CliDiagnostic> {
    for config_path_str in FLAT_CONFIG_FILES {
        let path = Utf8Path::new(config_path_str);
        if fs.path_exists(path) {
            return load_flat_config_data(path, console).map(|data| Config {
                path: config_path_str,
                data: data.into(),
            });
        }
    }
    for config_path_str in LEGACY_CONFIG_FILES {
        let path = Utf8Path::new(config_path_str);
        if fs.path_exists(path) {
            return load_legacy_config_data(fs, path, console).map(|data| Config {
                path: config_path_str,
                data: data.into(),
            });
        }
    }
    // We don't report an error if ESLint config is not embedded in `PACKAGE_JSON`.
    if let Ok(data) = load_legacy_config_data(fs, Utf8Path::new(PACKAGE_JSON), console) {
        return Ok(Config {
            path: PACKAGE_JSON,
            data: data.into(),
        });
    }
    Err(CliDiagnostic::MigrateError(MigrationDiagnostic { reason: "The default ESLint configuration file `.eslintrc[.*]` was not found in the working directory.".to_string()}))
}

#[derive(Debug)]
pub(crate) struct Config {
    /// Path of the ESlint config file
    pub(crate) path: &'static str,
    /// Resolved ESlint config
    pub(crate) data: eslint_eslint::AnyConfigData,
}

/// Load an ESlint Flat config
/// See https://eslint.org/docs/latest/use/configure/configuration-files-new
fn load_flat_config_data(
    path: &Utf8Path,
    console: &mut dyn Console,
) -> Result<eslint_eslint::FlatConfigData, CliDiagnostic> {
    let node::Resolution { content, .. } = node::load_config(path.as_ref())?;
    let (deserialized, diagnostics) = deserialize_from_json_str::<eslint_eslint::FlatConfigData>(
        &content,
        JsonParserOptions::default(),
        "",
    )
    .consume();
    let path_str = path.to_string();
    for diagnostic in diagnostics.into_iter().filter(|diag| {
        matches!(
            diag.severity(),
            biome_diagnostics::Severity::Fatal
                | biome_diagnostics::Severity::Error
                | biome_diagnostics::Severity::Warning
        )
    }) {
        let diagnostic = diagnostic.with_file_path(path_str.to_string());
        console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
    }
    if let Some(result) = deserialized {
        Ok(result)
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Could not deserialize the Eslint configuration file".to_string(),
        }))
    }
}

/// Load an ESlint legacy config
/// See https://eslint.org/docs/latest/use/configure/configuration-files
fn load_legacy_config_data(
    fs: &dyn FileSystem,
    path: &Utf8Path,
    console: &mut dyn Console,
) -> Result<eslint_eslint::LegacyConfigData, CliDiagnostic> {
    let (deserialized, diagnostics) = match path.extension() {
        None | Some("json") => {
            let mut file = fs.open_with_options(path, OpenOptions::default().read(true))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            if path.file_name().is_some_and(|name| name == PACKAGE_JSON) {
                let (deserialized, diagnostics) =
                    deserialize_from_json_str::<eslint_eslint::EslintPackageJson>(
                        &content,
                        JsonParserOptions::default()
                            .with_allow_trailing_commas()
                            .with_allow_comments(),
                        "",
                    )
                    .consume();
                (
                    deserialized.and_then(|mut packagejson| {
                        if let Some(eslint_config) = packagejson.eslint_config.as_mut() {
                            eslint_config
                                .ignore_patterns
                                .extend(packagejson.eslint_ignore);
                        }
                        packagejson.eslint_config
                    }),
                    diagnostics,
                )
            } else {
                deserialize_from_json_str::<eslint_eslint::LegacyConfigData>(
                    &content,
                    JsonParserOptions::default()
                        .with_allow_trailing_commas()
                        .with_allow_comments(),
                    "",
                )
                .consume()
            }
        }
        Some("js" | "cjs") => {
            let node::Resolution { content, .. } = node::load_config(path.as_ref())?;
            deserialize_from_json_str::<eslint_eslint::LegacyConfigData>(
                &content,
                JsonParserOptions::default(),
                "",
            )
            .consume()
        }
        Some(ext) => {
            return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: format!(
                    "ESLint configuration ending with the extension `{ext}` are not supported."
                ),
            }))
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
        let diagnostic = diagnostic.with_file_path(path_str.to_string());
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
            reason: "Could not deserialize the Eslint configuration file".to_string(),
        }))
    }
}

/// Returns the configuration from a preset or an error if the resolution failed.
///
/// This handles:
/// - native ESLint presets such as `eslint:recommended`;
/// - plugin presets such as `plugin:@typescript-eslint/recommended`;
/// - and shared configurations such as `standard`.
fn load_eslint_extends_config(
    name: &str,
) -> Result<eslint_eslint::LegacyConfigData, CliDiagnostic> {
    let (specifier, resolved_path, deserialized) = if let Some((protocol, rest)) =
        name.split_once(':')
    {
        let (module_name, config_name) = match protocol {
            // e.g. `eslint:recommended`
            //      - module_name: `@eslint/js`
            //      - config_name: `recommended`
            "eslint" => (Cow::Borrowed("@eslint/js"), rest),
            // e.g. `plugin:@typescript-eslint/recommended`
            //      - module_name: `@typescript-eslint/eslint-plugin`
            //      - config_name: `recommended`
            // e.g. `plugin:unicorn/recommended`
            //      - module_name: `eslint-plugin-unicorn`
            //      - config_name: `recommended`
            "plugin" => {
                let Some(config_name) = rest.split('/').last() else {
                    return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                        reason: format!(
                            "The configuration {rest} cannot be resolved. Make sure that your ESLint configuration file is valid."
                        ),
                    }));
                };
                let rest = rest.trim_end_matches(config_name);
                let module_name = rest.trim_end_matches('/');
                let module_name = EslintPackage::Plugin.resolve_name(module_name);
                (module_name, config_name)
            }
            name => {
                return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                    reason: format!(
                        "The module {name} cannot be resolved. This is likely an internal error."
                    ),
                }));
            }
        };
        // load ESLint preset
        let node::Resolution {
            content,
            resolved_path,
        } = node::load_config(&module_name)?;
        let deserialized = deserialize_from_json_str::<eslint_eslint::PluginExport>(
            &content,
            JsonParserOptions::default(),
            "",
        )
        .into_deserialized();
        if let Some(mut deserialized) = deserialized {
            let deserialized = deserialized.configs.remove(config_name);
            if deserialized.is_none() {
                return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                    reason: format!("The ESLint configuration '{config_name}' cannot be extracted from the module '{module_name}'. Make sure that '{config_name}' is a valid configuration name.")
                }));
            }
            (module_name, resolved_path, deserialized)
        } else {
            (module_name, resolved_path, None)
        }
    } else {
        // load ESLint shared config
        let module_name = if matches!(name.as_bytes().first(), Some(b'.' | b'/' | b'#')) {
            // local path
            Cow::Borrowed(name)
        } else {
            EslintPackage::Config.resolve_name(name)
        };
        // Try to load `module_name` or else try to load diretcly `name`.
        let node::Resolution {
            content,
            resolved_path,
        } = node::load_config(&module_name).or_else(|err| {
            if module_name != name {
                node::load_config(name)
            } else {
                Err(err)
            }
        })?;
        let deserialized = deserialize_from_json_str::<eslint_eslint::LegacyConfigData>(
            &content,
            JsonParserOptions::default(),
            "",
        )
        .into_deserialized();
        (module_name, resolved_path, deserialized)
    };
    let Some(mut deserialized) = deserialized else {
        return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: format!("The ESLint configuration of the module '{specifier}' cannot be extracted. This is likely an internal error.")
        }));
    };
    // Resolve relative path in `extends`.
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
/// Unknown presets are ignored.
/// `self.extends` is replaced by an empty array.
fn resolve_extends(config: &mut eslint_eslint::LegacyConfigData, console: &mut dyn Console) {
    let extensions: Vec<_> = config
        .extends
        .iter()
        .filter_map(|preset| match load_eslint_extends_config(preset) {
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

/// ESLint to specific rules to resolve a module name.
/// See https://eslint.org/docs/latest/extend/shareable-configs#using-a-shareable-config
/// See also https://eslint.org/docs/latest/extend/plugins
#[derive(Debug)]
enum EslintPackage {
    Config,
    Plugin,
}
impl EslintPackage {
    fn resolve_name<'a>(&self, name: &'a str) -> Cow<'a, str> {
        let artifact = match self {
            EslintPackage::Config => "eslint-config-",
            EslintPackage::Plugin => "eslint-plugin-",
        };
        if name.starts_with('@') {
            // handle scoped package
            if let Some((scope, rest)) = name.split_once('/') {
                let package = rest.split('/').next().unwrap_or(rest);
                if rest.starts_with(artifact) || package == artifact.trim_end_matches('-') {
                    Cow::Borrowed(name)
                } else {
                    Cow::Owned(format!("{scope}/{artifact}{rest}"))
                }
            } else {
                let artifact = artifact.trim_end_matches('-');
                Cow::Owned(format!("{name}/{artifact}"))
            }
        } else if name.starts_with(artifact) {
            Cow::Borrowed(name)
        } else {
            Cow::Owned(format!("{artifact}{name}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eslint_package_resolve_name() {
        assert_eq!(
            EslintPackage::Config.resolve_name("@scope/package"),
            "@scope/eslint-config-package"
        );
        assert_eq!(
            EslintPackage::Config.resolve_name("@scope/eslint-config-package"),
            "@scope/eslint-config-package"
        );
        assert_eq!(
            EslintPackage::Config.resolve_name("@scope/eslint-config"),
            "@scope/eslint-config"
        );

        assert_eq!(
            EslintPackage::Config.resolve_name("@scope/package/path"),
            "@scope/eslint-config-package/path"
        );
        assert_eq!(
            EslintPackage::Config.resolve_name("@scope/eslint-config-package/path"),
            "@scope/eslint-config-package/path"
        );
        assert_eq!(
            EslintPackage::Config.resolve_name("@scope/eslint-config/path"),
            "@scope/eslint-config/path"
        );

        assert_eq!(
            EslintPackage::Config.resolve_name("package"),
            "eslint-config-package"
        );
        assert_eq!(
            EslintPackage::Config.resolve_name("eslint-config-package"),
            "eslint-config-package"
        );
    }
}
