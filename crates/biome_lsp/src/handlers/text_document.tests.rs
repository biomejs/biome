use crate::WorkspaceSettings;
use crate::server_test_utils::*;
use crate::session::ConfigurationStatus;
use anyhow::{Context, Result};
use biome_fs::{MemoryFileSystem, TemporaryFs};
use biome_service::Workspace;
use biome_service::workspace::{FormatFileParams, GetFileContentParams, GetModuleGraphParams};
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

#[tokio::test]
async fn first_document_uses_absolute_external_configuration_without_root_uri() -> Result<()> {
    first_document_uses_external_configuration_without_root_uri(false).await
}

#[tokio::test]
async fn first_document_uses_relative_external_configuration_without_root_uri() -> Result<()> {
    first_document_uses_external_configuration_without_root_uri(true).await
}

async fn first_document_uses_external_configuration_without_root_uri(relative: bool) -> Result<()> {
    let mut fs = TemporaryFs::new(if relative {
        "lsp_initial_relative_external_configuration_without_root_uri"
    } else {
        "lsp_initial_absolute_external_configuration_without_root_uri"
    });
    let workspace_path = fs.working_directory.join("project");
    fs.create_file(
        "project/biome.json",
        r#"{"javascript":{"formatter":{"quoteStyle":"double"}}}"#,
    );
    fs.create_file(
        "shared/biome.json",
        r#"{"javascript":{"formatter":{"quoteStyle":"double"}}}"#,
    );
    let external_config = fs.create_file(
        "shared/biome.jsonc",
        r#"{"javascript":{"formatter":{"quoteStyle":"single"}}}"#,
    );
    let document_path = fs.create_file("project/document.js", "const word = \"hello\";\n");
    fs.create_file("daemon/.keep", "");
    fs.append_to_working_directory("daemon");
    let factory = ServerFactory::new_with_fs(Arc::new(fs.create_os()));
    let configuration_path = if relative {
        "../shared/biome.jsonc".to_string()
    } else {
        external_config.to_string()
    };

    assert_first_document_configuration(
        factory,
        lsp::Uri::from_file_path(workspace_path).context("invalid workspace URI")?,
        lsp::Uri::from_file_path(document_path).context("invalid document URI")?,
        configuration_path,
    )
    .await
}

#[tokio::test]
async fn shared_external_configuration_keeps_workspace_projects_distinct() -> Result<()> {
    let mut fs = TemporaryFs::new("lsp_shared_external_configuration_workspace_projects");
    let mut documents = Vec::new();
    for name in ["project-one", "project-two"] {
        let root = fs.working_directory.join(name);
        let path = fs.create_file(&format!("{name}/document.js"), "statement();\n");
        documents.push((
            lsp::Uri::from_file_path(root).context("invalid workspace URI")?,
            lsp::Uri::from_file_path(path).context("invalid document URI")?,
        ));
    }
    let config = fs.create_file("shared/biome.jsonc", "{}");
    fs.create_file("daemon/.keep", "");
    fs.append_to_working_directory("daemon");
    let factory = ServerFactory::new_with_fs(Arc::new(fs.create_os()));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (stream, sink) = client.split();
    let (sender, _receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler_with_settings(
        stream,
        sink,
        sender,
        WorkspaceSettings {
            configuration_path: Some(config.to_string()),
            ..Default::default()
        },
    ));
    let folders: Vec<_> = documents
        .iter()
        .map(|(root, _)| json!({ "uri": root, "name": root.as_str() }))
        .collect();
    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": {},
                    "workspaceFolders": folders
                }))
                .finish(),
        )
        .await?;
    let mut server = Server::new(service);
    server.initialized().await?;

    let mut project_keys = Vec::new();
    for (root, uri) in documents {
        server
            .open_named_document("statement();\n", uri.clone(), "javascript")
            .await?;
        let document = session
            .document(&uri)
            .context("document was not registered in its workspace")?;
        let project_key = session
            .project_for_path(&to_utf8_file_path_buf(root))
            .context("workspace has no registered project")?;
        assert_eq!(document.project_key, project_key);
        assert_eq!(document.version, 0);
        assert!(!project_keys.contains(&project_key));
        project_keys.push(project_key);
        server
            .notify(
                "textDocument/didClose",
                json!({ "textDocument": { "uri": uri } }),
            )
            .await?;
    }
    reader.abort();
    Ok(())
}

#[tokio::test]
async fn initiating_document_preserves_changes_during_its_project_scan() -> Result<()> {
    let mut fs = TemporaryFs::new("lsp_initiating_document_project_scan");
    let config = fs.create_file(
        "shared/biome.jsonc",
        r#"{"javascript":{"formatter":{"quoteStyle":"single"}}}"#,
    );
    let root = fs.working_directory.join("project");
    let path = fs.create_file("project/document.js", "const word = \"disk\";\n");
    let uri = lsp::Uri::from_file_path(&path).context("invalid document URI")?;
    fs.create_file("daemon/.keep", "");
    fs.append_to_working_directory("daemon");
    let factory = ServerFactory::new_with_fs(Arc::new(fs.create_os()));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (mut stream, mut sink) = client.split();
    let (mut scan_started, mut scan_barrier) = channel(1);
    let (mut resume_scan, mut scan_resume) = channel(1);
    let (mut scan_finished, mut scan_completion) = channel(1);
    let reader = tokio::spawn(async move {
        let mut scans = 0;
        while let Some(request) = stream.next().await {
            if request.method() == "$/progress"
                && scans == 2
                && request
                    .params()
                    .is_some_and(|params| params["value"]["kind"] == "end")
            {
                scan_finished.send(()).await?;
            }
            let Some(id) = request.id().cloned() else {
                continue;
            };
            let response = match request.method() {
                "workspace/configuration" => Response::from_ok(
                    id,
                    json!([{ "configurationPath": config, "goToDefinition": true }]),
                ),
                "window/workDoneProgress/create" => {
                    scans += 1;
                    if scans == 2 {
                        scan_started.send(()).await?;
                        scan_resume
                            .next()
                            .await
                            .context("scan barrier was not released")?;
                    }
                    Response::from_ok(id, json!(null))
                }
                "client/registerCapability" | "client/unregisterCapability" => {
                    Response::from_ok(id, json!(null))
                }
                _ => Response::from_error(id, jsonrpc::Error::method_not_found()),
            };
            sink.send(response).await?;
        }
        Ok::<_, anyhow::Error>(())
    });
    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": { "window": { "workDoneProgress": true } },
                    "workspaceFolders": [{
                        "uri": lsp::Uri::from_file_path(&root).context("invalid workspace URI")?,
                        "name": "project"
                    }]
                }))
                .finish(),
        )
        .await?;
    tokio::time::timeout(
        Duration::from_secs(3),
        service.call(Request::build("initialized").params(json!({})).finish()),
    )
    .await??;
    assert!(session.has_initialized());
    assert!(session.project_for_path(&root).is_none());

    let mut opened = Box::pin(
        service.call(
            Request::build("textDocument/didOpen")
                .params(json!({ "textDocument": {
                "uri": uri, "languageId": "javascript", "version": 1,
                "text": "const word = \"original\";\n"
            } }))
                .finish(),
        ),
    );
    let open_result = futures::poll!(&mut opened);
    tokio::time::timeout(Duration::from_secs(3), scan_barrier.next())
        .await?
        .context("document did not initiate its project scan")?;
    let project_key = session
        .project_for_path(&root)
        .context("project was not registered")?;
    assert!(session.configuration_status(project_key).is_loaded());
    let updated = "const word = \"updated\";\n";
    service
        .call(
            Request::build("textDocument/didChange")
                .params(json!({
                    "textDocument": { "uri": uri, "version": 2 },
                    "contentChanges": [{ "text": updated }]
                }))
                .finish(),
        )
        .await?;
    resume_scan.send(()).await?;
    match open_result {
        Poll::Ready(result) => {
            result?;
        }
        Poll::Pending => {
            tokio::time::timeout(Duration::from_secs(3), opened).await??;
        }
    }
    tokio::time::timeout(Duration::from_secs(3), scan_completion.next())
        .await?
        .context("document project scan did not finish")?;
    reader.abort();

    let document = session.document(&uri).context("document was not opened")?;
    assert_eq!(document.project_key, project_key);
    assert_eq!(document.version, 2);
    assert_eq!(
        session.workspace().get_file_content(GetFileContentParams {
            project_key,
            path: path.clone().into(),
        })?,
        updated
    );
    assert_eq!(
        session
            .workspace()
            .format_file(FormatFileParams {
                project_key,
                path: path.clone().into(),
                inline_config: None,
            })?
            .into_code(),
        "const word = 'updated';\n"
    );
    assert!(
        session
            .workspace()
            .get_module_graph(GetModuleGraphParams {})?
            .data
            .contains_key(path.as_str())
    );
    Ok(())
}

#[tokio::test]
async fn removed_workspace_project_is_not_reloaded_and_can_be_added_again() -> Result<()> {
    let fs = MemoryFileSystem::default();
    for config in [
        uri!("project-one/biome.json"),
        uri!("project-two/biome.json"),
    ] {
        fs.insert(
            to_utf8_file_path_buf(config),
            r#"{"javascript":{"formatter":{"quoteStyle":"single"}}}"#,
        );
    }
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (stream, sink) = client.split();
    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let drain = tokio::spawn(async move { while receiver.next().await.is_some() {} });
    let reader = tokio::spawn(client_handler_with_settings(
        stream,
        sink,
        sender,
        WorkspaceSettings::default(),
    ));
    let removed = json!({ "uri": uri!("project-one"), "name": "one" });
    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": {},
                    "workspaceFolders": [removed, { "uri": uri!("project-two"), "name": "two" }]
                }))
                .finish(),
        )
        .await?;
    let mut server = Server::new(service);
    server.initialized().await?;
    let root = to_utf8_file_path_buf(uri!("project-one"));
    let old_key = session
        .project_for_path(&root)
        .context("missing first project")?;
    let retained_key = session
        .project_for_path(&to_utf8_file_path_buf(uri!("project-two")))
        .context("missing second project")?;
    assert!(session.configuration_status(old_key).is_loaded());
    server
        .notify(
            "workspace/didChangeWorkspaceFolders",
            json!({
                "event": { "added": [], "removed": [removed] }
            }),
        )
        .await?;
    server
        .load_configuration_with_settings(WorkspaceSettings::default())
        .await?;

    assert!(
        session.project_for_path(&root).is_none(),
        "closed project registration survived reload"
    );
    assert!(matches!(
        session.configuration_status(old_key),
        ConfigurationStatus::Missing
    ));
    assert_eq!(
        session.project_for_path(&to_utf8_file_path_buf(uri!("project-two"))),
        Some(retained_key)
    );
    assert!(session.configuration_status(retained_key).is_loaded());

    server
        .notify(
            "workspace/didChangeWorkspaceFolders",
            json!({
                "event": { "added": [removed], "removed": [] }
            }),
        )
        .await?;
    let new_key = session
        .project_for_path(&root)
        .context("re-added project was not registered")?;
    assert_ne!(new_key, old_key);
    assert!(session.configuration_status(new_key).is_loaded());
    assert!(matches!(
        session.configuration_status(old_key),
        ConfigurationStatus::Missing
    ));
    let uri = uri!("project-one/document.js");
    server
        .open_named_document("const word = \"hello\";\n", uri.clone(), "javascript")
        .await?;
    assert_eq!(
        session
            .document(&uri)
            .context("re-added project did not open document")?
            .project_key,
        new_key
    );
    assert_eq!(
        session
            .workspace()
            .format_file(FormatFileParams {
                project_key: new_key,
                path: to_utf8_file_path_buf(uri).into(),
                inline_config: None,
            })?
            .into_code(),
        "const word = 'hello';\n"
    );
    reader.abort();
    drain.abort();
    Ok(())
}

#[tokio::test]
async fn absolute_configuration_path_changes_update_open_projects() -> Result<()> {
    assert_configuration_path_changes_update_open_projects(false, true).await
}

#[tokio::test]
async fn relative_configuration_path_changes_update_open_projects() -> Result<()> {
    assert_configuration_path_changes_update_open_projects(true, true).await
}

#[tokio::test]
async fn absolute_configuration_path_changes_update_unadvertised_projects() -> Result<()> {
    assert_configuration_path_changes_update_open_projects(false, false).await
}

async fn assert_configuration_path_changes_update_open_projects(
    relative: bool,
    advertise_workspace_folders: bool,
) -> Result<()> {
    let mut fs = TemporaryFs::new(if !advertise_workspace_folders {
        "lsp_absolute_configuration_path_changes_update_unadvertised_projects"
    } else if relative {
        "lsp_relative_configuration_path_changes_update_open_projects"
    } else {
        "lsp_absolute_configuration_path_changes_update_open_projects"
    });
    let double_config = fs.create_file(
        "shared/double.jsonc",
        r#"{"javascript":{"formatter":{"quoteStyle":"double"}}}"#,
    );
    let single_config = fs.create_file(
        "shared/single.jsonc",
        r#"{"javascript":{"formatter":{"quoteStyle":"single"}}}"#,
    );
    let invalid_config = fs.create_file("shared/invalid.jsonc", "{");
    let mut documents = Vec::new();
    for name in ["project-one", "project-two"] {
        let root = fs.working_directory.join(name);
        let path = fs.create_file(&format!("{name}/document.js"), "const word = \"disk\";\n");
        fs.create_file(
            &format!("{name}/biome.json"),
            r#"{"javascript":{"formatter":{"quoteStyle":"double"}}}"#,
        );
        documents.push((
            lsp::Uri::from_file_path(root).context("invalid workspace URI")?,
            lsp::Uri::from_file_path(path).context("invalid document URI")?,
        ));
    }
    fs.create_file("daemon/.keep", "");
    fs.append_to_working_directory("daemon");
    let factory = ServerFactory::new_with_fs(Arc::new(fs.create_os()));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (stream, sink) = client.split();
    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let drain = tokio::spawn(async move { while receiver.next().await.is_some() {} });
    let reader = tokio::spawn(client_handler_with_settings(
        stream,
        sink,
        sender,
        WorkspaceSettings {
            configuration_path: Some(if relative {
                "../shared/double.jsonc".to_string()
            } else {
                double_config.to_string()
            }),
            ..Default::default()
        },
    ));
    let folders: Vec<_> = documents
        .iter()
        .map(|(root, _)| json!({ "uri": root, "name": root.as_str() }))
        .collect();
    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(if advertise_workspace_folders {
                    json!({ "capabilities": {}, "workspaceFolders": folders })
                } else {
                    json!({ "capabilities": {} })
                })
                .finish(),
        )
        .await?;
    let mut server = Server::new(service);
    server.initialized().await?;

    let source = "const word = \"updated\";\n";
    let mut open_documents = Vec::new();
    for (root, uri) in documents {
        server
            .open_named_document("const word = \"original\";\n", uri.clone(), "javascript")
            .await?;
        server
            .notify(
                "textDocument/didChange",
                json!({
                    "textDocument": { "uri": uri, "version": 2 },
                    "contentChanges": [{ "text": source }]
                }),
            )
            .await?;
        let project_key = session
            .project_for_path(&to_utf8_file_path_buf(root))
            .context("workspace has no registered project")?;
        let document = session.document(&uri).context("document was not opened")?;
        anyhow::ensure!(
            document.project_key == project_key,
            "initial document project mismatch"
        );
        anyhow::ensure!(
            session.configuration_status(project_key).is_loaded(),
            "initial configuration was not loaded for {uri:?}"
        );
        anyhow::ensure!(
            open_documents.iter().all(|(_, key)| *key != project_key),
            "workspace projects are not distinct"
        );
        open_documents.push((uri, project_key));
    }

    let configuration_path = |config_path: &camino::Utf8Path| {
        if relative {
            format!("../shared/{}", config_path.file_name().unwrap())
        } else {
            config_path.to_string()
        }
    };
    let invalid_path = configuration_path(&invalid_config);
    let single_path = configuration_path(&single_config);
    let double_path = configuration_path(&double_config);
    for (configuration_path, expected) in [
        (Some(invalid_path.clone()), None),
        (None, Some("const word = \"updated\";\n")),
        (Some(single_path.clone()), Some("const word = 'updated';\n")),
        (Some(String::new()), Some("const word = \"updated\";\n")),
        (Some(invalid_path), None),
        (Some(String::new()), Some("const word = \"updated\";\n")),
        (Some(single_path), Some("const word = 'updated';\n")),
        (None, Some("const word = \"updated\";\n")),
        (Some(double_path), Some("const word = \"updated\";\n")),
    ] {
        server
            .load_configuration_with_settings(WorkspaceSettings {
                configuration_path,
                ..Default::default()
            })
            .await?;

        for (uri, project_key) in &open_documents {
            let document = session
                .document(uri)
                .context("document was closed during reload")?;
            anyhow::ensure!(
                document.project_key == *project_key,
                "reload changed project for {uri:?}"
            );
            anyhow::ensure!(
                document.version == 2,
                "reload changed version for {uri:?}: {}",
                document.version
            );
            let content = session.workspace().get_file_content(GetFileContentParams {
                project_key: *project_key,
                path: to_utf8_file_path_buf(uri.clone()).into(),
            })?;
            anyhow::ensure!(
                content == source,
                "reload changed content for {uri:?}: {content:?}"
            );
            let status = session.configuration_status(*project_key);
            if let Some(expected) = expected {
                anyhow::ensure!(
                    status.is_loaded(),
                    "expected loaded project status for {uri:?}, got {status:?}"
                );
                let edits: Vec<TextEdit> = server
                    .request(
                        "textDocument/formatting",
                        "formatting",
                        json!({
                            "textDocument": { "uri": uri },
                            "options": { "tabSize": 2, "insertSpaces": true }
                        }),
                    )
                    .await?
                    .context("missing formatting response")?;
                let mut formatted = source.to_string();
                for edit in edits.into_iter().rev() {
                    assert_eq!(edit.range.start.line, 0);
                    assert_eq!(edit.range.end.line, 0);
                    formatted.replace_range(
                        edit.range.start.character as usize..edit.range.end.character as usize,
                        &edit.new_text,
                    );
                }
                anyhow::ensure!(
                    formatted == expected,
                    "configuration path change did not update {uri:?}: expected {expected:?}, got {formatted:?}"
                );
            } else {
                anyhow::ensure!(
                    status.is_error(),
                    "expected error project status for {uri:?}, got {status:?}"
                );
            }
        }
    }
    reader.abort();
    drain.abort();
    Ok(())
}

#[tokio::test]
async fn document_changes_are_preserved_during_second_project_reload_scan() -> Result<()> {
    let fs = MemoryFileSystem::default();
    for (initial, updated) in [
        (
            uri!("project-one/initial.jsonc"),
            uri!("project-one/updated.jsonc"),
        ),
        (
            uri!("project-two/initial.jsonc"),
            uri!("project-two/updated.jsonc"),
        ),
    ] {
        fs.insert(
            to_utf8_file_path_buf(initial),
            r#"{"javascript":{"formatter":{"quoteStyle":"double"}}}"#,
        );
        fs.insert(
            to_utf8_file_path_buf(updated),
            r#"{"javascript":{"formatter":{"quoteStyle":"single"}}}"#,
        );
    }
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (mut stream, mut sink) = client.split();
    let (mut scan_started, mut scan_barrier) = channel(1);
    let (mut resume_scan, mut scan_resume) = channel(1);
    let reader = tokio::spawn(async move {
        let mut scans = 0;
        while let Some(request) = stream.next().await {
            let Some(id) = request.id().cloned() else {
                continue;
            };
            let response = match request.method() {
                "workspace/configuration" => {
                    Response::from_ok(id, json!([{ "configurationPath": "initial.jsonc" }]))
                }
                "window/workDoneProgress/create" => {
                    scans += 1;
                    if scans == 4 {
                        scan_started.send(()).await?;
                        scan_resume
                            .next()
                            .await
                            .context("scan barrier was not released")?;
                    }
                    Response::from_ok(id, json!(null))
                }
                "client/registerCapability" | "client/unregisterCapability" => {
                    Response::from_ok(id, json!(null))
                }
                _ => Response::from_error(id, jsonrpc::Error::method_not_found()),
            };
            sink.send(response).await?;
        }
        Ok::<_, anyhow::Error>(())
    });
    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": { "window": { "workDoneProgress": true } },
                    "workspaceFolders": [
                        { "uri": uri!("project-one"), "name": "one" },
                        { "uri": uri!("project-two"), "name": "two" }
                    ]
                }))
                .finish(),
        )
        .await?;
    tokio::time::timeout(
        Duration::from_secs(3),
        service.call(Request::build("initialized").params(json!({})).finish()),
    )
    .await??;
    for uri in [
        uri!("project-one/existing.js"),
        uri!("project-two/existing.js"),
    ] {
        tokio::time::timeout(
            Duration::from_secs(3),
            service.call(
                Request::build("textDocument/didOpen")
                    .params(json!({
                        "textDocument": {
                            "uri": uri,
                            "languageId": "javascript",
                            "version": 1,
                            "text": "const word = \"existing\";\n"
                        }
                    }))
                    .finish(),
            ),
        )
        .await??;
    }
    let project_key = session
        .project_for_path(&to_utf8_file_path_buf(uri!("project-two")))
        .context("second project was not registered")?;
    let reload = tokio::spawn(
        service.call(
            Request::build("workspace/didChangeConfiguration")
                .params(json!({ "settings": { "configurationPath": "updated.jsonc" } }))
                .finish(),
        ),
    );
    tokio::time::timeout(Duration::from_secs(3), scan_barrier.next())
        .await?
        .context("second project reload did not reach its scan barrier")?;
    let formatted = session
        .workspace()
        .format_file(FormatFileParams {
            project_key,
            path: to_utf8_file_path_buf(uri!("project-two/existing.js")).into(),
            inline_config: None,
        })?
        .into_code();
    anyhow::ensure!(
        formatted == "const word = 'existing';\n",
        "configuration was not applied before the scan barrier: {formatted:?}"
    );

    let uri = uri!("project-two/document.js");
    let mut opened = Box::pin(
        service.call(
            Request::build("textDocument/didOpen")
                .params(json!({
                    "textDocument": {
                        "uri": uri,
                        "languageId": "javascript",
                        "version": 1,
                        "text": "const word = \"original\";\n"
                    }
                }))
                .finish(),
        ),
    );
    let open_result = futures::poll!(&mut opened);
    let updated = "const word = \"updated\";\n";
    service
        .call(
            Request::build("textDocument/didChange")
                .params(json!({
                    "textDocument": { "uri": uri, "version": 2 },
                    "contentChanges": [{ "text": updated }]
                }))
                .finish(),
        )
        .await?;
    resume_scan.send(()).await?;
    tokio::time::timeout(Duration::from_secs(3), reload).await???;
    match open_result {
        Poll::Ready(result) => {
            result?;
        }
        Poll::Pending => {
            tokio::time::timeout(Duration::from_secs(3), opened).await??;
        }
    }
    reader.abort();
    let document = session.document(&uri).context("document was not opened")?;
    anyhow::ensure!(
        document.project_key == project_key,
        "document changed project"
    );
    anyhow::ensure!(
        document.version == 2,
        "expected version 2, got {}",
        document.version
    );
    let content = session.workspace().get_file_content(GetFileContentParams {
        project_key,
        path: to_utf8_file_path_buf(uri.clone()).into(),
    })?;
    anyhow::ensure!(content == updated, "edit was not preserved: {content:?}");
    let formatted = session
        .workspace()
        .format_file(FormatFileParams {
            project_key,
            path: to_utf8_file_path_buf(uri).into(),
            inline_config: None,
        })?
        .into_code();
    anyhow::ensure!(
        formatted == "const word = 'updated';\n",
        "wrong configuration: {formatted:?}"
    );
    Ok(())
}

#[tokio::test]
async fn document_changes_are_preserved_with_cached_parent_workspace_configuration() -> Result<()> {
    assert_document_changes_with_cached_parent_project(
        "../biome.jsonc",
        uri!("project/biome.jsonc"),
    )
    .await
}

#[tokio::test]
async fn document_changes_are_preserved_with_cached_external_parent_configuration() -> Result<()> {
    assert_document_changes_with_cached_parent_project(
        "../../shared/biome.jsonc",
        uri!("shared/biome.jsonc"),
    )
    .await
}

async fn assert_document_changes_with_cached_parent_project(
    configuration_path: &str,
    config_uri: lsp::Uri,
) -> Result<()> {
    let fs = MemoryFileSystem::default();
    fs.insert(to_utf8_file_path_buf(config_uri), "{}");
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (stream, sink) = client.split();
    let (sender, _receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler_with_settings(
        stream,
        sink,
        sender,
        WorkspaceSettings {
            configuration_path: Some(configuration_path.to_string()),
            ..Default::default()
        },
    ));
    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": {},
                    "rootUri": uri!("project"),
                    "workspaceFolders": [
                        { "uri": uri!("project/nested"), "name": "nested" },
                        { "uri": uri!("project"), "name": "parent" }
                    ]
                }))
                .finish(),
        )
        .await?;
    tokio::time::timeout(
        Duration::from_secs(3),
        service.call(Request::build("initialized").params(json!({})).finish()),
    )
    .await??;
    assert!(session.has_initialized());
    let project_key = session
        .project_for_path(&to_utf8_file_path_buf(uri!("project")))
        .context("parent workspace has no project")?;

    let uri = uri!("project/nested/document.js");
    let mut opened = Box::pin(
        service.call(
            Request::build("textDocument/didOpen")
                .params(json!({
                    "textDocument": {
                        "uri": uri,
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
                    "textDocument": { "uri": uri, "version": 2 },
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
            tokio::time::timeout(Duration::from_secs(3), opened).await??;
        }
    }
    reader.abort();
    let document = session.document(&uri).context("document was not opened")?;
    assert_eq!(document.project_key, project_key);
    assert_eq!(document.version, 2);
    assert_eq!(
        session.workspace().get_file_content(GetFileContentParams {
            project_key,
            path: to_utf8_file_path_buf(uri).into(),
        })?,
        updated
    );
    Ok(())
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
    assert_document_changes_during_initial_project_scan(
        Ok(json!([{
            "configurationPath": to_utf8_file_path_buf(uri!("biome.jsonc"))
        }])),
        false,
    )
    .await
}

#[tokio::test]
async fn document_changes_are_preserved_after_initial_settings_request_fails() -> Result<()> {
    assert_document_changes_during_initial_project_scan(
        Err(jsonrpc::Error::internal_error()),
        false,
    )
    .await
}

#[tokio::test]
async fn project_scan_preserves_unsaved_document_content_and_parse() -> Result<()> {
    assert_document_changes_during_initial_project_scan(
        Ok(json!([{
            "configurationPath": "biome.jsonc",
            "goToDefinition": true
        }])),
        true,
    )
    .await
}

async fn assert_document_changes_during_initial_project_scan(
    initial_configuration: jsonrpc::Result<Value>,
    project_scan: bool,
) -> Result<()> {
    let mut disk_fs =
        project_scan.then(|| TemporaryFs::new("lsp_project_scan_preserves_unsaved_document"));
    let (factory, workspace_uri, document_uri) = if let Some(disk_fs) = disk_fs.as_mut() {
        disk_fs.create_file("biome.jsonc", "{}");
        let document_path = disk_fs.create_file("document.js", "const word = 'disk';\n");
        (
            ServerFactory::new_with_fs(Arc::new(disk_fs.create_os())),
            lsp::Uri::from_file_path(&disk_fs.working_directory)
                .context("invalid workspace URI")?,
            lsp::Uri::from_file_path(document_path).context("invalid document URI")?,
        )
    } else {
        let fs = MemoryFileSystem::default();
        fs.insert(to_utf8_file_path_buf(uri!("biome.jsonc")), "{}");
        (
            ServerFactory::new_with_fs(Arc::new(fs)),
            uri!(""),
            uri!("document.js"),
        )
    };
    let (mut service, client) = factory.create().into_inner();
    let session = service.inner().session.clone();
    let (mut stream, mut sink) = client.split();

    service
        .call(
            Request::build("initialize")
                .id("initialize")
                .params(json!({
                    "capabilities": { "window": { "workDoneProgress": true } },
                    "workspaceFolders": [{ "uri": workspace_uri, "name": "workspace" }]
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
        .project_for_path(&to_utf8_file_path_buf(document_uri.clone()))
        .context("initial project has not been registered")?;

    let mut opened = Box::pin(
        service.call(
            Request::build("textDocument/didOpen")
                .params(json!({
                    "textDocument": {
                        "uri": document_uri,
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
                    "textDocument": { "uri": document_uri, "version": 2 },
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
        .document(&document_uri)
        .context("document was not opened")?;
    assert_eq!(document.project_key, project_key);
    assert_eq!(document.version, 2);
    assert_eq!(
        session.workspace().get_file_content(GetFileContentParams {
            project_key,
            path: to_utf8_file_path_buf(document_uri.clone()).into(),
        })?,
        updated
    );
    if project_scan {
        let path = to_utf8_file_path_buf(document_uri);
        let formatted = session
            .workspace()
            .format_file(FormatFileParams {
                project_key,
                path: path.clone().into(),
                inline_config: None,
            })?
            .into_code();
        assert_eq!(formatted, "const word = \"updated\";\n");
        let graph = session
            .workspace()
            .get_module_graph(GetModuleGraphParams {})?;
        assert!(
            graph.data.contains_key(path.as_str()),
            "client-owned file was not indexed"
        );
    }
    Ok(())
}
