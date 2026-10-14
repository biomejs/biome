use crate::DocumentFileSource;
use camino::Utf8Path;

#[salsa::db]
pub trait LanguageDb: biome_db::Db {
    /// Returns a previously inserted file source by index.
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource>;

    /// Whether the document at `path` is HTML-like or has embedded snippets.
    fn has_embedded_content(&self, path: &Utf8Path) -> bool {
        let Some(file) = self.parsed_source_for_path(path) else {
            return false;
        };
        !file.snippets(self).is_empty()
            || self
                .source_from_index(file.document_source_index(self))
                .is_some_and(|source| source.is_html_like())
    }
}
