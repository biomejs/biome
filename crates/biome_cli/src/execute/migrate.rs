use crate::commands::MigrateSubCommand;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::diagnostics::{ContentDiffAdvice, MigrateDiffDiagnostic};
use crate::{CliDiagnostic, CliSession};
use biome_analyze::AnalysisFilter;
use biome_configuration::Configuration;
use biome_console::{markup, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_deserialize::Merge;
use biome_diagnostics::Diagnostic;
use biome_diagnostics::{category, PrintDiagnostic};
use biome_formatter::ParseFormatNumberError;
use biome_fs::{BiomePath, OpenOptions};
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_formatter::format_node;
use biome_json_parser::{parse_json_with_cache, JsonParserOptions};
use biome_json_syntax::{JsonFileSource, JsonRoot};
use biome_migrate::{migrate_configuration, ControlFlow};
use biome_rowan::{AstNode, NodeCache};
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
    pub(crate) verbose: bool,
    pub(crate) sub_command: Option<MigrateSubCommand>,
}

pub(crate) fn run(migrate_payload: MigratePayload) -> Result<(), CliDiagnostic> {
    let MigratePayload {
        session,
        project_key,
        write,
        configuration_file_path,
        verbose,
        sub_command,
    } = migrate_payload;
    let mut cache = NodeCache::default();
    let console = session.app.console;
    let workspace = session.app.workspace;
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
        content: FileContent::FromClient(biome_config_content.to_string()),
        version: 0,
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
                return Ok(());
            };
            let old_biome_config = biome_config.clone();
            let prettier_biome_config =
                prettier_config
                    .try_into()
                    .map_err(|err: ParseFormatNumberError| {
                        CliDiagnostic::MigrateError(MigrationDiagnostic {
                            reason: err.to_string(),
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
                    console.log(markup! {
                        <Info>"Run the command with the option "<Emphasis>"--write"</Emphasis>" to apply the changes."</Info>
                    })
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
                return Ok(());
            };
            let (biome_eslint_config, results) =
                eslint_config.into_biome_config(&eslint_to_biome::MigrationOptions {
                    include_inspired,
                    include_nursery,
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
            if biome_config == old_biome_config {
                console.log(markup! {
                    <Info>"No changes to apply to the Biome configuration file."</Info>
                });
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
                    console.log(markup! {
                        <Info>"Run the command with the option "<Emphasis>"--write"</Emphasis>" to apply the changes."</Info>
                    })
                }
            }
            if results.has_inspired_rules {
                console.log(markup! {
                    <Info>"Run the command with the option "<Emphasis>"--include-inspired"</Emphasis>" to also migrate inspired rules."</Info>
                })
            }
        }
        None => {
            let mut errors = 0;
            let mut tree = parsed.tree();
            let mut actions = Vec::new();
            loop {
                let (action, _) = migrate_configuration(
                    &tree,
                    AnalysisFilter::default(),
                    configuration_file_path.as_path(),
                    |signal| {
                        let current_diagnostic = signal.diagnostic();
                        if current_diagnostic.is_some() {
                            errors += 1;
                        }
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
                                    return Err(CliDiagnostic::check_error(category!("migrate")))
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
                    let format_options = JsonFormatOptions::default();
                    let formatted = format_node(format_options, tree.syntax())
                        .ok()
                        .map(|formatted| formatted.print())
                        .and_then(|printed| printed.ok());

                    if let Some(formatted) = formatted {
                        configuration_file.set_content(formatted.as_code().as_bytes())?;
                    } else {
                        configuration_file.set_content(new_configuration_content.as_bytes())?;
                    }
                    console.log(markup!{
                            <Info>"The configuration "<Emphasis>{{configuration_file_path.to_string()}}</Emphasis>" has been successfully migrated."</Info>
                        })
                } else {
                    let file_name = configuration_file_path.to_string();
                    let diagnostic = MigrateDiffDiagnostic {
                        file_name,
                        diff: ContentDiffAdvice {
                            old: biome_config_content,
                            new: new_configuration_content,
                        },
                    };
                    if diagnostic.tags().is_verbose() {
                        if verbose {
                            console.error(markup! {{PrintDiagnostic::verbose(&diagnostic)}})
                        }
                    } else {
                        console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}})
                    }
                    console.log(markup! {
                            <Info>"Run the command with the option "<Emphasis>"--write"</Emphasis>" to apply the changes."</Info>
                        })
                }
            } else {
                console.log(markup! {
                    <Info>
                    "Your configuration file is up to date."
                    </Info>
                })
            }
        }
    }
    Ok(())
}
