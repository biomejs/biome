use crate::scanner::{ScannerWatcherBridge, WorkspaceWatcherBridge};
use crate::v2::handle::{WorkspaceHandle, WorkspaceWatcher};
use crate::v2::request::{
    ControlRequest, WatcherRequest, WorkspaceCall, WorkspaceCommit, WorkspaceRequest,
    WorkspaceSender,
};
use crate::v2::server::WorkspaceServer;
use crate::v2::snapshots::{StagedCommitResult, StagedSnapshotRun, StagedWorkerTask, WorkerTask};
use crate::v2::staged::{
    FixFileOperation, StagedCommitMessage, StagedOperation, StagedRequest, StagedRetry,
};
use crate::workspace::ServerInfo;
use crate::{WorkspaceError, v2::request::WorkspaceResponder};
use biome_resolver::FsWithResolverProxy;
use crossbeam::channel::{Receiver, unbounded};
use std::sync::Arc;

/// It holds a mutable reference to the `WorkspaceServer`
pub(crate) struct WorkspaceOwner {
    server: WorkspaceServer,
    receiver: Receiver<WorkspaceRequest>,
    worker_pool: WorkspaceWorkerPool,
}

pub(crate) struct WorkspaceWorkerPool {
    pool: rayon::ThreadPool,
}

impl WorkspaceWorkerPool {
    pub(crate) fn new(threads: Option<usize>) -> Self {
        let mut builder = rayon::ThreadPoolBuilder::new()
            .thread_name(|index| format!("biome::workspace_worker_{index}"));
        if let Some(threads) = threads
            && threads > 0
        {
            builder = builder.num_threads(threads);
        }

        let pool = builder
            .build()
            .expect("failed to initialize workspace worker pool");

        Self { pool }
    }

    /// Runs a read-only task on the worker pool and sends its result directly
    /// back to the original caller.
    fn spawn<R>(&self, task: WorkerTask<R>, response: WorkspaceResponder<R>)
    where
        R: Send + 'static,
    {
        // `WorkerTask` construction is private to `v2::snapshots`; owner code
        // cannot wrap arbitrary workspace mutations in worker-pool closures.
        self.pool.spawn(move || {
            let result = task.run();
            response.send(result);
        });
    }

    /// Runs a read-only task whose result must return to the owner before the
    /// request can finish.
    ///
    /// The callback receives either owner-side work to commit, or the original
    /// request payload when the snapshot was stale and needs to be rebuilt.
    fn spawn_staged<C, P>(
        &self,
        task: StagedWorkerTask<C, P>,
        on_result: impl FnOnce(Result<StagedSnapshotRun<C, P>, WorkspaceError>) + Send + 'static,
    ) where
        C: Send + 'static,
        P: Send + 'static,
    {
        self.pool.spawn(move || {
            let result = task.run();
            on_result(result);
        });
    }
}

impl WorkspaceOwner {
    pub(crate) fn new(
        server: WorkspaceServer,
        fs: Arc<dyn FsWithResolverProxy>,
        server_info: Option<ServerInfo>,
        threads: Option<usize>,
    ) -> (Self, WorkspaceHandle, WorkspaceWatcher) {
        let (sender, receiver) = unbounded();
        let handle = WorkspaceHandle::new(sender.clone(), fs.clone(), server_info);
        let watcher = WorkspaceWatcher::new(sender, fs);
        let owner = Self {
            server,
            receiver,
            worker_pool: WorkspaceWorkerPool::new(threads),
        };

        (owner, handle, watcher)
    }

    pub(crate) fn run(self) {
        let Self {
            server,
            receiver,
            worker_pool,
        } = self;

        let mut owner = RunningWorkspaceOwner {
            server,
            worker_pool,
        };

        while let Ok(request) = receiver.recv() {
            if !owner.handle_request(request) {
                break;
            }
        }
    }
}

struct RunningWorkspaceOwner {
    server: WorkspaceServer,
    worker_pool: WorkspaceWorkerPool,
}

impl RunningWorkspaceOwner {
    fn handle_request(&mut self, request: WorkspaceRequest) -> bool {
        match request {
            WorkspaceRequest::Call { call, sender } => self.handle_call(*call, sender),
            WorkspaceRequest::Watcher(request) => self.handle_watcher_request(request),
            WorkspaceRequest::Commit(request) => self.handle_commit_request(*request),
            WorkspaceRequest::Control(request) => return self.handle_control_request(request),
        }

        true
    }

    // Dispatch policy:
    // - Mutations and inexpensive owner-state reads run directly on the owner.
    // - CPU-heavy read-only calls build an owned snapshot on the owner, then run
    //   it as a worker task.
    // - Calls that both compute heavily and mutate owner state stay direct until
    //   their mutation can be split into an owner-side commit.
    fn handle_call(&mut self, request: WorkspaceCall, sender: WorkspaceSender) {
        match request {
            WorkspaceCall::OpenProject(params, response) => {
                response.send(self.server.open_project_on_owner(params));
            }
            WorkspaceCall::ScanProject(params, response) => {
                response.send(self.server.scan_project_on_owner(params));
            }
            WorkspaceCall::UpdateSettings(params, response) => {
                response.send(self.server.update_settings_on_owner(params));
            }
            WorkspaceCall::CloseProject(params, response) => {
                response.send(self.server.close_project_on_owner(params));
            }
            WorkspaceCall::OpenFile(params, response) => {
                response.send(self.server.open_file_on_owner(params));
            }
            WorkspaceCall::FileExists(params, response) => {
                response.send(self.server.file_exists_on_owner(params));
            }
            WorkspaceCall::FileFeatures(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_file_features(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::IsPathIgnored(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_path_is_ignored(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetFileContent(params, response) => {
                response.send(self.server.get_file_content_on_owner(params));
            }
            WorkspaceCall::CheckFileSize(params, response) => {
                response.send(self.server.check_file_size_on_owner(params));
            }
            WorkspaceCall::ChangeFile(params, response) => {
                response.send(self.server.change_file_on_owner(params));
            }
            WorkspaceCall::PullDiagnostics(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_pull_diagnostics(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::PullActions(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_pull_actions(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::PullDiagnosticsAndActions(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_pull_diagnostics_and_actions(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::FormatFile(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_format_file(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::FormatRange(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_format_range(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::FormatOnType(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_format_on_type(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::FixFile(params, response) => {
                let request = StagedRequest::<FixFileOperation>::new(response, sender);
                self.dispatch_staged::<FixFileOperation>(params, request);
            }
            WorkspaceCall::Rename(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_rename(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GoToDefinition(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_go_to_definition(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::CloseFile(params, response) => {
                response.send(self.server.close_file_on_owner(params));
            }
            WorkspaceCall::UpdateModuleGraph(params, response) => {
                response.send(self.server.update_module_graph_on_owner(params));
            }
            WorkspaceCall::ParsePattern(params, response) => {
                response.send(self.server.parse_pattern_on_owner(params));
            }
            WorkspaceCall::SearchPattern(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_search_pattern(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::DropPattern(params, response) => {
                response.send(self.server.drop_pattern_on_owner(params));
            }
            WorkspaceCall::GetSyntaxTree(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_get_syntax_tree(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetControlFlowGraph(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_get_control_flow_graph(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetFormatterIr(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_get_formatter_ir(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetTypeInfo(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_get_type_info(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetRegisteredTypes(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_get_registered_types(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetSemanticModel(params, response) => {
                self.dispatch_worker_task(
                    self.server
                        .prepare_get_semantic_model(params)
                        .map(|snapshot| snapshot.into_worker_task()),
                    response,
                );
            }
            WorkspaceCall::GetModuleGraph(params, response) => {
                response.send(self.server.get_module_graph_on_owner(params));
            }
            WorkspaceCall::Rage(params, response) => {
                response.send(self.server.rage_on_owner(params));
            }
        }
    }

    fn dispatch_worker_task<R>(
        &self,
        task: Result<WorkerTask<R>, WorkspaceError>,
        response: WorkspaceResponder<R>,
    ) where
        R: Send + 'static,
    {
        match task {
            Ok(task) => self.worker_pool.spawn(task, response),
            Err(error) => response.send(Err(error)),
        }
    }

    fn dispatch_staged<O: StagedOperation>(
        &mut self,
        params: O::Params,
        request: StagedRequest<O>,
    ) {
        match O::prepare(&mut self.server, params) {
            Ok(task) => self.dispatch_staged_task::<O>(task, request),
            Err(error) => request.send(Err(error)),
        }
    }

    fn dispatch_staged_task<O: StagedOperation>(
        &self,
        task: StagedWorkerTask<O::WorkerCommit, O::Params>,
        request: StagedRequest<O>,
    ) {
        self.worker_pool
            .spawn_staged(task, move |result| match result {
                Ok(StagedSnapshotRun::Commit(commit)) => {
                    request.send_commit(commit);
                }
                Ok(StagedSnapshotRun::Stale(params)) => {
                    request.send_stale(params);
                }
                Err(error) => request.send(Err(error)),
            });
    }

    fn handle_commit_request(&mut self, request: WorkspaceCommit) {
        match request {
            WorkspaceCommit::FixFile(message) => {
                self.handle_staged_commit::<FixFileOperation>(message);
            }
        }
    }

    fn handle_staged_commit<O: StagedOperation>(&mut self, message: StagedCommitMessage<O>) {
        match message {
            StagedCommitMessage::Commit { commit, request } => {
                match O::commit(&mut self.server, commit) {
                    Ok(StagedCommitResult::Done(result)) => request.send(Ok(result)),
                    Ok(StagedCommitResult::Stale(params)) => {
                        self.retry_or_fallback_staged::<O>(params, request);
                    }
                    Ok(StagedCommitResult::Continue(task)) => {
                        self.dispatch_staged_task::<O>(task, request);
                    }
                    Err(error) => request.send(Err(error)),
                }
            }
            StagedCommitMessage::Stale { params, request } => {
                self.retry_or_fallback_staged::<O>(params, request);
            }
        }
    }

    fn retry_or_fallback_staged<O: StagedOperation>(
        &mut self,
        params: O::Params,
        request: StagedRequest<O>,
    ) {
        match request.retry() {
            StagedRetry::Retry(request) => self.dispatch_staged::<O>(params, request),
            StagedRetry::Fallback(request) => request.send(O::fallback(&mut self.server, params)),
        }
    }

    fn handle_watcher_request(&mut self, request: WatcherRequest) {
        let watcher = ScannerWatcherBridge::new((&self.server.scanner, &self.server));

        match request {
            WatcherRequest::FindProjectForPath(path, response) => {
                response.send(Ok(watcher.find_project_for_path(&path)));
            }
            WatcherRequest::FindProjectWithScanKindForPath(path, response) => {
                response.send(Ok(watcher.find_project_with_scan_kind_for_path(&path)));
            }
            WatcherRequest::IsIgnored {
                project_key,
                scan_kind,
                path,
                path_kind,
                tx,
            } => {
                tx.send(watcher.is_ignored(project_key, &scan_kind, &path, path_kind));
            }
            WatcherRequest::IndexFile(project_key, path, response) => {
                response.send(watcher.index_file(project_key, path));
            }
            WatcherRequest::IndexFolder(path, response) => {
                response.send(watcher.index_folder(&path));
            }
            WatcherRequest::InsertWatchedFolder(path, response) => {
                response.send(Ok(watcher.insert_watched_folder(path)));
            }
            WatcherRequest::RemoveWatchedFoldersUnder(path, response) => {
                response.send(Ok(watcher.remove_watched_folders_under(&path)));
            }
            WatcherRequest::UnloadFile(path, project_key, response) => {
                response.send(watcher.unload_file(&path, project_key));
            }
            WatcherRequest::UnloadPath(path, project_key, response) => {
                response.send(watcher.unload_path(&path, project_key));
            }
            WatcherRequest::NotifyStopped => {
                watcher.notify_stopped();
            }
        }
    }

    fn handle_control_request(&mut self, request: ControlRequest) -> bool {
        match request {
            ControlRequest::Shutdown => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{RunningWorkspaceOwner, WorkspaceWorkerPool};
    use crate::projects::ProjectKey;
    use crate::test_utils::setup_workspace_and_open_project;
    use crate::v2::request::{WorkspaceCommit, WorkspaceRequest, WorkspaceResponder};
    use crate::v2::snapshots::{FixFileCommit, FixFileFinishCommit, test_worker_task};
    use crate::v2::staged::{FixFileOperation, StagedCommitMessage, StagedRequest};
    use crate::workspace::{
        ChangeFileParams, FileContent, FixFileMode, FixFileParams, FixFileResult, OpenFileParams,
    };
    use crate::{TransportError, WorkspaceError};
    use biome_analyze::RuleCategories;
    use biome_fs::{BiomePath, MemoryFileSystem};
    use crossbeam::channel::{Receiver, unbounded};
    use std::panic::resume_unwind;
    use std::time::Duration;

    fn setup_fix_file_owner() -> (RunningWorkspaceOwner, ProjectKey, BiomePath) {
        let fs = MemoryFileSystem::default();
        let (mut server, project_key) = setup_workspace_and_open_project(fs, "/");
        let path = BiomePath::new("/project/a.js");

        server
            .open_file_on_owner(OpenFileParams {
                project_key,
                path: path.clone(),
                content: FileContent::FromClient {
                    content: "let value = 1;".into(),
                    version: 1,
                },
                document_file_source: None,
                persist_node_cache: false,
                inline_config: None,
                editor_features: None,
            })
            .unwrap();

        (
            RunningWorkspaceOwner {
                server,
                worker_pool: WorkspaceWorkerPool::new(Some(1)),
            },
            project_key,
            path,
        )
    }

    fn fix_file_params(project_key: ProjectKey, path: BiomePath) -> FixFileParams {
        FixFileParams {
            project_key,
            path,
            fix_file_mode: FixFileMode::SafeFixes,
            should_format: false,
            only: vec![],
            skip: vec![],
            enabled_rules: vec![],
            rule_categories: RuleCategories::default(),
            suppression_reason: None,
            inline_config: None,
        }
    }

    fn stale_fix_file_commit(
        project_key: ProjectKey,
        path: BiomePath,
        revision: u64,
    ) -> FixFileCommit {
        FixFileCommit::Finish(Box::new(FixFileFinishCommit {
            params: fix_file_params(project_key, path),
            revision,
            result: FixFileResult {
                code: "stale commit result".into(),
                ..FixFileResult::default()
            },
        }))
    }

    fn response_channel() -> (
        WorkspaceResponder<FixFileResult>,
        Receiver<Result<FixFileResult, WorkspaceError>>,
    ) {
        let (tx, rx) = crossbeam::channel::bounded(1);
        (WorkspaceResponder::new(tx), rx)
    }

    fn expect_retry_commit(rx: &Receiver<WorkspaceRequest>) -> WorkspaceRequest {
        let request = rx
            .recv_timeout(Duration::from_secs(5))
            .expect("retry should send a commit request");

        let WorkspaceRequest::Commit(commit) = &request else {
            panic!("expected retry to send a commit request");
        };
        let WorkspaceCommit::FixFile(StagedCommitMessage::Commit {
            request: staged_request,
            ..
        }) = commit.as_ref()
        else {
            panic!("expected retry to send a fix-file commit");
        };
        assert_eq!(staged_request.retries_left(), 0);

        request
    }

    fn recv_fix_file_result(rx: &Receiver<Result<FixFileResult, WorkspaceError>>) -> FixFileResult {
        rx.recv_timeout(Duration::from_secs(5))
            .expect("fix_file should send a response")
            .unwrap()
    }

    fn expect_channel_closed(rx: &Receiver<Result<FixFileResult, WorkspaceError>>) {
        let error = rx
            .recv_timeout(Duration::from_secs(5))
            .expect("staged request should send channel-closed response")
            .unwrap_err();

        let WorkspaceError::TransportError(TransportError::ChannelClosed) = error else {
            panic!("expected channel-closed transport error");
        };
    }

    fn drive_owner_until_fix_file_result(
        owner: &mut RunningWorkspaceOwner,
        request_rx: &Receiver<WorkspaceRequest>,
        response_rx: &Receiver<Result<FixFileResult, WorkspaceError>>,
    ) -> FixFileResult {
        for _ in 0..8 {
            if let Ok(result) = response_rx.try_recv() {
                return result.unwrap();
            }

            let request = request_rx
                .recv_timeout(Duration::from_secs(5))
                .expect("fix_file should send another owner request or finish");
            assert!(owner.handle_request(request));
        }

        recv_fix_file_result(response_rx)
    }

    #[test]
    fn worker_pool_sends_worker_task_result() {
        let pool = WorkspaceWorkerPool::new(Some(1));
        let (tx, rx) = crossbeam::channel::bounded(1);
        let response = WorkspaceResponder::new(tx);
        let task = test_worker_task(|| Ok::<_, WorkspaceError>(7));

        pool.spawn(task, response);

        assert_eq!(rx.recv().unwrap().unwrap(), 7);
    }

    #[test]
    fn worker_pool_sends_worker_task_cancellation() {
        let pool = WorkspaceWorkerPool::new(Some(1));
        let (tx, rx) = crossbeam::channel::bounded(1);
        let response = WorkspaceResponder::new(tx);
        let task = test_worker_task(|| -> Result<(), WorkspaceError> {
            resume_unwind(Box::new(salsa::Cancelled::Local));
        });

        pool.spawn(task, response);

        let error = rx.recv().unwrap().unwrap_err();
        let WorkspaceError::Cancelled(cancelled) = error else {
            panic!("expected cancellation error, got {error}");
        };

        assert_eq!(
            cancelled.reason,
            "cancelled because of local cancellation request"
        );
    }

    #[test]
    fn fix_file_commit_stale_retries_worker_once() {
        let (mut owner, project_key, path) = setup_fix_file_owner();
        let revision = owner.server.state_revision();
        owner
            .server
            .change_file_on_owner(ChangeFileParams {
                project_key,
                path: path.clone(),
                content: "let value = 2;".into(),
                version: 2,
                inline_config: None,
                editor_features: None,
            })
            .unwrap();
        let commit = stale_fix_file_commit(project_key, path, revision);
        let (response, response_rx) = response_channel();
        let (sender, request_rx) = unbounded();

        let request = StagedRequest::<FixFileOperation>::with_retries_left(response, 1, sender);
        owner.handle_commit_request(WorkspaceCommit::FixFile(StagedCommitMessage::Commit {
            commit,
            request,
        }));

        let request = expect_retry_commit(&request_rx);
        assert!(owner.handle_request(request));
        let result = drive_owner_until_fix_file_result(&mut owner, &request_rx, &response_rx);
        assert_ne!(result.code, "stale commit result");
    }

    #[test]
    fn fix_file_commit_stale_falls_back_when_retries_are_exhausted() {
        let (mut owner, project_key, path) = setup_fix_file_owner();
        let revision = owner.server.state_revision();
        owner
            .server
            .change_file_on_owner(ChangeFileParams {
                project_key,
                path: path.clone(),
                content: "let value = 2;".into(),
                version: 2,
                inline_config: None,
                editor_features: None,
            })
            .unwrap();
        let commit = stale_fix_file_commit(project_key, path, revision);
        let (response, response_rx) = response_channel();
        let (sender, request_rx) = unbounded();

        let request = StagedRequest::<FixFileOperation>::with_retries_left(response, 0, sender);
        owner.handle_commit_request(WorkspaceCommit::FixFile(StagedCommitMessage::Commit {
            commit,
            request,
        }));

        let result = recv_fix_file_result(&response_rx);
        assert_ne!(result.code, "stale commit result");
        assert!(request_rx.try_recv().is_err());
    }

    #[test]
    fn fix_file_stale_retries_worker_once() {
        let (mut owner, project_key, path) = setup_fix_file_owner();
        let params = fix_file_params(project_key, path);
        let (response, response_rx) = response_channel();
        let (sender, request_rx) = unbounded();

        let request = StagedRequest::<FixFileOperation>::with_retries_left(response, 1, sender);
        owner.handle_commit_request(WorkspaceCommit::FixFile(StagedCommitMessage::Stale {
            params,
            request,
        }));

        let request = expect_retry_commit(&request_rx);
        assert!(owner.handle_request(request));
        let _ = drive_owner_until_fix_file_result(&mut owner, &request_rx, &response_rx);
    }

    #[test]
    fn fix_file_stale_falls_back_when_retries_are_exhausted() {
        let (mut owner, project_key, path) = setup_fix_file_owner();
        let params = fix_file_params(project_key, path);
        let (response, response_rx) = response_channel();
        let (sender, request_rx) = unbounded();

        let request = StagedRequest::<FixFileOperation>::with_retries_left(response, 0, sender);
        owner.handle_commit_request(WorkspaceCommit::FixFile(StagedCommitMessage::Stale {
            params,
            request,
        }));

        let _ = recv_fix_file_result(&response_rx);
        assert!(request_rx.try_recv().is_err());
    }

    #[test]
    fn staged_request_reports_channel_closed_for_commit_reentry_failure() {
        let (owner, project_key, path) = setup_fix_file_owner();
        let commit = stale_fix_file_commit(project_key, path, owner.server.state_revision());
        let (response, response_rx) = response_channel();
        let (sender, receiver) = unbounded();
        drop(receiver);
        let request = StagedRequest::<FixFileOperation>::with_retries_left(response, 1, sender);

        request.send_commit(commit);

        expect_channel_closed(&response_rx);
    }

    #[test]
    fn staged_request_reports_channel_closed_for_stale_reentry_failure() {
        let (_owner, project_key, path) = setup_fix_file_owner();
        let params = fix_file_params(project_key, path);
        let (response, response_rx) = response_channel();
        let (sender, receiver) = unbounded();
        drop(receiver);
        let request = StagedRequest::<FixFileOperation>::with_retries_left(response, 1, sender);

        request.send_stale(params);

        expect_channel_closed(&response_rx);
    }
}
