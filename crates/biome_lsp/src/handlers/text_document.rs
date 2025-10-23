use crate::diagnostics::LspError;
use crate::session::ConfigurationStatus;
use crate::utils::apply_document_changes;
use crate::{documents::Document, session::Session};
use biome_configuration::ConfigurationPathHint;
use biome_service::workspace::{
    ChangeFileParams, CloseFileParams, DocumentFileSource, FeaturesBuilder, FileContent,
    GetFileContentParams, IgnoreKind, OpenFileParams, PathIsIgnoredParams,
};
use camino::Utf8PathBuf;
use std::sync::Arc;
use tower_lsp_server::{UriExt, lsp_types};
use tracing::{debug, error, field, info};

/// Handler for `textDocument/didOpen` LSP notification
#[tracing::instrument(
    level = "debug",
    skip_all,
    fields(
        text_document_uri = display(params.text_document.uri.as_str()),
        text_document_language_id = display(&params.text_document.language_id),
    )
)]
pub(crate) async fn did_open(
    session: &Arc<Session>,
    params: lsp_types::DidOpenTextDocumentParams,
) -> Result<(), LspError> {
    let url = params.text_document.uri;
    let version = params.text_document.version;
    let content = params.text_document.text;
    let language_hint = DocumentFileSource::from_language_id(&params.text_document.language_id);

    let path = session.file_path(&url)?;

    eprintln!("Session id {:?}", session.key);
    let project_key = match session.project_for_path(&path) {
        Some(project_key) => project_key,
        None => {
            info!("No open project for path: {path:?}. Opening new project.");

            let project_path = path
                .parent()
                .map(|parent| parent.to_path_buf())
                .unwrap_or_default();

            // First check if the current file belongs to any registered workspace folder.
            // If so, return that folder; otherwise, use the folder computed by did_open.
            let project_path = if let Some(workspace_folders) = session.get_workspace_folders() {
                if let Some(ws_root) = workspace_folders
                    .iter()
                    .filter_map(|folder| {
                        folder.uri.to_file_path().map(|p| {
                            Utf8PathBuf::from_path_buf(p.to_path_buf())
                                .expect("To have a valid UTF-8 path")
                        })
                    })
                    .find(|ws| project_path.starts_with(ws))
                {
                    ws_root
                } else {
                    project_path.clone()
                }
            } else if let Some(base_path) = session.base_path() {
                if project_path.starts_with(&base_path) {
                    base_path
                } else {
                    project_path.clone()
                }
            } else {
                project_path
            };

            session.set_configuration_status(ConfigurationStatus::Loading);
            eprintln!(
                "Loading configuration from text_document {:?}",
                &project_path
            );
            if !session.has_initialized() {
                session.load_extension_settings().await;
            }
            let status = session
                .load_biome_configuration_file(ConfigurationPathHint::FromLsp(project_path), false)
                .await;

            session.set_configuration_status(status);

            if status.is_loaded() {
                match session.project_for_path(&path) {
                    Some(project_key) => project_key,

                    None => {
                        error!("Could not find project for {path}");

                        return Ok(());
                    }
                }
            } else {
                error!("Configuration could not be loaded for {path}");

                return Ok(());
            }
        }
    };

    let is_ignored = session
        .workspace
        .is_path_ignored(PathIsIgnoredParams {
            project_key,
            path: path.clone(),
            features: FeaturesBuilder::new().build(),
            ignore_kind: IgnoreKind::Ancestors,
        })
        .unwrap_or_default();

    if is_ignored {
        return Ok(());
    }

    let doc = Document::new(project_key, version, &content);

    session.workspace.open_file(OpenFileParams {
        project_key,
        path,
        content: FileContent::FromClient { content, version },
        document_file_source: Some(language_hint),
        persist_node_cache: true,
    })?;

    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Handler for `textDocument/didChange` LSP notification
#[tracing::instrument(level = "debug", skip_all, fields(url = field::display(&params.text_document.uri.as_str()), version = params.text_document.version), err)]
pub(crate) async fn did_change(
    session: &Session,
    params: lsp_types::DidChangeTextDocumentParams,
) -> Result<(), LspError> {
    let url = params.text_document.uri;
    let version = params.text_document.version;

    let path = session.file_path(&url)?;
    let Some(doc) = session.document(&url) else {
        return Ok(());
    };
    if !session.workspace.file_exists(path.clone().into())? {
        return Ok(());
    }
    let features = FeaturesBuilder::new().build();
    if session.workspace.is_path_ignored(PathIsIgnoredParams {
        path: path.clone(),
        project_key: doc.project_key,
        features,
        ignore_kind: IgnoreKind::Ancestors,
    })? {
        return Ok(());
    }

    let old_text = session.workspace.get_file_content(GetFileContentParams {
        project_key: doc.project_key,
        path: path.clone(),
    })?;
    debug!("old document: {:?}", old_text);
    debug!("content changes: {:?}", params.content_changes);

    let text = apply_document_changes(
        session.position_encoding(),
        old_text,
        params.content_changes,
    );

    debug!("new document: {:?}", text);

    session.insert_document(url.clone(), Document::new(doc.project_key, version, &text));

    session.workspace.change_file(ChangeFileParams {
        project_key: doc.project_key,
        path,
        version,
        content: text,
    })?;

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Handler for `textDocument/didClose` LSP notification
#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) async fn did_close(
    session: &Session,
    params: lsp_types::DidCloseTextDocumentParams,
) -> Result<(), LspError> {
    let uri = params.text_document.uri;
    let path = session.file_path(&uri)?;
    let Some(project_key) = session.remove_document(&uri) else {
        debug!("Document wasn't open: {}", uri.as_str());
        return Ok(());
    };

    session
        .workspace
        .close_file(CloseFileParams { project_key, path })?;

    session
        .client
        .publish_diagnostics(uri, Vec::new(), None)
        .await;

    Ok(())
}
