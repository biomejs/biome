pub mod testing;
use camino::{Utf8Path, Utf8PathBuf};

#[salsa::db]
pub trait Db: salsa::Database {
    fn file_source_for_path(&self, path: &Utf8Path) -> Option<FileSource>;

    fn for_each_file_source(&self, f: &mut dyn FnMut(FileSource));
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
