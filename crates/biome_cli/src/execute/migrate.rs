use crate::commands::MigrateSubCommand;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::migrate::eslint_to_biome::MigrationOptions;
use crate::runner::diagnostics::{ContentDiffAdvice, MigrateDiffDiagnostic};
use crate::{CliDiagnostic, CliSession};
use biome_analyze::{ActionFilter, AnalysisFilter};
use biome_configuration::bool::Bool;
use biome_configuration::{Configuration, ConfigurationPathHint};
use biome_console::fmt::{Display, Formatter};
use biome_console::{Console, ConsoleExt, markup};
use biome_deserialize::Merge;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::{
    Category, Diagnostic, LogCategory, PrintDiagnostic, Severity, Visit, category,
};
use biome_fs::{BiomePath, ConfigName, OpenOptions};
use biome_json_parser::{JsonParserOptions, parse_json_with_cache};
use biome_json_syntax::JsonRoot;
use biome_languages::JsonFileSource;
use biome_migrate::{ControlFlow, migrate_configuration};
use biome_resolver::FsWithResolverProxy;
use biome_rowan::{AstNode, NodeCache};
use biome_service::Workspace;
use biome_service::configuration::{LoadedConfiguration, read_config};
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    ChangeFileParams, FileContent, FixAction, FormatFileParams, OpenFileParams,
};
use camino::Utf8PathBuf;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::fmt::Debug;

mod eslint;
mod eslint_any_rule_to_biome;
mod eslint_eslint;
mod eslint_jest;
mod eslint_jsxa11y;
mod eslint_react;
mod eslint_svelte;
mod eslint_to_biome;
mod eslint_typescript;
mod eslint_unicorn;
mod eslint_vue;
mod ignorefile;
mod node;
mod prettier;
mod stylelint;
mod stylelint_any_rule_to_biome;
mod stylelint_stylelint;
mod stylelint_to_biome;
mod stylelint_unsupported_rules;

pub(crate) struct MigratePayload<'a> {
    pub(crate) session: CliSession<'a>,
    pub(crate) project_key: ProjectKey,
    pub(crate) write: bool,
    pub(crate) configuration_file_path: Utf8PathBuf,
    pub(crate) sub_command: Option<MigrateSubCommand>,
    pub(crate) nested_configuration_files: Vec<BiomePath>,
    /// The root configuration merged with the configurations it extends.
    pub(crate) resolved_configuration: Option<Configuration>,
}

pub(crate) fn run(migrate_payload: MigratePayload) -> Result<(), CliDiagnostic> {
    let MigratePayload {
        session,
        project_key,
        write,
        configuration_file_path,
        sub_command,
        nested_configuration_files,
        resolved_configuration,
    } = migrate_payload;
    let workspace = &*session.app.workspace;
    let console = session.app.console;

    let root_configuration_path: BiomePath = configuration_file_path.into();
    let mut configuration_list = BTreeSet::default();
    configuration_list.insert(root_configuration_path.clone());
    configuration_list.extend(nested_configuration_files);
    let mut diagnostic_result = MigrationResultDiagnostic::default();

    for configuration_file_path in configuration_list {
        let file_resolved_configuration = if configuration_file_path == root_configuration_path {
            resolved_configuration.clone()
        } else if matches!(sub_command, Some(MigrateSubCommand::Prettier)) {
            resolve_nested_configuration(
                workspace.fs(),
                &configuration_file_path,
                resolved_configuration.as_ref(),
            )
        } else {
            None
        };
        let migrate_file_payload = MigrateFile {
            workspace,
            console,
            configuration_file_path: &configuration_file_path,
            project_key,
            sub_command: sub_command.as_ref(),
            write,
            resolved_configuration: file_resolved_configuration.as_ref(),
        };

        let result = migrate_file(migrate_file_payload)?;
        diagnostic_result
            .outcomes
            .push((configuration_file_path, result));
    }

    console.log(markup! {{PrintDiagnostic::simple(&diagnostic_result)}});

    Ok(())
}

/// Returns the configuration of the nested file at `path` merged with the
/// configurations it extends, including `root_configuration` for `"//"`.
/// Returns `None` when the file or a configuration it extends can't be loaded.
fn resolve_nested_configuration(
    fs: &dyn FsWithResolverProxy,
    path: &BiomePath,
    root_configuration: Option<&Configuration>,
) -> Option<Configuration> {
    // Pass the file path like the workspace does. `extends` entries resolve
    // from it, so passing the directory can pick different files.
    let payload = read_config(
        fs,
        ConfigurationPathHint::FromWorkspace(path.to_path_buf()),
        false,
    )
    .ok()??;
    let loaded_configuration = LoadedConfiguration::try_from_payload(Some(payload), fs).ok()?;
    if loaded_configuration.has_errors() {
        return None;
    }
    let nested_configuration = loaded_configuration.resolved_configuration();
    if nested_configuration.extends_root()
        && let Some(root_configuration) = root_configuration
    {
        let mut configuration = root_configuration.clone();
        configuration.merge_with(nested_configuration);
        // Otherwise the merge keeps `root: true` from the root configuration.
        configuration.root = Some(Bool(false));
        Some(configuration)
    } else {
        Some(nested_configuration)
    }
}

struct MigrateFile<'a> {
    pub(crate) workspace: &'a dyn Workspace,
    pub(crate) console: &'a mut dyn Console,
    pub(crate) project_key: ProjectKey,
    pub(crate) write: bool,
    pub(crate) configuration_file_path: &'a BiomePath,
    pub(crate) sub_command: Option<&'a MigrateSubCommand>,
    /// This file's configuration merged with the configurations it extends.
    /// Nested files have it only in the Prettier migration.
    pub(crate) resolved_configuration: Option<&'a Configuration>,
}

#[derive(Debug, Default)]
struct MigrationResultDiagnostic {
    outcomes: Vec<(BiomePath, MigrationFileResult)>,
}

impl Diagnostic for MigrationResultDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("configuration"))
    }

    fn severity(&self) -> Severity {
        Severity::Information
    }

    fn description(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str("Migration results:")
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        fmt.write_markup(markup! { "Migration results:" })
    }

    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        let list: Vec<_> = self
            .outcomes
            .iter()
            .map(|(path, result)| {
                markup! {
                    {path.as_str()}": "{result}
                }
                .to_owned()
            })
            .collect();

        let list: Vec<_> = list.iter().map(|item| item as &dyn Display).collect();
        visitor.record_list(list.as_slice())?;
        if self
            .outcomes
            .iter()
            .any(|(_, result)| result == &MigrationFileResult::NeedsMigration)
        {
            visitor.record_log(
                LogCategory::Info,
                &markup! { "Use "<Emphasis>"--write"</Emphasis>" to apply the changes." },
            )?;
            visitor.record_command("biome migrate --write")?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
enum MigrationFileResult {
    Migrated,
    NeedsMigration,
    NoMigrationNeeded,
    HasErrors,
}

impl biome_console::fmt::Display for MigrationFileResult {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            Self::Migrated => fmt.write_markup(markup! { "configuration successfully migrated." }),
            Self::NeedsMigration => fmt.write_markup(markup! { "configuration needs migration." }),
            Self::NoMigrationNeeded => fmt.write_markup(markup! { "no migration needed." }),
            Self::HasErrors => fmt.write_markup(markup! { "migration failed due to errors." }),
        }
    }
}

fn migrate_file(payload: MigrateFile) -> Result<MigrationFileResult, CliDiagnostic> {
    let MigrateFile {
        workspace,
        console,
        project_key,
        write,
        configuration_file_path,
        sub_command,
        resolved_configuration,
    } = payload;
    let mut cache = NodeCache::default();
    let fs = workspace.fs();

    let open_options = if write {
        OpenOptions::default().read(true).write(true)
    } else {
        OpenOptions::default().read(true)
    };
    let mut biome_config_file =
        fs.open_with_options(configuration_file_path.as_path(), open_options)?;
    let mut biome_config_content = String::new();
    biome_config_file.read_to_string(&mut biome_config_content)?;

    let biome_path = BiomePath::new(configuration_file_path.as_path());
    let parse_options = match configuration_file_path.extension() {
        Some("jsonc") => JsonParserOptions::default()
            .with_allow_comments()
            .with_allow_trailing_commas(),
        _ => JsonParserOptions::default(),
    };
    workspace.open_file(OpenFileParams {
        project_key,
        path: biome_path.clone(),
        content: FileContent::from_client(&biome_config_content),
        document_file_source: Some(
            JsonFileSource::try_from(biome_path.as_path())
                .unwrap_or_default()
                .into(),
        ),
        persist_node_cache: false,
        inline_config: None,
        editor_features: None,
    })?;
    let parsed = parse_json_with_cache(&biome_config_content, &mut cache, parse_options);

    match sub_command {
        Some(MigrateSubCommand::Prettier) => {
            let prettier::Config {
                path: prettier_path,
                data: prettier_config,
            } = prettier::read_config_file(fs, console)?;
            let biome_config =
                deserialize_from_json_ast::<Configuration>(&parsed.tree(), "").into_deserialized();
            let Some(mut biome_config) = biome_config else {
                return Ok(MigrationFileResult::HasErrors);
            };
            let old_biome_config = biome_config.clone();
            let prettier_biome_config: Configuration =
                prettier_config.try_into().map_err(|err| {
                    CliDiagnostic::MigrateError(MigrationDiagnostic {
                        reason: format!("{err:#}",),
                    })
                })?;
            let mut format_configuration = resolved_configuration
                .cloned()
                .unwrap_or_else(|| biome_config.clone());
            format_configuration.merge_with(prettier_biome_config.clone());
            biome_config.merge_with(prettier_biome_config);
            if let Ok(ignore_patterns) = ignorefile::read_ignore_file(fs, prettier::IGNORE_FILE) {
                if !ignore_patterns.patterns.is_empty() {
                    biome_config
                        .formatter
                        .get_or_insert(Default::default())
                        .includes
                        .get_or_insert(Default::default())
                        .extend(ignore_patterns.patterns);
                }
                if write && biome_config != old_biome_config {
                    console.log(markup!{
                        <Info><Emphasis>{prettier::IGNORE_FILE}</Emphasis>" has been successfully migrated."</Info>
                    });
                }
            }
            if biome_config == old_biome_config {
                console.log(markup! {
                    <Info>"No changes to apply to the Biome configuration file."</Info>
                });
                Ok(MigrationFileResult::NoMigrationNeeded)
            } else {
                let new_content = serde_json::to_string(&biome_config).map_err(|err| {
                    CliDiagnostic::MigrateError(MigrationDiagnostic {
                        reason: err.to_string(),
                    })
                })?;
                workspace.change_file(ChangeFileParams {
                    project_key,
                    path: biome_path.clone(),
                    content: new_content,
                    version: 1,
                    inline_config: None,
                    editor_features: None,
                })?;
                // The project settings predate the migration. The inline
                // configuration replaces whole sections, so it needs the options
                // inherited through `extends`.
                let printed = workspace.format_file(FormatFileParams {
                    project_key,
                    path: biome_path,
                    inline_config: Some(format_configuration),
                })?;
                if write {
                    biome_config_file.set_content(printed.as_code().as_bytes())?;
                    console.log(markup!{
                        <Info><Emphasis>{prettier_path}</Emphasis>" has been successfully migrated."</Info>
                    });
                    Ok(MigrationFileResult::Migrated)
                } else {
                    let file_name = configuration_file_path.to_string();
                    let diagnostic = MigrateDiffDiagnostic {
                        file_name,
                        diff: ContentDiffAdvice {
                            old: biome_config_content,
                            new: printed.as_code().to_string(),
                        },
                    };
                    console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
                    Ok(MigrationFileResult::NeedsMigration)
                }
            }
        }
        Some(MigrateSubCommand::Eslint {
            include_inspired,
            include_nursery,
        }) => {
            let eslint::Config {
                path: eslint_path,
                data: eslint_config,
            } = eslint::read_eslint_config(fs, console)?;
            let biome_config =
                deserialize_from_json_ast::<Configuration>(&parsed.tree(), "").into_deserialized();
            let Some(mut biome_config) = biome_config else {
                return Ok(MigrationFileResult::HasErrors);
            };
            let old_biome_config = biome_config.clone();
            let (updated_biome_config, mut results) =
                eslint_to_biome::merge_biome_config_with_eslint(
                    biome_config,
                    eslint_config,
                    &MigrationOptions {
                        include_inspired: *include_inspired,
                        include_nursery: *include_nursery,
                    },
                );
            biome_config = updated_biome_config;
            if let Ok(ignore_patterns) = ignorefile::read_ignore_file(fs, eslint::IGNORE_FILE) {
                if !ignore_patterns.patterns.is_empty() {
                    biome_config
                        .linter
                        .get_or_insert(Default::default())
                        .includes
                        .get_or_insert(Default::default())
                        .extend(ignore_patterns.patterns);
                }
                if write && biome_config != old_biome_config {
                    console.log(markup!{
                        <Info><Emphasis>{eslint::IGNORE_FILE}</Emphasis>" has been successfully migrated."</Info>
                    });
                }
            }
            let result = if biome_config == old_biome_config {
                MigrationFileResult::NoMigrationNeeded
            } else {
                let new_content = serde_json::to_string(&biome_config).map_err(|err| {
                    CliDiagnostic::MigrateError(MigrationDiagnostic {
                        reason: err.to_string(),
                    })
                })?;
                workspace.change_file(ChangeFileParams {
                    project_key,
                    path: biome_path.clone(),
                    content: new_content,
                    version: 1,
                    inline_config: None,
                    editor_features: None,
                })?;
                let printed = workspace.format_file(FormatFileParams {
                    project_key,
                    path: biome_path,
                    inline_config: None,
                })?;
                if write {
                    biome_config_file.set_content(printed.as_code().as_bytes())?;
                    MigrationFileResult::Migrated
                } else {
                    let file_name = configuration_file_path.to_string();
                    let diagnostic = MigrateDiffDiagnostic {
                        file_name,
                        diff: ContentDiffAdvice {
                            old: biome_config_content,
                            new: printed.as_code().to_string(),
                        },
                    };
                    console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
                    MigrationFileResult::NeedsMigration
                }
            };
            if let Some(working_directory) = fs.working_directory() {
                let path = working_directory.join(eslint_path);
                let path = path.canonicalize_utf8().unwrap_or(path);
                results.eslint_path = Some(path.to_string().into());
            }
            results.write = write;
            console.log(markup! {{PrintDiagnostic::simple(&results)}});
            Ok(result)
        }
        Some(MigrateSubCommand::Stylelint {
            include_inspired,
            include_nursery,
        }) => {
            let stylelint::Config {
                path: stylelint_path,
                data: stylelint_config,
            } = stylelint::read_stylelint_config(fs, console)?;
            let biome_config =
                deserialize_from_json_ast::<Configuration>(&parsed.tree(), "").into_deserialized();
            let Some(mut biome_config) = biome_config else {
                return Ok(MigrationFileResult::HasErrors);
            };
            let old_biome_config = biome_config.clone();
            let (updated_biome_config, mut results) =
                stylelint_to_biome::merge_biome_config_with_stylelint(
                    biome_config,
                    stylelint_config,
                    &MigrationOptions {
                        include_inspired: *include_inspired,
                        include_nursery: *include_nursery,
                    },
                );
            biome_config = updated_biome_config;
            if let Ok(ignore_patterns) = ignorefile::read_ignore_file(fs, stylelint::IGNORE_FILE) {
                if !ignore_patterns.patterns.is_empty() {
                    biome_config
                        .linter
                        .get_or_insert(Default::default())
                        .includes
                        .get_or_insert(Default::default())
                        .extend(ignore_patterns.patterns);
                }
                if write && biome_config != old_biome_config {
                    console.log(markup!{
                        <Info><Emphasis>{stylelint::IGNORE_FILE}</Emphasis>" has been successfully migrated."</Info>
                    });
                }
            }
            let result = if biome_config == old_biome_config {
                MigrationFileResult::NoMigrationNeeded
            } else {
                let new_content = serde_json::to_string(&biome_config).map_err(|err| {
                    CliDiagnostic::MigrateError(MigrationDiagnostic {
                        reason: err.to_string(),
                    })
                })?;
                workspace.change_file(ChangeFileParams {
                    project_key,
                    path: biome_path.clone(),
                    content: new_content,
                    version: 1,
                    inline_config: None,
                    editor_features: None,
                })?;
                let printed = workspace.format_file(FormatFileParams {
                    project_key,
                    path: biome_path,
                    inline_config: None,
                })?;
                if write {
                    biome_config_file.set_content(printed.as_code().as_bytes())?;
                    MigrationFileResult::Migrated
                } else {
                    let file_name = configuration_file_path.to_string();
                    let diagnostic = MigrateDiffDiagnostic {
                        file_name,
                        diff: ContentDiffAdvice {
                            old: biome_config_content,
                            new: printed.as_code().to_string(),
                        },
                    };
                    console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
                    MigrationFileResult::NeedsMigration
                }
            };
            if let Some(working_directory) = fs.working_directory() {
                let path = working_directory.join(stylelint_path);
                let path = path.canonicalize_utf8().unwrap_or(path);
                results.stylelint_path = Some(path.to_string().into());
            }
            results.write = write;
            console.log(markup! {{PrintDiagnostic::simple(&results)}});
            Ok(result)
        }
        None => {
            let mut tree = parsed.tree();
            let mut actions = Vec::new();
            let is_root = workspace.fs().working_directory().is_some_and(|wd| {
                configuration_file_path.strip_prefix(wd).is_ok_and(|path| {
                    path.starts_with(ConfigName::biome_json())
                        || path.starts_with(ConfigName::biome_jsonc())
                })
            });
            loop {
                let (action, _) = migrate_configuration(
                    &tree,
                    AnalysisFilter::default(),
                    configuration_file_path.as_path(),
                    is_root,
                    |signal| {
                        if let Some(action) = signal.actions(ActionFilter::rule_fix()).next() {
                            return ControlFlow::Break(action);
                        }
                        ControlFlow::Continue(())
                    },
                );
                match action {
                    Some(action) => {
                        if let (root, Some((range, _))) =
                            action.mutation.commit_with_text_range_and_edit(true)
                        {
                            tree = match JsonRoot::cast(root) {
                                Some(tree) => tree,
                                None => {
                                    return Err(CliDiagnostic::check_error(category!("migrate")));
                                }
                            };
                            actions.push(FixAction {
                                rule_name: action.rule_name.map(|(group, rule)| {
                                    (Cow::Borrowed(group), Cow::Borrowed(rule))
                                }),
                                range,
                            });
                        }
                    }
                    None => {
                        break;
                    }
                }
            }

            let new_configuration_content = tree.to_string();
            if biome_config_content != new_configuration_content {
                if write {
                    let mut configuration_file = biome_config_file;
                    workspace.change_file(ChangeFileParams {
                        project_key,
                        path: biome_path.clone(),
                        content: new_configuration_content,
                        version: 1,
                        inline_config: None,
                        editor_features: None,
                    })?;
                    let printed = workspace.format_file(FormatFileParams {
                        project_key,
                        path: biome_path,
                        inline_config: None,
                    })?;
                    configuration_file.set_content(printed.as_code().as_bytes())?;
                    Ok(MigrationFileResult::Migrated)
                } else {
                    let file_name = configuration_file_path.to_string();
                    let diagnostic = MigrateDiffDiagnostic {
                        file_name,
                        diff: ContentDiffAdvice {
                            old: biome_config_content,
                            new: new_configuration_content,
                        },
                    };
                    console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
                    Ok(MigrationFileResult::NeedsMigration)
                }
            } else {
                console.log(markup! {
                    <Info>
                    "Your configuration file is up to date."
                    </Info>
                });
                Ok(MigrationFileResult::NoMigrationNeeded)
            }
        }
    }
}
