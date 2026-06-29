use crate::module_graph::PathInfoCache;
use biome_workspace_db::SharedWorkspaceDb;

/// Represents the state of the database in the workspace.
#[derive(Default)]
pub struct DbState {
    pub(crate) shared_db: SharedWorkspaceDb,
    pub(crate) path_info_cache: PathInfoCache,
}
