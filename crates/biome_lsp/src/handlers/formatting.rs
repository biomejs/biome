use crate::converters::{from_proto, to_proto};
use crate::diagnostics::LspError;
use crate::session::Session;
use anyhow::Context;
use biome_fs::BiomePath;
use biome_rowan::{TextRange, TextSize};
use biome_service::file_handlers::{AstroFileHandler, SvelteFileHandler, VueFileHandler};
use biome_service::workspace::{
    FeaturesBuilder, FileFeaturesResult, FormatFileParams, FormatOnTypeParams, FormatRangeParams,
    GetFileContentParams, SupportsFeatureParams,
};
use biome_service::{extension_error, WorkspaceError};
use std::ops::{Add, Sub};
use tower_lsp::lsp_types::*;
use tracing::debug;

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format(
    session: &Session,
    params: DocumentFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document.uri;
    let biome_path = session.file_path(&url)?;

    let doc = session.document(&url)?;

    let file_features = session.workspace.file_features(SupportsFeatureParams {
        path: biome_path.clone(),
        features: FeaturesBuilder::new().with_formatter().build(),
    })?;

    if file_features.supports_format() {
        debug!("Formatting...");
        let printed = session.workspace.format_file(FormatFileParams {
            path: biome_path.clone(),
        })?;

        let mut output = printed.into_code();
        let file_extension = biome_path.extension().and_then(|s| s.to_str());
        let input = session.workspace.get_file_content(GetFileContentParams {
            path: biome_path.clone(),
        })?;
        if output.is_empty() {
            return Ok(None);
        }
        match file_extension {
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
            new_text: output,
        }];

        Ok(Some(edits))
    } else {
        notify_user(file_features, biome_path)
    }
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_range(
    session: &Session,
    params: DocumentRangeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document.uri;
    let biome_path = session.file_path(&url)?;

    let file_features = session.workspace.file_features(SupportsFeatureParams {
        path: biome_path.clone(),
        features: FeaturesBuilder::new().with_formatter().build(),
    })?;

    if file_features.supports_format() {
        let doc = session.document(&url)?;

        let position_encoding = session.position_encoding();
        let format_range = from_proto::text_range(&doc.line_index, params.range, position_encoding)
            .with_context(|| {
                format!(
                    "failed to convert range {:?} in document {url}",
                    params.range.end
                )
            })?;
        let content = session.workspace.get_file_content(GetFileContentParams {
            path: biome_path.clone(),
        })?;
        let offset = match biome_path.extension().and_then(|s| s.to_str()) {
            Some("vue") => VueFileHandler::start(content.as_str()),
            Some("astro") => AstroFileHandler::start(content.as_str()),
            Some("svelte") => SvelteFileHandler::start(content.as_str()),
            _ => None,
        };
        let format_range = if let Some(offset) = offset {
            if format_range.start() - TextSize::from(offset) >= TextSize::from(0) {
                TextRange::new(
                    format_range.start().sub(TextSize::from(offset)),
                    format_range.end().sub(TextSize::from(offset)),
                )
            } else {
                format_range
            }
        } else {
            format_range
        };

        let formatted = session.workspace.format_range(FormatRangeParams {
            path: biome_path,
            range: format_range,
        })?;

        // Recalculate the actual range that was reformatted from the formatter result
        let formatted_range = match formatted.range() {
            Some(range) => {
                let position_encoding = session.position_encoding();
                let range = if let Some(offset) = offset {
                    TextRange::new(
                        range.start().add(TextSize::from(offset)),
                        range.end().add(TextSize::from(offset)),
                    )
                } else {
                    range
                };
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
        notify_user(file_features, biome_path)
    }
}

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn format_on_type(
    session: &Session,
    params: DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<TextEdit>>, LspError> {
    let url = params.text_document_position.text_document.uri;
    let position = params.text_document_position.position;

    let biome_path = session.file_path(&url)?;

    let file_features = session.workspace.file_features(SupportsFeatureParams {
        path: biome_path.clone(),
        features: FeaturesBuilder::new().with_formatter().build(),
    })?;

    if file_features.supports_format() {
        let doc = session.document(&url)?;

        let position_encoding = session.position_encoding();
        let offset = from_proto::offset(&doc.line_index, position, position_encoding)
            .with_context(|| format!("failed to access position {position:?} in document {url}"))?;

        let formatted = session.workspace.format_on_type(FormatOnTypeParams {
            path: biome_path,
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
        notify_user(file_features, biome_path)
    }
}

fn notify_user<T>(file_features: FileFeaturesResult, biome_path: BiomePath) -> Result<T, LspError> {
    let error = if file_features.is_ignored() {
        WorkspaceError::file_ignored(biome_path.display().to_string())
    } else if file_features.is_protected() {
        WorkspaceError::protected_file(biome_path.display().to_string())
    } else {
        extension_error(&biome_path)
    };

    Err(error.into())
}
