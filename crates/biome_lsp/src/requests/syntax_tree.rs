use crate::session::Session;
use anyhow::Result;
use biome_service::workspace::GetSyntaxTreeParams;
use serde::{Deserialize, Serialize};
use tower_lsp::lsp_types::{TextDocumentIdentifier, Url};
use tracing::info;

pub const SYNTAX_TREE_REQUEST: &str = "biome_lsp/syntaxTree";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreePayload {
    pub text_document: TextDocumentIdentifier,
}

pub(crate) fn syntax_tree(session: &Session, url: &Url) -> Result<String> {
    info!("Showing syntax tree");
    let biome_path = session.file_path(url)?;
    let syntax_tree = session
        .workspace
        .get_syntax_tree(GetSyntaxTreeParams { path: biome_path })?;
    Ok(syntax_tree.ast)
}
