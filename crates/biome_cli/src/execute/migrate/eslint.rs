use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Merge;
use biome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use biome_fs::{FileSystem, OpenOptions};
use biome_json_parser::JsonParserOptions;
use biome_service::DynRef;
use indexmap::IndexSet;
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec;

use crate::diagnostics::MigrationDiagnostic;
use crate::CliDiagnostic;

use super::eslint_eslint;

/// This modules includes implementations for loading and deserializing an eslint configuration.
///
/// See [super::eslint_eslint] for the data representation of an ESLint configuration.
///
/// Each ESLint plugin has its own module in which rule options are defined.
/// For example, the ESLint TypeScript plugin is defined in [super::eslint_typescript].
/// Note that we don't need to deserialise every existing rule option.
/// We only need to deserialise options that have equivalent biome options.
/// This greatly reduces the amount of work involved.

/// ESLint flat configuration filenames.
///
/// See https://eslint.org/docs/latest/use/configure/configuration-files-new
const FLAT_CONFIG_FILES: [&str; 3] = [
    // Prefixed with `./` to ensure that it is loadable via NodeJS's `import()`
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
const LEGACY_CONFIG_FILES: [&str; 5] = [
    // Prefixed with `./` to ensure that it is loadable via NodeJS's `import()`
    "./.eslintrc.js",
    // Prefixed with `./` to ensure that it is loadable via NodeJS's `import()`
    "./.eslintrc.cjs",
    ".eslintrc.yaml",
    ".eslintrc.yml",
    ".eslintrc.json",
];

/// An ESLint config can be embedded in `package.json`
const PACKAGE_JSON: &str = "package.json";

/// ESLint Ignore file. Use the same syntax as gitignore.
const IGNORE_FILE: &str = ".eslintignore";

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
/// Other errors (File Not found, unspported config format, ...) are directly returned.
///
/// We extract the ESLint configuration from a JavaScript file, by invoking `node`.
///
/// The `extends` field is recusively resolved.
pub(crate) fn read_eslint_config(
    fs: &DynRef<'_, dyn FileSystem>,
    console: &mut dyn Console,
) -> Result<Config, CliDiagnostic> {
    for config_path_str in FLAT_CONFIG_FILES {
        let path = Path::new(config_path_str);
        if fs.path_exists(path) {
            return load_flat_config_data(path, console).map(|data| Config {
                path: config_path_str,
                data: data.into(),
            });
        }
    }
    for config_path_str in LEGACY_CONFIG_FILES {
        let path = Path::new(config_path_str);
        if fs.path_exists(path) {
            return load_legacy_config_data(fs, path, console).map(|data| Config {
                path: config_path_str,
                data: data.into(),
            });
        }
    }
    let path = Path::new(PACKAGE_JSON);
    if fs.path_exists(path) {
        // We don't report errors in `PACKAGE_JSON`.
        if let Ok(data) = load_legacy_config_data(fs, path, console) {
            return Ok(Config {
                path: PACKAGE_JSON,
                data: data.into(),
            });
        }
    }
    Err(CliDiagnostic::MigrateError(MigrationDiagnostic { reason: "The default ESLint configuration file `.eslintrc.*` was not found in the working directory.".to_string()}))
}

#[derive(Debug)]
pub(crate) struct Config {
    /// Path of the ESlint config file
    pub(crate) path: &'static str,
    /// Resolved ESlint config
    pub(crate) data: eslint_eslint::AnyConfigData,
}

pub(crate) fn read_ignore_file(fs: &DynRef<'_, dyn FileSystem>) -> Option<IgnorePatterns> {
    let path = Path::new(IGNORE_FILE);
    if fs.path_exists(path) {
        let Ok(mut file) = fs.open_with_options(path, OpenOptions::default().read(true)) else {
            return None;
        };
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);
        let patterns = content
            .lines()
            .filter(|line| !line.is_empty() && !line.starts_with('#'))
            .map(|line| line.trim())
            // Biome doesn't support negated globs
            .filter(|line| !line.starts_with('!'))
            .map(String::from)
            .collect::<IndexSet<_>>();
        Some(IgnorePatterns(patterns))
    } else {
        None
    }
}

#[derive(Debug)]
pub(crate) struct IgnorePatterns(pub(crate) IndexSet<String>);

/// Load an ESlint Flat config
/// See https://eslint.org/docs/latest/use/configure/configuration-files-new
fn load_flat_config_data(
    path: &Path,
    console: &mut dyn Console,
) -> Result<eslint_eslint::FlatConfigData, CliDiagnostic> {
    let NodeResolveResult { content, .. } = load_config_with_node(&path.to_string_lossy())?;
    let (deserialized, diagnostics) = deserialize_from_json_str::<eslint_eslint::FlatConfigData>(
        &content,
        JsonParserOptions::default(),
        "",
    )
    .consume();
    let path_str = path.to_string_lossy();
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
    fs: &DynRef<'_, dyn FileSystem>,
    path: &Path,
    console: &mut dyn Console,
) -> Result<eslint_eslint::LegacyConfigData, CliDiagnostic> {
    let (deserialized, diagnostics) = match path.extension().and_then(|file_ext| file_ext.to_str()) {
        Some("json") => {
            let mut file = fs.open_with_options(path, OpenOptions::default().read(true))?;
            let mut content = String::new();
            file.read_to_string(&mut content)?;
            if path.file_name().is_some_and(|name| name == PACKAGE_JSON) {
                let (deserialized, _) = deserialize_from_json_str::<eslint_eslint::EslintPackageJson>(
                    &content,
                    JsonParserOptions::default()
                        .with_allow_trailing_commas()
                        .with_allow_comments(),
                    "",
                ).consume();
                (deserialized.and_then(|packagejson| packagejson.eslint_config), vec![])
            } else {
                deserialize_from_json_str::<eslint_eslint::LegacyConfigData>(
                    &content,
                    JsonParserOptions::default()
                        .with_allow_trailing_commas()
                        .with_allow_comments(),
                    "",
                ).consume()
            }
        },
        Some("js" | "cjs") => {
            let NodeResolveResult { content, ..} = load_config_with_node(&path.to_string_lossy())?;
            deserialize_from_json_str::<eslint_eslint::LegacyConfigData>(
                &content,
                JsonParserOptions::default(),
                "",
            ).consume()
        },
        Some(ext) => return Err(CliDiagnostic::MigrateError(MigrationDiagnostic{ reason: format!("ESLint configuration ending with the extension `{ext}` are not supported.") })),
        None => return Err(CliDiagnostic::MigrateError(MigrationDiagnostic{ reason: "The ESLint configuration format cannot be determined because the file has no extension.".to_string() })),
    };
    let path_str = path.to_string_lossy();
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

#[derive(Debug)]
struct NodeResolveResult {
    /// Resolved path of the file
    resolved_path: String,
    /// File content
    content: String,
}

/// Imports `specifier` using Node's `import()` or node's `require()` and
/// returns the JSONified content of its default export.
fn load_config_with_node(specifier: &str) -> Result<NodeResolveResult, CliDiagnostic> {
    let content_output = Command::new("node")
        .arg("--eval")
        .arg(format!(
            "import('{specifier}').then((c) => console.log(JSON.stringify(c.default)))"
        ))
        .output();
    match content_output {
        Err(_) => {
            Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: "The `node` program doesn't exist or cannot be invoked by Biome.\n`node` is invoked to resolve ESLint configurations written in JavaScript.\nThis includes shared configurations and plugin configurations imported with ESLint's `extends`.".to_string()
            }))
        },
        Ok(output) => {
            let path_output = Command::new("node")
                .arg("--print")
                .arg(format!(
                    "require.resolve('{specifier}')"
                ))
                .output();
            let resolved_path = path_output.ok().map_or(String::new(), |path_output| String::from_utf8_lossy(&path_output.stdout).trim().to_string());
            if !output.stderr.is_empty() {
                // Try with `require` before giving up.
                let output2 = Command::new("node")
                    .arg("--print")
                    .arg(format!(
                        "JSON.stringify(require('{specifier}'))"
                    ))
                    .output();
                if let Ok(output2) = output2 {
                    if output2.stderr.is_empty() {
                        return Ok(NodeResolveResult {
                            content: String::from_utf8_lossy(&output2.stdout).to_string(),
                            resolved_path,
                        });
                    }
                }
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                    reason: format!("`node` was invoked to resolve an ESLint configuration. This invocation failed with the following error:\n{stderr}")
                }));
            }
            Ok(NodeResolveResult {
                content: String::from_utf8_lossy(&output.stdout).to_string(),
                resolved_path,
            })
        }
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
        let Ok(NodeResolveResult {
            content,
            resolved_path,
        }) = load_config_with_node(&module_name)
        else {
            return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: format!(
                    "The module '{rest}' cannot be loaded. Make sure that the module exists."
                ),
            }));
        };
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
        let Ok(NodeResolveResult {
            content,
            resolved_path,
        }) = load_config_with_node(&module_name)
        else {
            return Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: format!(
                    "The module '{module_name}' cannot be loaded. Make sure that the module exists."
                ),
            }));
        };
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
        debug_assert!(matches!(artifact, "eslint-plugin-" | "eslint-config-"));
        if name.starts_with('@') {
            // handle scoped module
            if let Some((scope, scoped)) = name.split_once('/') {
                if scoped.starts_with(artifact) {
                    Cow::Borrowed(name)
                } else {
                    Cow::Owned(format!("{scope}/{artifact}{scoped}"))
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
