use std::collections::HashMap;

use crate::diagnostics::LspError;
use crate::{session::Session, utils};
use anyhow::{Context, Result};
use biome_lsp_converters::from_proto;
use tower_lsp::lsp_types::{RenameParams, WorkspaceEdit};

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn rename(
    session: &Session,
    params: RenameParams,
) -> Result<Option<WorkspaceEdit>, LspError> {
    let url = params.text_document_position.text_document.uri;
    let path = session.file_path(&url)?;

    let doc = session.document(&url)?;
    let position_encoding = session.position_encoding();
    let cursor_range = from_proto::offset(
        &doc.line_index,
        params.text_document_position.position,
        position_encoding,
    )
    .with_context(|| {
        format!(
            "failed to access position {:?} in document {url}",
            params.text_document_position.position
        )
    })?;

    let result = session
        .workspace
        .rename(biome_service::workspace::RenameParams {
            project_key: doc.project_key,
            path,
            symbol_at: cursor_range,
            new_name: params.new_name,
        })?;

    let mut changes = HashMap::new();
    changes.insert(
        url,
        utils::text_edit(&doc.line_index, result.indels, position_encoding, None)?,
    );

    let workspace_edit = WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(workspace_edit))
}
