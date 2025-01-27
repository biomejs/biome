use crate::{diagnostics::LspError, session::Session};
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

pub(crate) fn syntax_tree(session: &Session, url: &Url) -> Result<String, LspError> {
    info!("Showing syntax tree");
    let path = session.file_path(url)?;
    let doc = session.document(url)?;
    let syntax_tree = session.workspace.get_syntax_tree(GetSyntaxTreeParams {
        project_key: doc.project_key,
        path,
    })?;
    Ok(syntax_tree.ast)
}
