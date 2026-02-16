use crate::CliDiagnostic;
use crate::runner::crawler::CrawlerContext;
use crate::runner::impls::process_file::format::FormatProcessFile;
use crate::runner::impls::process_file::lint_and_assist::LintAssistProcessFile;
use crate::runner::process_file::{
    FileStatus, Message, ProcessFile, ProcessStdinFilePayload, WorkspaceFile,
};
use biome_service::workspace::FeaturesSupported;

pub(crate) struct CheckProcessFile;

impl ProcessFile for CheckProcessFile {
    fn process_file<Ctx>(
        ctx: &Ctx,
        workspace_file: &mut WorkspaceFile,
        features_supported: &FeaturesSupported,
    ) -> Result<FileStatus, Message>
    where
        Ctx: CrawlerContext,
    {
        let execution = ctx.execution();
        let mut has_failures = false;
        let analyzer_result =
            LintAssistProcessFile::process_file(ctx, workspace_file, features_supported);

        let mut changed = false;
        // To reduce duplication of the same error on format and lint_and_assist
        let mut skipped_parse_error = false;

        match analyzer_result {
            Ok(status) => {
                if matches!(status, FileStatus::Ignored) && execution.should_skip_parse_errors() {
                    skipped_parse_error = true;
                }

                if status.is_changed() {
                    changed = true
                }
                if let FileStatus::Message(msg) = status {
                    if msg.is_failure() {
                        has_failures = true;
                    }
                    ctx.push_message(msg);
                }
            }
            Err(err) => {
                ctx.push_message(err);
                has_failures = true;
            }
        }

        if features_supported.supports_format() {
            if execution.should_skip_parse_errors() && skipped_parse_error {
                // Parse errors are already skipped during the analyze phase, so no need to do it here.
            } else {
                let format_result =
                    FormatProcessFile::process_file(ctx, workspace_file, features_supported);

                match format_result {
                    Ok(status) => {
                        if status.is_changed() {
                            changed = true
                        }
                        if let FileStatus::Message(msg) = status {
                            if msg.is_failure() {
                                has_failures = true;
                            }
                            ctx.push_message(msg);
                        }
                    }
                    Err(err) => {
                        ctx.push_message(err);
                        has_failures = true;
                    }
                }
            }
        }

        if has_failures {
            Ok(FileStatus::Message(Message::Failure))
        } else if changed {
            Ok(FileStatus::Changed)
        } else {
            Ok(FileStatus::Unchanged)
        }
    }

    fn process_std_in(payload: ProcessStdinFilePayload) -> Result<(), CliDiagnostic> {
        LintAssistProcessFile::process_std_in(payload)
    }
}
