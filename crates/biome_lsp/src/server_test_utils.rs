use std::any::type_name;
use std::fmt::Display;
use std::slice;
use std::str::FromStr;

use crate::WorkspaceSettings;
pub(crate) use crate::server::{LSPServer, ServerFactory};
use anyhow::{Context, Error, Result, bail};
use futures::channel::mpsc::Sender;
use futures::{Sink, SinkExt, Stream, StreamExt};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{from_value, to_value};
use tokio::time::{Duration, sleep};
use tower::timeout::Timeout;
use tower::{Service, ServiceExt};
use tower_lsp_server::LspService;
use tower_lsp_server::jsonrpc::{self, Request, Response};
use tower_lsp_server::ls_types::{
    self as lsp, ClientCapabilities, CodeActionCapabilityResolveSupport,
    CodeActionClientCapabilities, DidChangeConfigurationParams, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult,
    InitializedParams, PublishDiagnosticsParams, ShowMessageParams, TextDocumentClientCapabilities,
    TextDocumentContentChangeEvent, TextDocumentIdentifier, TextDocumentItem, Uri,
    VersionedTextDocumentIdentifier, WorkspaceFolder,
};

/// Statically build an [Uri] instance that points to the file at `$path`
/// within the workspace.
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
        sleep(Duration::from_millis(1000)).await;

        tokio::time::timeout(Duration::from_secs(2), $channel.changed())
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

pub(crate) use await_notification;
pub(crate) use clear_notifications;
pub(crate) use uri;

pub(crate) fn to_utf8_file_path_buf(uri: Uri) -> camino::Utf8PathBuf {
    camino::Utf8PathBuf::from_path_buf(uri.to_file_path().unwrap().to_path_buf()).unwrap()
}

pub(crate) struct Server {
    service: Timeout<LspService<LSPServer>>,
}

impl Server {
    pub(crate) fn new(service: LspService<LSPServer>) -> Self {
        Self {
            service: Timeout::new(service, Duration::from_secs(1)),
        }
    }

    pub(crate) async fn notify<P>(&mut self, method: &'static str, params: P) -> Result<()>
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

    pub(crate) async fn request<P, R>(
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
    pub(crate) async fn initialize(&mut self) -> Result<()> {
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
    #[expect(deprecated)]
    pub(crate) async fn initialize_projects(&mut self) -> Result<()> {
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

    /// Like [`Self::initialize`] but advertises `codeAction/resolve` support.
    #[expect(deprecated)]
    pub async fn initialize_with_resolve_support(&mut self) -> Result<()> {
        let _res: InitializeResult = self
            .request(
                "initialize",
                "_init",
                InitializeParams {
                    process_id: None,
                    root_path: None,
                    root_uri: Some(uri!("")),
                    initialization_options: None,
                    capabilities: ClientCapabilities {
                        text_document: Some(TextDocumentClientCapabilities {
                            code_action: Some(CodeActionClientCapabilities {
                                resolve_support: Some(CodeActionCapabilityResolveSupport {
                                    properties: vec!["edit".to_string()],
                                }),
                                ..Default::default()
                            }),
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

        Ok(())
    }

    pub(crate) async fn initialized(&mut self) -> Result<()> {
        self.notify("initialized", InitializedParams {}).await
    }

    pub(crate) async fn shutdown(&mut self) -> Result<()> {
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

    pub(crate) async fn open_document(&mut self, text: impl Display) -> Result<()> {
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

    pub(crate) async fn open_untitled_document(&mut self, text: impl Display) -> Result<()> {
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

    pub(crate) async fn open_named_document(
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

    pub(crate) async fn load_configuration(&mut self) -> Result<()> {
        self.notify(
            "workspace/didChangeConfiguration",
            DidChangeConfigurationParams {
                settings: to_value(())?,
            },
        )
        .await
    }

    pub(crate) async fn load_configuration_with_settings<V>(&mut self, value: V) -> Result<()>
    where
        V: Serialize,
    {
        self.notify(
            "workspace/didChangeConfiguration",
            DidChangeConfigurationParams {
                settings: to_value(value)?,
            },
        )
        .await
    }

    pub(crate) async fn change_document(
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

    pub(crate) async fn close_document(&mut self) -> Result<()> {
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

    pub(crate) async fn biome_shutdown(&mut self) -> Result<()> {
        self.request::<_, ()>("biome/shutdown", "_biome_shutdown", ())
            .await?
            .context("biome/shutdown returned None")?;
        Ok(())
    }
}

/// Number of notifications buffered by the server-to-client channel before it starts blocking the current task
pub(crate) const CHANNEL_BUFFER_SIZE: usize = 8;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ServerNotification {
    PublishDiagnostics(PublishDiagnosticsParams),
    ShowMessage(ShowMessageParams),
}

impl ServerNotification {
    pub(crate) fn is_publish_diagnostics(&self) -> bool {
        matches!(self, Self::PublishDiagnostics(_))
    }

    pub(crate) fn is_show_message(&self) -> bool {
        matches!(self, Self::ShowMessage(_))
    }
}

pub(crate) async fn wait_for_notification(
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
pub(crate) async fn client_handler<I, O>(
    stream: I,
    sink: O,
    notify: Sender<ServerNotification>,
) -> Result<()>
where
    I: Stream<Item = Request> + Unpin,
    O: Sink<Response> + Unpin,
{
    client_handler_with_settings(stream, sink, notify, WorkspaceSettings::default()).await
}

/// Handler for requests and notifications coming from the server for tests with custom settings
pub(crate) async fn client_handler_with_settings<I, O>(
    mut stream: I,
    mut sink: O,
    mut notify: Sender<ServerNotification>,
    settings: WorkspaceSettings,
) -> Result<()>
where
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
