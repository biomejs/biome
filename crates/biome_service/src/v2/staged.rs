use crate::WorkspaceError;
use crate::v2::request::{WorkspaceCommit, WorkspaceRequest, WorkspaceResponder, WorkspaceSender};
use crate::v2::server::WorkspaceServer;
use crate::v2::snapshots::{FixFileCommit, StagedCommitResult, StagedWorkerTask};
use crate::workspace::{FixFileParams, FixFileResult};
use std::marker::PhantomData;

type CommitResult<O, P, W> = Result<StagedCommitResult<O, P, W>, WorkspaceError>;

/// A workspace operation that runs on a worker and finishes on the owner.
pub(crate) trait StagedOperation: Send + 'static {
    type Params: Clone + Send + 'static;
    type WorkerCommit: Send + 'static;
    type Output: Send + 'static;

    /// How many times stale worker state should be rebuilt before falling back.
    const MAX_RETRIES: u8 = 1;

    /// Captures current owner state into worker-owned data.
    fn prepare(
        server: &mut WorkspaceServer,
        params: Self::Params,
    ) -> Result<StagedWorkerTask<Self::WorkerCommit, Self::Params>, WorkspaceError>;

    /// Applies worker output on the owner, or asks for another worker step.
    fn commit(
        server: &mut WorkspaceServer,
        commit: Self::WorkerCommit,
    ) -> CommitResult<Self::Output, Self::Params, Self::WorkerCommit>;

    /// Runs the original operation on the owner when worker retries are exhausted.
    fn fallback(
        server: &mut WorkspaceServer,
        params: Self::Params,
    ) -> Result<Self::Output, WorkspaceError>;

    /// Wraps a staged owner message in the workspace request enum.
    fn workspace_commit(message: StagedCommitMessage<Self>) -> WorkspaceCommit
    where
        Self: Sized;
}

/// Request state shared by all owner messages for one staged operation.
pub(crate) struct StagedRequest<O: StagedOperation> {
    response: WorkspaceResponder<O::Output>,
    retries_left: u8,
    sender: WorkspaceSender,
    _operation: PhantomData<fn() -> O>,
}

impl<O: StagedOperation> StagedRequest<O> {
    pub(crate) fn new(response: WorkspaceResponder<O::Output>, sender: WorkspaceSender) -> Self {
        Self::with_retries_left(response, O::MAX_RETRIES, sender)
    }

    pub(crate) fn with_retries_left(
        response: WorkspaceResponder<O::Output>,
        retries_left: u8,
        sender: WorkspaceSender,
    ) -> Self {
        Self {
            response,
            retries_left,
            sender,
            _operation: PhantomData,
        }
    }

    #[cfg(test)]
    pub(crate) fn retries_left(&self) -> u8 {
        self.retries_left
    }

    pub(crate) fn send(self, result: Result<O::Output, WorkspaceError>) {
        self.response.send(result);
    }

    pub(crate) fn send_commit(self, commit: O::WorkerCommit) {
        let sender = self.sender.clone();
        let message = StagedCommitMessage::Commit {
            commit,
            request: self,
        };
        if let Err(error) = sender.send(WorkspaceRequest::Commit(Box::new(O::workspace_commit(
            message,
        )))) {
            send_channel_closed(error.into_inner());
        }
    }

    pub(crate) fn send_stale(self, params: O::Params) {
        let sender = self.sender.clone();
        let message = StagedCommitMessage::Stale {
            params,
            request: self,
        };
        if let Err(error) = sender.send(WorkspaceRequest::Commit(Box::new(O::workspace_commit(
            message,
        )))) {
            send_channel_closed(error.into_inner());
        }
    }

    pub(crate) fn retry(self) -> StagedRetry<O> {
        let Self {
            response,
            retries_left,
            sender,
            _operation,
        } = self;

        let request = Self {
            response,
            retries_left: retries_left.saturating_sub(1),
            sender,
            _operation,
        };

        if retries_left == 0 {
            StagedRetry::Fallback(request)
        } else {
            StagedRetry::Retry(request)
        }
    }
}

fn send_channel_closed(request: WorkspaceRequest) {
    if let WorkspaceRequest::Commit(commit) = request {
        commit.send_channel_closed();
    } else {
        tracing::warn!("failed to re-enter workspace owner with a non-commit staged request");
    }
}

/// What the owner should do after a staged operation becomes stale.
pub(crate) enum StagedRetry<O: StagedOperation> {
    Retry(StagedRequest<O>),
    Fallback(StagedRequest<O>),
}

/// A message sent back to the owner after a staged worker step.
pub(crate) enum StagedCommitMessage<O: StagedOperation> {
    Commit {
        commit: O::WorkerCommit,
        request: StagedRequest<O>,
    },
    Stale {
        params: O::Params,
        request: StagedRequest<O>,
    },
}

impl<O: StagedOperation> StagedCommitMessage<O> {
    pub(crate) fn send(self, result: Result<O::Output, WorkspaceError>) {
        match self {
            Self::Commit { request, .. } | Self::Stale { request, .. } => request.send(result),
        }
    }
}

pub(crate) struct FixFileOperation;

impl StagedOperation for FixFileOperation {
    type Params = FixFileParams;
    type WorkerCommit = FixFileCommit;
    type Output = FixFileResult;

    fn prepare(
        server: &mut WorkspaceServer,
        params: Self::Params,
    ) -> Result<StagedWorkerTask<Self::WorkerCommit, Self::Params>, WorkspaceError> {
        server
            .prepare_fix_file(params)
            .map(|snapshot| snapshot.into_staged_worker_task())
    }

    fn commit(
        server: &mut WorkspaceServer,
        commit: Self::WorkerCommit,
    ) -> Result<StagedCommitResult<Self::Output, Self::Params, Self::WorkerCommit>, WorkspaceError>
    {
        server.commit_fix_file(commit)
    }

    fn fallback(
        server: &mut WorkspaceServer,
        params: Self::Params,
    ) -> Result<Self::Output, WorkspaceError> {
        server.fix_file_on_owner(params)
    }

    fn workspace_commit(message: StagedCommitMessage<Self>) -> WorkspaceCommit {
        WorkspaceCommit::FixFile(message)
    }
}
