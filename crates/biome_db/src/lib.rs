pub mod testing;
use biome_rowan::NodeCache;
use camino::{Utf8Path, Utf8PathBuf};
use std::sync::{Arc, Mutex};

#[salsa::db]
pub trait Db: salsa::Database {
    fn file_source_for_path(&self, path: &Utf8Path) -> Option<FileSource>;

    fn for_each_file_source(&self, f: &mut dyn FnMut(FileSource));

    fn node_cache_for_path(&self, _path: &Utf8Path) -> Option<Arc<Mutex<NodeCache>>> {
        None
    }
}
/// The primordial type of the biome database. It represents a file on disk
#[salsa::input]
#[derive(Debug)]
pub struct FileSource {
    #[returns(deref)]
    pub path: Utf8PathBuf,
    #[returns(ref)]
    pub content: String,
    #[returns(copy)]
    pub document_source_index: usize,

    #[returns(copy)]
    pub version: Option<i32>,
}
