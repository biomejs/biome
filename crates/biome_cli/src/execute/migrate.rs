mod prettier;

use crate::diagnostics::MigrationDiagnostic;
use crate::execute::diagnostics::{ContentDiffAdvice, MigrateDiffDiagnostic};
use crate::execute::migrate::prettier::read_prettier_files;
use crate::{CliDiagnostic, CliSession};
use biome_console::{markup, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Merge;
use biome_diagnostics::Diagnostic;
use biome_diagnostics::{category, PrintDiagnostic};
use biome_fs::{FileSystemExt, OpenOptions, RomePath};
use biome_json_parser::{parse_json_with_cache, JsonParserOptions};
use biome_json_syntax::JsonRoot;
use biome_migrate::{migrate_configuration, ControlFlow};
use biome_rowan::{AstNode, NodeCache};
use biome_service::workspace::{
    ChangeFileParams, FixAction, FormatFileParams, Language, OpenFileParams,
};
use biome_service::{PartialConfiguration, VERSION};
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

pub(crate) struct MigratePayload<'a> {
    pub(crate) session: CliSession<'a>,
    pub(crate) write: bool,
    pub(crate) configuration_file_path: PathBuf,
    pub(crate) configuration_directory_path: PathBuf,
    pub(crate) verbose: bool,
    #[allow(unused)]
    pub(crate) prettier: bool,
}

pub(crate) fn run(migrate_payload: MigratePayload) -> Result<(), CliDiagnostic> {
    let MigratePayload {
        session,
        write,
        configuration_file_path,
        configuration_directory_path,
        verbose,
        prettier,
    } = migrate_payload;
    let mut cache = NodeCache::default();
    let fs = &session.app.fs;
    let console = session.app.console;
    let workspace = session.app.workspace;

    let has_deprecated_configuration =
        configuration_file_path.file_name() == Some(OsStr::new("rome.json"));

    let open_options = if write {
        OpenOptions::default().read(true).write(true)
    } else {
        OpenOptions::default().read(true)
    };

    let mut configuration_file =
        fs.open_with_options(configuration_file_path.as_path(), open_options)?;
    let mut configuration_content = String::new();
    configuration_file.read_to_string(&mut configuration_content)?;

    let rome_path = RomePath::new(configuration_file_path.as_path());
    workspace.open_file(OpenFileParams {
        path: rome_path.clone(),
        content: configuration_content.to_string(),
        version: 0,
        language_hint: Language::Json,
    })?;

    let parsed = parse_json_with_cache(
        &configuration_content,
        &mut cache,
        JsonParserOptions::default(),
    );

    let mut errors = 0;
    let mut tree = parsed.tree();
    let mut actions = Vec::new();
    loop {
        let (action, _) = migrate_configuration(
            &tree,
            configuration_file_path.as_path(),
            VERSION.to_string(),
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
                if let Some((range, _)) = action.mutation.as_text_edits() {
                    tree = match JsonRoot::cast(action.mutation.commit()) {
                        Some(tree) => tree,
                        None => return Err(CliDiagnostic::check_error(category!("migrate"))),
                    };
                    actions.push(FixAction {
                        rule_name: action
                            .rule_name
                            .map(|(group, rule)| (Cow::Borrowed(group), Cow::Borrowed(rule))),
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

    if prettier {
        let prettier_configuration = read_prettier_files(fs, console)?;

        if prettier_configuration.has_configuration() {
            let configuration = deserialize_from_json_str::<PartialConfiguration>(
                configuration_content.as_str(),
                JsonParserOptions::default(),
                "",
            )
            .into_deserialized();
            if let Some(mut configuration) = configuration {
                configuration.merge_with(prettier_configuration.as_biome_configuration());

                let new_content = serde_json::to_string(&configuration).map_err(|err| {
                    CliDiagnostic::MigrateError(MigrationDiagnostic {
                        reason: err.to_string(),
                    })
                })?;

                workspace.change_file(ChangeFileParams {
                    path: rome_path.clone(),
                    content: new_content,
                    version: 1,
                })?;

                let printed = workspace.format_file(FormatFileParams {
                    path: rome_path.clone(),
                })?;

                if write {
                    configuration_file.set_content(printed.as_code().as_bytes())?;
                    console.log(markup!{
                        <Info>"The configuration "<Emphasis>{{configuration_file_path.display().to_string()}}</Emphasis>" has been successfully migrated."</Info>
                    });
                    if prettier_configuration.has_ignore_file() {
                        console.log(markup!{
                            <Warn>"Please make sure that the globs of the "<Emphasis>".prettierignore"</Emphasis>" file still work in Biome. Prettier's globs use git globs, while Biome's globs use uni-style globs. They both seem similar, but their semantics differ."</Warn>
                        })
                    }
                } else {
                    let file_name = configuration_file_path.display().to_string();
                    let diagnostic = MigrateDiffDiagnostic {
                        file_name,
                        diff: ContentDiffAdvice {
                            old: configuration_content,
                            new: printed.as_code().to_string(),
                        },
                    };
                    console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});

                    console.log(markup! {
                        "Run the command with the option "<Emphasis>"--write"</Emphasis>" to apply the changes."
                    })
                }
            }
        }
    } else if configuration_content != new_configuration_content || has_deprecated_configuration {
        if write {
            let mut configuration_file = if has_deprecated_configuration {
                let biome_file_path = configuration_directory_path.join(fs.config_name());
                fs.create_new(biome_file_path.as_path())?
            } else {
                configuration_file
            };
            configuration_file.set_content(tree.to_string().as_bytes())?;
            console.log(markup!{
                    <Info>"The configuration "<Emphasis>{{configuration_file_path.display().to_string()}}</Emphasis>" has been successfully migrated."</Info>
                })
        } else {
            let file_name = configuration_file_path.display().to_string();
            let diagnostic = if has_deprecated_configuration {
                MigrateDiffDiagnostic {
                    file_name,
                    diff: ContentDiffAdvice {
                        old: "rome.json".to_string(),
                        new: "biome.json".to_string(),
                    },
                }
            } else {
                MigrateDiffDiagnostic {
                    file_name,
                    diff: ContentDiffAdvice {
                        old: configuration_content,
                        new: new_configuration_content,
                    },
                }
            };
            if diagnostic.tags().is_verbose() {
                if verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&diagnostic)}})
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}})
            }
            console.log(markup! {
                    "Run the command with the option "<Emphasis>"--write"</Emphasis>" to apply the changes."
                })
        }
    } else {
        console.log(markup! {
            <Info>
            "Your configuration file is up to date."
            </Info>
        })
    }

    Ok(())
}
