use crate::{diagnostics::LspError, session::Session};
use biome_service::workspace::{
    FeaturesBuilder, GetSyntaxTreeParams, IgnoreKind, PathIsIgnoredParams,
};
use serde::{Deserialize, Serialize};
use tower_lsp_server::lsp_types::{TextDocumentIdentifier, Uri};
use tracing::info;

pub const SYNTAX_TREE_REQUEST: &str = "biome_lsp/syntaxTree";

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SyntaxTreePayload {
    pub text_document: TextDocumentIdentifier,
}

pub(crate) fn syntax_tree(session: &Session, url: &Uri) -> Result<Option<String>, LspError> {
    info!("Showing syntax tree");
    let path = session.file_path(url)?;
    let Some(doc) = session.document(url) else {
        return Ok(None);
    };
    let features = FeaturesBuilder::new().build();

    if session.workspace.is_path_ignored(PathIsIgnoredParams {
        path: path.clone(),
        project_key: doc.project_key,
        features,
        ignore_kind: IgnoreKind::Ancestors,
    })? {
        return Ok(None);
    }
    let syntax_tree = session.workspace.get_syntax_tree(GetSyntaxTreeParams {
        project_key: doc.project_key,
        path,
    })?;
    Ok(Some(syntax_tree.ast))
}
