use super::*;
use crate::server_test_utils::Server as TestServer;
use anyhow::Result;
use futures::FutureExt;
use std::convert::Infallible;
use std::future::{Pending, pending};
use std::task::{Context, Poll};
use tokio::io::AsyncReadExt;
use tower::Service;
use tower_lsp_server::jsonrpc::Request;

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
