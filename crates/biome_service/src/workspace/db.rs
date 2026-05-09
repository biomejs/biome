use crate::WorkspaceError;
use biome_db::Db;
use biome_module_graph::ModuleInfoKind;
use biome_module_graph::{ModuleDb, ModuleInfo, PathInfoCache};
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;
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

#[salsa::db]
#[derive(Default, Clone)]
pub struct ProjectDatabase {
    pub(crate) modules: HashMap<Utf8PathBuf, ModuleInfo>,
    storage: Storage<ProjectDatabase>,
}

impl ProjectDatabase {
    pub fn insert_module(&self, path: Utf8PathBuf, module: ModuleInfo) {
        self.modules.pin().insert(path, module);
    }
}

#[salsa::db]
impl salsa::Database for ProjectDatabase {}

#[salsa::db]
impl Db for ProjectDatabase {}

#[salsa::db]
impl ModuleDb for ProjectDatabase {
    fn module_for_path(&self, path: &Utf8Path) -> Option<ModuleInfo> {
        self.modules.pin().get(path).copied()
    }

    fn for_each_module(&self, f: &mut dyn FnMut(&Utf8Path, &ModuleInfoKind)) {
        let modules = self.modules.pin();
        for (path, &module_info) in modules.iter() {
            let kind = module_info.kind(self);
            f(path.as_path(), &kind);
        }
    }
}
