use crate::WorkspaceSettings;
use crate::capabilities::DEFAULT_CODE_ACTION_CAPABILITIES;
use crate::server_test_utils::*;
use anyhow::{Context, Result, bail};
use biome_analyze::RuleCategories;
use biome_configuration::analyzer::RuleSelector;
use biome_configuration::analyzer::assist::AssistConfiguration;
use biome_configuration::{Configuration, FormatterConfiguration, LinterConfiguration};
use biome_diagnostics::PrintDescription;
use biome_fs::{BiomePath, MemoryFileSystem, TemporaryFs};
use biome_languages::DocumentFileSource;
use biome_service::workspace::{
    ChangeFileParams, ChangeFileResult, FileContent, FileExistsParams, FormatFileParams,
    GetFileContentParams, GetModuleGraphParams, GetModuleGraphResult, GetSyntaxTreeParams,
    GetSyntaxTreeResult, OpenFileParams, OpenFileResult, OpenProjectParams, OpenProjectResult,
    PullDiagnosticsParams, PullDiagnosticsResult, ScanKind, ScanProjectParams, ScanProjectResult,
};
use biome_service::{Watcher, WatcherOptions, Workspace};
use futures::StreamExt;
use futures::channel::mpsc::channel;
use std::collections::{BTreeMap, HashMap};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::task::spawn_blocking;
use tokio::time::sleep;
use tower_lsp_server::jsonrpc;
use tower_lsp_server::ls_types::{
    self as lsp, ClientCapabilities, CodeAction, CodeActionContext, CodeActionKind,
    CodeActionOrCommand, CodeActionParams, CodeActionResponse, CodeDescription, Diagnostic,
    DiagnosticRelatedInformation, DiagnosticSeverity, DidChangeConfigurationParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams,
    DocumentFormattingParams, DocumentRangeFormattingParams, FormattingOptions, InitializeParams,
    InitializeResult, Location, MessageType, NumberOrString, PartialResultParams, Position,
    PublishDiagnosticsParams, Range, ShowMessageParams, TextDocumentContentChangeEvent,
    TextDocumentIdentifier, TextDocumentItem, TextEdit, Uri, WorkDoneProgressParams, WorkspaceEdit,
    WorkspaceFolder,
};

#[test]
fn catches_regular_panics_as_panic_errors() {
    let result: std::result::Result<
        std::result::Result<(), salsa::Cancelled>,
        biome_diagnostics::panic::PanicError,
    > = super::catch_lsp_operation(|| panic!("boom"));

    let error = result.expect_err("regular panic should be caught as a panic error");
    assert!(error.info.contains("boom"), "{error:?}");
}

fn fixable_diagnostic(line: u32) -> Result<lsp::Diagnostic> {
    Ok(lsp::Diagnostic {
        range: Range {
            start: Position { line, character: 3 },
            end: Position {
                line,
                character: 11,
            },
        },
        severity: Some(lsp::DiagnosticSeverity::ERROR),
        code: Some(lsp::NumberOrString::String(String::from(
            "lint/suspicious/noCompareNegZero",
        ))),
        code_description: None,
        source: Some(String::from("biome")),
        message: String::from("Do not use the === operator to compare against -0."),
        related_information: None,
        tags: None,
        data: None,
    })
}

#[tokio::test]
async fn basic_lifecycle() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

fn create_document_content_change_event() -> Vec<TextDocumentContentChangeEvent> {
    vec![
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 2,
                    character: 6,
                },
                end: Position {
                    line: 2,
                    character: 10,
                },
            }),
            range_length: None,
            text: String::from("statement"),
        },
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 1,
                    character: 7,
                },
                end: Position {
                    line: 1,
                    character: 11,
                },
            }),
            range_length: None,
            text: String::from("statement"),
        },
        TextDocumentContentChangeEvent {
            range: Some(Range {
                start: Position {
                    line: 0,
                    character: 6,
                },
                end: Position {
                    line: 0,
                    character: 10,
                },
            }),
            range_length: None,
            text: String::from("statement"),
        },
    ]
}

fn full_document_change(text: impl Into<String>) -> Vec<TextDocumentContentChangeEvent> {
    vec![TextDocumentContentChangeEvent {
        range: None,
        range_length: None,
        text: text.into(),
    }]
}

const EXPECTED_CST: &str = "0: JS_MODULE@0..57
  0: (empty)
  1: (empty)
  2: JS_DIRECTIVE_LIST@0..0
  3: JS_MODULE_ITEM_LIST@0..57
    0: JS_EXPRESSION_STATEMENT@0..18
      0: JS_CALL_EXPRESSION@0..17
        0: JS_IDENTIFIER_EXPRESSION@0..15
          0: JS_REFERENCE_IDENTIFIER@0..15
            0: IDENT@0..15 \"first_statement\" [] []
        1: (empty)
        2: (empty)
        3: JS_CALL_ARGUMENTS@15..17
          0: L_PAREN@15..16 \"(\" [] []
          1: JS_CALL_ARGUMENT_LIST@16..16
          2: R_PAREN@16..17 \")\" [] []
      1: SEMICOLON@17..18 \";\" [] []
    1: JS_EXPRESSION_STATEMENT@18..38
      0: JS_CALL_EXPRESSION@18..37
        0: JS_IDENTIFIER_EXPRESSION@18..35
          0: JS_REFERENCE_IDENTIFIER@18..35
            0: IDENT@18..35 \"second_statement\" [Newline(\"\\n\")] []
        1: (empty)
        2: (empty)
        3: JS_CALL_ARGUMENTS@35..37
          0: L_PAREN@35..36 \"(\" [] []
          1: JS_CALL_ARGUMENT_LIST@36..36
          2: R_PAREN@36..37 \")\" [] []
      1: SEMICOLON@37..38 \";\" [] []
    2: JS_EXPRESSION_STATEMENT@38..57
      0: JS_CALL_EXPRESSION@38..56
        0: JS_IDENTIFIER_EXPRESSION@38..54
          0: JS_REFERENCE_IDENTIFIER@38..54
            0: IDENT@38..54 \"third_statement\" [Newline(\"\\n\")] []
        1: (empty)
        2: (empty)
        3: JS_CALL_ARGUMENTS@54..56
          0: L_PAREN@54..55 \"(\" [] []
          1: JS_CALL_ARGUMENT_LIST@55..55
          2: R_PAREN@55..56 \")\" [] []
      1: SEMICOLON@56..57 \";\" [] []
  4: EOF@57..57 \"\" [] []
";

#[tokio::test]
async fn document_lifecycle() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("first_line();\nsecond_line();\nthird_line();")
        .await?;

    server
        .change_document(1, create_document_content_change_event())
        .await?;

    // `open_project()` will return an existing key if called with a path
    // for an existing project.
    let OpenProjectResult { project_key } = server
        .request(
            "biome/open_project",
            "open_project",
            OpenProjectParams {
                path: BiomePath::new(""),
                open_uninitialized: true,
            },
        )
        .await?
        .expect("open_project returned an error");

    let res: GetSyntaxTreeResult = server
        .request(
            "biome/get_syntax_tree",
            "get_syntax_tree",
            GetSyntaxTreeParams {
                project_key,
                path: BiomePath::try_from(uri!("document.js").to_file_path().unwrap()).unwrap(),
            },
        )
        .await?
        .expect("get_syntax_tree returned None");

    assert_eq!(res.cst, EXPECTED_CST);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn lifecycle_with_multiple_connections() -> Result<()> {
    let factory = ServerFactory::default();

    // First connection:
    {
        let (service, client) = factory.create().into_inner();
        let (stream, sink) = client.split();
        let mut server = Server::new(service);

        let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
        let reader = tokio::spawn(client_handler(stream, sink, sender));

        server.initialize().await?;
        server.initialized().await?;

        server
            .open_document("first_line();\nsecond_line();\nthird_line();")
            .await?;

        server
            .change_document(1, create_document_content_change_event())
            .await?;

        // `open_project()` will return an existing key if called with a path
        // for an existing project.
        let OpenProjectResult { project_key } = server
            .request(
                "biome/open_project",
                "open_project",
                OpenProjectParams {
                    path: BiomePath::new(""),
                    open_uninitialized: true,
                },
            )
            .await?
            .expect("open_project returned an error");

        let res: GetSyntaxTreeResult = server
            .request(
                "biome/get_syntax_tree",
                "get_syntax_tree",
                GetSyntaxTreeParams {
                    project_key,
                    path: BiomePath::try_from(uri!("document.js").to_file_path().unwrap()).unwrap(),
                },
            )
            .await?
            .expect("get_syntax_tree returned None");

        assert_eq!(res.cst, EXPECTED_CST);

        server.shutdown().await?;
        reader.abort();
    }

    // Second connection, the document will still be there:
    {
        let (service, client) = factory.create().into_inner();
        let (stream, sink) = client.split();
        let mut server = Server::new(service);

        let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
        let reader = tokio::spawn(client_handler(stream, sink, sender));

        server.initialize().await?;
        server.initialized().await?;

        // `open_project()` will return an existing key if called with a path
        // for an existing project.
        let OpenProjectResult { project_key } = server
            .request(
                "biome/open_project",
                "open_project",
                OpenProjectParams {
                    path: BiomePath::new(""),
                    open_uninitialized: true,
                },
            )
            .await?
            .expect("open_project returned an error");

        let res: GetSyntaxTreeResult = server
            .request(
                "biome/get_syntax_tree",
                "get_syntax_tree",
                GetSyntaxTreeParams {
                    project_key,
                    path: BiomePath::try_from(uri!("document.js").to_file_path().unwrap()).unwrap(),
                },
            )
            .await?
            .expect("get_syntax_tree returned None");

        assert_eq!(res.cst, EXPECTED_CST);

        server.shutdown().await?;
        reader.abort();
    }

    Ok(())
}

#[tokio::test]
async fn document_no_extension() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from("statement()"),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    let edits = res.context("formatting did not return an edit list")?;
    assert!(!edits.is_empty(), "formatting returned an empty edit list");

    server
        .notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document"),
                },
            },
        )
        .await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn document_range_formatting() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from("doNotFormatHere()\nformatHere()\ndoNotFormatHere()\n"),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/rangeFormatting",
            "formatting",
            DocumentRangeFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range::new(Position::new(1, 0), Position::new(2, 0)),
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert_eq!(
        res.context("formatting did not return an edit list")?,
        vec![TextEdit::new(
            Range::new(Position::new(1, 12), Position::new(1, 12)),
            ";".to_string()
        )]
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("const a = 1; a = 2;").await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: Some(0),
                diagnostics: vec![
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 6,
                            },
                            end: Position {
                                line: 0,
                                character: 7,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noUnusedVariables",
                        ))),
                        code_description: Some(lsp::CodeDescription {
                            href: "https://biomejs.dev/linter/rules/no-unused-variables"
                                .parse()
                                .unwrap(),
                        }),
                        source: Some(String::from("biome")),
                        message: String::from("This variable a is unused.",),
                        related_information: None,
                        tags: None,
                        data: None,
                    },
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 13,
                            },
                            end: Position {
                                line: 0,
                                character: 14,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noConstAssign",
                        ))),
                        code_description: Some(CodeDescription {
                            href: "https://biomejs.dev/linter/rules/no-const-assign".parse()?
                        }),
                        source: Some(String::from("biome")),
                        message: String::from("Can't assign a because it's a constant.",),
                        related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                            location: lsp::Location {
                                uri: uri!("document.js"),
                                range: Range {
                                    start: Position {
                                        line: 0,
                                        character: 6,
                                    },
                                    end: Position {
                                        line: 0,
                                        character: 7,
                                    },
                                },
                            },
                            message: "This is where the variable is defined as constant. "
                                .to_string(),
                        }]),
                        tags: None,
                        data: None,
                    }
                ],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn debounces_diagnostics_after_rapid_changes() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("const a = 1; a = 2;").await?;
    let _ = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    server
        .change_document(1, full_document_change("const b = 1; b = 2;"))
        .await?;
    server
        .change_document(2, full_document_change("const c = 1; c = 2;"))
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;
    let Some(ServerNotification::PublishDiagnostics(params)) = notification else {
        panic!("expected publishDiagnostics notification");
    };

    assert_eq!(params.version, Some(2));

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn does_not_publish_debounced_diagnostics_after_close() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("const a = 1; a = 2;").await?;
    let _ = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    server
        .change_document(1, full_document_change("const b = 1; b = 2;"))
        .await?;
    server.close_document().await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;
    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: None,
                diagnostics: vec![],
            }
        ))
    );

    wait_for_no_notification(&mut receiver, Duration::from_millis(750), |n| {
        n.is_publish_diagnostics()
    })
    .await;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics_of_syntax_rules() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("class A { #foo; #foo }").await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: Some(0),
                diagnostics: vec![
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 16,
                            },
                            end: Position {
                                line: 0,
                                character: 20,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "syntax/correctness/noDuplicatePrivateClassMembers",
                        ))),
                        code_description: None,
                        source: Some(String::from("biome")),
                        message: String::from("Duplicate private class member \"#foo\"",),
                        related_information: None,
                        tags: None,
                        data: None,
                    },
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 10,
                            },
                            end: Position {
                                line: 0,
                                character: 14,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noUnusedPrivateClassMembers",
                        ))),
                        code_description: Some(lsp::CodeDescription {
                            href:
                                "https://biomejs.dev/linter/rules/no-unused-private-class-members"
                                    .parse()
                                    .unwrap(),
                        }),
                        source: Some(String::from("biome")),
                        message: String::from(
                            "This private class member is defined but never used.",
                        ),
                        related_information: None,
                        tags: None,
                        data: None,
                    },
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 16,
                            },
                            end: Position {
                                line: 0,
                                character: 20,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noUnusedPrivateClassMembers",
                        ))),
                        code_description: Some(lsp::CodeDescription {
                            href:
                                "https://biomejs.dev/linter/rules/no-unused-private-class-members"
                                    .parse()
                                    .unwrap(),
                        }),
                        source: Some(String::from("biome")),
                        message: String::from(
                            "This private class member is defined but never used.",
                        ),
                        related_information: None,
                        tags: None,
                        data: None,
                    },
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 6,
                            },
                            end: Position {
                                line: 0,
                                character: 7,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noUnusedVariables",
                        ))),
                        code_description: Some(lsp::CodeDescription {
                            href: "https://biomejs.dev/linter/rules/no-unused-variables"
                                .parse()
                                .unwrap(),
                        }),
                        source: Some(String::from("biome")),
                        message: String::from("This class A is unused.",),
                        related_information: None,
                        tags: None,
                        data: None,
                    }
                ],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics_from_new_file() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_untitled_document("const a = 1; a = 2;").await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("untitled-1"),
                version: Some(0),
                diagnostics: vec![
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 6,
                            },
                            end: Position {
                                line: 0,
                                character: 7,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::WARNING),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noUnusedVariables",
                        ))),
                        code_description: Some(lsp::CodeDescription {
                            href: "https://biomejs.dev/linter/rules/no-unused-variables"
                                .parse()
                                .unwrap(),
                        }),
                        source: Some(String::from("biome")),
                        message: String::from("This variable a is unused.",),
                        related_information: None,
                        tags: None,
                        data: None,
                    },
                    lsp::Diagnostic {
                        range: Range {
                            start: Position {
                                line: 0,
                                character: 13,
                            },
                            end: Position {
                                line: 0,
                                character: 14,
                            },
                        },
                        severity: Some(lsp::DiagnosticSeverity::ERROR),
                        code: Some(lsp::NumberOrString::String(String::from(
                            "lint/correctness/noConstAssign",
                        ))),
                        code_description: Some(CodeDescription {
                            href: "https://biomejs.dev/linter/rules/no-const-assign".parse()?
                        }),
                        source: Some(String::from("biome")),
                        message: String::from("Can't assign a because it's a constant.",),
                        related_information: Some(vec![lsp::DiagnosticRelatedInformation {
                            location: lsp::Location {
                                uri: uri!("untitled-1"),
                                range: Range {
                                    start: Position {
                                        line: 0,
                                        character: 6,
                                    },
                                    end: Position {
                                        line: 0,
                                        character: 7,
                                    },
                                },
                            },
                            message: "This is where the variable is defined as constant. "
                                .to_string(),
                        }]),
                        tags: None,
                        data: None,
                    }
                ],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_quick_fixes() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("if(a === -0) {}").await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 6,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(0)?],
                    only: Some(vec![CodeActionKind::QUICKFIX]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();
    changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 9,
                },
                end: Position {
                    line: 0,
                    character: 10,
                },
            },
            new_text: String::new(),
        }],
    );

    let expected_code_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Replace -0 with 0"),
        kind: Some(CodeActionKind::new(
            "quickfix.biome.suspicious.noCompareNegZero",
        )),
        diagnostics: Some(vec![fixable_diagnostic(0)?]),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    let mut suppression_changes = HashMap::default();
    suppression_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::from(
                "// biome-ignore lint/suspicious/noCompareNegZero: <explanation>\n",
            ),
        }],
    );

    let expected_inline_suppression_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Suppress rule lint/suspicious/noCompareNegZero for this line."),
        kind: Some(CodeActionKind::new("quickfix.suppressRule.inline.biome")),
        diagnostics: Some(vec![fixable_diagnostic(0)?]),
        edit: Some(WorkspaceEdit {
            changes: Some(suppression_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    let mut top_level_changes = HashMap::default();
    top_level_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::from(
                "/** biome-ignore-all lint/suspicious/noCompareNegZero: <explanation> */\n",
            ),
        }],
    );

    let expected_top_level_suppression_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Suppress rule lint/suspicious/noCompareNegZero for the whole file."),
        kind: Some(CodeActionKind::new("quickfix.suppressRule.topLevel.biome")),
        diagnostics: Some(vec![fixable_diagnostic(0)?]),
        edit: Some(WorkspaceEdit {
            changes: Some(top_level_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    assert_eq!(
        res,
        vec![
            expected_code_action,
            expected_inline_suppression_action,
            expected_top_level_suppression_action,
        ]
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_quick_fixes_with_resolve() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize_with_resolve_support().await?;
    server.initialized().await?;

    server.open_document("if(a === -0) {}").await?;

    // Phase 1: request code actions — should return unresolved actions (no edit)
    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 6,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(0)?],
                    only: Some(vec![CodeActionKind::QUICKFIX]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    // All actions should have data (resolve token) and no edit
    assert!(!res.is_empty(), "expected at least one code action");
    for action_or_command in &res {
        let CodeActionOrCommand::CodeAction(action) = action_or_command else {
            panic!("expected CodeAction, got Command");
        };
        assert!(
            action.edit.is_none(),
            "expected no edit in unresolved action, got edit in: {}",
            action.title
        );
        assert!(
            action.data.is_some(),
            "expected resolve data in action: {}",
            action.title
        );
    }

    // Phase 2: resolve the first action (the quickfix)
    let first_action = match &res[0] {
        CodeActionOrCommand::CodeAction(action) => action.clone(),
        _ => panic!("expected CodeAction"),
    };

    let resolved: CodeAction = server
        .request("codeAction/resolve", "resolve_code_action", first_action)
        .await?
        .context("codeAction/resolve returned None")?;

    // The resolved action should now have an edit
    assert!(resolved.edit.is_some(), "expected edit in resolved action");

    // Verify it's the correct fix (replace -0 with 0)
    let edit = resolved.edit.unwrap();
    let changes = edit.changes.unwrap();
    let edits = &changes[&uri!("document.js")];
    assert_eq!(edits.len(), 1);
    assert_eq!(edits[0].new_text, "");

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_biome_quick_fixes() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("if(a === -0) {}").await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 10,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(0)?],
                    only: Some(vec![CodeActionKind::new(
                        "quickfix.biome.suspicious.noCompareNegZero",
                    )]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();
    changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 9,
                },
                end: Position {
                    line: 0,
                    character: 10,
                },
            },
            new_text: String::new(),
        }],
    );

    let expected_code_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Replace -0 with 0"),
        kind: Some(CodeActionKind::new(
            "quickfix.biome.suspicious.noCompareNegZero",
        )),
        diagnostics: Some(vec![fixable_diagnostic(0)?]),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_code_action]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_quick_fixes_include_unsafe() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let unsafe_fixable = Diagnostic {
        range: Range {
            start: Position {
                line: 0,
                character: 6,
            },
            end: Position {
                line: 0,
                character: 9,
            },
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String(String::from(
            "lint/suspicious/noDoubleEquals",
        ))),
        code_description: None,
        source: Some(String::from("biome")),
        message: String::from("Use === instead of ==."),
        related_information: None,
        tags: None,
        data: None,
    };

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("if(a == 0) {}").await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 6,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![unsafe_fixable.clone()],
                    only: Some(
                        DEFAULT_CODE_ACTION_CAPABILITIES
                            .iter()
                            .map(|s| CodeActionKind::from(*s))
                            .collect::<Vec<_>>(),
                    ),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::<Uri, Vec<TextEdit>>::default();
    changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 7,
                },
                end: Position {
                    line: 0,
                    character: 7,
                },
            },
            new_text: "=".to_string(),
        }],
    );

    let mut suppression_changes = HashMap::<Uri, Vec<TextEdit>>::default();
    suppression_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::from(
                "// biome-ignore lint/suspicious/noDoubleEquals: <explanation>\n",
            ),
        }],
    );

    let expected_inline_suppression_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Suppress rule lint/suspicious/noDoubleEquals for this line."),
        kind: Some(CodeActionKind::new("quickfix.suppressRule.inline.biome")),
        diagnostics: Some(vec![unsafe_fixable.clone()]),
        edit: Some(WorkspaceEdit {
            changes: Some(suppression_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    let mut top_level_changes = HashMap::default();
    top_level_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::from(
                "/** biome-ignore-all lint/suspicious/noDoubleEquals: <explanation> */\n",
            ),
        }],
    );

    let expected_toplevel_suppression_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Suppress rule lint/suspicious/noDoubleEquals for the whole file."),
        kind: Some(CodeActionKind::new("quickfix.suppressRule.topLevel.biome")),
        diagnostics: Some(vec![unsafe_fixable.clone()]),
        edit: Some(WorkspaceEdit {
            changes: Some(top_level_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    let mut unsafe_action_changes = HashMap::default();
    unsafe_action_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 7,
                },
                end: Position {
                    line: 0,
                    character: 7,
                },
            },
            new_text: String::from("="),
        }],
    );
    let unsafe_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Use === instead."),
        kind: Some(CodeActionKind::new(
            "quickfix.biome.suspicious.noDoubleEquals",
        )),
        diagnostics: Some(vec![unsafe_fixable]),
        edit: Some(WorkspaceEdit {
            changes: Some(unsafe_action_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    assert_eq!(
        res,
        vec![
            unsafe_action,
            expected_inline_suppression_action,
            expected_toplevel_suppression_action,
        ]
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics_for_rome_json() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let incorrect_config = r#"{
        "formatter": {
            "indentStyle": "magic"
        }
    }"#;
    server
        .open_named_document(incorrect_config, uri!("biome.json"), "json")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("biome.json"),
                version: Some(0),
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 2,
                            character: 27,
                        },
                        end: Position {
                            line: 2,
                            character: 34,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(String::from("deserialize",))),
                    code_description: None,
                    source: Some(String::from("biome")),
                    message: String::from("Found an unknown value `magic`.",),
                    related_information: None,
                    tags: None,
                    data: None,
                }],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn plugin_load_error_show_message() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
        "css": {
            "linter": { "enabled": true }
        },
        "plugins": ["./plugin"],
        "linter": {
            "rules": { "correctness": { "noUnknownProperty": "error" } }
        }
    }"#;

    const INVALID_PLUGIN_CONTENT: &[u8] = br#"foo"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);
    fs.insert(
        to_utf8_file_path_buf(uri!("plugin")),
        INVALID_PLUGIN_CONTENT,
    );

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();

    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    let incorrect_config = r#"a {colr: blue;}"#;
    server
        .open_named_document(incorrect_config, uri!("document.css"), "css")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_show_message()).await;

    assert_eq!(notification, Some(ServerNotification::ShowMessage(ShowMessageParams {
        typ: MessageType::WARNING,
        message: "The plugin loading has failed. Biome will report only parsing errors until the file is fixed or its usage is disabled.".to_string(),
    })));

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn plugin_rewrite_pull_diagnostics() -> Result<()> {
    let fs = MemoryFileSystem::default();

    let config = r#"{
        "plugins": ["useConsoleInfo.grit"],
        "linter": {
            "rules": { "recommended": false }
        }
    }"#;

    let plugin = br#"language js

`console.log($msg)` as $call where {
    register_diagnostic(
        span = $call,
        message = "Use console.info instead of console.log.",
        severity = "warn"
    ),
    $call => `console.info($msg)`
}"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);
    fs.insert(to_utf8_file_path_buf(uri!("useConsoleInfo.grit")), plugin);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();

    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    server
        .open_named_document("console.log(\"hello\");", uri!("document.js"), "javascript")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    // Verify that the plugin diagnostic is emitted
    if let Some(ServerNotification::PublishDiagnostics(params)) = &notification {
        assert!(
            !params.diagnostics.is_empty(),
            "Expected at least one diagnostic"
        );
        let diag = &params.diagnostics[0];
        assert_eq!(diag.severity, Some(lsp::DiagnosticSeverity::WARNING));
        assert_eq!(
            diag.code,
            Some(lsp::NumberOrString::String(String::from("plugin")))
        );
        assert!(
            diag.message.contains("console.info"),
            "Diagnostic message should mention console.info, got: {}",
            diag.message
        );
    } else {
        panic!("Expected PublishDiagnostics, got {notification:?}");
    }

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics_for_css_files() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
        "css": {
            "linter": { "enabled": true }
        },
        "linter": {
            "rules": { "correctness": { "noUnknownProperty": "error" } }
        }
    }"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();

    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    let incorrect_config = r#"a {colr: blue;}"#;
    server
        .open_named_document(incorrect_config, uri!("document.css"), "css")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.css"),
                version: Some(0),
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 3,
                        },
                        end: Position {
                            line: 0,
                            character: 7,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(String::from(
                        "lint/correctness/noUnknownProperty"
                    ))),
                    code_description: Some(CodeDescription {
                        href: "https://biomejs.dev/linter/rules/no-unknown-property".parse()?
                    }),
                    source: Some(String::from("biome")),
                    message: String::from("Unknown property is not allowed.",),
                    related_information: None,
                    tags: None,
                    data: None,
                }],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_diagnostics_for_svg_files() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
        "html": {
            "formatter": { "enabled": true }
        }
    }"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();

    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    let incorrect_config = r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><circle cx="50" cy="50" r="40" fill="red" /></svg>"#;
    server
        .open_named_document(incorrect_config, uri!("document.svg"), "xml")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.svg"),
                version: Some(0),
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 0,
                        },
                        end: Position {
                            line: 0,
                            character: 112,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(String::from(
                        "lint/a11y/noSvgWithoutTitle"
                    ))),
                    code_description: Some(CodeDescription {
                        href: "https://biomejs.dev/linter/rules/no-svg-without-title".parse()?
                    }),
                    source: Some(String::from("biome")),
                    message: String::from("Alternative text title element cannot be empty",),
                    related_information: None,
                    tags: None,
                    data: None,
                }],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn no_code_actions_for_ignored_json_files() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let incorrect_config = r#"{
       "name": "test"
    }"#;
    server
        .open_named_document(
            incorrect_config,
            uri!("./node_modules/preact/package.json"),
            "json",
        )
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("./node_modules/preact/package.json"),
                version: Some(0),
                diagnostics: vec![],
            }
        ))
    );
    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("./node_modules/preact/package.json"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 7,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![],
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    assert_eq!(res, vec![]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_code_actions_with_import_sorting() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document(
            r#"
import z from "zod";
import { test } from "./test";
import { describe } from "node:test";

export { describe, test, z };

if(a === -0) {}
"#,
        )
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 6,
                    },
                    end: Position {
                        line: 0,
                        character: 10,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(0)?],
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();
    changes.insert(
        uri!("document.js"),
        vec![
            TextEdit {
                range: Range {
                    start: Position {
                        line: 1,
                        character: 7,
                    },
                    end: Position {
                        line: 1,
                        character: 8,
                    },
                },
                new_text: String::from("{ describe }"),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 1,
                        character: 15,
                    },
                    end: Position {
                        line: 1,
                        character: 18,
                    },
                },
                new_text: String::from("node:test"),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 2,
                        character: 7,
                    },
                    end: Position {
                        line: 2,
                        character: 15,
                    },
                },
                new_text: String::from("z"),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 2,
                        character: 22,
                    },
                    end: Position {
                        line: 2,
                        character: 28,
                    },
                },
                new_text: String::from("zod"),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 3,
                        character: 9,
                    },
                    end: Position {
                        line: 3,
                        character: 17,
                    },
                },
                new_text: String::from("test"),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 3,
                        character: 26,
                    },
                    end: Position {
                        line: 3,
                        character: 35,
                    },
                },
                new_text: String::from("./test"),
            },
        ],
    );

    let expected_code_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Organize imports and exports (Biome)"),
        kind: Some(CodeActionKind::new("source.organizeImports.biome")),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    let mut top_level_changes = HashMap::default();
    top_level_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 0,
                },
            },
            new_text: String::from(
                "/** biome-ignore-all assist/source/organizeImports: <explanation> */\n",
            ),
        }],
    );

    let mut inline_changes = HashMap::default();
    inline_changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 1,
                    character: 0,
                },
                end: Position {
                    line: 1,
                    character: 0,
                },
            },
            new_text: String::from(
                "// biome-ignore assist/source/organizeImports: <explanation>\n",
            ),
        }],
    );

    let expected_toplevel_suppression_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Suppress action assist/source/organizeImports for the whole file."),
        kind: Some(CodeActionKind::new("quickfix.suppressRule.topLevel.biome")),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(top_level_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    let expected_line_suppression_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Suppress action assist/source/organizeImports for this line."),
        kind: Some(CodeActionKind::new("quickfix.suppressRule.inline.biome")),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(inline_changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: None,
        disabled: None,
        data: None,
    });

    assert_eq!(
        res,
        vec![
            expected_code_action,
            expected_line_suppression_action,
            expected_toplevel_suppression_action
        ]
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_organize_imports_when_only_filter_is_set_issue_9741() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize_with_resolve_support().await?;
    server.initialized().await?;

    server
        .open_document(
            r#"
import z from "zod";
import { test } from "./test";
import { describe } from "node:test";

export { describe, test, z };
"#,
        )
        .await?;

    // Request code actions with only: ["source.organizeImports.biome"]
    // This is what editors like Zed send when configured with
    // "code_actions_on_format": { "source.organizeImports.biome": true }
    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 6,
                        character: 0,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![Diagnostic {
                        range: Range {
                            start: Position {
                                line: 1,
                                character: 0,
                            },
                            end: Position {
                                line: 1,
                                character: 19,
                            },
                        },
                        severity: Some(DiagnosticSeverity::INFORMATION),
                        code: Some(NumberOrString::String(String::from(
                            "assist/source/organizeImports",
                        ))),
                        source: Some(String::from("biome")),
                        message: String::from("The imports and exports are not sorted."),
                        ..Default::default()
                    }],
                    only: Some(vec![CodeActionKind::new("source.organizeImports.biome")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    // The response must contain the organize imports action.
    // Before the fix for #9741, this returned an empty array because the
    // "source.organizeImports.biome" filter was not matching the action category.
    assert!(
        !res.is_empty(),
        "Expected at least one code action, but got an empty response"
    );

    let action = match &res[0] {
        CodeActionOrCommand::CodeAction(action) => action,
        other => panic!("Expected CodeAction, got {:?}", other),
    };

    assert_eq!(action.title, "Apply safe fix for organizeImports");
    assert_eq!(
        action.kind,
        Some(CodeActionKind::new("source.organizeImports.biome"))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn resolve_organize_imports_action_issue_9812() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize_with_resolve_support().await?;
    server.initialized().await?;

    server
        .open_document(
            r#"
import z from "zod";
import { test } from "./test";
import { describe } from "node:test";

export { describe, test, z };
"#,
        )
        .await?;

    // Request code actions with only: ["source.organizeImports.biome"]
    // This is less to behave like #9741's test, and more to make it so that I only get a single
    // code action back from the server.
    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 6,
                        character: 0,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![Diagnostic {
                        range: Range {
                            start: Position {
                                line: 1,
                                character: 0,
                            },
                            end: Position {
                                line: 1,
                                character: 19,
                            },
                        },
                        severity: Some(DiagnosticSeverity::INFORMATION),
                        code: Some(NumberOrString::String(String::from(
                            "assist/source/organizeImports",
                        ))),
                        source: Some(String::from("biome")),
                        message: String::from("The imports and exports are not sorted."),
                        ..Default::default()
                    }],
                    only: Some(vec![CodeActionKind::new("source.organizeImports.biome")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    // This is already fixed and proven in #9741
    assert!(
        !res.is_empty(),
        "Expected at least one code action, but got an empty response"
    );

    let action = match &res[0] {
        CodeActionOrCommand::CodeAction(action) => action,
        other => panic!("Expected CodeAction, got {:?}", other),
    };

    assert_eq!(action.title, "Apply safe fix for organizeImports");
    assert_eq!(
        action.kind,
        Some(CodeActionKind::new("source.organizeImports.biome"))
    );
    // With resolve support, the edit should be deferred so we need to call resolve in order to get
    // the changes back.
    assert!(
        action.edit.is_none(),
        "expected no edit in unresolved action"
    );

    // With the changes in #9812 this now succeeds, if you comment out the changes in #9812 this
    // part will fail with an `Internal error: The rule doesn't exist`.
    let action_to_resolve = action.clone();
    let resolved: CodeAction = server
        .request(
            "codeAction/resolve",
            "resolve_code_action",
            action_to_resolve,
        )
        .await?
        .context("codeAction/resolve returned None")?;

    assert!(resolved.edit.is_some(), "expected edit in resolved action");
    let edit = resolved.edit.unwrap();
    let changes = edit.changes.unwrap();
    let edits = &changes[&uri!("document.js")];
    assert!(
        !edits.is_empty(),
        "expected at least one text edit for the organize imports fix"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn does_not_pull_action_for_disabled_rule_in_override_issue_2782() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
    "$schema": "https://biomejs.dev/schemas/1.7.3/schema.json",
    "assist": { "enabled": false },
    "linter": {
        "enabled": true,
        "rules": {
            "recommended": false,
            "style": {
                "useEnumInitializers": "error"
            }
        }
    },
    "overrides": [
        {
            "includes": ["**/*.ts", "**/*.tsx"],
            "linter": {
                "rules": {
                    "style": {
                        "useEnumInitializers": "off"
                    }
                }
            }
        }
    ]
}"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;
    server
        .open_named_document(config, uri!("biome.json"), "json")
        .await?;
    server
        .open_named_document(
            r#"enum X {
	A,
	B,
	C,
}"#,
            uri!("test.ts"),
            "typescript",
        )
        .await?;

    server.load_configuration().await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("test.ts"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 1,
                    },
                    end: Position {
                        line: 3,
                        character: 10,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(0)?],
                    only: Some(vec![CodeActionKind::new(
                        "quickfix.biome.style.useEnumInitializers",
                    )]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    assert!(res.is_empty(), "This should not have code actions");

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_refactors() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("let variable = \"value\"; func(variable);")
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 7,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![],
                    only: Some(vec![CodeActionKind::REFACTOR]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();

    changes.insert(
        uri!("document.js"),
        vec![
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 15,
                    },
                },
                new_text: String::from("func("),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 22,
                    },
                    end: Position {
                        line: 0,
                        character: 37,
                    },
                },
                new_text: String::new(),
            },
        ],
    );

    let _expected_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Inline variable"),
        kind: Some(CodeActionKind::new("refactor.inline.biome")),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_fix_all() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("if(a === -0) {}\nif(a === -0) {}\nif(a === -0) {}")
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 7,
                    },
                    end: Position {
                        line: 0,
                        character: 7,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![
                        fixable_diagnostic(0)?,
                        fixable_diagnostic(1)?,
                        fixable_diagnostic(2)?,
                    ],
                    only: Some(vec![CodeActionKind::new("source.fixAll")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();

    changes.insert(
        uri!("document.js"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 3,
                    character: 0,
                },
            },
            new_text: String::from("if (a === 0) {\n}\nif (a === 0) {\n}\nif (a === 0) {\n}\n"),
        }],
    );

    let expected_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Apply all safe fixes (Biome)"),
        kind: Some(CodeActionKind::new("source.fixAll.biome")),
        diagnostics: Some(vec![
            fixable_diagnostic(0)?,
            fixable_diagnostic(1)?,
            fixable_diagnostic(2)?,
        ]),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_action]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn fix_all_does_not_sort_imports_unless_requested() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    // Document with unsorted imports AND a lint error (comparing to -0).
    // If fix-all respects the filter, it should fix the lint error but
    // leave the import order unchanged.
    server
        .open_document("import { b } from \"b\";\nimport { a } from \"a\";\nif(a === -0) {}")
        .await?;

    // Request source.fixAll WITHOUT source.organizeImports
    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 2,
                        character: 15,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![fixable_diagnostic(2)?],
                    only: Some(vec![CodeActionKind::new("source.fixAll")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    // The fix-all action should exist
    assert_eq!(res.len(), 1);

    let CodeActionOrCommand::CodeAction(action) = &res[0] else {
        panic!("expected CodeAction");
    };
    assert_eq!(
        action.kind,
        Some(CodeActionKind::new("source.fixAll.biome"))
    );

    // The edit should fix the -0 comparison but NOT reorder imports.
    // If imports were sorted, "a" would come before "b".
    let edit = action.edit.as_ref().context("expected edit")?;
    let changes = edit.changes.as_ref().context("expected changes")?;
    let edits = changes
        .get(&uri!("document.js"))
        .context("expected edits for document.js")?;
    let new_text = &edits[0].new_text;
    assert!(
        new_text.starts_with("import { b }"),
        "imports should NOT be reordered when organizeImports is not requested, got: {new_text}"
    );

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn change_document_remove_line() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document(
            r#"("Jan 1, 2018 – Jan 1, 2019");
("Jan 1, 2018 – Jan 1, 2019");
isSpreadAssignment;
"#,
        )
        .await?;

    server
        .change_document(
            1,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 0,
                        character: 30,
                    },
                    end: Position {
                        line: 1,
                        character: 0,
                    },
                }),
                range_length: None,
                text: String::new(),
            }],
        )
        .await?;

    // `open_project()` will return an existing key if called with a path
    // for an existing project.
    let OpenProjectResult { project_key } = server
        .request(
            "biome/open_project",
            "open_project",
            OpenProjectParams {
                path: BiomePath::new(""),
                open_uninitialized: true,
            },
        )
        .await?
        .expect("open_project returned an error");

    let actual: String = server
        .request(
            "biome/get_file_content",
            "get_file_content",
            GetFileContentParams {
                project_key,
                path: BiomePath::try_from(uri!("document.js").to_file_path().unwrap()).unwrap(),
            },
        )
        .await?
        .context("get file content error")?;

    let expected = r#"("Jan 1, 2018 – Jan 1, 2019");("Jan 1, 2018 – Jan 1, 2019");
isSpreadAssignment;
"#;

    assert_eq!(&actual, expected);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn format_with_syntax_errors() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("expression(").await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(res.is_none());

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn format_jsx_in_javascript_file() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from("const f  =  () => <div/>;"),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document"),
                },
                options: FormattingOptions::default(),
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    let edits = res.context("formatting did not return an edit list")?;
    assert!(!edits.is_empty(), "formatting returned an empty edit list");

    server
        .notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document"),
                },
            },
        )
        .await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn does_not_format_ignored_files_inside_includes() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
        "files": {
            "includes": ["**", "!document.js"]
        }
    }"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    server
        .open_named_document("statement (   );", uri!("document.js"), "javascript")
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(res.is_none());

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn does_not_format_ignored_files_inside_ignore_file() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
    "vcs": {
        "useIgnoreFile": true,
        "clientKind": "git",
        "enabled": true
    }
}"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);
    fs.insert(to_utf8_file_path_buf(uri!(".gitignore")), "document.js\n");

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    server
        .open_named_document("statement (   );", uri!("document.js"), "javascript")
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(res.is_none());

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn does_not_format_ignored_files_inside_ignore_file_with_dir() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
    "vcs": {
        "useIgnoreFile": true,
        "clientKind": "git",
        "enabled": true
    }
}"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);
    fs.insert(to_utf8_file_path_buf(uri!(".gitignore")), "dist/\n");

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    server
        .open_named_document("statement (   );", uri!("dist/document.js"), "javascript")
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(res.is_none());

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
#[ignore = "Find a way to retrieve the last notification sent"]
async fn pull_diagnostics_from_manifest() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let config = r#"{
        "linter": {
            "rules": {
                "all": false,
                "nursery": { "noDeclaredDependencies": "error" }
            }
        }
    }"#;
    server
        .open_named_document(config, uri!("biome.json"), "json")
        .await?;

    let manifest = r#"{
        "dependencies": { "react": "latest" }
    }"#;
    server
        .open_named_document(manifest, uri!("package.json"), "json")
        .await?;

    server.load_configuration().await?;

    server
        .open_document(r#"import "lodash"; import "react"; "#)
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: Some(0),
                diagnostics: vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 0,
                            character: 5,
                        },
                        end: Position {
                            line: 0,
                            character: 7,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: Some(NumberOrString::String(String::from(
                        "lint/suspicious/noDoubleEquals",
                    ))),
                    code_description: Some(CodeDescription {
                        href: "https://biomejs.dev/linter/rules/no-double-equals".parse()?
                    }),
                    source: Some(String::from("biome")),
                    message: String::from(
                        "Use === instead of ==.\n== is only allowed when comparing against `null`",
                    ),
                    related_information: Some(vec![DiagnosticRelatedInformation {
                        location: Location {
                            uri: uri!("untitled-1"),
                            range: Range {
                                start: Position {
                                    line: 0,
                                    character: 5,
                                },
                                end: Position {
                                    line: 0,
                                    character: 7,
                                },
                            },
                        },
                        message: String::new(),
                    }]),
                    tags: None,
                    data: None,
                }],
            }
        ))
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn server_shutdown() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let cancellation = factory.cancellation();
    let cancellation = cancellation.notified();

    server.biome_shutdown().await?;

    cancellation.await;

    reader.abort();

    Ok(())
}

#[tokio::test]
async fn multiple_projects() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize_projects().await?;
    server.initialized().await?;

    let config_only_formatter = r#"{
        "linter": {
            "enabled": false
        },
        "organizeImports": {
            "enabled": false
        }
    }"#;
    server
        .open_named_document(config_only_formatter, uri!("test_one/biome.json"), "json")
        .await?;

    let config_only_linter = r#"{
        "formatter": {
            "enabled": false
        },
        "organizeImports": {
            "enabled": false
        }
    }"#;
    server
        .open_named_document(config_only_linter, uri!("test_two/biome.json"), "json")
        .await?;

    // it should add a `;` but no diagnostics
    let file_format_only = r#"debugger"#;
    server
        .open_named_document(file_format_only, uri!("test_one/file.js"), "javascript")
        .await?;

    // it should raise a diagnostic, but no formatting
    let file_lint_only = r#"debugger;\n"#;
    server
        .open_named_document(file_lint_only, uri!("test_two/file.js"), "javascript")
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("test_two/file.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_none(),
        "We should not have any edits here, we call the project where formatting is disabled."
    );

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("test_one/file.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_some(),
        "We should have any edits here, we call the project where formatting is enabled."
    );

    let cancellation = factory.cancellation();
    let cancellation = cancellation.notified();

    server.biome_shutdown().await?;

    cancellation.await;

    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_source_assist_action() -> Result<()> {
    let fs = MemoryFileSystem::default();
    let config = r#"{
        "assist": {
            "enabled": true,
            "actions": {
                "source": {
                    "useSortedKeys": "on"
                }
            }
        }
    }"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let unsafe_fixable = Diagnostic {
        range: Range {
            start: Position {
                line: 0,
                character: 6,
            },
            end: Position {
                line: 0,
                character: 9,
            },
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String(String::from(
            "lint/suspicious/noDoubleEquals",
        ))),
        code_description: None,
        source: Some(String::from("biome")),
        message: String::from("Use === instead of ==."),
        related_information: None,
        tags: None,
        data: None,
    };

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_named_document(
            r#"{"zod": true,"lorem": "ipsum","foo": "bar"}"#,
            uri!("file.json"),
            "json",
        )
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("file.json"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 15,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![unsafe_fixable.clone()],
                    only: Some(vec![CodeActionKind::new("source.biome.useSortedKeys")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;
    let mut changes = HashMap::default();
    changes.insert(
        uri!("file.json"),
        vec![
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 2,
                    },
                    end: Position {
                        line: 0,
                        character: 5,
                    },
                },
                new_text: "foo".to_string(),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 8,
                    },
                    end: Position {
                        line: 0,
                        character: 12,
                    },
                },
                new_text: "\"bar\"".to_string(),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 31,
                    },
                    end: Position {
                        line: 0,
                        character: 34,
                    },
                },
                new_text: "zod".to_string(),
            },
            TextEdit {
                range: Range {
                    start: Position {
                        line: 0,
                        character: 37,
                    },
                    end: Position {
                        line: 0,
                        character: 42,
                    },
                },
                new_text: "true".to_string(),
            },
        ],
    );
    let expected_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Sort the members by key."),
        kind: Some(CodeActionKind::new("source.biome.useSortedKeys")),
        diagnostics: None,
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_action]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
#[ignore]
async fn watcher_updates_module_graph_simple() -> Result<()> {
    const FOO_CONTENT: &str = r#"import { bar } from "./bar.ts";

export function foo() {
    bar();
}
"#;
    const BAR_CONTENT: &str = r#"import { foo } from "./foo.ts";

export function bar() {
    foo();
}
"#;
    const BAR_CONTENT_FIXED: &str = r#"import { foo } from "./shared.ts";

export function bar() {
    foo();
}
"#;

    // ARRANGE: Set up FS and LSP connection in order to test import cycles.
    let mut fs = TemporaryFs::new("watcher_updates_module_graph");
    fs.create_file(
        "biome.json",
        r#"{
  "linter": {
    "enabled": true,
    "rules": {
      "nursery": {
        "noImportCycles": "error"
      }
    }
  }
}
"#,
    );

    fs.create_file("foo.ts", FOO_CONTENT);
    fs.create_file("bar.ts", BAR_CONTENT);

    let (watcher, instruction_channel) = Watcher::new(WatcherOptions::default())?;

    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());

    let workspace = factory.workspace();
    let db_state = factory.db_state();
    spawn_blocking(move || {
        workspace.start_watcher(&db_state, watcher);
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

    // ARRANGE: Scanning the project folder initializes the service data.
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

    let _: OpenFileResult = server
        .request(
            "biome/open_file",
            "open_file",
            OpenFileParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                editor_features: None,
            },
        )
        .await?
        .expect("open_file returned an error");

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::all(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: One diagnostic should be emitted for the cyclic dependency.
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        PrintDescription(&result.diagnostics[0]).to_string(),
        "This import is part of a cycle."
    );

    // ARRANGE: Remove `bar.ts`.
    clear_notifications!(factory.service_rx);
    std::fs::remove_file(fs.working_directory.join("bar.ts")).expect("Cannot remove bar.ts");
    await_notification!(factory.service_rx);

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::empty(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: Diagnostic should've disappeared because `bar.ts` is removed.
    assert_eq!(result.diagnostics.len(), 0);

    // ARRANGE: Recreate `bar.ts`.
    clear_notifications!(factory.service_rx);
    fs.create_file("bar.ts", BAR_CONTENT);
    await_notification!(factory.service_rx);

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::all(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: Diagnostic is expected to reappear.
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        PrintDescription(&result.diagnostics[0]).to_string(),
        "This import is part of a cycle."
    );

    // ARRANGE: Fix `bar.ts`.
    clear_notifications!(factory.service_rx);
    fs.create_file("bar.ts", BAR_CONTENT_FIXED);
    await_notification!(factory.service_rx);

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::all(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: Diagnostic should disappear again with a fixed `bar.ts`.
    assert_eq!(result.diagnostics.len(), 0);

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
#[ignore]
async fn watcher_updates_module_graph_with_directories() -> Result<()> {
    const FOO_CONTENT: &str = r#"import { bar } from "./utils/bar.ts";

export function foo() {
    bar();
}
"#;
    const BAR_CONTENT: &str = r#"import { foo } from "../foo.ts";

export function bar() {
    foo();
}
"#;

    // ARRANGE: Set up FS and LSP connection in order to test import cycles.
    let mut fs = TemporaryFs::new("watcher_updates_module_graph_with_directories");
    fs.create_file(
        "biome.json",
        r#"{
  "linter": {
    "enabled": true,
    "rules": {
      "nursery": {
        "noImportCycles": "error"
      }
    }
  }
}
"#,
    );

    fs.create_file("foo.ts", FOO_CONTENT);
    fs.create_file("utils/bar.ts", BAR_CONTENT);

    let (watcher, instruction_channel) = Watcher::new(WatcherOptions::default())?;

    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());

    let workspace = factory.workspace();
    let db_state = factory.db_state();
    spawn_blocking(move || {
        workspace.start_watcher(&db_state, watcher);
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

    // ARRANGE: Scanning the project folder initializes the service data.
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

    let _: OpenFileResult = server
        .request(
            "biome/open_file",
            "open_file",
            OpenFileParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                content: FileContent::FromServer,
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                editor_features: None,
            },
        )
        .await?
        .expect("open_file returned an error");

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::all(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: One diagnostic should be emitted for the cyclic dependency.
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        PrintDescription(&result.diagnostics[0]).to_string(),
        "This import is part of a cycle."
    );

    // ARRANGE: Move `utils` directory.
    clear_notifications!(factory.service_rx);
    std::fs::rename(
        fs.working_directory.join("utils"),
        fs.working_directory.join("bin"),
    )
    .expect("Cannot move utils");
    await_notification!(factory.service_rx);

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::empty(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: Diagnostic should've disappeared because `utils/bar.ts` is no
    //         longer there.
    assert_eq!(result.diagnostics.len(), 0);

    // ARRANGE: Move `utils` back.
    clear_notifications!(factory.service_rx);
    std::fs::rename(
        fs.working_directory.join("bin"),
        fs.working_directory.join("utils"),
    )
    .expect("Cannot restore utils");
    await_notification!(factory.service_rx);

    // ACT: Pull diagnostics.
    let result: PullDiagnosticsResult = server
        .request(
            "biome/pull_diagnostics",
            "pull_diagnostics",
            PullDiagnosticsParams {
                project_key,
                path: fs.working_directory.join("foo.ts").into(),
                categories: RuleCategories::all(),
                only: Vec::new(),
                skip: Vec::new(),
                enabled_rules: vec![RuleSelector::Rule("nursery", "noImportCycles").into()],
                include_code_fix: false,
                inline_config: None,
                max_diagnostics: None,
                diagnostic_level: biome_diagnostics::Severity::Hint,
                enforce_assist: false,
            },
        )
        .await?
        .expect("pull_diagnostics returned an error");

    // ASSERT: Diagnostic is expected to reappear.
    assert_eq!(result.diagnostics.len(), 1);
    assert_eq!(
        PrintDescription(&result.diagnostics[0]).to_string(),
        "This import is part of a cycle."
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[ignore]
#[tokio::test]
async fn should_open_and_update_nested_files() -> Result<()> {
    // ARRANGE: Set up folder.
    const FILE_PATH: &str = "src/a.js";
    const FILE_CONTENT_BEFORE: &str = "import 'foo';";
    const FILE_CONTENT_AFTER: &str = "import 'bar';";

    let mut fs = TemporaryFs::new("should_open_and_update_nested_files");
    fs.create_file(FILE_PATH, FILE_CONTENT_BEFORE);

    let (watcher, instruction_channel) = Watcher::new(WatcherOptions::default())?;

    // ARRANGE: Start server.
    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());

    let workspace = factory.workspace();
    let db_state = factory.db_state();
    spawn_blocking(move || {
        workspace.start_watcher(&db_state, watcher);
    });

    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;

    // ARRANGE: Open project.
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

    // ACT: Scanning the project folder initialises the service data.
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
        .await
        .expect("scan_project returned an error")
        .expect("result must not be empty");
    assert_eq!(result.diagnostics.len(), 0);

    // ASSERT: File should be indexed.
    let result: GetModuleGraphResult = server
        .request(
            "biome/get_module_graph",
            "get_module_graph",
            GetModuleGraphParams {},
        )
        .await
        .expect("get module graph error")
        .expect("result must not be empty");
    assert_eq!(
        result
            .data
            .get(fs.working_directory.join("src").join("a.js").as_str())
            .map(|module_info| module_info
                .as_js_module_info()
                .unwrap()
                .static_import_paths
                .clone()),
        Some(BTreeMap::from([("foo".to_string(), "foo".to_string())]))
    );

    // ACT: Update the file content.
    clear_notifications!(factory.service_rx);
    std::fs::write(
        fs.working_directory.join("src").join("a.js"),
        FILE_CONTENT_AFTER,
    )
    .expect("cannot update file");
    await_notification!(factory.service_rx);

    // ASSERT: Index should have updated.
    let result: GetModuleGraphResult = server
        .request(
            "biome/get_module_graph",
            "get_module_graph",
            GetModuleGraphParams {},
        )
        .await
        .expect("get module graph error")
        .expect("result must not be empty");
    assert_eq!(
        result
            .data
            .get(fs.working_directory.join("src").join("a.js").as_str())
            .map(|module_info| module_info
                .as_js_module_info()
                .unwrap()
                .static_import_paths
                .clone()),
        Some(BTreeMap::from([("bar".to_string(), "bar".to_string())]))
    );

    // ACT: Remove the directory.
    clear_notifications!(factory.service_rx);
    std::fs::remove_dir_all(fs.working_directory.join("src")).expect("cannot remove dir");
    await_notification!(factory.service_rx);

    // ASSERT: File should be unloaded from the index.
    let result: GetModuleGraphResult = server
        .request(
            "biome/get_module_graph",
            "get_module_graph",
            GetModuleGraphParams {},
        )
        .await
        .expect("get module graph error")
        .expect("result must not be empty");
    assert!(
        !result
            .data
            .contains_key(fs.working_directory.join("src").join("a.js").as_str())
    );

    // ARRANGE: Shutdown server.
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn did_save_syncs_content_from_text_parameter() -> Result<()> {
    const INITIAL_CONTENT: &str = "const   a=1;";
    const SAVED_CONTENT: &str = "const   b=2;";

    let fs = Arc::new(MemoryFileSystem::default());
    let file_path = to_utf8_file_path_buf(uri!("document.js"));
    fs.insert(file_path, INITIAL_CONTENT);

    let factory = ServerFactory::new_with_fs(fs.clone());
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document(INITIAL_CONTENT).await?;

    // Send didSave with text parameter (as per LSP spec)
    server
        .notify(
            "textDocument/didSave",
            DidSaveTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                text: Some(SAVED_CONTENT.to_string()),
            },
        )
        .await?;

    // Format the document to verify the content was updated
    let edits: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    // If content was properly updated to "const   b=2;",
    // formatting should add spaces around = and ;
    let edits = edits.context("formatting did not return edits")?;
    assert!(
        !edits.is_empty(),
        "Formatting should produce edits for 'const   b=2;'"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

// #region MONOREPO TESTS

#[tokio::test]
#[ignore]
async fn pull_diagnostics_monorepo() -> Result<()> {
    let fs = MemoryFileSystem::default();

    fs.insert(
        to_utf8_file_path_buf(uri!("biome.json")),
        r#"{
  "root": true,
  "linter": {
    "enabled": false
  }
}
"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("packages/lib/biome.json")),
        r#"{
  "linter": {
    "enabled": true
  }
}
"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("file.ts")),
        r#"const a = 1; a = 2;"#,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("packages/lib/file.ts")),
        r#"const a = 1; a = 2;"#,
    );

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    server
        .open_named_document(
            r#"const a = 1; a = 2;"#,
            uri!("packages/lib/file.ts"),
            "typescript",
        )
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;
    let notification = notification.unwrap();
    assert_diagnostics_count(&notification, 1);
    assert_diagnostic_code(&notification, "noConstAssign");

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_correctly_fix_all_astro_files() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_named_document(
            r#"---
loop: for (let i = 0; i < 5; i++) {
  continue loop;
}
---
<!doctype html>"#,
            uri!("document.astro"),
            "astro",
        )
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.astro"),
                },
                range: Range {
                    start: Position {
                        line: 1,
                        character: 7,
                    },
                    end: Position {
                        line: 1,
                        character: 7,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![],
                    only: Some(vec![CodeActionKind::new("source.fixAll.biome")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    let mut changes = HashMap::default();

    changes.insert(
        uri!("document.astro"),
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 6,
                    character: 0,
                },
            },
            new_text: String::from(
                r#"---
for (let i = 0; i < 5; i++) {}
---
<!doctype html>"#,
            ),
        }],
    );

    let expected_action = CodeActionOrCommand::CodeAction(CodeAction {
        title: String::from("Apply all safe fixes (Biome)"),
        kind: Some(CodeActionKind::new("source.fixAll.biome")),
        diagnostics: Some(vec![]),
        edit: Some(WorkspaceEdit {
            changes: Some(changes),
            document_changes: None,
            change_annotations: None,
        }),
        command: None,
        is_preferred: Some(true),
        disabled: None,
        data: None,
    });

    assert_eq!(res, vec![expected_action]);

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_not_return_error_on_code_actions_for_grit_files() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_named_document(r#"`console.log($args);`"#, uri!("example.grit"), "grit")
        .await?;

    let res: CodeActionResponse = server
        .request(
            "textDocument/codeAction",
            "pull_code_actions",
            CodeActionParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("example.grit"),
                },
                range: Range {
                    start: Position {
                        line: 0,
                        character: 0,
                    },
                    end: Position {
                        line: 0,
                        character: 21,
                    },
                },
                context: CodeActionContext {
                    diagnostics: vec![],
                    only: Some(vec![CodeActionKind::new("source.fixAll.biome")]),
                    ..Default::default()
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
                partial_result_params: PartialResultParams {
                    partial_result_token: None,
                },
            },
        )
        .await?
        .context("codeAction returned None")?;

    assert_eq!(res, vec![]);

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_apply_the_inline_configuration_when_formatting_a_file() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .load_configuration_with_settings(WorkspaceSettings {
            inline_config: Some(Configuration {
                formatter: Some(FormatterConfiguration {
                    enabled: Some(false.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from(r#"function f() {return "Foobar"}"#),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_none(),
        "It should not format the file because the inline settings disabled the formatter"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_acknowledge_changes_in_settings_in_formatting() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from(r#"function f() {return "Foobar"}"#),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_some(),
        "It should format the file because by the default the formatter is enabled"
    );

    sleep(Duration::from_millis(300)).await;

    server
        .load_configuration_with_settings(WorkspaceSettings {
            inline_config: Some(Configuration {
                formatter: Some(FormatterConfiguration {
                    enabled: Some(false.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    server
        .notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: String::from(r#"function f() {return "Foobar"}"#),
                },
            },
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_none(),
        "It should not format the file because the inline settings disabled the formatter now"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_apply_the_inline_configuration_when_pulling_diagnostics() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .load_configuration_with_settings(WorkspaceSettings {
            inline_config: Some(Configuration {
                linter: Some(LinterConfiguration {
                    enabled: Some(false.into()),
                    ..Default::default()
                }),
                assist: Some(AssistConfiguration {
                    enabled: Some(false.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;
    server.open_document("const a = 1; a = 2;").await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: Some(0),
                diagnostics: vec![],
            }
        )),
        "diagnostics should be empty because linting and assist are disabled"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_acknowledge_changes_in_settings_when_pulling_diagnostics() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.open_document("const a = 1; a = 2;").await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert!(notification.is_some());

    let notification = notification.expect("notification");
    assert!(matches!(
        notification,
        ServerNotification::PublishDiagnostics(_)
    ));
    if let ServerNotification::PublishDiagnostics(result) = notification {
        assert!(!result.diagnostics.is_empty(), "should contain diagnostics");
    }

    sleep(Duration::from_millis(300)).await;

    server
        .load_configuration_with_settings(WorkspaceSettings {
            inline_config: Some(Configuration {
                linter: Some(LinterConfiguration {
                    enabled: Some(false.into()),
                    ..Default::default()
                }),
                assist: Some(AssistConfiguration {
                    enabled: Some(false.into()),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        })
        .await?;

    server.open_document("const a = 1; a = 2;").await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: Some(0),
                diagnostics: vec![],
            }
        )),
        "diagnostics should be empty because linting and assist are disabled"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn should_apply_wrapped_biome_settings_from_did_change_configuration() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server
        .open_document("import { b, a } from \"./foo\";\n")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert!(notification.is_some());

    let notification = notification.expect("notification");
    assert!(matches!(
        notification,
        ServerNotification::PublishDiagnostics(_)
    ));
    if let ServerNotification::PublishDiagnostics(result) = notification {
        assert!(
            !result.diagnostics.is_empty(),
            "should contain diagnostics before applying wrapped biome settings"
        );
    }

    sleep(Duration::from_millis(300)).await;

    server
        .notify(
            "workspace/didChangeConfiguration",
            DidChangeConfigurationParams {
                settings: serde_json::json!({
                    "biome": {
                        "requireConfiguration": true,
                        "configurationPath": null,
                    }
                }),
            },
        )
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    assert_eq!(
        notification,
        Some(ServerNotification::PublishDiagnostics(
            PublishDiagnosticsParams {
                uri: uri!("document.js"),
                version: Some(0),
                diagnostics: vec![],
            }
        )),
        "diagnostics should be cleared after applying wrapped biome settings from didChangeConfiguration"
    );

    server.close_document().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

#[tokio::test]
async fn pull_plugin_diagnostics_for_vue_files() -> Result<()> {
    let fs = MemoryFileSystem::default();

    let config = r#"{
        "plugins": ["./noFoo.grit"]
    }"#;

    let plugin = br#"language js;

JsIdentifierBinding() as $name where {
    $name <: r"^foo$",
    register_diagnostic(
        span = $name,
        message = "Avoid using 'foo' as a variable name.",
        severity = "error"
    )
}
"#;

    fs.insert(to_utf8_file_path_buf(uri!("biome.json")), config);
    fs.insert(to_utf8_file_path_buf(uri!("noFoo.grit")), plugin.as_slice());

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    server.load_configuration().await?;

    // The template is intentionally multi-line so that if the diagnostic offset
    // is not applied, the reported span would point into the template instead of
    // the script section.
    let vue_file = r#"<template>
<p>line 1</p>
<p>line 2</p>
<p>line 3</p>
<p>line 4</p>
<p>line 5</p>
<p>line 6</p>
<p>line 7</p>
<p>line 8</p>
<p>line 9</p>
<p>line 10</p>
</template>

<script setup lang="ts">
const foo = 'bad'
</script>
"#;

    server
        .open_named_document(vue_file, uri!("file.vue"), "vue")
        .await?;

    let notification = wait_for_notification(&mut receiver, |n| n.is_publish_diagnostics()).await;

    // The plugin diagnostic for `foo` should point to line 14 (0-indexed),
    // character 6-9 in the full Vue file, not within the extracted script block.
    match &notification {
        Some(ServerNotification::PublishDiagnostics(params)) => {
            assert_eq!(params.uri, uri!("file.vue"));
            let plugin_diag = params
                .diagnostics
                .iter()
                .find(|d| d.message.contains("Avoid using 'foo'"))
                .expect("expected a plugin diagnostic for 'foo'");
            assert_eq!(
                plugin_diag.range,
                Range {
                    start: Position {
                        line: 14,
                        character: 6,
                    },
                    end: Position {
                        line: 14,
                        character: 9,
                    },
                },
                "plugin diagnostic should point to 'foo' on line 14 of the full Vue file"
            );
        }
        other => panic!("expected PublishDiagnostics, got {other:?}"),
    }

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

// #region CONFIGURATION PATH RESOLUTION

/// Verifies that a relative `configurationPath` in the extension settings is
/// resolved against the workspace root URI.
///
/// Regression test for <https://github.com/biomejs/biome/issues/9217>
#[tokio::test]
async fn relative_configuration_path_resolves_against_root_uri() -> Result<()> {
    let fs = MemoryFileSystem::default();

    // Place the config in a sub-directory so the path must be relative.
    let config = r#"{
        "formatter": {
            "enabled": false
        }
    }"#;
    fs.insert(to_utf8_file_path_buf(uri!("configs/biome.json")), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    // Set configurationPath to a relative path (the bug: this used to be
    // resolved against the daemon's cwd instead of the workspace root).
    server
        .load_configuration_with_settings(WorkspaceSettings {
            configuration_path: Some("configs/biome.json".to_string()),
            ..Default::default()
        })
        .await?;

    server.open_document(r#"statement(   );"#).await?;

    // The config disables the formatter, so formatting should return no edits.
    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_none(),
        "Expected no formatting edits because the config at configs/biome.json disables the formatter. \
         If this fails, the relative configurationPath was not resolved against the workspace root."
    );

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

/// Verifies that a relative `configurationPath` is resolved against the
/// workspace folder that **contains the file being opened**, not always the
/// first workspace folder.
///
/// Regression test for <https://github.com/biomejs/biome/issues/9217>
#[tokio::test]
#[expect(deprecated)]
async fn relative_configuration_path_resolves_against_correct_workspace_folder() -> Result<()> {
    let fs = MemoryFileSystem::default();

    // test_one has formatting enabled (default), test_two disables it.
    // Both configs live at `<folder>/configs/biome.json`.
    let config_one = r#"{}"#;
    let config_two = r#"{
        "formatter": {
            "enabled": false
        }
    }"#;

    fs.insert(
        to_utf8_file_path_buf(uri!("test_one/configs/biome.json")),
        config_one,
    );
    fs.insert(
        to_utf8_file_path_buf(uri!("test_two/configs/biome.json")),
        config_two,
    );

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    // Initialize with two workspace folders (test_one, test_two).
    let _res: InitializeResult = server
        .request(
            "initialize",
            "_init",
            InitializeParams {
                process_id: None,
                root_path: None,
                root_uri: Some(uri!("/")),
                initialization_options: None,
                capabilities: ClientCapabilities::default(),
                trace: None,
                workspace_folders: Some(vec![
                    WorkspaceFolder {
                        name: "test_one".to_string(),
                        uri: uri!("test_one"),
                    },
                    WorkspaceFolder {
                        name: "test_two".to_string(),
                        uri: uri!("test_two"),
                    },
                ]),
                client_info: None,
                locale: None,
                work_done_progress_params: Default::default(),
            },
        )
        .await?
        .context("initialize returned None")?;

    server.initialized().await?;

    // Set a relative configurationPath. Each workspace folder has its own
    // `configs/biome.json`; the correct one must be picked per file.
    server
        .load_configuration_with_settings(WorkspaceSettings {
            configuration_path: Some("configs/biome.json".to_string()),
            ..Default::default()
        })
        .await?;

    // Open a file in test_one first, so a project is already open.
    server
        .open_named_document(
            r#"statement(   );"#,
            uri!("test_one/document.js"),
            "javascript",
        )
        .await?;

    // Now open a file in test_two — its config disables the formatter.
    server
        .open_named_document(
            r#"statement(   );"#,
            uri!("test_two/document.js"),
            "javascript",
        )
        .await?;

    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("test_two/document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_none(),
        "Expected no formatting edits because test_two/configs/biome.json disables the formatter. \
         If this fails, the relative configurationPath was resolved against the wrong workspace folder."
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

/// Regression test for <https://github.com/biomejs/biome/issues/9566>.
///
/// In a multi-root workspace, a configuration that fails to load in one workspace
/// folder must not disable lint/assist diagnostics for files in another, healthy
/// workspace folder.
#[tokio::test]
#[expect(deprecated)]
async fn broken_configuration_in_one_workspace_folder_does_not_disable_another() -> Result<()> {
    let fs = MemoryFileSystem::default();

    // `good` has a valid config that enables a lint rule; `bad` has a malformed
    // config that cannot be parsed, so loading it yields an error status.
    let good_config = r#"{
        "linter": {
            "rules": {
                "recommended": false,
                "suspicious": { "noDoubleEquals": "error" }
            }
        }
    }"#;
    let bad_config = r#"{ "linter": { "rules": "#;

    fs.insert(to_utf8_file_path_buf(uri!("good/biome.json")), good_config);
    fs.insert(to_utf8_file_path_buf(uri!("bad/biome.json")), bad_config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, mut receiver) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    let _res: InitializeResult = server
        .request(
            "initialize",
            "_init",
            InitializeParams {
                process_id: None,
                root_path: None,
                root_uri: Some(uri!("/")),
                initialization_options: None,
                capabilities: ClientCapabilities::default(),
                trace: None,
                workspace_folders: Some(vec![
                    WorkspaceFolder {
                        name: "good".to_string(),
                        uri: uri!("good"),
                    },
                    WorkspaceFolder {
                        name: "bad".to_string(),
                        uri: uri!("bad"),
                    },
                ]),
                client_info: None,
                locale: None,
                work_done_progress_params: Default::default(),
            },
        )
        .await?
        .context("initialize returned None")?;

    server.initialized().await?;

    let good_uri = uri!("good/document.js");

    // Open the healthy file: it must produce a lint diagnostic.
    server
        .open_named_document("a == b;", good_uri.clone(), "javascript")
        .await?;

    let notification = wait_for_notification(
        &mut receiver,
        |n| matches!(n, ServerNotification::PublishDiagnostics(params) if params.uri == good_uri),
    )
    .await;
    let Some(ServerNotification::PublishDiagnostics(params)) = notification else {
        panic!("Expected PublishDiagnostics for the healthy file, got {notification:?}");
    };
    assert!(
        !params.diagnostics.is_empty(),
        "Expected a lint diagnostic for the healthy file on open"
    );

    // Open a file in the folder with the broken configuration. Its configuration
    // fails to load; previously this overwrote the shared status.
    server
        .open_named_document("a == b;", uri!("bad/document.js"), "javascript")
        .await?;

    // Re-trigger diagnostics for the healthy file. The lint diagnostic must still
    // be reported, proving the broken folder did not disable the healthy one.
    server
        .notify(
            "textDocument/didChange",
            lsp::DidChangeTextDocumentParams {
                text_document: lsp::VersionedTextDocumentIdentifier {
                    uri: good_uri.clone(),
                    version: 1,
                },
                content_changes: vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: "a == b;".to_string(),
                }],
            },
        )
        .await?;

    let notification = wait_for_notification(
        &mut receiver,
        |n| matches!(n, ServerNotification::PublishDiagnostics(params) if params.uri == good_uri),
    )
    .await;
    let Some(ServerNotification::PublishDiagnostics(params)) = notification else {
        panic!("Expected PublishDiagnostics for the healthy file, got {notification:?}");
    };
    assert!(
        !params.diagnostics.is_empty(),
        "The healthy file must keep its lint diagnostic even after a sibling \
         workspace folder failed to load its configuration"
    );

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

/// Verifies that an absolute `configurationPath` (e.g. `C:/shared-config/biome.json`)
/// pointing to a file outside the workspace roots properly attributes the registered
/// project to the workspace root, so that files opened inside the workspace are
/// matched correctly.
///
/// Regression test for external absolute config path bug (PR #9049).
#[tokio::test]
async fn absolute_configuration_path_resolves_outside_workspace() -> Result<()> {
    let fs = MemoryFileSystem::default();

    // The config lives outside the workspace.
    let external_config_path = to_utf8_file_path_buf(
        lsp::Uri::from_str(if cfg!(windows) {
            "file:///z%3A/shared-config/biome.json"
        } else {
            "file:///shared-config/biome.json"
        })
        .unwrap(),
    );

    let config = r#"{
        "formatter": {
            "enabled": true
        }
    }"#;

    fs.insert(external_config_path.clone(), config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let settings = WorkspaceSettings {
        configuration_path: Some(external_config_path.to_string()),
        ..Default::default()
    };

    // To reproduce the bug, the initial settings must have
    // `configuration_path` set. This matches what happens when an IDE starts.
    let reader = tokio::spawn(client_handler_with_settings(stream, sink, sender, settings));

    server.initialize().await?;
    server.initialized().await?;

    // Open a document inside the workspace.
    server.open_document("statement(   );\n").await?;

    // The document has extra whitespace, so there should be formatting changes.
    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_some(),
        "Expected formatting edits because the external config is enabled and there's extra spaces. \
         If this is None, the configurationPath caused the project to be created with the wrong path."
    );

    let edits = res.unwrap();
    assert_eq!(
        edits,
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 10,
                },
                end: Position {
                    line: 0,
                    character: 13,
                },
            },
            new_text: String::new(),
        }]
    );

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

/// Verifies that a relative `configurationPath` (e.g. `../shared-config/biome.json`)
/// that resolves to a file outside the workspace roots properly attributes the registered
/// project to the workspace root, so that files opened inside the workspace are
/// matched correctly.
///
/// Same scenario as [absolute_configuration_path_resolves_outside_workspace] but with
/// a relative path instead of an absolute one.
#[tokio::test]
async fn relative_configuration_path_resolves_outside_workspace() -> Result<()> {
    let fs = MemoryFileSystem::default();

    let absolute_external_config_path = to_utf8_file_path_buf(
        lsp::Uri::from_str(if cfg!(windows) {
            "file:///z%3A/shared-config/biome.json"
        } else {
            "file:///shared-config/biome.json"
        })
        .unwrap(),
    );

    let config = r#"{
        "formatter": {
            "enabled": true
        }
    }"#;
    fs.insert(absolute_external_config_path, config);

    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let settings = WorkspaceSettings {
        configuration_path: Some("../shared-config/biome.json".to_string()),
        ..Default::default()
    };

    let reader = tokio::spawn(client_handler_with_settings(stream, sink, sender, settings));

    server.initialize().await?;
    server.initialized().await?;

    // Open a document inside the workspace.
    server.open_document("statement(   );\n").await?;

    // The document has extra whitespace, so there should be formatting changes.
    let res: Option<Vec<TextEdit>> = server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
                options: FormattingOptions {
                    tab_size: 4,
                    insert_spaces: false,
                    properties: HashMap::default(),
                    trim_trailing_whitespace: None,
                    insert_final_newline: None,
                    trim_final_newlines: None,
                },
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned None")?;

    assert!(
        res.is_some(),
        "Expected formatting edits because the external config is enabled and there's extra spaces. \
         If this is None, the relative configurationPath caused the project to be created with the wrong path."
    );

    let edits = res.unwrap();
    assert_eq!(
        edits,
        vec![TextEdit {
            range: Range {
                start: Position {
                    line: 0,
                    character: 10,
                },
                end: Position {
                    line: 0,
                    character: 13,
                },
            },
            new_text: String::new(),
        }]
    );

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

// #endregion

// #region TEST UTILS

fn assert_diagnostic_code(server_notification: &ServerNotification, code: &str) {
    match server_notification {
        ServerNotification::PublishDiagnostics(publish) => {
            assert!(publish.diagnostics.iter().all(|d| {
                d.code
                    .as_ref()
                    .is_some_and(|c| &NumberOrString::String(code.to_string()) == c)
            }));
        }
        ServerNotification::ShowMessage(_) => {
            panic!("Unexpected notification: {server_notification:?}",);
        }
    }
}

fn assert_diagnostics_count(server_notification: &ServerNotification, expected_count: usize) {
    match server_notification {
        ServerNotification::PublishDiagnostics(publish) => {
            assert_eq!(publish.diagnostics.len(), expected_count)
        }
        ServerNotification::ShowMessage(_) => {
            panic!("Unexpected notification: {server_notification:?}",);
        }
    }
}

/// Regression test: an inverted LSP range in a `textDocument/didChange` event
/// must not panic inside `TextRange::new`. The invalid change should be
/// silently skipped and the document content must remain unchanged.
#[tokio::test]
async fn change_document_inverted_range_does_not_panic() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;

    let original = "abc\ndef\nghi";
    server.open_document(original).await?;

    // Send a change with an inverted range (start is after end)
    server
        .change_document(
            1,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range {
                    start: Position {
                        line: 1,
                        character: 3,
                    },
                    end: Position {
                        line: 0,
                        character: 0,
                    },
                }),
                range_length: None,
                text: String::from("replaced"),
            }],
        )
        .await?;

    let OpenProjectResult { project_key } = server
        .request(
            "biome/open_project",
            "open_project",
            OpenProjectParams {
                path: BiomePath::new(""),
                open_uninitialized: true,
            },
        )
        .await?
        .expect("open_project returned an error");

    let actual: String = server
        .request(
            "biome/get_file_content",
            "get_file_content",
            GetFileContentParams {
                project_key,
                path: BiomePath::try_from(uri!("document.js").to_file_path().unwrap()).unwrap(),
            },
        )
        .await?
        .context("get file content error")?;

    // The inverted range should be skipped; document content stays the same
    assert_eq!(&actual, original);

    server.close_document().await?;
    server.shutdown().await?;
    reader.abort();

    Ok(())
}

/// Regression test: the LSP server should not crash when the client sends
/// `didChangeWatchedFiles.dynamicRegistration: true` but no `workspaceFolders`
/// in `InitializeParams`. This is valid per the LSP spec — `workspaceFolders`
/// is optional and some clients only send `rootUri`.
#[tokio::test]
#[expect(deprecated)]
async fn initialize_without_workspace_folders_does_not_panic() -> Result<()> {
    let factory = ServerFactory::default();
    let (service, client) = factory.create().into_inner();
    let (stream, sink) = client.split();
    let mut server = Server::new(service);

    let (sender, _) = channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    let _res: InitializeResult = server
        .request(
            "initialize",
            "_init",
            InitializeParams {
                process_id: None,
                root_path: None,
                root_uri: Some(uri!("")),
                initialization_options: None,
                capabilities: ClientCapabilities {
                    workspace: Some(lsp::WorkspaceClientCapabilities {
                        did_change_watched_files: Some(
                            lsp::DidChangeWatchedFilesClientCapabilities {
                                dynamic_registration: Some(true),
                                relative_pattern_support: None,
                            },
                        ),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                trace: None,
                workspace_folders: None,
                client_info: None,
                locale: None,
                work_done_progress_params: Default::default(),
            },
        )
        .await?
        .context("initialize returned None")?;

    // `initialized` triggers `setup_capabilities` which registers file watchers.
    // Before the fix, this panicked because it tried to parse a filesystem path as a URI.
    server.initialized().await?;

    server.shutdown().await?;
    reader.abort();

    Ok(())
}

// #endregion

// #region Multiple clients sharing a document

/// A client connected to a shared [ServerFactory], i.e. an editor connected to
/// the daemon while other editors are connected too.
struct ConnectedClient {
    server: Server,
    reader: tokio::task::JoinHandle<Result<()>>,
    drain: Option<tokio::task::JoinHandle<()>>,
}

impl ConnectedClient {
    async fn connect(factory: &ServerFactory) -> Result<Self> {
        let (client, mut receiver) = Self::connect_with_notifications(factory).await?;
        // Keep the client handler alive for the whole test by draining the
        // notifications it forwards, so requests from the server never block.
        let drain = tokio::spawn(async move { while receiver.next().await.is_some() {} });
        Ok(Self {
            drain: Some(drain),
            ..client
        })
    }

    /// Like [`Self::connect`], but hands the notifications sent by the server
    /// to the caller, who must keep draining them.
    async fn connect_with_notifications(
        factory: &ServerFactory,
    ) -> Result<(Self, futures::channel::mpsc::Receiver<ServerNotification>)> {
        let (service, client) = factory.create().into_inner();
        let (stream, sink) = client.split();
        let mut server = Server::new(service);

        let (sender, receiver) = channel(CHANNEL_BUFFER_SIZE);
        let reader = tokio::spawn(client_handler(stream, sink, sender));

        server.initialize().await?;
        server.initialized().await?;

        Ok((
            Self {
                server,
                reader,
                drain: None,
            },
            receiver,
        ))
    }

    async fn open_document_with_version(&mut self, text: &str, version: i32) -> Result<()> {
        self.server
            .notify(
                "textDocument/didOpen",
                DidOpenTextDocumentParams {
                    text_document: TextDocumentItem {
                        uri: uri!("document.js"),
                        language_id: String::from("javascript"),
                        version,
                        text: text.to_string(),
                    },
                },
            )
            .await
    }

    async fn replace_document(&mut self, text: &str, version: i32) -> Result<()> {
        self.server
            .change_document(
                version,
                vec![TextDocumentContentChangeEvent {
                    range: None,
                    range_length: None,
                    text: text.to_string(),
                }],
            )
            .await
    }

    /// Requests formatting, re-sending the request when the server answers
    /// `ContentModified`, as an editor does.
    async fn format_document(&mut self) -> Result<Vec<TextEdit>> {
        for _ in 0..5 {
            match self.request_formatting().await {
                Err(err)
                    if err
                        .downcast_ref::<jsonrpc::Error>()
                        .is_some_and(|err| err.code == jsonrpc::ErrorCode::ContentModified) => {}
                result => return result,
            }
        }
        bail!("the server kept answering formatting with ContentModified")
    }

    async fn request_formatting(&mut self) -> Result<Vec<TextEdit>> {
        let res: Option<Vec<TextEdit>> = self
            .server
            .request(
                "textDocument/formatting",
                "formatting",
                DocumentFormattingParams {
                    text_document: TextDocumentIdentifier {
                        uri: uri!("document.js"),
                    },
                    options: FormattingOptions::default(),
                    work_done_progress_params: WorkDoneProgressParams {
                        work_done_token: None,
                    },
                },
            )
            .await?
            .context("formatting returned None")?;
        res.context("formatting did not return an edit list")
    }

    async fn shutdown(mut self) -> Result<()> {
        self.server.shutdown().await?;
        self.reader.abort();
        if let Some(drain) = self.drain {
            drain.abort();
        }
        Ok(())
    }
}

/// Waits until the server has sent no diagnostics for `idle`, and panics if
/// it keeps sending them for more than five seconds.
async fn wait_for_diagnostics_to_settle(
    receiver: &mut futures::channel::mpsc::Receiver<ServerNotification>,
    idle: Duration,
) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    loop {
        match tokio::time::timeout(idle, receiver.next()).await {
            Ok(Some(notification)) if notification.is_publish_diagnostics() => {
                assert!(
                    tokio::time::Instant::now() < deadline,
                    "the server kept publishing diagnostics without any change"
                );
            }
            Ok(Some(_)) => {}
            Ok(None) | Err(_) => return,
        }
    }
}

const SHARED_V1: &str = "const a  =  1;\nconst b  =  2;\n";
const SHARED_V1_FORMATTED: &str = "const a = 1;\nconst b = 2;\n";
/// Client A's unsaved edit: a line is added on top so every offset shifts,
/// and a statement is appended so the formatted output is longer.
const SHARED_V2: &str = "// changed by client A\nconst a  =  1;\nconst b  =  2;\nconst c  =  3;\n";
const SHARED_V2_FORMATTED: &str =
    "// changed by client A\nconst a = 1;\nconst b = 2;\nconst c = 3;\n";

#[tokio::test]
async fn formatting_with_two_clients_does_not_mix_documents() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client_a = ConnectedClient::connect(&factory).await?;
    let mut client_b = ConnectedClient::connect(&factory).await?;

    client_a.open_document_with_version(SHARED_V1, 0).await?;
    client_b.open_document_with_version(SHARED_V1, 0).await?;

    // Only client A edits its buffer.
    client_a.replace_document(SHARED_V2, 1).await?;

    // Client B still has the original text, so the edits it receives must
    // format *that* text, not client A's.
    let edits = client_b.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits), SHARED_V1_FORMATTED);

    let edits = client_a.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V2, edits), SHARED_V2_FORMATTED);

    // And client B again, now that client A has been formatted last.
    let edits = client_b.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits), SHARED_V1_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn did_change_with_two_clients_applies_to_own_text() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client_a = ConnectedClient::connect(&factory).await?;
    let mut client_b = ConnectedClient::connect(&factory).await?;

    client_a.open_document_with_version(SHARED_V1, 0).await?;
    client_b.open_document_with_version(SHARED_V1, 0).await?;

    client_a.replace_document(SHARED_V2, 1).await?;

    // Client B replaces the `1` in `const a  =  1;` with `10`. The range only
    // makes sense on client B's own text.
    client_b
        .server
        .change_document(
            1,
            vec![TextDocumentContentChangeEvent {
                range: Some(Range::new(Position::new(0, 12), Position::new(0, 13))),
                range_length: None,
                text: String::from("10"),
            }],
        )
        .await?;
    let expected_b = "const a  =  10;\nconst b  =  2;\n";
    let expected_b_formatted = "const a = 10;\nconst b = 2;\n";

    let edits = client_b.format_document().await?;
    assert_eq!(apply_text_edits(expected_b, edits), expected_b_formatted);

    let edits = client_a.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V2, edits), SHARED_V2_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn did_change_with_lower_version_from_other_client_is_not_dropped() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client_a = ConnectedClient::connect(&factory).await?;
    let mut client_b = ConnectedClient::connect(&factory).await?;

    // Version counters are per client, so client B's are far behind client A's.
    client_a.open_document_with_version(SHARED_V1, 50).await?;
    client_b.open_document_with_version(SHARED_V1, 3).await?;

    client_a.replace_document(SHARED_V2, 51).await?;

    let text_b = "const   x = 'b';\n";
    let text_b_formatted = "const x = \"b\";\n";
    client_b.replace_document(text_b, 4).await?;

    let edits = client_b.format_document().await?;
    assert_eq!(apply_text_edits(text_b, edits), text_b_formatted);

    let edits = client_a.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V2, edits), SHARED_V2_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn closing_document_in_one_client_keeps_other_client_working() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client_a = ConnectedClient::connect(&factory).await?;
    let mut client_b = ConnectedClient::connect(&factory).await?;

    client_a.open_document_with_version(SHARED_V1, 0).await?;
    client_b.open_document_with_version(SHARED_V1, 0).await?;

    client_b.server.close_document().await?;

    let edits = client_a.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits), SHARED_V1_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

/// Writing an indexed file broadcasts an index update, which makes every
/// session refresh its diagnostics. Two clients holding different texts of
/// the same file must not keep re-opening it in turn.
#[tokio::test]
async fn diagnostics_settle_with_two_clients_on_indexed_file() -> Result<()> {
    let path = to_utf8_file_path_buf(uri!("document.js"));
    let fs = MemoryFileSystem::default();
    fs.insert(path.clone(), SHARED_V1);
    let factory = ServerFactory::new_with_fs(Arc::new(fs));

    let (mut client_a, mut notifications_a) =
        ConnectedClient::connect_with_notifications(&factory).await?;
    let (mut client_b, mut notifications_b) =
        ConnectedClient::connect_with_notifications(&factory).await?;

    // Put the file in the module graph, as the scanner would.
    let db_state = factory.db_state();
    let workspace = factory.workspace();
    let workspace = workspace.with_db_state(&db_state);
    let OpenProjectResult { project_key } = workspace.open_project(OpenProjectParams {
        path: BiomePath::new(to_utf8_file_path_buf(uri!(""))),
        open_uninitialized: true,
    })?;
    workspace.index_files_for_test(
        project_key,
        [(
            BiomePath::new(path.clone()),
            DocumentFileSource::from_path(&path, false),
        )],
    );

    client_a.open_document_with_version(SHARED_V1, 0).await?;
    client_b.open_document_with_version(SHARED_V1, 0).await?;
    client_a.replace_document(SHARED_V2, 1).await?;

    // Each client gets the diagnostics of its own text...
    wait_for_notification(&mut notifications_a, |notification| {
        matches!(
            notification,
            ServerNotification::PublishDiagnostics(params) if params.version == Some(1)
        )
    })
    .await;
    wait_for_notification(&mut notifications_b, |notification| {
        notification.is_publish_diagnostics()
    })
    .await;

    // ...and then the refreshes stop: nothing changes anymore, so nobody may
    // keep re-opening the file with its own text.
    tokio::join!(
        wait_for_diagnostics_to_settle(&mut notifications_a, Duration::from_millis(750)),
        wait_for_diagnostics_to_settle(&mut notifications_b, Duration::from_millis(750)),
    );

    // A configuration change refreshes every document of a client with its
    // own text, which is allowed to write to the workspace and must settle
    // as well.
    client_b.server.load_configuration().await?;
    wait_for_notification(&mut notifications_b, |notification| {
        notification.is_publish_diagnostics()
    })
    .await;
    client_a.server.load_configuration().await?;
    wait_for_notification(&mut notifications_a, |notification| {
        matches!(
            notification,
            ServerNotification::PublishDiagnostics(params) if params.version == Some(1)
        )
    })
    .await;
    tokio::join!(
        wait_for_diagnostics_to_settle(&mut notifications_a, Duration::from_millis(750)),
        wait_for_diagnostics_to_settle(&mut notifications_b, Duration::from_millis(750)),
    );

    // Both clients still get edits for their own text.
    let edits = client_b.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits), SHARED_V1_FORMATTED);
    let edits = client_a.format_document().await?;
    assert_eq!(apply_text_edits(SHARED_V2, edits), SHARED_V2_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

/// The workspace doesn't store dependency files a client opens. Such a file
/// must not be re-opened, and so parsed again, by every request made on it.
#[tokio::test]
async fn requests_on_dependency_files_do_not_reopen_them() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client = ConnectedClient::connect(&factory).await?;
    let uri = uri!("node_modules/dep/index.js");
    client
        .server
        .open_named_document(SHARED_V1, uri.clone(), "javascript")
        .await?;

    // Any `open_file` from now on is a re-open.
    let reopened = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag = reopened.clone();
    factory
        .workspace()
        .set_hook_between_parse_and_content_update(move |_| {
            flag.store(true, std::sync::atomic::Ordering::Release);
        });

    let res: Option<Vec<TextEdit>> = client
        .server
        .request(
            "textDocument/formatting",
            "formatting",
            DocumentFormattingParams {
                text_document: TextDocumentIdentifier { uri },
                options: FormattingOptions::default(),
                work_done_progress_params: WorkDoneProgressParams {
                    work_done_token: None,
                },
            },
        )
        .await?
        .context("formatting returned no response")?;
    assert!(res.is_none(), "dependency files are not formatted");
    assert!(
        !reopened.load(std::sync::atomic::Ordering::Acquire),
        "the request re-opened a file the workspace doesn't store"
    );

    client.shutdown().await?;
    Ok(())
}

/// The workspace stores the parsed file before the document content. A
/// request from client A that runs while client B's change sits between the
/// two writes must not format B's parse and return the edits to A.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn formatting_does_not_observe_another_clients_change_between_writes() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client_a = ConnectedClient::connect(&factory).await?;
    let mut client_b = ConnectedClient::connect(&factory).await?;

    client_a.open_document_with_version(SHARED_V1, 0).await?;
    client_b.open_document_with_version(SHARED_V1, 0).await?;
    // Let the diagnostics triggered by the opens run before we block a writer.
    sleep(Duration::from_millis(500)).await;

    // Client B's change will stop between the two writes until we say so.
    let (in_window_tx, in_window_rx) = tokio::sync::oneshot::channel::<()>();
    let (proceed_tx, proceed_rx) = std::sync::mpsc::channel::<()>();
    factory
        .workspace()
        .set_hook_between_parse_and_content_update(move |_| {
            let _ = in_window_tx.send(());
            let _ = proceed_rx.recv_timeout(Duration::from_secs(5));
        });
    let change_b = tokio::spawn(async move {
        let result = client_b.replace_document(SHARED_V2, 1).await;
        (client_b, result)
    });
    in_window_rx.await?;

    // Client A asks for formatting while B's parse is stored but B's content
    // isn't yet.
    let format_a = tokio::spawn(async move {
        let result = client_a.format_document().await;
        (client_a, result)
    });
    sleep(Duration::from_millis(200)).await;
    proceed_tx.send(())?;

    let (client_b, change_result) = change_b.await?;
    change_result?;
    let (client_a, edits) = format_a.await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits?), SHARED_V1_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

/// Workspace clients such as the CLI or the JS API write through
/// `biome/change_file`, outside any LSP text synchronization. Their writes
/// must not be observable between the two stores either.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn formatting_does_not_observe_a_workspace_client_change_between_writes() -> Result<()> {
    let factory = ServerFactory::default();
    let mut client_a = ConnectedClient::connect(&factory).await?;
    let mut client_b = ConnectedClient::connect(&factory).await?;

    client_a.open_document_with_version(SHARED_V1, 0).await?;
    client_b.open_document_with_version(SHARED_V1, 0).await?;
    sleep(Duration::from_millis(500)).await;

    let OpenProjectResult { project_key } = client_b
        .server
        .request(
            "biome/open_project",
            "open_project",
            OpenProjectParams {
                path: BiomePath::new(to_utf8_file_path_buf(uri!(""))),
                open_uninitialized: true,
            },
        )
        .await?
        .context("open_project returned None")?;

    let (in_window_tx, in_window_rx) = tokio::sync::oneshot::channel::<()>();
    let (proceed_tx, proceed_rx) = std::sync::mpsc::channel::<()>();
    factory
        .workspace()
        .set_hook_between_parse_and_content_update(move |_| {
            let _ = in_window_tx.send(());
            let _ = proceed_rx.recv_timeout(Duration::from_secs(5));
        });
    let change_b = tokio::spawn(async move {
        let result: Result<Option<ChangeFileResult>> = client_b
            .server
            .request(
                "biome/change_file",
                "change_file",
                ChangeFileParams {
                    project_key,
                    path: BiomePath::new(to_utf8_file_path_buf(uri!("document.js"))),
                    content: SHARED_V2.to_string(),
                    version: 1,
                    inline_config: None,
                    editor_features: None,
                },
            )
            .await;
        (client_b, result)
    });
    in_window_rx.await?;

    let format_a = tokio::spawn(async move {
        let result = client_a.format_document().await;
        (client_a, result)
    });
    sleep(Duration::from_millis(200)).await;
    proceed_tx.send(())?;

    let (client_b, change_result) = change_b.await?;
    change_result?.context("change_file returned None")?;
    let (client_a, edits) = format_a.await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits?), SHARED_V1_FORMATTED);

    client_a.shutdown().await?;
    client_b.shutdown().await?;
    Ok(())
}

/// The scanner writes files it indexes, and the watcher re-indexes a file
/// that changed on disk while an editor has it open. Such a write goes through
/// the same two stores, and must not be observable in between either.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn formatting_does_not_observe_a_scanner_write_between_writes() -> Result<()> {
    let path = to_utf8_file_path_buf(uri!("document.js"));
    let fs = MemoryFileSystem::default();
    // What is on disk differs from what the editor holds.
    fs.insert(path.clone(), SHARED_V2);
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let mut client = ConnectedClient::connect(&factory).await?;

    client.open_document_with_version(SHARED_V1, 0).await?;
    sleep(Duration::from_millis(500)).await;

    let db_state = factory.db_state();
    let workspace = factory.workspace();
    let OpenProjectResult { project_key } =
        workspace
            .with_db_state(&db_state)
            .open_project(OpenProjectParams {
                path: BiomePath::new(to_utf8_file_path_buf(uri!(""))),
                open_uninitialized: true,
            })?;

    let (in_window_tx, in_window_rx) = tokio::sync::oneshot::channel::<()>();
    let (proceed_tx, proceed_rx) = std::sync::mpsc::channel::<()>();
    workspace.set_hook_between_parse_and_content_update(move |_| {
        let _ = in_window_tx.send(());
        let _ = proceed_rx.recv_timeout(Duration::from_secs(5));
    });
    let index_path = path.clone();
    let index = tokio::task::spawn_blocking(move || {
        workspace.with_db_state(&db_state).index_files_for_test(
            project_key,
            [(
                BiomePath::new(index_path.clone()),
                DocumentFileSource::from_path(&index_path, false),
            )],
        );
    });
    in_window_rx.await?;

    let format = tokio::spawn(async move {
        let result = client.format_document().await;
        (client, result)
    });
    sleep(Duration::from_millis(200)).await;
    proceed_tx.send(())?;

    index.await?;
    let (client, edits) = format.await?;
    assert_eq!(apply_text_edits(SHARED_V1, edits?), SHARED_V1_FORMATTED);

    client.shutdown().await?;
    Ok(())
}

/// The scanner may reach a file before any editor has it open. When an editor
/// opens it while the scanner is about to store its parse, the scanner's
/// parse must not land next to the editor's content: a request holding the
/// document lock would format the file on disk and map the edits onto the
/// editor's buffer.
///
/// The reader is driven through the workspace directly, because a request
/// handler offers no way to hold it between taking the lock and reading. The
/// orchestration doesn't use the runtime's timers either: a task blocked on
/// the document lock, like the diagnostics scheduled by `didOpen`, stalls
/// them.
#[tokio::test(flavor = "multi_thread", worker_threads = 8)]
async fn scanner_write_started_before_first_open_is_not_observed() -> Result<()> {
    let path = to_utf8_file_path_buf(uri!("document.js"));
    let fs = MemoryFileSystem::default();
    // What is on disk differs from what the editor holds.
    fs.insert(path.clone(), SHARED_V2);
    let factory = ServerFactory::new_with_fs(Arc::new(fs));
    let mut client = ConnectedClient::connect(&factory).await?;

    let db_state = factory.db_state();
    let workspace = factory.workspace();
    let OpenProjectResult { project_key } =
        workspace
            .with_db_state(&db_state)
            .open_project(OpenProjectParams {
                path: BiomePath::new(to_utf8_file_path_buf(uri!(""))),
                open_uninitialized: true,
            })?;

    // Hold the scanner right before it stores its parse.
    let (in_window_tx, in_window_rx) = tokio::sync::oneshot::channel::<()>();
    let (proceed_tx, proceed_rx) = std::sync::mpsc::channel::<()>();
    workspace.set_hook_before_parse_update(move |_| {
        let _ = in_window_tx.send(());
        let _ = proceed_rx.recv_timeout(Duration::from_secs(5));
    });
    let index_workspace = workspace.clone();
    let index_db_state = db_state.clone();
    let index_path = path.clone();
    let index = spawn_blocking(move || {
        index_workspace
            .with_db_state(&index_db_state)
            .index_files_for_test(
                project_key,
                [(
                    BiomePath::new(index_path.clone()),
                    DocumentFileSource::from_path(&index_path, false),
                )],
            );
    });
    in_window_rx.await?;

    // The editor opens the file while the scanner is held. Not awaited: when
    // the scanner holds the lock, this completes only once it is released.
    let open = tokio::spawn(async move {
        let result = client.open_document_with_version(SHARED_V1, 0).await;
        (client, result)
    });

    // A reader takes the lock once the document is stored, as every request
    // handler does, and reads the parsed file and the content once the
    // scanner had the chance to write.
    let (read_tx, read_rx) = std::sync::mpsc::channel::<()>();
    let reader_path = path.clone();
    let reader = spawn_blocking(move || {
        let workspace = workspace.with_db_state(&db_state);
        let stored = || {
            workspace.file_exists(FileExistsParams {
                file_path: BiomePath::new(reader_path.clone()),
            })
        };
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        while !stored()? && std::time::Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        let _guard = workspace.lock_document(&reader_path);
        let _ = read_rx.recv_timeout(Duration::from_secs(5));
        let printed = workspace.format_file(FormatFileParams {
            project_key,
            path: BiomePath::new(reader_path.clone()),
            inline_config: None,
        })?;
        let content = workspace.get_file_content(GetFileContentParams {
            project_key,
            path: BiomePath::new(reader_path),
        })?;
        Ok::<_, anyhow::Error>((printed.into_code(), content))
    });
    spawn_blocking(|| std::thread::sleep(Duration::from_millis(200))).await?;
    proceed_tx.send(())?;
    spawn_blocking(|| std::thread::sleep(Duration::from_millis(200))).await?;
    read_tx.send(())?;

    let (printed, content) = reader.await??;
    let expected = match content.as_str() {
        SHARED_V1 => SHARED_V1_FORMATTED,
        SHARED_V2 => SHARED_V2_FORMATTED,
        other => bail!("unexpected content: {other:?}"),
    };
    assert_eq!(printed, expected, "the parsed file was not the content's");

    index.await?;
    let (client, opened) = open.await?;
    opened?;
    client.shutdown().await?;
    Ok(())
}

// #endregion
