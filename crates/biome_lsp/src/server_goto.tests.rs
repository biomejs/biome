use std::str::FromStr;
use std::sync::Arc;

use crate::server_test_utils::*;
use anyhow::{Context, Result};
use biome_fs::{MemoryFileSystem, TemporaryFs};
use biome_service::workspace::{
    OpenProjectParams, OpenProjectResult, ScanKind, ScanProjectParams, ScanProjectResult,
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
/// Opens a document with the given content, then requests go-to definition
/// at the given cursor position. When `config` is provided, a `biome.json`
/// is inserted into a `MemoryFileSystem` and `load_configuration` is called
/// so settings like `experimentalFullSupportEnabled` take effect.
async fn goto_definition_single_file(
    file_name: &str,
    language_id: &str,
    source: &str,
    cursor: Position,
    config: Option<&str>,
) -> Result<Option<lsp::GotoDefinitionResponse>> {
    let factory = if let Some(config) = config {
        let fs = MemoryFileSystem::default();
        fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);
        ServerFactory::new_with_fs(Arc::new(fs))
    } else {
        ServerFactory::default()
    };

    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    if let Some(config) = config {
        server
            .open_named_document(config, uri!("biome.json"), "json")
            .await?;
        server.load_configuration().await?;
    }

    let document_uri = test_uri(file_name);
    server
        .open_named_document(source, document_uri.clone(), language_id)
        .await?;

    let res: Option<lsp::GotoDefinitionResponse> = server
        .request(
            "textDocument/definition",
            "goto_definition",
            lsp::GotoDefinitionParams {
                text_document_position_params: lsp::TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: document_uri },
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

    server.shutdown().await?;
    reader.abort();

    Ok(res)
}

struct CrossFileTestParams {
    name: &'static str,
    config: &'static str,
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
    fs.create_file("biome.json", params.config);
    for (name, content) in &params.files {
        fs.create_file(name, content);
    }

    let root_uri = lsp::Uri::from_str(
        url::Url::from_file_path(&fs.working_directory)
            .unwrap()
            .as_str(),
    )
    .unwrap();

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

    server.initialize_with_root(root_uri).await?;
    server.initialized().await?;
    server.load_configuration().await?;

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
            assert_eq!(
                location.uri.to_file_path(),
                expected_uri.to_file_path(),
                "URI file paths differ: got {:?}, expected {:?}",
                location.uri,
                expected_uri,
            );
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

fn test_uri(file_name: &str) -> lsp::Uri {
    let base = if cfg!(windows) {
        "file:///z%3A/workspace/"
    } else {
        "file:///workspace/"
    };
    lsp::Uri::from_str(&format!("{base}{file_name}")).unwrap()
}

// #region SINGLE-FILE TESTS

#[tokio::test]
async fn goto_definition_same_file_local_binding() -> Result<()> {
    // Cursor on `myVar` in `console.log(myVar)` (line 1, character 12)
    let res = goto_definition_single_file(
        "document.js",
        "javascript",
        "const myVar = 42;\nconsole.log(myVar);\n",
        pos(1, 12),
        None,
    )
    .await?;

    assert_definition(res, test_uri("document.js"), range(0, 6, 0, 11));

    Ok(())
}

#[tokio::test]
async fn goto_definition_returns_none_for_non_identifier() -> Result<()> {
    // Cursor on `=` (line 0, character 8) — not an identifier
    let res = goto_definition_single_file(
        "document.js",
        "javascript",
        "const x = 1;\n",
        pos(0, 8),
        None,
    )
    .await?;

    assert!(res.is_none(), "expected None for non-identifier position");

    Ok(())
}

#[tokio::test]
async fn goto_definition_jsx_component_same_file() -> Result<()> {
    // Cursor on `MyComponent` in `<MyComponent />` (line 1, character 40)
    let res = goto_definition_single_file(
        "document.js",
        "javascript",
        "function MyComponent() { return <div />; }\nexport default function App() { return <MyComponent />; }\n",
        pos(1, 40),
        None,
    )
    .await?;

    assert_definition(res, test_uri("document.js"), range(0, 9, 0, 20));

    Ok(())
}

#[tokio::test]
async fn goto_definition_html_ish_expression() -> Result<()> {
    // Cursor on `foo` in `{foo}` (line 3, character 5)
    let res = goto_definition_single_file(
        "document.astro",
        "astro",
        r#"---
const foo = "bar";
---
<h1>{foo}</h1>
"#,
        pos(3, 5),
        Some(r#"{ "linter": { "enabled": true }, "html": { "experimentalFullSupportEnabled": true } }"#),
    )
    .await?;

    assert_definition(res, test_uri("document.astro"), range(1, 6, 1, 9));

    Ok(())
}

// #endregion

// #region CROSS-FILE TESTS

#[tokio::test]
async fn goto_definition_cross_file_named_import() -> Result<()> {
    // Cursor on `greet` in `greet()` at line 1, character 0
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_cross_file_named_import",
        config: r#"{ "linter": { "enabled": true } }"#,
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
        config: r#"{ "linter": { "enabled": true } }"#,
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
        config: r#"{ "linter": { "enabled": true } }"#,
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
        config: r#"{ "linter": { "enabled": true } }"#,
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
        config: r#"{ "linter": { "enabled": true } }"#,
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

#[tokio::test]
async fn goto_definition_html_component() -> Result<()> {
    // Cursor on `utils` in `const utils = await import(...)` at line 0, character 6
    let (res, fs) = goto_definition_cross_file(CrossFileTestParams {
        name: "goto_definition_html_component",
        config: r#"{ "linter": { "enabled": true }, "html": { "experimentalFullSupportEnabled": true } }"#,
        files: vec![
            ("Component.astro", "<h1>Hello, world!</h1>"),
            (
                "Page.astro",
                r#"---
import Component from "./Component.astro";
---
<Component />
"#,
            ),
        ],
        open_file: "Page.astro",
        language_id: "astro",
        source: r#"---
import Component from "./Component.astro";
---
<Component />
"#,
        cursor: pos(3, 2),
    })
    .await?;

    assert_definition(res, file_uri(&fs, "Component.astro"), range(0, 0, 0, 0));

    Ok(())
}

// #endregion
