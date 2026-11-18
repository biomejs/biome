use crate::WorkspaceSettings;
use crate::server::ServerFactory;
use crate::server_test_utils::{Server, to_utf8_file_path_buf, uri};
use anyhow::{Context, Result};
use biome_fs::MemoryFileSystem;
use futures::{SinkExt, StreamExt};
use serde_json::json;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tower_lsp_server::jsonrpc::Response;
use tower_lsp_server::ls_types as lsp;

#[tokio::test]
async fn force_ignored_watched_files_do_not_rescan() -> Result<()> {
    let fs = MemoryFileSystem::default();
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "files": { "includes": ["src/**", "!!**/ignored", "!!**/node_modules"] },
            "linter": { "rules": { "suspicious": { "noImportCycles": "error" } } }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("src/main.js")),
        "export const a = 1;",
    );
    let fs = Arc::new(fs);
    let TestClient {
        mut server,
        scans,
        client_settings,
        reader,
    } = start_server(fs.clone(), None).await?;
    assert_eq!(scans.load(Ordering::SeqCst), 1);

    for directory in ["ignored/worktree", "node_modules/dependency"] {
        for filename in ["biome.json", ".biome.jsonc", ".gitignore", ".ignore"] {
            for typ in [
                lsp::FileChangeType::CREATED,
                lsp::FileChangeType::CHANGED,
                lsp::FileChangeType::DELETED,
            ] {
                let uri =
                    lsp::Uri::from_str(&format!("{}{directory}/{filename}", uri!("").as_str()))?;
                server
                    .notify(
                        "workspace/didChangeWatchedFiles",
                        lsp::DidChangeWatchedFilesParams {
                            changes: vec![lsp::FileEvent { uri, typ }],
                        },
                    )
                    .await?;
                assert_eq!(
                    scans.load(Ordering::SeqCst),
                    1,
                    "{directory}/{filename}: {typ:?}"
                );
            }
        }
    }

    for (index, uri) in [
        uri!("biome.json"),
        uri!("src/.gitignore"),
        uri!("config/biome.json"),
        uri!(".editorconfig"),
        uri!("pnpm-workspace.yaml"),
    ]
    .into_iter()
    .enumerate()
    {
        server
            .notify(
                "workspace/didChangeWatchedFiles",
                lsp::DidChangeWatchedFilesParams {
                    changes: vec![
                        lsp::FileEvent {
                            uri: uri!("ignored/worktree/biome.json"),
                            typ: lsp::FileChangeType::CHANGED,
                        },
                        lsp::FileEvent {
                            uri,
                            typ: lsp::FileChangeType::CHANGED,
                        },
                    ],
                },
            )
            .await?;
        assert_eq!(scans.load(Ordering::SeqCst), index + 2);
    }

    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{ "files": { "includes": ["src/**", "!!**/biome.json", "!!**/ignored"] } }"#,
    );
    for expected in [7, 8] {
        server
            .notify(
                "workspace/didChangeWatchedFiles",
                lsp::DidChangeWatchedFilesParams {
                    changes: vec![lsp::FileEvent {
                        uri: uri!("biome.json"),
                        typ: lsp::FileChangeType::CHANGED,
                    }],
                },
            )
            .await?;
        assert_eq!(scans.load(Ordering::SeqCst), expected);
    }

    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "files": { "includes": ["**", "!!**/ignored"] } }"#,
    );
    let custom_settings = json!({
        "configurationPath": to_utf8_file_path_buf(uri!("ignored/biome.json")).as_str(),
    });
    *client_settings.lock().unwrap() = json!([custom_settings.clone()]);
    server
        .load_configuration_with_settings(custom_settings)
        .await?;
    assert_eq!(scans.load(Ordering::SeqCst), 9);
    server
        .notify(
            "workspace/didChangeWatchedFiles",
            lsp::DidChangeWatchedFilesParams {
                changes: vec![lsp::FileEvent {
                    uri: uri!("ignored/biome.json"),
                    typ: lsp::FileChangeType::CHANGED,
                }],
            },
        )
        .await?;
    assert_eq!(scans.load(Ordering::SeqCst), 10);

    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "files": { "includes": ["**", "!!**/.editorconfig"] }, "formatter": { "useEditorconfig": true } }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/.editorconfig")),
        "root = true\n[*]\nindent_style = space\nindent_size = 3\n",
    );
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    server
        .open_named_document(
            "function f(){return 1;}",
            uri!("ignored/main.js"),
            "javascript",
        )
        .await?;
    assert_indentation(&mut server, uri!("ignored/main.js"), 3).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/.editorconfig")),
        "root = true\n[*]\nindent_style = space\nindent_size = 5\n",
    );
    notify_configuration_change(&mut server, uri!("ignored/.editorconfig")).await?;
    assert_indentation(&mut server, uri!("ignored/main.js"), 5).await?;

    *client_settings.lock().unwrap() = json!([WorkspaceSettings::default()]);
    server.load_configuration_with_settings(json!({})).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{ "extends": ["./ignored/biome.json"], "files": { "includes": ["**", "!!**/ignored"] } }"#,
    );
    notify_configuration_change(&mut server, uri!("biome.json")).await?;
    server
        .open_named_document("function f(){return 1;}", uri!("src/main.js"), "javascript")
        .await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 4 } }"#,
    );
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 4).await?;

    fs.insert(to_utf8_file_path_buf(uri!("ignored/biome.json")), "{");
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 6 } }"#,
    );
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 6).await?;

    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{ "files": { "includes": ["**", "!!**/ignored"] } }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("src/nested/biome.json")),
        r#"{ "root": false, "extends": ["../../ignored/biome.json"] }"#,
    );
    notify_configuration_change(&mut server, uri!("biome.json")).await?;
    server
        .open_named_document(
            "function f(){return 1;}",
            uri!("src/nested/main.js"),
            "javascript",
        )
        .await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 8 } }"#,
    );
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    assert_indentation(&mut server, uri!("src/nested/main.js"), 8).await?;

    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{ "files": { "includes": ["**", "!!**/.editorconfig", "!!**/.gitignore"] }, "formatter": { "useEditorconfig": true } }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!(".editorconfig")),
        "root = true\n[*]\nindent_style = space\nindent_size = 3\n",
    );
    notify_configuration_change(&mut server, uri!("biome.json")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 3).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!(".editorconfig")),
        "root = true\n[*]\nindent_style = space\nindent_size = 5\n",
    );
    notify_configuration_change(&mut server, uri!(".editorconfig")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 5).await?;

    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "files": { "includes": ["**", "!!**/.gitignore"] },
            "vcs": { "enabled": true, "clientKind": "git", "useIgnoreFile": true },
            "formatter": { "indentStyle": "space", "indentWidth": 2 }
        }"#,
    );
    fs.insert(to_utf8_file_path_buf(uri!(".gitignore")), "");
    notify_configuration_change(&mut server, uri!("biome.json")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 2).await?;
    fs.insert(to_utf8_file_path_buf(uri!(".gitignore")), "src/main.js\n");
    let previous_scans = scans.load(Ordering::SeqCst);
    notify_configuration_change(&mut server, uri!(".gitignore")).await?;
    assert_eq!(scans.load(Ordering::SeqCst), previous_scans + 1);
    assert!(
        formatting_edits(&mut server, uri!("src/main.js"))
            .await?
            .is_none()
    );
    fs.insert(to_utf8_file_path_buf(uri!(".gitignore")), "");
    notify_configuration_change(&mut server, uri!(".gitignore")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 2).await?;

    fs.insert(
        to_utf8_file_path_buf(uri!("src/nested/biome.json")),
        r#"{ "root": false }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{ "files": { "includes": ["**", "!!**/ignored"] } }"#,
    );
    notify_configuration_change(&mut server, uri!("biome.json")).await?;
    let previous_scans = scans.load(Ordering::SeqCst);
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    assert_eq!(scans.load(Ordering::SeqCst), previous_scans);

    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/new-root/biome.json")),
        "{",
    );
    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), r#"{ "extends": ["./ignored/new-root/biome.json"], "files": { "includes": ["**", "!!**/ignored"] } }"#);
    notify_configuration_change(&mut server, uri!("biome.json")).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/new-root/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 4 } }"#,
    );
    notify_configuration_change(&mut server, uri!("ignored/new-root/biome.json")).await?;
    assert_indentation(&mut server, uri!("src/main.js"), 4).await?;

    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/new-nested/biome.json")),
        "{",
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("src/nested/biome.json")),
        r#"{ "root": false, "extends": ["../../ignored/new-nested/biome.json"] }"#,
    );
    notify_configuration_change(&mut server, uri!("src/nested/biome.json")).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/new-nested/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 6 } }"#,
    );
    notify_configuration_change(&mut server, uri!("ignored/new-nested/biome.json")).await?;
    assert_indentation(&mut server, uri!("src/nested/main.js"), 6).await?;

    server.shutdown().await?;
    reader.abort();
    Ok(())
}

#[tokio::test]
async fn watched_pnpm_catalogs_are_not_suppressed_by_file_exclusions() -> Result<()> {
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "files": { "includes": ["**", "!!**/pnpm-workspace.yaml"] },
            "javascript": { "resolver": { "experimentalPnpmCatalogs": true } },
            "linter": { "rules": { "suspicious": { "noImportCycles": "error" } } }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("sub/package.json")),
        r#"{ "name": "app", "dependencies": { "react": "catalog:react19" } }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("sub/pnpm-workspace.yaml")),
        "catalogs:\n  react19:\n    react: 19.0.0\n",
    );
    let TestClient {
        mut server,
        scans,
        reader,
        ..
    } = start_server(fs.clone(), None).await?;
    let previous_scans = scans.load(Ordering::SeqCst);
    fs.insert(
        to_utf8_file_path_buf(uri!("sub/pnpm-workspace.yaml")),
        "catalogs:\n  react19:\n    react: 18.3.1\n",
    );
    notify_configuration_change(&mut server, uri!("sub/pnpm-workspace.yaml")).await?;
    assert_eq!(scans.load(Ordering::SeqCst), previous_scans + 1);
    server.shutdown().await?;
    reader.abort();
    Ok(())
}

#[tokio::test]
async fn watched_nested_configuration_recovers_from_plugin_errors() -> Result<()> {
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
            "files": { "includes": ["**", "!!**/ignored"] },
            "linter": { "rules": { "suspicious": { "noImportCycles": "error" } } }
        }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("src/biome.json")),
        r#"{ "root": false, "extends": ["../ignored/biome.json"] }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "plugins": ["./missing.grit"] }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("src/main.js")),
        "function f(){return 1;}",
    );
    let TestClient {
        mut server,
        scans,
        reader,
        ..
    } = start_server(fs.clone(), None).await?;
    server
        .open_named_document("function f(){return 1;}", uri!("src/main.js"), "javascript")
        .await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("ignored/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 4 } }"#,
    );
    let previous_scans = scans.load(Ordering::SeqCst);
    notify_configuration_change(&mut server, uri!("ignored/biome.json")).await?;
    assert_eq!(scans.load(Ordering::SeqCst), previous_scans + 1);
    assert_indentation(&mut server, uri!("src/main.js"), 4).await?;
    server.shutdown().await?;
    reader.abort();
    Ok(())
}

#[tokio::test]
async fn watched_ignore_file_under_vcs_root_keeps_updating() -> Result<()> {
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
        "files": { "includes": ["sub/**", "!!**/.gitignore"] },
        "vcs": { "enabled": true, "clientKind": "git", "useIgnoreFile": true, "root": "sub" },
        "formatter": { "indentStyle": "space", "indentWidth": 2 }
    }"#,
    );
    fs.insert(to_utf8_file_path_buf(uri!("sub/.gitignore")), "");
    let TestClient {
        mut server, reader, ..
    } = start_server(fs.clone(), None).await?;
    server
        .open_named_document("function f(){return 1;}", uri!("sub/main.js"), "javascript")
        .await?;
    assert_indentation(&mut server, uri!("sub/main.js"), 2).await?;
    fs.insert(to_utf8_file_path_buf(uri!("sub/.gitignore")), "main.js\n");
    notify_configuration_change(&mut server, uri!("sub/.gitignore")).await?;
    assert!(
        formatting_edits(&mut server, uri!("sub/main.js"))
            .await?
            .is_none()
    );
    fs.insert(to_utf8_file_path_buf(uri!("sub/.gitignore")), "");
    notify_configuration_change(&mut server, uri!("sub/.gitignore")).await?;
    assert_indentation(&mut server, uri!("sub/main.js"), 2).await?;
    server.shutdown().await?;
    reader.abort();
    Ok(())
}

#[tokio::test]
async fn watched_configuration_dependencies_cross_workspace_folders() -> Result<()> {
    let fs = Arc::new(MemoryFileSystem::default());
    fs.insert(
        to_utf8_file_path_buf(uri!("a/biome.json")),
        r#"{ "extends": ["../b/ignored/biome.json"] }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("b/biome.json")),
        r#"{ "files": { "includes": ["**", "!!**/ignored"] } }"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("b/ignored/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 4 } }"#,
    );
    let folders = vec![
        lsp::WorkspaceFolder {
            uri: uri!("a/"),
            name: String::from("a"),
        },
        lsp::WorkspaceFolder {
            uri: uri!("b/"),
            name: String::from("b"),
        },
    ];
    let TestClient {
        mut server,
        scans,
        reader,
        ..
    } = start_server(fs.clone(), Some(folders)).await?;
    server
        .open_named_document("function f(){return 1;}", uri!("a/main.js"), "javascript")
        .await?;
    assert_indentation(&mut server, uri!("a/main.js"), 4).await?;
    fs.insert(
        to_utf8_file_path_buf(uri!("b/ignored/biome.json")),
        r#"{ "formatter": { "indentStyle": "space", "indentWidth": 6 } }"#,
    );
    let previous_scans = scans.load(Ordering::SeqCst);
    notify_configuration_change(&mut server, uri!("b/ignored/biome.json")).await?;
    assert_eq!(scans.load(Ordering::SeqCst), previous_scans + 2);
    assert_indentation(&mut server, uri!("a/main.js"), 6).await?;
    server.shutdown().await?;
    reader.abort();
    Ok(())
}

struct TestClient {
    server: Server,
    scans: Arc<AtomicUsize>,
    client_settings: Arc<std::sync::Mutex<serde_json::Value>>,
    reader: tokio::task::JoinHandle<Result<()>>,
}

#[expect(deprecated)]
async fn start_server(
    fs: Arc<MemoryFileSystem>,
    folders: Option<Vec<lsp::WorkspaceFolder>>,
) -> Result<TestClient> {
    let factory = ServerFactory::new_with_fs(fs.clone());
    let (service, client) = factory.create().into_inner();
    let (mut stream, mut sink) = client.split();
    let scans = Arc::new(AtomicUsize::new(0));
    let scan_counter = scans.clone();
    let client_settings = Arc::new(std::sync::Mutex::new(json!([WorkspaceSettings::default()])));
    let settings = client_settings.clone();
    let reader = tokio::spawn(async move {
        while let Some(request) = stream.next().await {
            let Some(id) = request.id() else {
                continue;
            };
            let result = match request.method() {
                "workspace/configuration" => settings.lock().unwrap().clone(),
                "window/workDoneProgress/create" => {
                    scan_counter.fetch_add(1, Ordering::SeqCst);
                    json!(null)
                }
                _ => json!(null),
            };
            sink.send(Response::from_ok(id.clone(), result)).await?;
        }
        anyhow::Ok(())
    });
    let mut server = Server::new(service);
    server
        .request::<_, lsp::InitializeResult>(
            "initialize",
            "init",
            lsp::InitializeParams {
                root_uri: Some(uri!("")),
                workspace_folders: folders,
                capabilities: lsp::ClientCapabilities {
                    window: Some(lsp::WindowClientCapabilities {
                        work_done_progress: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .await?;
    server.initialized().await?;
    Ok(TestClient {
        server,
        scans,
        client_settings,
        reader,
    })
}

async fn notify_configuration_change(server: &mut Server, uri: lsp::Uri) -> Result<()> {
    server
        .notify(
            "workspace/didChangeWatchedFiles",
            lsp::DidChangeWatchedFilesParams {
                changes: vec![lsp::FileEvent {
                    uri,
                    typ: lsp::FileChangeType::CHANGED,
                }],
            },
        )
        .await
}

async fn formatting_edits(
    server: &mut Server,
    uri: lsp::Uri,
) -> Result<Option<Vec<lsp::TextEdit>>> {
    server
        .request(
            "textDocument/formatting",
            "format",
            lsp::DocumentFormattingParams {
                text_document: lsp::TextDocumentIdentifier { uri },
                options: lsp::FormattingOptions {
                    tab_size: 2,
                    insert_spaces: true,
                    ..Default::default()
                },
                work_done_progress_params: Default::default(),
            },
        )
        .await?
        .context("missing formatting response")
}

async fn assert_indentation(server: &mut Server, uri: lsp::Uri, spaces: usize) -> Result<()> {
    let mut edits = formatting_edits(server, uri)
        .await?
        .context("missing formatting edits")?;
    edits.sort_by_key(|edit| edit.range.start.character);
    let mut formatted = String::from("function f(){return 1;}");
    for edit in edits.into_iter().rev() {
        assert_eq!(edit.range.start.line, 0);
        assert_eq!(edit.range.end.line, 0);
        formatted.replace_range(
            usize::try_from(edit.range.start.character)?
                ..usize::try_from(edit.range.end.character)?,
            &edit.new_text,
        );
    }
    assert_eq!(
        formatted,
        format!("function f() {{\n{}return 1;\n}}\n", " ".repeat(spaces))
    );
    Ok(())
}
