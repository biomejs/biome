use super::*;
use crate::server_test_utils::{CHANNEL_BUFFER_SIZE, Server as TestServer, client_handler, uri};
use anyhow::Result;
use biome_fs::BiomePath;
use biome_service::WorkspaceError;
use biome_service::workspace::FileExistsParams;
use futures::FutureExt;
use std::convert::Infallible;
use std::future::{Pending, pending};
use std::str::FromStr;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::AsyncReadExt;
use tower::Service;
use tower_lsp_server::jsonrpc::Request;
use tower_lsp_server::ls_types as lsp;

struct PendingService;

impl Service<Request> for PendingService {
    type Response = ();
    type Error = Infallible;
    type Future = Pending<std::result::Result<(), Infallible>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<std::result::Result<(), Infallible>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _request: Request) -> Self::Future {
        pending()
    }
}

#[tokio::test]
async fn shutdown_does_not_cancel_other_sessions() -> Result<()> {
    let factory = ServerFactory::default();
    let cancellation = factory.cancellation();

    let (first_service, _first_client) = factory.create().into_inner();
    let (second_service, _second_client) = factory.create().into_inner();
    let mut first = TestServer::new(first_service);
    let mut second = TestServer::new(second_service);

    first.initialize().await?;
    second.initialize().await?;

    let result = first.request::<_, ()>("shutdown", "shutdown", ()).await?;
    assert_eq!(result, Some(()));
    assert!(cancellation.notified().now_or_never().is_none());

    Ok(())
}

#[test]
fn exit_disconnects_before_the_service_future_completes() {
    let factory = ServerFactory::default();
    let cancellation = factory.cancellation();
    let first = factory.create();
    let second = factory.create();

    first.lifecycle.mark_initialized();

    let mut first_service = DisconnectOnExit::new(PendingService, first.lifecycle.clone());
    let mut second_service = DisconnectOnExit::new(PendingService, second.lifecycle.clone());

    let _first_response = first_service.call(Request::build("exit").finish());
    assert_eq!(factory.sessions.lock().unwrap().len(), 1);
    assert!(cancellation.notified().now_or_never().is_none());

    let _second_response = second_service.call(Request::build("exit").finish());
    assert!(factory.sessions.lock().unwrap().is_empty());
    assert!(cancellation.notified().now_or_never().is_some());
}

#[tokio::test]
async fn eof_disconnects_the_session() {
    let factory = ServerFactory::default();
    let cancellation = factory.cancellation();
    let connection = factory.create();

    connection.lifecycle.mark_initialized();

    let mut input = DisconnectOnEof::new(tokio::io::empty(), connection.lifecycle.clone());
    let mut buffer = [0; 1];
    assert_eq!(input.read(&mut buffer).await.unwrap(), 0);
    assert!(factory.sessions.lock().unwrap().is_empty());
    assert!(cancellation.notified().now_or_never().is_some());
}

#[test]
fn disconnect_does_not_stop_a_persistent_daemon() {
    let factory = ServerFactory {
        stop_on_disconnect: false,
        ..ServerFactory::default()
    };
    let cancellation = factory.cancellation();
    let connection = factory.create();

    connection.lifecycle.mark_initialized();
    connection.lifecycle.disconnect();

    assert!(factory.sessions.lock().unwrap().is_empty());
    assert!(cancellation.notified().now_or_never().is_none());
}

#[tokio::test]
async fn disconnect_closes_the_files_of_the_session() -> Result<()> {
    let factory = ServerFactory::default();
    let connection = factory.create();
    let lifecycle = connection.lifecycle.clone();
    let (service, client) = connection.into_inner();
    let (stream, sink) = client.split();
    let mut server = TestServer::new(service);
    let (sender, _) = futures::channel::mpsc::channel(CHANNEL_BUFFER_SIZE);
    let reader = tokio::spawn(client_handler(stream, sink, sender));

    server.initialize().await?;
    server.initialized().await?;
    server.open_document("let a = 1;").await?;

    let session = factory
        .sessions
        .lock()
        .unwrap()
        .values()
        .next()
        .cloned()
        .unwrap();
    let file_exists = || {
        session.workspace().file_exists(FileExistsParams {
            file_path: BiomePath::try_from(uri!("document.js").to_file_path().unwrap()).unwrap(),
        })
    };
    assert!(file_exists()?);

    // The files are closed in the background.
    lifecycle.disconnect();
    let closed = tokio::time::timeout(Duration::from_secs(5), async {
        while file_exists()? {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        Ok::<_, WorkspaceError>(())
    })
    .await;
    assert!(matches!(closed, Ok(Ok(()))), "{closed:?}");

    reader.abort();
    Ok(())
}
