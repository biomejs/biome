use crate::CliDiagnostic;
use crate::diagnostics::StdinDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::runner::execution::AnalyzerSelectors;
use crate::runner::process_file::{
    FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_console::{ConsoleExt, markup};
use biome_css_syntax::TextSize;
use biome_diagnostics::{Diagnostic, DiagnosticExt, Error, PrintDiagnostic, Severity};
use biome_service::WorkspaceError;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::{
    ChangeFileParams, CloseFileParams, FeaturesSupported, FileContent, FileFeaturesResult,
    FixFileParams, FormatFileParams, OpenFileParams, SupportsFeatureParams,
};
use std::borrow::Cow;
use tracing::info;

pub struct LintAssistProcessFile;

impl ProcessFile for LintAssistProcessFile {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        features_supported: &FeaturesSupported,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        let execution = ctx.execution();
        let mut categories = RuleCategoriesBuilder::default().with_syntax();
        if features_supported.supports_lint() {
            categories = categories.with_lint();
        }
        if features_supported.supports_assist() && (execution.is_check() || execution.is_ci()) {
            categories = categories.with_assist();
        }
        let categories = categories.build();
        let mut input = workspace_file.input()?;
        let mut changed = false;
        let AnalyzerSelectors { only, skip } = execution.analyzer_selectors();
        if let Some(fix_mode) = execution.as_fix_file_mode() {
            let suppression_explanation =
                if execution.suppress() && execution.suppression_reason().is_none() {
                    "ignored using `--suppress`"
                } else {
                    execution.suppression_reason().unwrap_or("<explanation>")
                };

            let fix_result = workspace_file
                .guard()
                .fix_file(
                    fix_mode,
                    features_supported.supports_format(),
                    categories,
                    only.clone(),
                    skip.clone(),
                    Some(suppression_explanation.to_string()),
                )
                .with_file_path_and_code(
                    workspace_file.path.to_string(),
                    execution.as_diagnostic_category(),
                )?;

            info!(
                "Fix file summary result. Errors {}, skipped fixes {}, actions {}",
                fix_result.errors,
                fix_result.skipped_suggested_fixes,
                fix_result.actions.len()
            );

            ctx.push_message(Message::SkippedFixes {
                skipped_suggested_fixes: fix_result.skipped_suggested_fixes,
            });

            let mut output = fix_result.code;

            if !features_supported.supports_full_html_support() {
                match workspace_file.as_extension() {
                    Some("astro") => {
                        output = AstroFileHandler::output(input.as_str(), output.as_str());
                    }
                    Some("vue") => {
                        output = VueFileHandler::output(input.as_str(), output.as_str());
                    }
                    Some("svelte") => {
                        output = SvelteFileHandler::output(input.as_str(), output.as_str());
                    }
                    _ => {}
                }
            }
            if output != input {
                changed = true;
                workspace_file.update_file(output)?;
                input = workspace_file.input()?;
            }
        }

        let pull_diagnostics_result = workspace_file
            .guard()
            .pull_diagnostics(categories, only, skip, true)
            .with_file_path_and_code(
                workspace_file.path.to_string(),
                execution.as_diagnostic_category(),
            )?;

        let skip_parse_errors = execution.should_skip_parse_errors();
        if pull_diagnostics_result.errors > 0 && skip_parse_errors {
            ctx.push_message(Message::from(
                SkippedDiagnostic.with_file_path(workspace_file.path.to_string()),
            ));
            return Ok(FileStatus::Ignored);
        }

        let no_diagnostics = pull_diagnostics_result.diagnostics.is_empty()
            && pull_diagnostics_result.skipped_diagnostics == 0;

        if !no_diagnostics {
            let offset = if features_supported.supports_full_html_support() {
                None
            } else {
                match workspace_file.as_extension() {
                    Some("vue") => VueFileHandler::start(input.as_str()),
                    Some("astro") => AstroFileHandler::start(input.as_str()),
                    Some("svelte") => SvelteFileHandler::start(input.as_str()),
                    _ => None,
                }
            };

            ctx.push_message(Message::Diagnostics {
                file_path: workspace_file.path.to_string(),
                content: input,
                diagnostics: pull_diagnostics_result
                    .diagnostics
                    .into_iter()
                    .map(|d| {
                        if let Some(offset) = offset {
                            d.with_offset(TextSize::from(offset))
                        } else {
                            d
                        }
                    })
                    .map(|diagnostic| {
                        let category = diagnostic.category();
                        if let Some(category) = category
                            && category.name().starts_with("assist/")
                            && execution.should_enforce_assist()
                        {
                            return diagnostic.with_severity(Severity::Error);
                        }
                        Error::from(diagnostic)
                    })
                    .collect(),
                skipped_diagnostics: pull_diagnostics_result.skipped_diagnostics as u32,
            });
        }

        if changed {
            Ok(FileStatus::Changed)
        } else {
            Ok(FileStatus::Unchanged)
        }
    }

    fn process_std_in(payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        let ProcessStdinFilePayload {
            workspace,
            content,
            project_key,
            biome_path,
            console,
            cli_options,
            execution,
            skip_ignore_check,
        } = payload;

        let mut new_content = Cow::Borrowed(content);
        let mut version = 0;

        workspace.open_file(OpenFileParams {
            project_key,
            path: biome_path.clone(),
            content: FileContent::from_client(content),
            document_file_source: None,
            persist_node_cache: false,
            inline_config: None,
        })?;

        // apply fix file of the linter
        let FileFeaturesResult {
            features_supported: file_features,
        } = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: payload.execution.wanted_features(),
            inline_config: None,
            skip_ignore_check,
            not_requested_features: payload.execution.not_requested_features(),
        })?;

        if file_features.is_ignored() {
            console.append(markup! {{content}});
            return Ok(());
        }

        if file_features.is_protected() {
            let protected_diagnostic = WorkspaceError::protected_file(biome_path.to_string());
            if protected_diagnostic.tags().is_verbose() {
                if cli_options.verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&protected_diagnostic)}})
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&protected_diagnostic)}})
            }
            console.append(markup! {{content}});

            return Ok(());
        };

        let AnalyzerSelectors { only, skip } = execution.analyzer_selectors();

        if let Some(fix_file_mode) = execution.as_fix_file_mode()
            && (file_features.supports_lint() || file_features.supports_assist())
        {
            let mut rule_categories = RuleCategoriesBuilder::default().with_syntax();

            if file_features.supports_lint() {
                rule_categories = rule_categories.with_lint();
            }

            if file_features.supports_assist() {
                rule_categories = rule_categories.with_assist();
            }

            let fix_file_result = workspace.fix_file(FixFileParams {
                project_key,
                fix_file_mode,
                path: biome_path.clone(),
                should_format: file_features.supports_format(),
                only: only.clone(),
                skip: skip.clone(),
                suppression_reason: None,
                enabled_rules: vec![],
                rule_categories: rule_categories.build(),
                inline_config: None,
            })?;
            let code = fix_file_result.code;
            let output = if !file_features.supports_full_html_support() {
                match biome_path.extension() {
                    Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                    _ => code,
                }
            } else {
                code
            };
            if output != new_content {
                version += 1;
                workspace.change_file(ChangeFileParams {
                    project_key,
                    content: output.clone(),
                    path: biome_path.clone(),
                    version,
                    inline_config: None,
                })?;
                new_content = Cow::Owned(output);
            }
        }

        if file_features.supports_format() && execution.is_check() {
            let printed = workspace.format_file(FormatFileParams {
                project_key,
                path: biome_path.clone(),
                inline_config: None,
            })?;
            let code = printed.into_code();
            let output = if !file_features.supports_full_html_support() {
                match biome_path.extension() {
                    Some("astro") => AstroFileHandler::output(&new_content, code.as_str()),
                    Some("vue") => VueFileHandler::output(&new_content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(&new_content, code.as_str()),
                    _ => code,
                }
            } else {
                code
            };
            if (execution.is_safe_fixes_enabled() || execution.is_safe_and_unsafe_fixes_enabled())
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

                if !execution.requires_write_access() {
                    return Err(StdinDiagnostic::new_not_formatted().into());
                }
            }
            Cow::Owned(ref new_content) => {
                console.append(markup! {
                    {new_content}
                });
            }
        }
        workspace
            .close_file(CloseFileParams {
                project_key,
                path: biome_path.clone(),
            })
            .map_err(|e| e.into())
    }
}
