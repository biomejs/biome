use biome_module_graph::PathInfoCache;
use biome_workspace_db::WorkspaceDbHandle;

/// Represents the state of the database in the workspace.
#[derive(Default)]
pub(crate) struct DbState {
    pub(crate) handle: WorkspaceDbHandle,
    pub(crate) path_info_cache: PathInfoCache,
}
