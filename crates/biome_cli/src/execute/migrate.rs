use crate::commands::MigrateSubCommand;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::diagnostics::{ContentDiffAdvice, MigrateDiffDiagnostic};
use crate::{CliDiagnostic, CliSession};
use biome_analyze::AnalysisFilter;
use biome_configuration::Configuration;
use biome_console::{Console, ConsoleExt, markup};
use biome_deserialize::Merge;
use biome_deserialize::json::deserialize_from_json_ast;
use biome_diagnostics::{PrintDiagnostic, category};
use biome_fs::{BiomePath, OpenOptions};
use biome_json_parser::{JsonParserOptions, parse_json_with_cache};
use biome_json_syntax::{JsonFileSource, JsonRoot};
use biome_migrate::{ControlFlow, migrate_configuration};
use biome_rowan::{AstNode, NodeCache};
use biome_service::Workspace;
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    ChangeFileParams, FileContent, FixAction, FormatFileParams, OpenFileParams,
};
use camino::Utf8PathBuf;
use std::borrow::Cow;

mod eslint;
mod eslint_any_rule_to_biome;
mod eslint_eslint;
mod eslint_jsxa11y;
mod eslint_to_biome;
mod eslint_typescript;
mod eslint_unicorn;
mod ignorefile;
mod node;
mod prettier;

pub(crate) struct MigratePayload<'a> {
    pub(crate) session: CliSession<'a>,
    pub(crate) project_key: ProjectKey,
    pub(crate) write: bool,
    pub(crate) configuration_file_path: Utf8PathBuf,
    pub(crate) sub_command: Option<MigrateSubCommand>,
    pub(crate) nested_configuration_files: Vec<BiomePath>,
}

pub(crate) fn run(migrate_payload: MigratePayload) -> Result<(), CliDiagnostic> {
    let MigratePayload {
        session,
        project_key,
        write,
        configuration_file_path,
        sub_command,
        nested_configuration_files,
    } = migrate_payload;
    let workspace = &*session.app.workspace;
    let console = session.app.console;

    let mut configuration_list = vec![configuration_file_path.into()];
    configuration_list.extend(nested_configuration_files);
    let mut needs_migration = false;
    let mut migrated = false;
    for configuration_file_path in configuration_list {
        let migrate_file_payload = MigrateFile {
            workspace,
            console,
            configuration_file_path,
            project_key,
            sub_command: sub_command.as_ref(),
            write,
        };

        let result = migrate_file(migrate_file_payload)?;
        if let MigrationFileResult::NeedsMigration = result {
            needs_migration = true;
        } else if let MigrationFileResult::Migrated = result {
            migrated = true;
        }
    }

    if needs_migration {
        console.log(markup! {
            <Info>"Run the command with the option "<Emphasis>"--write"</Emphasis>" to apply the changes."</Info>
        })
    } else if migrated {
        console.log(markup! {
            <Info>"Your configuration file(s) have been successfully migrated."</Info>
        })
    } else {
        console.log(markup! {
            <Info>"No changes to apply to the Biome configuration file."</Info>
        });
    }

    Ok(())
}

struct MigrateFile<'a> {
    pub(crate) workspace: &'a dyn Workspace,
    pub(crate) console: &'a mut dyn Console,
    pub(crate) project_key: ProjectKey,
    pub(crate) write: bool,
    pub(crate) configuration_file_path: BiomePath,
    pub(crate) sub_command: Option<&'a MigrateSubCommand>,
}

#[derive(Debug, Eq, PartialEq)]
enum MigrationFileResult {
    Migrated,
    NeedsMigration,
    NoMigrationNeeded,
    HasErrors,
}

fn migrate_file(payload: MigrateFile) -> Result<MigrationFileResult, CliDiagnostic> {
    let MigrateFile {
        workspace,
        console,
        project_key,
        write,
        configuration_file_path,
        sub_command,
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
        document_file_source: Some(JsonFileSource::json().into()),
        persist_node_cache: false,
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
            let prettier_biome_config = prettier_config.try_into().map_err(|err| {
                CliDiagnostic::MigrateError(MigrationDiagnostic {
                    reason: format!("{:#}", err),
                })
            })?;
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
                })?;
                let printed = workspace.format_file(FormatFileParams {
                    project_key,
                    path: biome_path,
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
            let (biome_eslint_config, results) =
                eslint_config.into_biome_config(&eslint_to_biome::MigrationOptions {
                    include_inspired: *include_inspired,
                    include_nursery: *include_nursery,
                });
            let old_biome_config = biome_config.clone();
            biome_config.merge_with(biome_eslint_config);
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
                console.log(markup! {
                    <Info>"No changes to apply to the Biome configuration file."</Info>
                });
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
                })?;
                let printed = workspace.format_file(FormatFileParams {
                    project_key,
                    path: biome_path,
                })?;
                if write {
                    biome_config_file.set_content(printed.as_code().as_bytes())?;
                    console.log(markup!{
                        <Info><Emphasis>{eslint_path}</Emphasis>" has been successfully migrated."</Info>
                    });
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
            if results.has_inspired_rules {
                console.log(markup! {
                    <Info>"Run the command with the option "<Emphasis>"--include-inspired"</Emphasis>" to also migrate inspired rules."</Info>
                });
            }
            Ok(result)
        }
        None => {
            let mut tree = parsed.tree();
            let mut actions = Vec::new();
            loop {
                let (action, _) = migrate_configuration(
                    &tree,
                    AnalysisFilter::default(),
                    configuration_file_path.as_path(),
                    |signal| {
                        if let Some(action) = signal.actions().next() {
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
                    })?;
                    let printed = workspace.format_file(FormatFileParams {
                        project_key,
                        path: biome_path,
                    })?;
                    configuration_file.set_content(printed.as_code().as_bytes())?;
                    // console.log(markup!{
                    //     <Info>"The configuration "<Emphasis>{{configuration_file_path.to_string()}}</Emphasis>" has been successfully migrated."</Info>
                    // })
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
