//! In here, there are the operations that run via standard input
//!
use crate::execute::Execution;
use crate::{CliDiagnostic, CliSession, TraversalMode};
use biome_analyze::RuleCategoriesBuilder;
use biome_console::{markup, ConsoleExt};
use biome_diagnostics::Diagnostic;
use biome_diagnostics::PrintDiagnostic;
use biome_fs::BiomePath;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::projects::ProjectKey;
use biome_service::workspace::{
    ChangeFileParams, DropPatternParams, FeaturesBuilder, FileContent, FixFileParams,
    FormatFileParams, OpenFileParams, SupportsFeatureParams,
};
use biome_service::WorkspaceError;
use std::borrow::Cow;

pub(crate) fn run<'a>(
    session: CliSession,
    project_key: ProjectKey,
    mode: &'a Execution,
    biome_path: BiomePath,
    content: &'a str,
    verbose: bool,
) -> Result<(), CliDiagnostic> {
    let workspace = &*session.app.workspace;
    let console = &mut *session.app.console;
    let mut version = 0;

    if mode.is_format() {
        let file_features = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: FeaturesBuilder::new().with_formatter().build(),
        })?;
        if file_features.is_protected() {
            let protected_diagnostic = WorkspaceError::protected_file(biome_path.to_string());
            if protected_diagnostic.tags().is_verbose() {
                if verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&protected_diagnostic)}})
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&protected_diagnostic)}})
            }
            console.append(markup! {{content}});
            return Ok(());
        };
        if file_features.supports_format() {
            workspace.open_file(OpenFileParams {
                project_key,
                path: biome_path.clone(),
                version: 0,
                content: FileContent::FromClient(content.into()),
                document_file_source: None,
                persist_node_cache: false,
            })?;
            let printed = workspace.format_file(FormatFileParams {
                project_key,
                path: biome_path.clone(),
            })?;

            let code = printed.into_code();
            let output = match biome_path.extension() {
                Some("astro") => AstroFileHandler::output(content, code.as_str()),
                Some("vue") => VueFileHandler::output(content, code.as_str()),
                Some("svelte") => SvelteFileHandler::output(content, code.as_str()),
                _ => code,
            };
            console.append(markup! {
                {output}
            });
        } else {
            console.append(markup! {
                {content}
            });
            console.error(markup! {
                <Warn>"The content was not formatted because the formatter is currently disabled."</Warn>
            });
            return Err(CliDiagnostic::stdin());
        }
    } else if mode.is_check() || mode.is_lint() {
        let mut new_content = Cow::Borrowed(content);

        workspace.open_file(OpenFileParams {
            project_key,
            path: biome_path.clone(),
            version: 0,
            content: FileContent::FromClient(content.into()),
            document_file_source: None,
            persist_node_cache: false,
        })?;
        // apply fix file of the linter
        let file_features = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: FeaturesBuilder::new()
                .with_linter()
                .with_assist()
                .with_formatter()
                .build(),
        })?;

        if file_features.is_protected() {
            let protected_diagnostic = WorkspaceError::protected_file(biome_path.to_string());
            if protected_diagnostic.tags().is_verbose() {
                if verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&protected_diagnostic)}})
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&protected_diagnostic)}})
            }
            console.append(markup! {{content}});
            return Ok(());
        };

        let (only, skip) = if let TraversalMode::Lint { only, skip, .. } = mode.traversal_mode() {
            (only.clone(), skip.clone())
        } else {
            (Vec::new(), Vec::new())
        };

        if let Some(fix_file_mode) = mode.as_fix_file_mode() {
            if file_features.supports_lint() || file_features.supports_assist() {
                let mut rule_categories = RuleCategoriesBuilder::default().with_syntax();

                if file_features.supports_lint() {
                    rule_categories = rule_categories.with_lint();
                }

                if file_features.supports_assist() {
                    rule_categories = rule_categories.with_assist();
                }

                let fix_file_result = workspace.fix_file(FixFileParams {
                    project_key,
                    fix_file_mode: *fix_file_mode,
                    path: biome_path.clone(),
                    should_format: mode.is_check() && file_features.supports_format(),
                    only: only.clone(),
                    skip: skip.clone(),
                    suppression_reason: None,
                    enabled_rules: vec![],
                    rule_categories: rule_categories.build(),
                })?;
                let code = fix_file_result.code;
                let output = match biome_path.extension() {
                    Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                    _ => code,
                };
                if output != new_content {
                    version += 1;
                    workspace.change_file(ChangeFileParams {
                        project_key,
                        content: output.clone(),
                        path: biome_path.clone(),
                        version,
                    })?;
                    new_content = Cow::Owned(output);
                }
            }
        }

        if file_features.supports_format() && mode.is_check() {
            let printed = workspace.format_file(FormatFileParams {
                project_key,
                path: biome_path.clone(),
            })?;
            let code = printed.into_code();
            let output = match biome_path.extension() {
                Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                _ => code,
            };
            if (mode.is_safe_fixes_enabled() || mode.is_safe_and_unsafe_fixes_enabled())
                && output != new_content
            {
                new_content = Cow::Owned(output);
            }
        }

        match new_content {
            Cow::Borrowed(original_content) => {
                console.append(markup! {
                    {original_content}
                });

                if !mode.is_write() {
                    return Err(CliDiagnostic::stdin());
                }
            }
            Cow::Owned(ref new_content) => {
                console.append(markup! {
                    {new_content}
                });
            }
        }
    } else if let TraversalMode::Search { pattern, .. } = mode.traversal_mode() {
        // Make sure patterns are always cleaned up at the end of execution.
        let _ = session.app.workspace.drop_pattern(DropPatternParams {
            pattern: pattern.clone(),
        });

        console.append(markup! {{content}});
    } else {
        console.append(markup! {{content}});
    }
    Ok(())
}
