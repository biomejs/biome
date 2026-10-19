//! A test harness that runs [BiomeServer] on a test project and talks to it
//! through a real MCP client, so tests exercise the protocol end to end.

use crate::{BiomeServer, Project, load_in_background};
use biome_fs::{FileSystem, MemoryFileSystem, OsFileSystem, TemporaryFs};
use biome_service::workspace::ServiceNotification;
use biome_service::{WatcherKind, WatcherOptions};
use camino::{Utf8Path, Utf8PathBuf};
use rmcp::model::{CallToolRequestParams, CallToolResult, JsonObject, ServerPeerInfo, Tool};
use rmcp::serde_json;
use rmcp::service::RunningService;
use rmcp::{RoleClient, ServiceExt};
use std::future::Future;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch;
use tokio::task::JoinHandle;
use tokio::time::{sleep, timeout};

/// The directory that contains the files of a [TestServer] created with
/// [TestServer::new].
pub(crate) const ROOT: &str = "/project";

/// How long a test waits for the server before failing.
const TIMEOUT: Duration = Duration::from_secs(10);

/// A [BiomeServer] that serves a test project, with a connected MCP client.
pub(crate) struct TestServer {
    client: RunningService<RoleClient, ()>,
    server: JoinHandle<()>,
    /// The notifications of the server's project, if it has one.
    notifications: Option<watch::Receiver<ServiceNotification>>,
    /// The files of a server whose project is in memory.
    memory_fs: Option<Arc<MemoryFileSystem>>,
}

impl TestServer {
    /// Starts a server for a project at [ROOT] that contains `files`, given as
    /// paths relative to the root and their content. The files are kept in
    /// memory and aren't watched.
    pub(crate) async fn new(files: &[(&str, &str)]) -> Self {
        Self::new_in_memory(memory_fs(files)).await
    }

    /// Starts a server like [TestServer::new], with `configuration` as the
    /// content of `biome.json` in the user's configuration folder.
    pub(crate) async fn new_with_user_configuration(
        files: &[(&str, &str)],
        configuration: &str,
    ) -> Self {
        let fs = memory_fs(files);
        let configuration_path = fs
            .user_config_dir()
            .expect("the platform should have a user configuration folder")
            .join("biome.json");
        fs.insert(configuration_path, configuration.as_bytes());
        Self::new_in_memory(fs).await
    }

    /// Starts a server for the project in the directory of `fs`, with the file
    /// watcher that the real server uses.
    pub(crate) async fn new_on_disk(fs: &TemporaryFs) -> Self {
        let root = fs.working_directory.clone();
        let os_fs = OsFileSystem::new(root.clone());
        let project = Project::new(Arc::new(os_fs), root, WatcherOptions::default())
            .expect("the watcher should start");
        Self::start(Ok(Arc::new(project))).await
    }

    /// Starts a server whose project couldn't be created because of `reason`.
    pub(crate) async fn new_without_project(reason: &str) -> Self {
        Self::start(Err(reason.to_string())).await
    }

    async fn new_in_memory(fs: Arc<MemoryFileSystem>) -> Self {
        let watcher_options = WatcherOptions {
            watcher_kind: WatcherKind::None,
            ..WatcherOptions::default()
        };
        let project = Project::new(fs.clone(), Utf8PathBuf::from(ROOT), watcher_options)
            .expect("the watcher should start");
        let mut server = Self::start(Ok(Arc::new(project))).await;
        server.memory_fs = Some(fs);
        server
    }

    async fn start(project: Result<Arc<Project>, String>) -> Self {
        let notifications = project.as_ref().ok().map(|project| project.notifications());

        // Starts like `serve_stdio`: the session first, then the project.
        let (server_transport, client_transport) = tokio::io::duplex(64 * 1024);
        let server = tokio::spawn(async move {
            let service = BiomeServer::new(project.clone())
                .serve(server_transport)
                .await
                .expect("the server should start");
            if let Ok(project) = project {
                load_in_background(project);
            }
            service
                .waiting()
                .await
                .expect("the server should stop cleanly");
        });
        let client = within_timeout(().serve(client_transport))
            .await
            .expect("the client should connect");

        Self {
            client,
            server,
            notifications,
            memory_fs: None,
        }
    }

    /// Replaces the content of the file at `path`, relative to [ROOT], in the
    /// project of a server created with [TestServer::new]. The change isn't
    /// watched, so only reads from disk, such as loading the configuration,
    /// see it.
    pub(crate) fn write_file(&self, path: &str, content: &str) {
        self.memory_fs
            .as_ref()
            .expect("only in-memory projects can be edited")
            .insert(Utf8Path::new(ROOT).join(path), content.as_bytes());
    }

    /// Runs `edit`, which changes files on disk, and waits until the watcher
    /// updated the workspace.
    pub(crate) async fn edit_and_wait(&mut self, edit: impl FnOnce()) {
        // As in the LSP watcher tests, earlier events get time to arrive and
        // the watcher to settle, so that the notification awaited below is
        // the one caused by `edit`.
        let notifications = self
            .notifications
            .as_mut()
            .expect("only servers with a project are notified");
        sleep(Duration::from_millis(500)).await;
        notifications.borrow_and_update();

        edit();

        within_timeout(notifications.changed())
            .await
            .expect("the workspace should notify the update");
    }

    /// Returns what the server reported about itself when the session started.
    pub(crate) fn server_info(&self) -> Arc<ServerPeerInfo> {
        self.client
            .peer_info()
            .expect("the server should describe itself")
    }

    pub(crate) async fn list_tools(&self) -> Vec<Tool> {
        within_timeout(self.client.list_tools(None))
            .await
            .expect("the server should list its tools")
            .tools
    }

    /// Calls the tool `name` with `arguments`, a JSON object as an agent
    /// sends it.
    pub(crate) async fn call_tool(&self, name: &str, arguments: &str) -> CallToolResult {
        let arguments: JsonObject =
            serde_json::from_str(arguments).expect("the arguments should be a JSON object");
        within_timeout(
            self.client
                .call_tool(CallToolRequestParams::new(name.to_string()).with_arguments(arguments)),
        )
        .await
        .expect("the tool call should get a response")
    }

    /// Closes the session and waits for the server to stop.
    pub(crate) async fn shutdown(self) {
        within_timeout(self.client.cancel())
            .await
            .expect("the client should disconnect");
        within_timeout(self.server)
            .await
            .expect("the server task should finish");
    }
}

/// Returns a file system that contains `files`, given as paths relative to
/// [ROOT] and their content.
fn memory_fs(files: &[(&str, &str)]) -> Arc<MemoryFileSystem> {
    let fs = Arc::new(MemoryFileSystem::default());
    for (path, content) in files {
        fs.insert(Utf8Path::new(ROOT).join(path), content.as_bytes());
    }
    fs
}

/// Returns the text content of `result`, which is what the agent reads.
pub(crate) fn text_of(result: &CallToolResult) -> String {
    result
        .content
        .iter()
        .filter_map(|content| content.as_text())
        .map(|content| content.text.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// Calls the tool `name` once for each of `calls`, the arguments as an agent
/// sends them, checks that every call succeeds, and snapshots the calls and the
/// text that the agent receives, as a Markdown document with an `Input` and an
/// `Output` section per call. The snapshot is stored as
/// `snapshots/<name>/<snapshot_name>.snap`.
pub(crate) async fn assert_tool_snapshot(
    snapshot_name: &str,
    server: &TestServer,
    name: &str,
    calls: &[&str],
) {
    snapshot_tool_calls(snapshot_name, server, name, calls, false).await;
}

/// Like [assert_tool_snapshot], for calls that must fail with an MCP error
/// result.
pub(crate) async fn assert_tool_error_snapshot(
    snapshot_name: &str,
    server: &TestServer,
    name: &str,
    calls: &[&str],
) {
    snapshot_tool_calls(snapshot_name, server, name, calls, true).await;
}

async fn snapshot_tool_calls(
    snapshot_name: &str,
    server: &TestServer,
    name: &str,
    calls: &[&str],
    is_error: bool,
) {
    let mut sections = Vec::with_capacity(calls.len());
    for arguments in calls {
        let result = server.call_tool(name, arguments).await;
        let output = text_of(&result);
        assert_eq!(
            result.is_error == Some(true),
            is_error,
            "unexpected `isError` for `{name} {arguments}`: {output}"
        );
        sections.push(format!(
            "## Input\n\n```text\n{name} {arguments}\n```\n\n## Output\n\n```text\n{output}\n```\n"
        ));
    }
    let snapshot = sections.join("\n");

    let mut settings = insta::Settings::clone_current();
    settings.set_snapshot_path(format!("snapshots/{name}"));
    settings.set_prepend_module_to_snapshot(false);
    settings.bind(|| insta::assert_snapshot!(snapshot_name, snapshot));
}

async fn within_timeout<T>(future: impl Future<Output = T>) -> T {
    timeout(TIMEOUT, future)
        .await
        .expect("the server should answer in time")
}
