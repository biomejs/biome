use crate::diagnostics::LspError;
use crate::session::Session;
use anyhow::Context;
use biome_fs::BiomePath;
use biome_line_index::LineIndex;
use biome_lsp_converters::{from_proto, to_proto};
use biome_rowan::TextRange;
use biome_service::workspace::{GetFileContentParams, GoToDefinitionParams};
use std::str::FromStr;
use tower_lsp_server::ls_types::*;

pub(crate) fn goto_definition(
    session: &Session,
    params: GotoDefinitionParams,
) -> Result<Option<GotoDefinitionResponse>, LspError> {
    let url = params.text_document_position_params.text_document.uri;
    let path = session.file_path(&url)?;
    let Some(doc) = session.document(&url) else {
        return Ok(None);
    };

    let position_encoding = session.position_encoding();
    let cursor_offset = from_proto::offset(
        &doc.line_index,
        params.text_document_position_params.position,
        position_encoding,
    )
    .with_context(|| {
        format!(
            "failed to access position {:?} in document {}",
            params.text_document_position_params.position,
            url.as_str()
        )
    })?;

    let cursor_range = TextRange::new(cursor_offset, cursor_offset);

    let enabled = session
        .extension_settings
        .read()
        .unwrap()
        .goto_definition_enabled();

    let result = session.workspace.go_to_definition(GoToDefinitionParams {
        project_key: doc.project_key,
        path: path.clone(),
        cursor_range,
        enabled,
    })?;

    match result {
        Some(definition) => {
            let target_uri = uri_from_path(&definition.path)?;

            // For same-file definitions, reuse the existing LineIndex.
            // For cross-file definitions, read the target and build a LineIndex.
            let target_range = if definition.path == path {
                to_proto::range(&doc.line_index, definition.range, position_encoding)?
            } else {
                match session.workspace.get_file_content(GetFileContentParams {
                    project_key: doc.project_key,
                    path: definition.path.clone(),
                }) {
                    Ok(content) => {
                        let target_line_index = LineIndex::new(&content);
                        to_proto::range(&target_line_index, definition.range, position_encoding)?
                    }
                    Err(_) => Range::default(),
                }
            };

            Ok(Some(GotoDefinitionResponse::Scalar(Location {
                uri: target_uri,
                range: target_range,
            })))
        }
        None => Ok(None),
    }
}

fn uri_from_path(path: &BiomePath) -> Result<Uri, LspError> {
    let url = url::Url::from_file_path(path.as_path()).map_err(|_| {
        LspError::from(anyhow::anyhow!(
            "failed to convert path to URL: {}",
            path.as_path()
        ))
    })?;
    Uri::from_str(url.as_str())
        .map_err(|err| LspError::from(anyhow::anyhow!("failed to convert URL to URI: {err}")))
}
