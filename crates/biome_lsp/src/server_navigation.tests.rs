use std::str::FromStr;

use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::TemporaryFs;
use biome_service::workspace::{
    FileContent, OpenFileParams, OpenFileResult, OpenProjectParams, OpenProjectResult, ScanKind,
    ScanProjectParams, ScanProjectResult,
};
use biome_service::{Watcher, WatcherOptions};
use futures::channel::mpsc::channel;
use tokio::task::spawn_blocking;
use tower_lsp_server::ls_types::{
    self as lsp, DidOpenTextDocumentParams, Position, Range, TextDocumentIdentifier,
    TextDocumentItem, WorkDoneProgressParams,
};

#[tokio::test]
async fn goto_definition_same_file_local_binding() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    // `const myVar = 42;\nconsole.log(myVar);\n`
    // Cursor on `myVar` in `console.log(myVar)` (line 1, character 12)
    server
        .open_document("const myVar = 42;\nconsole.log(myVar);\n")
        .await?;

    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier {
                        uri: uri!("document.js"),
                    },
                    position: Position {
                        line: 1,
                        character: 12,
                    },
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("goto_definition returned None")?;

    let definition = res.context("goto_definition returned empty response")?;

    match definition {
        lsp::GotoDefinitionResponse::Scalar(location) => {
            assert_eq!(location.uri, uri!("document.js"));
            // `myVar` declaration is at line 0, characters 6..11
            assert_eq!(
                location.range,
                Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 11,
                    },
                }
            );
        }
        other => panic!("expected Scalar response, got: {other:?}"),
    }

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn goto_definition_returns_none_for_non_identifier() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    // Cursor on `=` (line 0, character 8) — not an identifier
    server.open_document("const x = 1;\n").await?;

    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier {
                        uri: uri!("document.js"),
                    },
                    position: Position {
                        line: 0,
                        character: 8,
                    },
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("goto_definition returned None")?;

    assert!(res.is_none(), "expected None for non-identifier position");

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn goto_definition_cross_file_named_import() -> Result<()> {
    let mut fs = TemporaryFs::new("goto_definition_cross_file_named_import");
    fs.create_file("biome.json", r#"{ "linter": { "enabled": true } }"#);
    fs.create_file("utils.js", "export function greet() { return 'hello'; }\n");
    fs.create_file("main.js", "import { greet } from './utils.js';\ngreet();\n");

    let (watcher, instruction_channel) = Watcher::new(WatcherOptions::default())?;

    let factory = ServerFactory::new(true, instruction_channel.sender.clone());

    let workspace = factory.workspace();
    spawn_blocking(move || {
        workspace.start_watcher(watcher);
    });

    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;

    let OpenProjectResult { project_key } = server
        .request(
            "biome/open_project",
            "open_project",
            OpenProjectParams {
                path: fs.working_directory.clone().into(),
                open_uninitialized: true,
            },
        )
        .await?
        .expect("open_project returned an error");

    let result: ScanProjectResult = server
        .request(
            "biome/scan_project",
            "scan_project",
            ScanProjectParams {
                project_key,
                watch: true,
                force: false,
                scan_kind: ScanKind::Project,
                verbose: false,
            },
        )
        .await?
        .expect("scan_project returned an error");
    assert_eq!(result.diagnostics.len(), 0);

    let main_path = fs.working_directory.join("main.js");
    let main_uri =
        lsp::Uri::from_str(url::Url::from_file_path(&main_path).unwrap().as_str()).unwrap();

    let _: OpenFileResult = server
        .request(
            "biome/open_file",
            "open_file",
            OpenFileParams {
                project_key,
                path: main_path.clone().into(),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                needs_document_services: None,
            },
        )
        .await?
        .expect("open_file returned an error");

    // Open the document via didOpen so the session tracks it
    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: main_uri.clone(),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from("import { greet } from './utils.js';\ngreet();\n"),
                },
            },
        )
        .await?;

    // Cursor on `greet` in `greet()` at line 1, character 0
    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: main_uri },
                    position: Position {
                        line: 1,
                        character: 0,
                    },
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("goto_definition returned None")?;

    let definition = res.context("goto_definition returned empty response")?;

    match definition {
        lsp::GotoDefinitionResponse::Scalar(location) => {
            let utils_path = fs.working_directory.join("utils.js");
            let expected_uri =
                lsp::Uri::from_str(url::Url::from_file_path(&utils_path).unwrap().as_str())
                    .unwrap();
            assert_eq!(location.uri, expected_uri);
            // `greet` binding in `export function greet()` starts at character 16
            assert_eq!(
                location.range,
                Range {
                    start: Position {
                        line: 0,
                        character: 16,
                    },
                    end: Position {
                        line: 0,
                        character: 21,
                    },
                }
            );
        }
        other => panic!("expected Scalar response, got: {other:?}"),
    }

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn goto_definition_on_import_specifier() -> Result<()> {
    let mut fs = TemporaryFs::new("goto_definition_on_import_specifier");
    fs.create_file("biome.json", r#"{ "linter": { "enabled": true } }"#);
    fs.create_file("utils.js", "export function greet() { return 'hello'; }\n");
    fs.create_file("main.js", "import { greet } from './utils.js';\ngreet();\n");

    let (watcher, instruction_channel) = Watcher::new(WatcherOptions::default())?;

    let factory = ServerFactory::new(true, instruction_channel.sender.clone());

    let workspace = factory.workspace();
    spawn_blocking(move || {
        workspace.start_watcher(watcher);
    });

    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;

    let OpenProjectResult { project_key } = server
        .request(
            "biome/open_project",
            "open_project",
            OpenProjectParams {
                path: fs.working_directory.clone().into(),
                open_uninitialized: true,
            },
        )
        .await?
        .expect("open_project returned an error");

    let result: ScanProjectResult = server
        .request(
            "biome/scan_project",
            "scan_project",
            ScanProjectParams {
                project_key,
                watch: true,
                force: false,
                scan_kind: ScanKind::Project,
                verbose: false,
            },
        )
        .await?
        .expect("scan_project returned an error");
    assert_eq!(result.diagnostics.len(), 0);

    let main_path = fs.working_directory.join("main.js");
    let main_uri =
        lsp::Uri::from_str(url::Url::from_file_path(&main_path).unwrap().as_str()).unwrap();

    let _: OpenFileResult = server
        .request(
            "biome/open_file",
            "open_file",
            OpenFileParams {
                project_key,
                path: main_path.clone().into(),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                needs_document_services: None,
            },
        )
        .await?
        .expect("open_file returned an error");

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: main_uri.clone(),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from("import { greet } from './utils.js';\ngreet();\n"),
                },
            },
        )
        .await?;

    // Cursor on `greet` in the import specifier `import { greet }` at line 0, character 9
    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: main_uri },
                    position: Position {
                        line: 0,
                        character: 9,
                    },
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: lsp::PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("goto_definition returned None")?;

    let definition = res.context("goto_definition returned empty response")?;

    match definition {
        lsp::GotoDefinitionResponse::Scalar(location) => {
            let utils_path = fs.working_directory.join("utils.js");
            let expected_uri =
                lsp::Uri::from_str(url::Url::from_file_path(&utils_path).unwrap().as_str())
                    .unwrap();
            assert_eq!(location.uri, expected_uri);
            // `greet` binding in `export function greet()` starts at character 16
            assert_eq!(
                location.range,
                Range {
                    start: Position {
                        line: 0,
                        character: 16,
                    },
                    end: Position {
                        line: 0,
                        character: 21,
                    },
                }
            );
        }
        other => panic!("expected Scalar response, got: {other:?}"),
    }

    server.shutdown().await?;
    reader.abort();

    Ok(())
}
