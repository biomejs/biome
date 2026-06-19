pub use biome_module_graph::ProjectDatabase;
use biome_module_graph::{PathInfoCache, ProjectDatabaseHandle};

/// Represents the state of the database in the workspace.
#[derive(Default)]
pub(crate) struct DbState {
    pub(crate) handle: ProjectDatabaseHandle,
    pub(crate) path_info_cache: PathInfoCache,
}
