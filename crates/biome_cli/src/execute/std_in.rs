//! In here, there are the operations that run via standard input
//!
use crate::execute::diagnostics::{ContentDiffAdvice, FormatDiffDiagnostic};
use crate::execute::Execution;
use crate::{CliDiagnostic, CliSession, TraversalMode};
use biome_console::{markup, ConsoleExt};
use biome_diagnostics::Diagnostic;
use biome_diagnostics::PrintDiagnostic;
use biome_fs::BiomePath;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::{
    ChangeFileParams, DropPatternParams, FeaturesBuilder, FixFileParams, FormatFileParams,
    OpenFileParams, OrganizeImportsParams, PullDiagnosticsParams, RuleCategories,
    SupportsFeatureParams,
};
use biome_service::WorkspaceError;
use std::borrow::Cow;

pub(crate) fn run<'a>(
    session: CliSession,
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
            path: biome_path.clone(),
            features: FeaturesBuilder::new().with_formatter().build(),
        })?;
        if file_features.is_protected() {
            let protected_diagnostic =
                WorkspaceError::protected_file(biome_path.display().to_string());
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
                path: biome_path.clone(),
                version: 0,
                content: content.into(),
                document_file_source: None,
            })?;
            let printed = workspace.format_file(FormatFileParams {
                path: biome_path.clone(),
            })?;

            let code = printed.into_code();
            let output = match biome_path.extension_as_str() {
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
                })
        }
    } else if mode.is_check() || mode.is_lint() {
        let mut diagnostics = Vec::new();
        let mut new_content = Cow::Borrowed(content);

        workspace.open_file(OpenFileParams {
            path: biome_path.clone(),
            version: 0,
            content: content.into(),
            document_file_source: None,
        })?;
        // apply fix file of the linter
        let file_features = workspace.file_features(SupportsFeatureParams {
            path: biome_path.clone(),
            features: FeaturesBuilder::new()
                .with_linter()
                .with_organize_imports()
                .with_formatter()
                .build(),
        })?;

        if file_features.is_protected() {
            let protected_diagnostic =
                WorkspaceError::protected_file(biome_path.display().to_string());
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

        if let Some(fix_file_mode) = mode.as_fix_file_mode() {
            if file_features.supports_lint() {
                let fix_file_result = workspace.fix_file(FixFileParams {
                    fix_file_mode: *fix_file_mode,
                    path: biome_path.clone(),
                    should_format: mode.is_check() && file_features.supports_format(),
                })?;
                let code = fix_file_result.code;
                let output = match biome_path.extension_as_str() {
                    Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                    _ => code,
                };
                if output != new_content {
                    version += 1;
                    workspace.change_file(ChangeFileParams {
                        content: output.clone(),
                        path: biome_path.clone(),
                        version,
                    })?;
                    new_content = Cow::Owned(output);
                }
            }

            if file_features.supports_organize_imports() && mode.is_check() {
                let result = workspace.organize_imports(OrganizeImportsParams {
                    path: biome_path.clone(),
                })?;
                let code = result.code;
                let output = match biome_path.extension_as_str() {
                    Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                    _ => code,
                };
                if output != new_content {
                    version += 1;
                    workspace.change_file(ChangeFileParams {
                        content: output.clone(),
                        path: biome_path.clone(),
                        version,
                    })?;
                    new_content = Cow::Owned(output);
                }
            }
        }

        let rule = if let TraversalMode::Lint { rule, .. } = mode.traversal_mode() {
            *rule
        } else {
            None
        };
        if !mode.is_check_apply_unsafe() {
            let result = workspace.pull_diagnostics(PullDiagnosticsParams {
                categories: RuleCategories::LINT | RuleCategories::SYNTAX,
                path: biome_path.clone(),
                max_diagnostics: mode.max_diagnostics.into(),
                rule,
            })?;
            diagnostics.extend(result.diagnostics);
        }

        if file_features.supports_format() && mode.is_check() {
            let printed = workspace.format_file(FormatFileParams {
                path: biome_path.clone(),
            })?;
            let code = printed.into_code();
            let output = match biome_path.extension_as_str() {
                Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                _ => code,
            };
            if mode.is_check_apply() || mode.is_check_apply_unsafe() {
                if output != new_content {
                    new_content = Cow::Owned(output);
                }
            } else {
                let diagnostic = FormatDiffDiagnostic {
                    file_name: biome_path.display().to_string(),
                    diff: ContentDiffAdvice {
                        new: output,
                        old: content.to_string(),
                    },
                };
                diagnostics.push(biome_diagnostics::serde::Diagnostic::new(diagnostic));
            }
        }

        match new_content {
            Cow::Borrowed(original_content) => {
                console.append(markup! {
                    {original_content}
                });
            }
            Cow::Owned(new_content) => {
                console.append(markup! {
                    {new_content}
                });
            }
        }
        if !diagnostics.is_empty() {
            for diag in diagnostics {
                console.error(markup! {
                    {if verbose { PrintDiagnostic::verbose(&diag) } else { PrintDiagnostic::simple(&diag) }}
                })
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
