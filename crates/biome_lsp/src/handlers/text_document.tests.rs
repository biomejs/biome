use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::MemoryFileSystem;
use biome_service::Workspace;
use biome_service::workspace::GetFileContentParams;
use futures::channel::mpsc::channel;
use futures::{SinkExt, StreamExt};
use serde_json::{Value, from_value, json};
use std::str::FromStr;
use std::sync::Arc;
use std::task::Poll;
use std::time::Duration;
use tower::Service;
use tower_lsp_server::jsonrpc::{self, Request, Response};
use tower_lsp_server::ls_types::{self as lsp, TextEdit};

#[tokio::test]
async fn first_document_uses_absolute_configuration_path_during_initialization() -> Result<()> {
    first_document_uses_configuration_path_during_initialization(
        to_utf8_file_path_buf(uri!("biome.jsonc")).to_string(),
    )
    .await
}

#[tokio::test]
async fn first_document_uses_relative_configuration_path_during_initialization() -> Result<()> {
    first_document_uses_configuration_path_during_initialization("biome.jsonc".into()).await
}

async fn first_document_uses_configuration_path_during_initialization(
    configuration_path: String,
) -> Result<()> {
    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{"javascript":{"formatter":{"quoteStyle":"double"}}}"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.jsonc")),
        r#"{"javascript":{"formatter":{"quoteStyle":"single"}}}"#,
    );
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    assert_first_document_configuration(factory, uri!(""), uri!("document.js"), configuration_path)
        .await
}

async fn assert_first_document_configuration(
    factory: ServerFactory,
    workspace_uri: lsp::Uri,
    document_uri: lsp::Uri,
    configuration_path: String,
) -> Result<()> {
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (mut stream, mut sink) = client.split();
    let source = "const word = \"hello\";\n";

    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": {},
                    "workspaceFolders": [{ "uri": workspace_uri, "name": "workspace" }]
                }))
                .finish(),
        )
        .await?;

    let mut initialized =
        tokio::spawn(service.call(Request::build("initialized").params(json!({})).finish()));
    let initialization_request = tokio::time::timeout(Duration::from_secs(3), stream.next())
        .await
        .context("initial configuration request timed out")?
        .context("missing initialization configuration request")?;
    assert_eq!(initialization_request.method(), "workspace/configuration");

    let opened = tokio::spawn(
        service.call(
            Request::build("textDocument/didOpen")
                .params(json!({
                    "textDocument": {
                        "uri": document_uri,
                        "languageId": "javascript",
                        "version": 1,
                        "text": source
                    }
                }))
                .finish(),
        ),
    );
    let open_request = tokio::time::timeout(Duration::from_secs(3), stream.next())
        .await
        .context("document configuration request timed out")?
        .context("missing document configuration request")?;
    assert_eq!(open_request.method(), "workspace/configuration");

    let settings = json!([{ "configurationPath": configuration_path }]);
    sink.send(Response::from_ok(
        initialization_request.id().unwrap().clone(),
        settings.clone(),
    ))
    .await?;
    tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            tokio::select! {
                result = &mut initialized => {
                    result??;
                    return Ok::<_, anyhow::Error>(());
                }
                request = stream.next() => {
                    let request = request.context("client stream ended during initialization")?;
                    if let Some(id) = request.id() {
                        assert!(matches!(request.method(), "client/registerCapability" | "client/unregisterCapability"));
                        sink.send(Response::from_ok(id.clone(), json!(null))).await?;
                    }
                }
            }
        }
    })
    .await
    .context("initialization timed out")??;

    sink.send(Response::from_ok(
        open_request.id().unwrap().clone(),
        settings,
    ))
    .await?;

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));
    tokio::time::timeout(Duration::from_secs(3), opened)
        .await
        .context("opening document timed out")???;
    let document = session
        .document(&document_uri)
        .context("first document was not registered in its workspace")?;
    let project_key = session
        .project_for_path(&to_utf8_file_path_buf(workspace_uri))
        .context("document workspace has no registered project")?;
    assert_eq!(document.project_key, project_key);
    wait_for_notification(&mut receiver, |notification| {
        matches!(notification, ServerNotification::PublishDiagnostics(params)
            if params.uri == document_uri && params.version == Some(1))
    })
    .await
    .context("missing document diagnostics")?;

    let response = service
        .call(
            Request::build("textDocument/formatting")
                .id("formatting")
                .params(json!({
                    "textDocument": { "uri": document_uri },
                    "options": { "tabSize": 2, "insertSpaces": true }
                }))
                .finish(),
        )
        .await?
        .context("missing formatting response")?;
    let (_, result) = response.into_parts();
    let edits: Vec<TextEdit> = from_value(result?)?;
    let mut formatted = source.to_string();
    for edit in edits.into_iter().rev() {
        assert_eq!(edit.range.start.line, 0);
        assert_eq!(edit.range.end.line, 0);
        formatted.replace_range(
            edit.range.start.character as usize..edit.range.end.character as usize,
            &edit.new_text,
        );
    }
    reader.abort();
    assert_eq!(formatted, "const word = 'hello';\n");

    Ok(())
}

#[tokio::test]
async fn document_changes_are_preserved_while_initial_project_scan_is_pending() -> Result<()> {
    assert_document_changes_during_initial_project_scan(Ok(json!([{
        "configurationPath": to_utf8_file_path_buf(uri!("biome.jsonc"))
    }])))
    .await
}

#[tokio::test]
async fn document_changes_are_preserved_after_initial_settings_request_fails() -> Result<()> {
    assert_document_changes_during_initial_project_scan(Err(jsonrpc::Error::internal_error())).await
}

async fn assert_document_changes_during_initial_project_scan(
    initial_configuration: jsonrpc::Result<Value>,
) -> Result<()> {
    let fs = MemoryFileSystem::default();
    fs.insert(to_utf8_file_path_buf(uri!("biome.jsonc")), "{}");
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (mut stream, mut sink) = client.split();

    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": { "window": { "workDoneProgress": true } },
                    "workspaceFolders": [{ "uri": uri!(""), "name": "workspace" }]
                }))
                .finish(),
        )
        .await?;
    let initialized =
        tokio::spawn(service.call(Request::build("initialized").params(json!({})).finish()));
    let configuration = tokio::time::timeout(Duration::from_secs(3), stream.next())
        .await?
        .context("missing initialization configuration request")?;
    assert_eq!(configuration.method(), "workspace/configuration");
    let settings = match &initial_configuration {
        Ok(settings) => settings.clone(),
        Err(_) => json!([{}]),
    };
    let id = configuration.id().unwrap().clone();
    sink.send(match initial_configuration {
        Ok(settings) => Response::from_ok(id, settings),
        Err(error) => Response::from_error(id, error),
    })
    .await?;
    let progress = tokio::time::timeout(Duration::from_secs(3), stream.next())
        .await?
        .context("missing project scan progress request")?;
    assert_eq!(progress.method(), "window/workDoneProgress/create");
    assert!(!session.has_initialized());
    let project_key = session
        .project_for_path(&to_utf8_file_path_buf(uri!("document.js")))
        .context("initial project has not been registered")?;

    let mut opened = Box::pin(
        service.call(
            Request::build("textDocument/didOpen")
                .params(json!({
                    "textDocument": {
                        "uri": uri!("document.js"),
                        "languageId": "javascript",
                        "version": 1,
                        "text": "const word = 'original';\n"
                    }
                }))
                .finish(),
        ),
    );
    let open_result = futures::poll!(&mut opened);
    let updated = "const word = 'updated';\n";
    service
        .call(
            Request::build("textDocument/didChange")
                .params(json!({
                    "textDocument": { "uri": uri!("document.js"), "version": 2 },
                    "contentChanges": [{ "text": updated }]
                }))
                .finish(),
        )
        .await?;

    match open_result {
        Poll::Ready(result) => {
            result?;
        }
        Poll::Pending => {
            let configuration = tokio::time::timeout(Duration::from_secs(3), stream.next())
                .await?
                .context("missing document configuration request")?;
            assert_eq!(configuration.method(), "workspace/configuration");
            sink.send(Response::from_ok(
                configuration.id().unwrap().clone(),
                settings,
            ))
            .await?;
            tokio::time::timeout(Duration::from_secs(3), opened).await??;
        }
    }

    sink.send(Response::from_ok(
        progress.id().unwrap().clone(),
        json!(null),
    ))
    .await?;
    let (sender, _receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));
    tokio::time::timeout(Duration::from_secs(3), initialized).await???;
    reader.abort();

    let document = session
        .document(&uri!("document.js"))
        .context("document was not opened")?;
    assert_eq!(document.project_key, project_key);
    assert_eq!(document.version, 2);
    assert_eq!(
        session.workspace().get_file_content(GetFileContentParams {
            project_key,
            path: to_utf8_file_path_buf(uri!("document.js")).into(),
        })?,
        updated
    );
    Ok(())
}
