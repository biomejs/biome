#![expect(clippy::mutable_key_type)]
use std::any::type_name;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::slice;
use std::str::FromStr;

use anyhow::{Context, Error, Result, bail};
use biome_analyze::RuleCategories;
use biome_configuration::analyzer::RuleSelector;
use biome_diagnostics::PrintDescription;
use biome_fs::{BiomePath, MemoryFileSystem, TemporaryFs};
use biome_service::Watcher;
use biome_service::workspace::{
    FileContent, GetFileContentParams, GetModuleGraphParams, GetModuleGraphResult,
    GetSyntaxTreeParams, GetSyntaxTreeResult, OpenFileParams, OpenFileResult, OpenProjectParams,
    OpenProjectResult, PullDiagnosticsParams, PullDiagnosticsResult, ScanKind, ScanProjectParams,
    ScanProjectResult,
};
use camino::Utf8PathBuf;
use futures::channel::mpsc::{Sender, channel};
use futures::{Sink, SinkExt, Stream, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_value, to_value};
use tokio::time::{Duration, sleep, timeout};
use tower::timeout::Timeout;
use tower::{Service, ServiceExt};
use tower_lsp_server::LspService;
use tower_lsp_server::jsonrpc::{self, Request, Response};
use tower_lsp_server::lsp_types::{
    self as lsp, ClientCapabilities, CodeDescription, DidChangeConfigurationParams,
    DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams,
    DocumentFormattingParams, FormattingOptions, InitializeParams, InitializeResult,
    InitializedParams, Position, PublishDiagnosticsParams, Range, TextDocumentContentChangeEvent,
    TextDocumentIdentifier, TextDocumentItem, TextEdit, Uri, VersionedTextDocumentIdentifier,
    WorkDoneProgressParams, WorkspaceFolder,
};

use crate::WorkspaceSettings;

use super::*;

/// Statically build an [Uri] instance that points to the file at `$path`
/// within the workspace. The filesystem path contained in the return URI is
/// guaranteed to be a valid path for the underlying operating system, but
/// doesn't have to refer to an existing file on the host machine.
macro_rules! uri {
    ($path:literal) => {
        if cfg!(windows) {
            lsp::Uri::from_str(concat!("file:///z%3A/workspace/", $path)).unwrap()
        } else {
            lsp::Uri::from_str(concat!("file:///workspace/", $path)).unwrap()
        }
    };
}

macro_rules! await_notification {
    ($channel:expr) => {
        sleep(Duration::from_millis(200)).await;

        timeout(Duration::from_secs(2), $channel.changed())
            .await
            .expect("expected notification within timeout")
            .expect("expected notification");
    };
}

macro_rules! clear_notifications {
    ($channel:expr) => {
        // On Windows, wait until any previous event has been delivered.
        // On macOS, wait until the fsevents watcher sets up before receiving the first event.
        let duration = if cfg!(any(target_os = "windows", target_os = "macos")) {
            Duration::from_secs(1)
        } else {
            Duration::from_millis(200)
        };
        sleep(duration).await;

        if $channel
            .has_changed()
            .expect("Channel should not be closed")
        {
            let _ = $channel.changed().await;
        }
    };
}

fn to_utf8_file_path_buf(uri: Uri) -> Utf8PathBuf {
    Utf8PathBuf::from_path_buf(uri.to_file_path().unwrap().to_path_buf()).unwrap()
}

fn fixable_diagnostic(line: u32) -> Result<Diagnostic> {
    Ok(Diagnostic {
        range: Range {
            start: Position { line, character: 3 },
            end: Position {
                line,
                character: 11,
            },
        },
        severity: Some(DiagnosticSeverity::ERROR),
        code: Some(NumberOrString::String(String::from(
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

struct Server {
    service: Timeout<LspService<LSPServer>>,
}

impl Server {
    fn new(service: LspService<LSPServer>) -> Self {
        Self {
            service: Timeout::new(service, Duration::from_secs(1)),
        }
    }

    async fn notify<P>(&mut self, method: &'static str, params: P) -> Result<()>
    where
        P: Serialize,
    {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(
                Request::build(method)
                    .params(to_value(&params).context("failed to serialize params")?)
                    .finish(),
            )
            .await
            .map_err(Error::msg)
            .context("call() returned an error")
            .and_then(|res| {
                if let Some(res) = res {
                    bail!("shutdown returned {:?}", res)
                } else {
                    Ok(())
                }
            })
    }

    async fn request<P, R>(
        &mut self,
        method: &'static str,
        id: &'static str,
        params: P,
    ) -> Result<Option<R>>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(
                Request::build(method)
                    .id(id)
                    .params(to_value(&params).context("failed to serialize params")?)
                    .finish(),
            )
            .await
            .map_err(Error::msg)
            .context("call() returned an error")?
            .map(|res| {
                let (_, body) = res.into_parts();

                let body =
                    body.with_context(|| format!("response to {method:?} contained an error"))?;

                from_value(body.clone()).with_context(|| {
                    format!(
                        "failed to deserialize type {} from response {body:?}",
                        type_name::<R>()
                    )
                })
            })
            .transpose()
    }

    /// Basic implementation of the `initialize` request for tests
    // The `root_path` field is deprecated, but we still need to specify it
    #[expect(deprecated)]
    async fn initialize(&mut self) -> Result<()> {
        let _res: InitializeResult = self
            .request(
                "initialize",
                "_init",
                InitializeParams {
                    process_id: None,
                    root_path: None,
                    root_uri: Some(uri!("")),
                    initialization_options: None,
                    capabilities: ClientCapabilities::default(),
                    trace: None,
                    workspace_folders: None,
                    client_info: None,
                    locale: None,
                    work_done_progress_params: Default::default(),
                },
            )
            .await?
            .context("initialize returned None")?;

        Ok(())
    }

    /// It creates two projects, one at folder `test_one` and the other in `test_two`.
    ///
    /// Hence, the two roots will be `/workspace/test_one` and `/workspace/test_two`
    // The `root_path` field is deprecated, but we still need to specify it
    #[expect(deprecated)]
    async fn initialize_projects(&mut self) -> Result<()> {
        let _res: InitializeResult = self
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

        Ok(())
    }

    /// Basic implementation of the `initialized` notification for tests
    async fn initialized(&mut self) -> Result<()> {
        self.notify("initialized", InitializedParams {}).await
    }

    /// Basic implementation of the `shutdown` notification for tests
    async fn shutdown(&mut self) -> Result<()> {
        self.service
            .ready()
            .await
            .map_err(Error::msg)
            .context("ready() returned an error")?
            .call(Request::build("shutdown").finish())
            .await
            .map_err(Error::msg)
            .context("call() returned an error")
            .and_then(|res| {
                if let Some(res) = res {
                    bail!("shutdown returned {:?}", res)
                } else {
                    Ok(())
                }
            })
    }

    async fn open_document(&mut self, text: impl Display) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("document.js"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    async fn open_untitled_document(&mut self, text: impl Display) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: uri!("untitled-1"),
                    language_id: String::from("javascript"),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    /// Opens a document with given contents and given name. The name must contain the extension too
    async fn open_named_document(
        &mut self,
        text: impl Display,
        document_name: Uri,
        language: impl Display,
    ) -> Result<()> {
        self.notify(
            "textDocument/didOpen",
            DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: document_name,
                    language_id: language.to_string(),
                    version: 0,
                    text: text.to_string(),
                },
            },
        )
        .await
    }

    /// When calling this function, remember to insert the file inside the memory file system
    async fn load_configuration(&mut self) -> Result<()> {
        self.notify(
            "workspace/didChangeConfiguration",
            DidChangeConfigurationParams {
                settings: to_value(())?,
            },
        )
        .await
    }

    async fn change_document(
        &mut self,
        version: i32,
        content_changes: Vec<TextDocumentContentChangeEvent>,
    ) -> Result<()> {
        self.notify(
            "textDocument/didChange",
            DidChangeTextDocumentParams {
                text_document: VersionedTextDocumentIdentifier {
                    uri: uri!("document.js"),
                    version,
                },
                content_changes,
            },
        )
        .await
    }

    async fn close_document(&mut self) -> Result<()> {
        self.notify(
            "textDocument/didClose",
            DidCloseTextDocumentParams {
                text_document: TextDocumentIdentifier {
                    uri: uri!("document.js"),
                },
            },
        )
        .await
    }

    /// Basic implementation of the `biome/shutdown` request for tests
    async fn biome_shutdown(&mut self) -> Result<()> {
        self.request::<_, ()>("biome/shutdown", "_biome_shutdown", ())
            .await?
            .context("biome/shutdown returned None")?;
        Ok(())
    }
}

/// Number of notifications buffered by the server-to-client channel before it starts blocking the current task
const CHANNEL_BUFFER_SIZE: usize = 8;

#[derive(Debug, PartialEq, Eq)]
enum ServerNotification {
    PublishDiagnostics(PublishDiagnosticsParams),
    ShowMessage(ShowMessageParams),
}
impl ServerNotification {
    pub fn is_publish_diagnostics(&self) -> bool {
        matches!(self, Self::PublishDiagnostics(_))
    }

    pub fn is_show_message(&self) -> bool {
        matches!(self, Self::ShowMessage(_))
    }
}

async fn wait_for_notification(
    receiver: &mut (impl Stream<Item = ServerNotification> + Unpin),
    check: impl Fn(&ServerNotification) -> bool,
) -> Option<ServerNotification> {
    loop {
        let notification = tokio::select! {
            msg = receiver.next() => msg,
            _ = sleep(Duration::from_secs(3)) => {
                panic!("timed out waiting for the server to send diagnostics")
            }
        };

        match notification {
            Some(notification) => {
                if check(&notification) {
                    return Some(notification);
                }
            }
            None => break None,
        }
    }
}

/// Basic handler for requests and notifications coming from the server for tests
async fn client_handler<I, O>(
    mut stream: I,
    mut sink: O,
    mut notify: Sender<ServerNotification>,
) -> Result<()>
where
    // This function has to be generic as `RequestStream` and `ResponseSink`
    // are not exported from `tower_lsp` and cannot be named in the signature
    I: Stream<Item = Request> + Unpin,
    O: Sink<Response> + Unpin,
{
    while let Some(req) = stream.next().await {
        let params = req.params().expect("invalid request").clone();
        if let Some(notification) = match req.method() {
            "textDocument/publishDiagnostics" => Some(ServerNotification::PublishDiagnostics(
                from_value(params).expect("invalid params"),
            )),
            "window/showMessage" => Some(ServerNotification::ShowMessage(
                from_value(params).expect("invalid params"),
            )),
            _ => None,
        } {
            match notify.send(notification).await {
                Ok(_) => continue,
                Err(_) => break,
            }
        }

        let id = match req.id() {
            Some(id) => id,
            None => continue,
        };

        let res = match req.method() {
            "workspace/configuration" => {
                let settings = WorkspaceSettings::default();
                let result =
                    to_value(slice::from_ref(&settings)).context("failed to serialize settings")?;

                Response::from_ok(id.clone(), result)
            }
            _ => Response::from_error(id.clone(), jsonrpc::Error::method_not_found()),
        };

        sink.send(res).await.ok();
    }

    Ok(())
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
export { z, test, describe };

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
        title: String::from("Organize Imports (Biome)"),
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

    let (watcher, instruction_channel) = Watcher::new()?;

    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());

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
                pull_code_actions: false,
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
                pull_code_actions: false,
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
                pull_code_actions: false,
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
                pull_code_actions: false,
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

    let (watcher, instruction_channel) = Watcher::new()?;

    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());

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
                pull_code_actions: false,
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
                pull_code_actions: false,
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
                pull_code_actions: false,
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

#[tokio::test]
async fn should_open_and_update_nested_files() -> Result<()> {
    // ARRANGE: Set up folder.
    const FILE_PATH: &str = "src/a.js";
    const FILE_CONTENT_BEFORE: &str = "import 'foo';";
    const FILE_CONTENT_AFTER: &str = "import 'bar';";

    let mut fs = TemporaryFs::new("should_open_and_update_nested_files");
    fs.create_file(FILE_PATH, FILE_CONTENT_BEFORE);

    let (watcher, instruction_channel) = Watcher::new()?;

    // ARRANGE: Start server.
    let mut factory = ServerFactory::new(true, instruction_channel.sender.clone());

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
            .map(|module_info| module_info.static_import_paths.clone()),
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
            .map(|module_info| module_info.static_import_paths.clone()),
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
let useConst = "Astro Test";
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
                    line: 4,
                    character: 0,
                },
            },
            new_text: String::from(
                r#"---
const useConst = "Astro Test";
---
<!doctype html>
"#,
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

// #endregion
