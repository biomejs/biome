use crate::converters::{from_proto, to_proto};
use crate::diagnostics::LspError;
use crate::session::Session;
use anyhow::Context;
use biome_fs::RomePath;
use biome_service::workspace::{
    FeatureName, FeaturesBuilder, FileFeaturesResult, FormatFileParams, FormatOnTypeParams,
    FormatRangeParams, SupportsFeatureParams,
};
use biome_service::{extension_error, WorkspaceError};
use tower_lsp::lsp_types::*;
use tracing::debug;

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format(
    session: &Session,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url)?;

    let doc = session.document(&url)?;

    let file_features = session.workspace.file_features(SupportsFeatureParams {
        path: rome_path.clone(),
        feature: FeaturesBuilder::new().with_formatter().build(),
    })?;

    if file_features.supports_for(&FeatureName::Format) {
        debug!("Formatting...");
        let printed = session
            .workspace
            .format_file(FormatFileParams { path: rome_path })?;

        let num_lines: u32 = doc.line_index.len();

        let range = Range {
            start: Position::default(),
            end: Position {
                line: num_lines,
                character: 0,
            },
        };

        let edits = vec![TextEdit {
            range,
            new_text: printed.into_code(),
        }];

        Ok(Some(edits))
    } else {
        notify_user(file_features, rome_path)
    }
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_range(
    session: &Session,
    params: DocumentRangeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document.uri;
    let rome_path = session.file_path(&url)?;

    let file_features = session.workspace.file_features(SupportsFeatureParams {
        path: rome_path.clone(),
        feature: FeaturesBuilder::new().with_formatter().build(),
    })?;

    if file_features.supports_for(&FeatureName::Format) {
        let doc = session.document(&url)?;

        let position_encoding = session.position_encoding();
        let format_range = from_proto::text_range(&doc.line_index, params.range, position_encoding)
            .with_context(|| {
                format!(
                    "failed to convert range {:?} in document {url}",
                    params.range.end
                )
            })?;

        let formatted = session.workspace.format_range(FormatRangeParams {
            path: rome_path,
            range: format_range,
        })?;

        // Recalculate the actual range that was reformatted from the formatter result
        let formatted_range = match formatted.range() {
            Some(range) => {
                let position_encoding = session.position_encoding();
                to_proto::range(&doc.line_index, range, position_encoding)?
            }
            None => Range {
                start: Position::default(),
                end: Position {
                    line: doc.line_index.len(),
                    character: 0,
                },
            },
        };

        Ok(Some(vec![TextEdit {
            range: formatted_range,
            new_text: formatted.into_code(),
        }]))
    } else {
        notify_user(file_features, rome_path)
    }
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_on_type(
    session: &Session,
    params: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let rome_path = session.file_path(&url)?;

    let file_features = session.workspace.file_features(SupportsFeatureParams {
        path: rome_path.clone(),
        feature: FeaturesBuilder::new().with_formatter().build(),
    })?;

    if file_features.supports_for(&FeatureName::Format) {
        let doc = session.document(&url)?;

        let position_encoding = session.position_encoding();
        let offset = from_proto::offset(&doc.line_index, position, position_encoding)
            .with_context(|| format!("failed to access position {position:?} in document {url}"))?;

        let formatted = session.workspace.format_on_type(FormatOnTypeParams {
            path: rome_path,
            offset,
        })?;

        // Recalculate the actual range that was reformatted from the formatter result
        let formatted_range = match formatted.range() {
            Some(range) => {
                let position_encoding = session.position_encoding();
                let start_loc =
                    to_proto::position(&doc.line_index, range.start(), position_encoding)?;
                let end_loc = to_proto::position(&doc.line_index, range.end(), position_encoding)?;
                Range {
                    start: start_loc,
                    end: end_loc,
                }
            }
            None => Range {
                start: Position::default(),
                end: Position {
                    line: doc.line_index.len(),
                    character: 0,
                },
            },
        };

        Ok(Some(vec![TextEdit {
            range: formatted_range,
            new_text: formatted.into_code(),
        }]))
    } else {
        notify_user(file_features, rome_path)
    }
}

fn notify_user<T>(file_features: FileFeaturesResult, rome_path: RomePath) -> Result<T, LspError> {
    let error = if file_features.is_ignored() {
        WorkspaceError::file_ignored(rome_path.display().to_string())
    } else if file_features.is_protected() {
        WorkspaceError::protected_file(rome_path.display().to_string())
    } else {
        extension_error(&rome_path)
    };

    Err(error.into())
}
