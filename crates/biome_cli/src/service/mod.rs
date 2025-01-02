//! Implements the OS dependent transport layer for the server protocol. This
//! uses a domain socket created in the global temporary directory on Unix
//! systems, and a named pipe on Windows. The protocol used for message frames
//! is based on the [Language Server Protocol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#baseProtocol),
//! a simplified derivative of the HTTP protocol

use std::{
    any::type_name,
    borrow::Cow,
    io,
    ops::Deref,
    panic::RefUnwindSafe,
    str::{from_utf8, FromStr},
    sync::Arc,
    time::Duration,
};

use anyhow::{bail, ensure, Context, Error};
use biome_service::{
    workspace::{TransportRequest, WorkspaceTransport},
    TransportError,
};
use dashmap::DashMap;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{
    from_slice, from_str, to_vec,
    value::{to_raw_value, RawValue},
    Value,
};
use tokio::{
    io::{
        AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt,
        BufReader, BufWriter,
    },
    runtime::Runtime,
    sync::{
        mpsc::{channel, Receiver, Sender},
        oneshot, Notify,
    },
    time::sleep,
};

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub(crate) use self::windows::{
    ensure_daemon, enumerate_pipes, open_socket, print_socket, run_daemon,
};

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub(crate) use self::unix::{
    ensure_daemon, enumerate_pipes, open_socket, print_socket, run_daemon,
};

/// Tries to open a connection to a running daemon instance, returning a
/// [WorkspaceTransport] instance if the socket is currently active
pub fn open_transport(runtime: Runtime) -> io::Result<Option<impl WorkspaceTransport>> {
    match runtime.block_on(open_socket()) {
        Ok(Some((read, write))) => Ok(Some(SocketTransport::open(runtime, read, write))),
        Ok(None) => Ok(None),
        Err(err) => Err(err),
    }
}

type JsonRpcResult = Result<Box<RawValue>, TransportError>;

/// Implementation of [WorkspaceTransport] for types implementing [AsyncRead]
/// and [AsyncWrite]
///
/// This structs holds an instance of the `tokio` runtime, as well as the
/// following fields:
/// - `write_send` is a sender handle to the "write channel", an MPSC channel
///     that's used to queue up requests to be sent to the server (for simplicity
///     the requests are pushed to the channel as serialized byte buffers)
/// - `pending_requests` is handle to a shared hashmap where the keys are `u64`
///     corresponding to request IDs, and the values are sender handles to oneshot
///     channel instances that can be consumed to fulfill the associated request
///
/// Creating a new `SocketTransport` instance requires providing a `tokio`
/// runtime instance as well as the "read half" and "write half" of the socket
/// object to be used by this transport instance. These two objects implement
/// [AsyncRead] and [AsyncWrite] respectively, and should generally map to the
/// same underlying I/O object but are represented as separate so they can be
/// used concurrently
///
/// This concurrent handling of I/O is implemented useing two "background tasks":
/// - the `write_task` pulls outgoing messages from the "write channel" and
///     writes them to the "write half" of the socket
/// - the `read_task` reads incoming messages from the "read half" of the
/// - the `read_task` reads incoming messages from the "read half" of the
///   socket, then looks up a request with an ID corresponding to the received
///   message in the "pending requests" map. If a pending request is found, it's
///   fulfilled with the content of the message that was just received
///
/// In addition to these, a new "foreground task" is created for each request.
/// Each foreground task creates a oneshot channel and stores it in the pending
/// requests map using the request ID as a key, then serialize the content of
/// the request and send it over the write channel. Finally, the task blocks
/// the current thread until a response is received over the oneshot channel
/// from the read task, or the request times out
pub struct SocketTransport {
    runtime: Runtime,
    write_send: Sender<(Vec<u8>, bool)>,
    pending_requests: PendingRequests,
}

/// Stores a handle to the map of pending requests, and clears the map
/// automatically when the handle is dropped
#[derive(Clone, Default)]
struct PendingRequests {
    inner: Arc<DashMap<u64, oneshot::Sender<JsonRpcResult>>>,
}

impl Deref for PendingRequests {
    type Target = DashMap<u64, oneshot::Sender<JsonRpcResult>>;

    fn deref(&self) -> &Self::Target {
        self.inner.as_ref()
    }
}

/// There are two live handles to the pending requests map: one is in the
/// `SocketTransport` and the other in the `read_task`. The `SocketTransport`
/// instance can only be dropped if it's empty (since the `request` method
/// blocks until the request is resolved, `&self` will always outlive any
/// pending request), but the `read_task` may abort if it encounters an error
/// or receives a shutdown broadcast while there are still pending requests. In
/// this case the `Drop` implementation will ensure that all pending requests
/// are cancelled immediately instead of timing out.
impl Drop for PendingRequests {
    fn drop(&mut self) {
        self.inner.clear();
    }
}

impl SocketTransport {
    pub fn open<R, W>(runtime: Runtime, socket_read: R, socket_write: W) -> Self
    where
        R: AsyncRead + Unpin + Send + 'static,
        W: AsyncWrite + Unpin + Send + 'static,
    {
        /// Capacity of the "write channel", once this many requests have been
        /// queued up, calls to `write_send.send` will block the sending task
        /// until enough capacity is available again
        ///
        /// Note that this does not limit how many requests can be in flight at
        /// a given time, it only serves as a loose rate-limit on how many new
        /// requests can be sent to the server within a given time frame
        const WRITE_CHANNEL_CAPACITY: usize = 16;

        let (write_send, write_recv) = channel(WRITE_CHANNEL_CAPACITY);

        let pending_requests = PendingRequests::default();
        let pending_requests_2 = pending_requests.clone();

        let socket_read = BufReader::new(socket_read);
        let socket_write = BufWriter::new(socket_write);

        let broadcast_shutdown = Arc::new(Notify::new());

        runtime.spawn(write_task(
            broadcast_shutdown.clone(),
            write_recv,
            socket_write,
        ));

        runtime.spawn(async move {
            tokio::select! {
                _ = read_task(socket_read, &pending_requests) => {}
                _ = broadcast_shutdown.notified() => {}
            }
        });

        Self {
            runtime,
            write_send,
            pending_requests: pending_requests_2,
        }
    }
}

// Allow the socket to be recovered across panic boundaries
impl RefUnwindSafe for SocketTransport {}

impl WorkspaceTransport for SocketTransport {
    fn request<P, R>(&self, request: TransportRequest<P>) -> Result<R, TransportError>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        let (send, recv) = oneshot::channel();

        self.pending_requests.insert(request.id, send);

        let is_shutdown = request.method == "biome/shutdown";

        let request = JsonRpcRequest {
            jsonrpc: Cow::Borrowed("2.0"),
            id: request.id,
            method: Cow::Borrowed(request.method),
            params: request.params,
        };

        let request = to_vec(&request).map_err(|err| {
            TransportError::SerdeError(format!(
                "failed to serialize {} into byte buffer: {err}",
                type_name::<P>()
            ))
        })?;

        let response = self.runtime.block_on(async move {
            self.write_send
                .send((request, is_shutdown))
                .await
                .map_err(|_| TransportError::ChannelClosed)?;

            tokio::select! {
                result = recv => {
                    match result {
                        Ok(Ok(response)) => Ok(response),
                        Ok(Err(error)) => Err(error),
                        Err(_) => Err(TransportError::ChannelClosed),
                    }
                }
                _ = sleep(Duration::from_secs(15)) => {
                    Err(TransportError::Timeout)
                }
            }
        })?;

        let response = response.get();
        let result = from_str(response).map_err(|err| {
            TransportError::SerdeError(format!(
                "failed to deserialize {} from {response:?}: {err}",
                type_name::<R>()
            ))
        })?;

        Ok(result)
    }
}

async fn read_task<R>(mut socket_read: BufReader<R>, pending_requests: &PendingRequests)
where
    R: AsyncRead + Unpin,
{
    loop {
        let message = read_message(&mut socket_read).await;
        let message = match message {
            Ok(message) => {
                let response = from_slice(&message).with_context(|| {
                    if let Ok(message) = from_utf8(&message) {
                        format!("failed to deserialize JSON-RPC response from {message:?}")
                    } else {
                        format!("failed to deserialize JSON-RPC response from {message:?}")
                    }
                });

                response.map(|response| (message, response))
            }
            Err(err) => Err(err),
        };

        let (message, response): (_, JsonRpcResponse) = match message {
            Ok(message) => message,
            Err(err) => {
                eprintln!(
                    "{:?}",
                    err.context("remote connection read task exited with an error")
                );
                break;
            }
        };

        if let Some((_, channel)) = pending_requests.remove(&response.id) {
            let response = match (response.result, response.error) {
                (Some(result), None) => Ok(result),
                (None, Some(err)) => Err(TransportError::RPCError(err.message)),

                // Both result and error will be None if the request
                // returns a null-ish result, in this case create a
                // "null" RawValue as the result
                //
                // SAFETY: Calling `to_raw_value` with a static "null"
                // JSON Value will always succeed
                (None, None) => Ok(to_raw_value(&Value::Null).unwrap()),

                _ => {
                    let message = if let Ok(message) = from_utf8(&message) {
                        format!("invalid response {message:?}")
                    } else {
                        format!("invalid response {message:?}")
                    };

                    Err(TransportError::SerdeError(message))
                }
            };

            channel.send(response).ok();
        }
    }
}

async fn read_message<R>(mut socket_read: R) -> Result<Vec<u8>, Error>
where
    R: AsyncBufRead + Unpin,
{
    let mut length = None;
    let mut line = String::new();

    loop {
        match socket_read
            .read_line(&mut line)
            .await
            .context("failed to read header line from the socket")?
        {
            // A read of 0 bytes means the connection was closed
            0 => {
                bail!("the connection to the remote workspace was unexpectedly closed");
            }
            // A read of two bytes corresponds to the "\r\n" sequence
            // that indicates the end of the header section
            2 => {
                if line != "\r\n" {
                    bail!("unexpected byte sequence received from the remote workspace, got {line:?} expected \"\\r\\n\"");
                }

                break;
            }
            _ => {
                let header: TransportHeader = line
                    .parse()
                    .context("failed to parse header from the remote workspace")?;

                match header {
                    TransportHeader::ContentLength(value) => {
                        length = Some(value);
                    }
                    TransportHeader::ContentType => {}
                    TransportHeader::Unknown(name) => {
                        eprintln!("ignoring unknown header {name:?}");
                    }
                }

                line.clear();
            }
        }
    }

    let length = length.context(
        "incoming response from the remote workspace is missing the Content-Length header",
    )?;

    let mut result = vec![0u8; length];
    socket_read
        .read_exact(&mut result)
        .await
        .with_context(|| format!("failed to read message of {length} bytes from the socket"))?;

    Ok(result)
}

async fn write_task<W>(
    broadcast_shutdown: Arc<Notify>,
    mut write_recv: Receiver<(Vec<u8>, bool)>,
    mut socket_write: BufWriter<W>,
) where
    W: AsyncWrite + Unpin,
{
    while let Some((message, is_shutdown)) = write_recv.recv().await {
        if is_shutdown {
            broadcast_shutdown.notify_waiters();
        }

        if let Err(err) = write_message(&mut socket_write, message).await {
            eprintln!(
                "{:?}",
                err.context("remote connection write task exited with an error")
            );
            break;
        }

        if is_shutdown {
            break;
        }
    }
}

async fn write_message<W>(mut socket_write: W, message: Vec<u8>) -> Result<(), Error>
where
    W: AsyncWrite + Unpin,
{
    socket_write.write_all(b"Content-Length: ").await?;

    let length = message.len().to_string();
    socket_write.write_all(length.as_bytes()).await?;
    socket_write.write_all(b"\r\n").await?;

    socket_write
        .write_all(b"Content-Type: application/vscode-jsonrpc; charset=utf-8\r\n")
        .await?;

    socket_write.write_all(b"\r\n").await?;

    socket_write.write_all(&message).await?;

    socket_write.flush().await?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct JsonRpcRequest<P> {
    jsonrpc: Cow<'static, str>,
    id: u64,
    method: Cow<'static, str>,
    params: P,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct JsonRpcResponse {
    #[expect(dead_code)]
    jsonrpc: Cow<'static, str>,
    id: u64,
    result: Option<Box<RawValue>>,
    error: Option<JsonRpcError>,
}

#[derive(Debug, Deserialize)]
struct JsonRpcError {
    #[expect(dead_code)]
    code: i64,
    message: String,
    #[expect(dead_code)]
    data: Option<Box<RawValue>>,
}

enum TransportHeader {
    ContentLength(usize),
    ContentType,
    Unknown(String),
}

impl FromStr for TransportHeader {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let colon = line
            .find(':')
            .with_context(|| format!("could not find colon token in {line:?}"))?;

        let (name, value) = line.split_at(colon);
        let value = value[1..].trim();

        match name {
            "Content-Length" => {
                let value = value.parse().with_context(|| {
                    format!("could not parse Content-Length header value {value:?}")
                })?;

                Ok(TransportHeader::ContentLength(value))
            }
            "Content-Type" => {
                ensure!(
                    value.starts_with( "application/vscode-jsonrpc"),
                    "invalid value for Content-Type expected \"application/vscode-jsonrpc\", got {value:?}"
                );

                Ok(TransportHeader::ContentType)
            }
            _ => Ok(TransportHeader::Unknown(name.into())),
        }
    }
}
