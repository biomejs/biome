use crate::WorkspaceError;
use biome_module_graph::PathInfoCache;
pub use biome_module_graph::ProjectDatabase;
use std::sync::{Mutex, MutexGuard};

/// Represents the state of the database in the workspace.
#[derive(Default)]
pub(crate) struct DbState {
    pub(crate) db: Mutex<ProjectDatabase>,
    pub(crate) path_info_cache: PathInfoCache,
}

impl DbState {
    pub(crate) fn lock_db(&self) -> Result<MutexGuard<'_, ProjectDatabase>, WorkspaceError> {
        self.db
            .lock()
            .map_err(|_| WorkspaceError::db_lock_poisoned())
    }
}
