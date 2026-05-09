use super::inputs::ModuleDb;
use crate::ModuleInfo;
use crate::module_graph::ModuleInfoKind;
use biome_db::Db;
use camino::{Utf8Path, Utf8PathBuf};
use papaya::HashMap;
use salsa::Storage;

#[salsa::db]
#[derive(Default, Clone)]
pub struct ProjectDatabase {
    pub modules: HashMap<Utf8PathBuf, ModuleInfo>,
    storage: Storage<Self>,
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
