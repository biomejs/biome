use crate::execute::diagnostics::{ContentDiffAdvice, MigrateDiffDiagnostic};
use crate::{CliDiagnostic, CliSession};
use biome_console::{markup, ConsoleExt};
use biome_diagnostics::{category, PrintDiagnostic};
use biome_migrate::{migrate_configuration, ControlFlow};
use rome_fs::{FileSystemExt, OpenOptions};
use rome_json_parser::JsonParserOptions;
use rome_json_syntax::JsonRoot;
use rome_rowan::AstNode;
use rome_service::workspace::FixAction;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::path::PathBuf;

pub(crate) fn run(
    session: CliSession,
    write: bool,
    configuration_file_path: PathBuf,
    configuration_directory_path: PathBuf,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    let fs = &*session.app.fs;
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
    let parsed = rome_json_parser::parse_json(&configuration_content, JsonParserOptions::default());
    let mut errors = 0;
    let mut tree = parsed.tree();
    let mut actions = Vec::new();
    loop {
        let (action, _) = migrate_configuration(
            &tree.value().unwrap(),
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
    let console = &mut *session.app.console;
    let new_configuration_content = tree.to_string();

    if configuration_content != new_configuration_content || has_deprecated_configuration {
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
            console.error(markup! {
                {if verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
            });
            console.log(markup! {
                "Run the command "<Emphasis>"biome migrate --write"</Emphasis>" to apply the changes."
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
