use crate::diagnostics::LspError;
use crate::session::ConfigurationStatus;
use crate::utils::apply_document_changes;
use crate::{documents::Document, session::Session};
use biome_configuration::ConfigurationPathHint;
use biome_service::workspace::{
    ChangeFileParams, CloseFileParams, DocumentFileSource, FeaturesBuilder, FileContent,
    GetFileContentParams, IgnoreKind, OpenFileParams, PathIsIgnoredParams, ProjectKey,
};
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::Arc;
use tower_lsp_server::ls_types as lsp;
use tracing::{debug, error, field, info, trace};

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
    params: lsp::DidOpenTextDocumentParams,
) -> Result<(), LspError> {
    let url = params.text_document.uri;
    let version = params.text_document.version;
    let content = params.text_document.text;
    let language_hint = DocumentFileSource::from_language_id(&params.text_document.language_id);

    let path = session.file_path(&url)?;
    let file_path = path.to_path_buf();
    let config_path = session.resolve_configuration_path(Some(&file_path));

    let Some(project_key) =
        ensure_project_for_opened_document(session, &path, config_path.as_ref()).await
    else {
        return Ok(());
    };

    let is_ignored = session
        .workspace
        .is_path_ignored(PathIsIgnoredParams {
            project_key,
            path: path.clone(),
            is_dir: false,
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
        inline_config: session.inline_config(),
    })?;

    session.insert_document(url.clone(), doc);

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Ensure the file is associated with a project and the correct configuration
/// is loaded before opening the document.
async fn ensure_project_for_opened_document(
    session: &Arc<Session>,
    path: &Utf8Path,
    config_path: Option<&ConfigurationPathHint>,
) -> Option<ProjectKey> {
    let is_relative_config_path = session
        .get_settings_configuration_path()
        .is_some_and(|config_path| !config_path.is_absolute());

    let mut load_status = None;

    if let Some(project_key) = session.project_for_path(path) {
        if is_relative_config_path {
            // Absolute configurationPath is global; only relative needs per-file resolution.
            // Use the per-file resolved configurationPath so each workspace folder
            // uses its own config, even when a project is already open.
            if let Some(resolved_path) = config_path {
                session.set_configuration_status(ConfigurationStatus::Loading);
                let status = session
                    .load_biome_configuration_file(resolved_path.clone(), false)
                    .await;
                load_status = Some(status);
            }
        }
        if load_status.is_none() {
            return Some(project_key);
        }
    } else {
        info!("No open project for path: {path:?}. Opening new project.");
    }

    if load_status.is_none() {
        session.set_configuration_status(ConfigurationStatus::Loading);
        if !session.has_initialized() {
            session.load_extension_settings(None).await;
        }
        load_status = Some(load_from_workspace_root_for_path(session, path, config_path).await);
    }
    let status = load_status.expect("load_status should be set");

    session.set_configuration_status(status);

    if status.is_loaded() {
        session.project_for_path(path).or_else(|| {
            error!("Could not find project for {path}");
            None
        })
    } else {
        error!("Configuration could not be loaded for {path}");
        None
    }
}

/// Load configuration anchored to the workspace root that contains `path`.
async fn load_from_workspace_root_for_path(
    session: &Arc<Session>,
    path: &Utf8Path,
    config_path: Option<&ConfigurationPathHint>,
) -> ConfigurationStatus {
    if let Some(path) = config_path {
        info!("Loading user configuration from text_document {:?}", &path);
        return session
            .load_biome_configuration_file(path.clone(), false)
            .await;
    }

    let project_path = path
        .parent()
        .map(|parent| parent.to_path_buf())
        .unwrap_or_default();
    info!("Loading configuration from text_document {}", &project_path);
    let project_path = resolve_workspace_base_path(session, &project_path);

    session
        .load_biome_configuration_file(ConfigurationPathHint::FromLsp(project_path), false)
        .await
}

fn resolve_workspace_base_path(session: &Session, project_path: &Utf8Path) -> Utf8PathBuf {
    let workspace_base = || {
        let workspace_folders = session.get_workspace_folders()?;
        workspace_folders
            .iter()
            .filter_map(|folder| {
                folder.uri.to_file_path().map(|p| {
                    Utf8PathBuf::from_path_buf(p.to_path_buf()).expect("To have a valid UTF-8 path")
                })
            })
            .filter(|ws| project_path.starts_with(ws))
            .max_by_key(|ws| ws.as_str().len())
    };
    workspace_base()
        .or_else(|| {
            session
                .base_path()
                .and_then(|base_path| project_path.starts_with(&base_path).then_some(base_path))
        })
        .unwrap_or_else(|| project_path.to_path_buf())
}

/// Handler for `textDocument/didChange` LSP notification
#[tracing::instrument(level = "debug", skip_all, fields(url = field::display(&params.text_document.uri.as_str()), version = params.text_document.version), err)]
pub(crate) async fn did_change(
    session: &Session,
    params: lsp::DidChangeTextDocumentParams,
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
        is_dir: false,
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

    trace!("content changes: {:?}", params.content_changes);

    let text = apply_document_changes(
        session.position_encoding(),
        old_text,
        params.content_changes,
    );

    session.insert_document(url.clone(), Document::new(doc.project_key, version, &text));

    session.workspace.change_file(ChangeFileParams {
        project_key: doc.project_key,
        path,
        version,
        content: text,
        inline_config: session.inline_config(),
    })?;

    if let Err(err) = session.update_diagnostics(url).await {
        error!("Failed to update diagnostics: {}", err);
    }

    Ok(())
}

/// Handler for `textDocument/didSave` LSP notification
#[tracing::instrument(level = "debug", skip_all, fields(url = field::display(&params.text_document.uri.as_str())), err)]
pub(crate) async fn did_save(
    session: &Session,
    params: lsp::DidSaveTextDocumentParams,
) -> Result<(), LspError> {
    let url = params.text_document.uri;

    // If text is provided in the notification (as per LSP spec), update the file
    if let Some(text) = params.text {
        let path = session.file_path(&url)?;
        let Some(doc) = session.document(&url) else {
            debug!("Document wasn't open: {}", url.as_str());
            return Ok(());
        };

        session.workspace.change_file(ChangeFileParams {
            project_key: doc.project_key,
            path,
            content: text.clone(),
            version: doc.version,
            inline_config: None,
        })?;

        session.insert_document(
            url.clone(),
            Document::new(doc.project_key, doc.version, &text),
        );

        // Update diagnostics with fresh content
        if let Err(err) = session.update_diagnostics(url).await {
            error!("Failed to update diagnostics after save: {}", err);
        }
    }

    Ok(())
}

/// Handler for `textDocument/didClose` LSP notification
#[tracing::instrument(level = "debug", skip(session), err)]
pub(crate) async fn did_close(
    session: &Session,
    params: lsp::DidCloseTextDocumentParams,
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
