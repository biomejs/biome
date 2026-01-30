use crate::CliDiagnostic;
use crate::diagnostics::StdinDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::diagnostics::{ResultExt, SkippedDiagnostic};
use crate::runner::process_file::{
    DiffKind, FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
};
use biome_analyze::RuleCategoriesBuilder;
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{Diagnostic, DiagnosticExt, Error, PrintDiagnostic, Severity, category};
use biome_service::WorkspaceError;
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::{
    CloseFileParams, FeaturesBuilder, FeaturesSupported, FileContent, FileFeaturesResult,
    FormatFileParams, OpenFileParams, SupportsFeatureParams,
};
use tracing::debug;

pub(crate) struct FormatProcessFile;

impl ProcessFile for FormatProcessFile {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        features_supported: &FeaturesSupported,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        // Open the file and create a workspace guard
        let execution = ctx.execution();
        let guard = workspace_file.guard();

        // Get file features
        let diagnostics_result = guard
            .pull_diagnostics(
                RuleCategoriesBuilder::default().with_syntax().build(),
                Vec::new(),
                Vec::new(),
                false, // NOTE: probably to revisit
            )
            .with_file_path_and_code(workspace_file.path.to_string(), category!("format"))?;

        let input = workspace_file.input()?;
        let should_write = execution.requires_write_access();
        let skip_parse_errors = execution.should_skip_parse_errors();

        if diagnostics_result.errors > 0 && skip_parse_errors {
            ctx.push_message(Message::from(
                SkippedDiagnostic.with_file_path(workspace_file.path.to_string()),
            ));
            return Ok(FileStatus::Ignored);
        }

        ctx.push_message(Message::Diagnostics {
            file_path: workspace_file.path.to_string(),
            content: input.clone(),
            diagnostics: diagnostics_result
                .diagnostics
                .into_iter()
                // Formatting is usually blocked by errors, so we want to print only diagnostics that
                // Have error severity
                .filter_map(|diagnostic| {
                    if diagnostic.severity() >= Severity::Error {
                        Some(Error::from(diagnostic))
                    } else {
                        None
                    }
                })
                .collect(),
            skipped_diagnostics: diagnostics_result.skipped_diagnostics as u32,
        });

        let printed = workspace_file
            .guard()
            .format_file()
            .with_file_path_and_code(workspace_file.path.to_string(), category!("format"))?;

        let mut output = printed.into_code();

        if !features_supported.supports_full_html_support() {
            match workspace_file.as_extension() {
                Some("astro") => {
                    if output.is_empty() {
                        return Ok(FileStatus::Unchanged);
                    }
                    output = AstroFileHandler::output(input.as_str(), output.as_str());
                }
                Some("vue") => {
                    if output.is_empty() {
                        return Ok(FileStatus::Unchanged);
                    }
                    output = VueFileHandler::output(input.as_str(), output.as_str());
                }

                Some("svelte") => {
                    if output.is_empty() {
                        return Ok(FileStatus::Unchanged);
                    }
                    output = SvelteFileHandler::output(input.as_str(), output.as_str());
                }
                _ => {}
            }
        }

        debug!("Format output is different from input: {}", output != input);
        if output != input {
            if should_write {
                workspace_file.update_file(output)?;
                Ok(FileStatus::Changed)
            } else {
                Ok(FileStatus::Message(Message::Diff {
                    file_name: workspace_file.path.to_string(),
                    old: input,
                    new: output,
                    diff_kind: DiffKind::Format,
                }))
            }
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
            execution: _,
            skip_ignore_check,
        } = payload;

        let FileFeaturesResult {
            features_supported: file_features,
        } = workspace.file_features(SupportsFeatureParams {
            project_key,
            path: biome_path.clone(),
            features: FeaturesBuilder::new().with_formatter().build(),
            inline_config: None,
            skip_ignore_check,
            not_requested_features: FeaturesBuilder::new()
                .with_all()
                .without_formatter()
                .build(),
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
        if file_features.supports_format() {
            workspace.open_file(OpenFileParams {
                project_key,
                path: biome_path.clone(),
                content: FileContent::from_client(content),
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
            })?;
            let printed = workspace.format_file(FormatFileParams {
                project_key,
                path: biome_path.clone(),
                inline_config: None,
            })?;

            let code = printed.into_code();
            let output = if !file_features.supports_full_html_support() {
                match biome_path.extension() {
                    Some("astro") => AstroFileHandler::output(content, code.as_str()),
                    Some("vue") => VueFileHandler::output(content, code.as_str()),
                    Some("svelte") => SvelteFileHandler::output(content, code.as_str()),
                    _ => code,
                }
            } else {
                code
            };
            console.append(markup! {
                {output}
            });
            workspace
                .close_file(CloseFileParams {
                    project_key,
                    path: biome_path.clone(),
                })
                .map_err(|err| err.into())
        } else {
            console.append(markup! {
                {content}
            });
            console.error(markup! {
                <Warn>"The content was not formatted because the formatter is currently disabled."</Warn>
            });
            Err(StdinDiagnostic::new_not_formatted().into())
        }
    }
}
