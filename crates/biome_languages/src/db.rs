use crate::DocumentFileSource;
use biome_db::FileSource;
use camino::Utf8Path;

#[salsa::db]
pub trait LanguageDb: biome_db::Db {
    /// Returns a previously inserted file source by index.
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource>;

    /// Returns the registered file and its document source for `path`.
    ///
    /// Returns `None` when the file is not registered. An unregistered document
    /// source index is represented by [`DocumentFileSource::Unknown`].
    fn file_and_source_from_path(
        &self,
        path: &Utf8Path,
    ) -> Option<(FileSource, DocumentFileSource)> {
        let file = self.file_source_for_path(path)?;
        let source = self
            .source_from_index(file.document_source_index(self))
            .unwrap_or_default();
        Some((file, source))
    }

    /// Return the [DocumentFileSource] for the given path.
    fn source_from_path(&self, path: &Utf8Path) -> Option<DocumentFileSource> {
        self.file_source_for_path(path)
            .and_then(|file| self.source_from_index(file.document_source_index(self)))
    }
}
