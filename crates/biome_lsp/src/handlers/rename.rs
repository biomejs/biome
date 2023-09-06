use std::collections::HashMap;

use crate::converters::from_proto;
use crate::{session::Session, utils};
use anyhow::{Context, Result};
use tower_lsp::lsp_types::{RenameParams, WorkspaceEdit};
use tracing::trace;

#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) fn rename(session: &Session, params: RenameParams) -> Result<Option<WorkspaceEdit>> {
    let url = params.text_document_position.text_document.uri;
    let rome_path = session.file_path(&url)?;

    trace!("Renaming...");

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
        .rename(rome_service::workspace::RenameParams {
            path: rome_path,
            symbol_at: cursor_range,
            new_name: params.new_name,
        })?;

    let mut changes = HashMap::new();
    changes.insert(
        url,
        utils::text_edit(&doc.line_index, result.indels, position_encoding)?,
    );

    let workspace_edit = WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    };

    Ok(Some(workspace_edit))
}
