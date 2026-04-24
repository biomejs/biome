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

/// Sends a `textDocument/definition` request for a single-file test.
///
/// Opens `document.js` with the given content, then requests go-to definition
/// at the given cursor position.
async fn goto_definition_single_file(
    source: &str,
    cursor: Position,
) -> Result<Option<lsp::GotoDefinitionResponse>> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document(source).await?;

    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier {
                        uri: uri!("document.js"),
                    },
                    position: cursor,
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

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(res)
}

struct CrossFileTestParams {
    name: &'static str,
    files: Vec<(&'static str, &'static str)>,
    open_file: &'static str,
    language_id: &'static str,
    source: &'static str,
    cursor: Position,
}

/// Sends a `textDocument/definition` request for a cross-file test.
///
/// Sets up a temporary project with the given files, scans it, opens the
/// specified file, then requests go-to definition at the cursor position.
async fn goto_definition_cross_file(
    params: CrossFileTestParams,
) -> Result<(Option<lsp::GotoDefinitionResponse>, TemporaryFs)> {
    let mut fs = TemporaryFs::new(params.name);
    fs.create_file("biome.json", r#"{ "linter": { "enabled": true } }"#);
    for (name, content) in &params.files {
        fs.create_file(name, content);
    }

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

    let file_path = fs.working_directory.join(params.open_file);
    let file_uri =
        lsp::Uri::from_str(url::Url::from_file_path(&file_path).unwrap().as_str()).unwrap();

    let _: OpenFileResult = server
        .request(
            "biome/open_file",
            "open_file",
            OpenFileParams {
                project_key,
                path: file_path.clone().into(),
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
                    uri: file_uri.clone(),
                    language_id: String::from(params.language_id),
                    version: 0,
                    text: String::from(params.source),
                },
            },
        )
        .await?;

    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: file_uri },
                    position: params.cursor,
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

    server.shutdown().await?;
    reader.abort();

    Ok((res, fs))
}

fn assert_definition(
    res: Option<lsp::GotoDefinitionResponse>,
    expected_uri: lsp::Uri,
    expected_range: Range,
) {
    let definition = res.expect("goto_definition returned empty response");
    match definition {
        lsp::GotoDefinitionResponse::Scalar(location) => {
            assert_eq!(location.uri, expected_uri);
            assert_eq!(location.range, expected_range);
        }
        other => panic!("expected Scalar response, got: {other:?}"),
    }
}

fn file_uri(fs: &TemporaryFs, name: &str) -> lsp::Uri {
    let path = fs.working_directory.join(name);
    lsp::Uri::from_str(url::Url::from_file_path(&path).unwrap().as_str()).unwrap()
}

fn range(start_line: u32, start_char: u32, end_line: u32, end_char: u32) -> Range {
    Range {
        start: Position {
            line: start_line,
            character: start_char,
        },
        end: Position {
            line: end_line,
            character: end_char,
        },
    }
}

fn pos(line: u32, character: u32) -> Position {
    Position { line, character }
}

// #region SINGLE-FILE TESTS

#[tokio::test]
async fn goto_definition_same_file_local_binding() -> Result<()> {
    // Cursor on `myVar` in `console.log(myVar)` (line 1, character 12)
    let res =
        goto_definition_single_file("const myVar = 42;\nconsole.log(myVar);\n", pos(1, 12)).await?;

    assert_definition(res, uri!("document.js"), range(0, 6, 0, 11));

    Ok(())
}

#[tokio::test]
async fn goto_definition_returns_none_for_non_identifier() -> Result<()> {
    // Cursor on `=` (line 0, character 8) — not an identifier
    let res = goto_definition_single_file("const x = 1;\n", pos(0, 8)).await?;

    assert!(res.is_none(), "expected None for non-identifier position");

    Ok(())
}

#[tokio::test]
async fn goto_definition_jsx_component_same_file() -> Result<()> {
    // Cursor on `MyComponent` in `<MyComponent />` (line 1, character 40)
    let res = goto_definition_single_file(
        "function MyComponent() { return <div />; }\nexport default function App() { return <MyComponent />; }\n",
        pos(1, 40),
    )
    .await?;

    assert_definition(res, uri!("document.js"), range(0, 9, 0, 20));

    Ok(())
}

// #endregion

// #region CROSS-FILE TESTS

#[tokio::test]
async fn goto_definition_cross_file_named_import() -> Result<()> {
    // Cursor on `greet` in `greet()` at line 1, character 0
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_cross_file_named_import",
        files: vec![
            ("utils.js", "export function greet() { return 'hello'; }\n"),
            ("main.js", "import { greet } from './utils.js';\ngreet();\n"),
        ],
        open_file: "main.js",
        language_id: "javascript",
        source: "import { greet } from './utils.js';\ngreet();\n",
        cursor: pos(1, 0),
    })
    .await?;

    assert_definition(res, file_uri(&fs, "utils.js"), range(0, 16, 0, 21));

    Ok(())
}

#[tokio::test]
async fn goto_definition_on_import_specifier() -> Result<()> {
    // Cursor on `greet` in the import specifier `import { greet }` at line 0, character 9
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_on_import_specifier",
        files: vec![
            ("utils.js", "export function greet() { return 'hello'; }\n"),
            ("main.js", "import { greet } from './utils.js';\ngreet();\n"),
        ],
        open_file: "main.js",
        language_id: "javascript",
        source: "import { greet } from './utils.js';\ngreet();\n",
        cursor: pos(0, 9),
    })
    .await?;

    assert_definition(res, file_uri(&fs, "utils.js"), range(0, 16, 0, 21));

    Ok(())
}

#[tokio::test]
async fn goto_definition_jsx_component_cross_file() -> Result<()> {
    // Cursor on `Button` in `<Button />` at line 1, character 40
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_jsx_component_cross_file",
        files: vec![
            ("Button.jsx", "export default function Button() { return <button />; }\n"),
            ("App.jsx", "import Button from './Button.jsx';\nexport default function App() { return <Button />; }\n"),
        ],
        open_file: "App.jsx",
        language_id: "javascriptreact",
        source: "import Button from './Button.jsx';\nexport default function App() { return <Button />; }\n",
        cursor: pos(1, 40),
    })
    .await?;

    assert_definition(res, file_uri(&fs, "Button.jsx"), range(0, 24, 0, 30));

    Ok(())
}

#[tokio::test]
async fn goto_definition_dynamic_import_variable_declarator() -> Result<()> {
    // Cursor on `utils` in `const utils = await import(...)` at line 0, character 6
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_dynamic_import",
        files: vec![
            ("utils.js", "export function greet() { return 'hello'; }\n"),
            ("main.js", "const utils = await import('./utils.js');\n"),
        ],
        open_file: "main.js",
        language_id: "javascript",
        source: "const utils = await import('./utils.js');\n",
        cursor: pos(0, 6),
    })
    .await?;

    assert_definition(res, file_uri(&fs, "utils.js"), range(0, 0, 0, 0));

    Ok(())
}

#[tokio::test]
async fn goto_definition_dynamic_import_reference() -> Result<()> {
    // Cursor on `utils` in `utils.greet()` at line 1, character 0
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_dynamic_import_reference",
        files: vec![
            ("utils.js", "export function greet() { return 'hello'; }\n"),
            (
                "main.js",
                "const utils = await import('./utils.js');\nutils.greet();\n",
            ),
        ],
        open_file: "main.js",
        language_id: "javascript",
        source: "const utils = await import('./utils.js');\nutils.greet();\n",
        cursor: pos(1, 0),
    })
    .await?;

    assert_definition(res, file_uri(&fs, "utils.js"), range(0, 0, 0, 0));

    Ok(())
}

// #endregion
