use crate::DocumentFileSource;
use camino::Utf8Path;

#[salsa::db]
pub trait LanguageDb: biome_db::Db {
    /// Returns a previously inserted file source by index.
    fn source_from_index(&self, index: usize) -> Option<DocumentFileSource>;
}

/// Whether the document at `path` is HTML-like or has embedded snippets.
pub fn has_embedded_content(db: &dyn LanguageDb, path: &Utf8Path) -> bool {
    let Some(file) = db.parsed_source_for_path(path) else {
        return false;
    };
    !file.snippets(db).is_empty()
        || db
            .source_from_index(file.document_source_index(db))
            .is_some_and(|source| source.is_html_like())
}
