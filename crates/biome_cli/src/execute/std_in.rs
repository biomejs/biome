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
use biome_service::workspace::{
    ChangeFileParams, DropPatternParams, FeaturesBuilder, FixFileParams, FormatFileParams,
    OpenFileParams, OrganizeImportsParams, SupportsFeatureParams,
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
            let output = match biome_path.extension().map(|ext| ext.as_encoded_bytes()) {
                Some(b"astro") => AstroFileHandler::output(content, code.as_str()),
                Some(b"vue") => VueFileHandler::output(content, code.as_str()),
                Some(b"svelte") => SvelteFileHandler::output(content, code.as_str()),
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

        let (only, skip) = if let TraversalMode::Lint { only, skip, .. } = mode.traversal_mode() {
            (only.clone(), skip.clone())
        } else {
            (Vec::new(), Vec::new())
        };

        if let Some(fix_file_mode) = mode.as_fix_file_mode() {
            if file_features.supports_lint() {
                let fix_file_result = workspace.fix_file(FixFileParams {
                    fix_file_mode: *fix_file_mode,
                    path: biome_path.clone(),
                    should_format: mode.is_check() && file_features.supports_format(),
                    only: only.clone(),
                    skip: skip.clone(),
                    rule_categories: RuleCategoriesBuilder::default()
                        .with_syntax()
                        .with_lint()
                        .build(),
                })?;
                let code = fix_file_result.code;
                let output = match biome_path.extension().map(|ext| ext.as_encoded_bytes()) {
                    Some(b"astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some(b"vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some(b"svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
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
                let output = match biome_path.extension().map(|ext| ext.as_encoded_bytes()) {
                    Some(b"astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some(b"vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some(b"svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
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

        if file_features.supports_format() && mode.is_check() {
            let printed = workspace.format_file(FormatFileParams {
                path: biome_path.clone(),
            })?;
            let code = printed.into_code();
            let output = match biome_path.extension().map(|ext| ext.as_encoded_bytes()) {
                Some(b"astro") => AstroFileHandler::output(&new_content, code.as_str()),
                Some(b"vue") => VueFileHandler::output(&new_content, code.as_str()),
                Some(b"svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                _ => code,
            };
            if (mode.is_check_apply() || mode.is_check_apply_unsafe()) && output != new_content {
                new_content = Cow::Owned(output);
            }
        }

        match new_content {
            Cow::Borrowed(original_content) => {
                console.append(markup! {
                    {original_content}
                });
                return Err(CliDiagnostic::stdin());
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
