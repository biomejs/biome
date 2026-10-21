use crate::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_db::Source;
use biome_languages::JsFileSource;
use biome_languages::LanguageDb;
use biome_parser::db::{AnyParsedSource, ParsedSnippet, ParsedSourceExt};

#[salsa::tracked]
pub fn semantic_model_from_source(db: &dyn LanguageDb, file: Source) -> SemanticModel {
    let parsed = file.parsed(db);
    let path = file.path(db);
    let source = db.source_from_index(file.document_source_index(db));
    let source_type = source
        .map_or(JsFileSource::try_from(path.as_path()).ok(), |s| {
            s.to_js_file_source()
        })
        .unwrap_or_default();
    semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type))
}

#[salsa::tracked]
pub fn semantic_model_from_snippet<'db>(
    db: &'db dyn LanguageDb,
    file: ParsedSnippet<'db>,
) -> SemanticModel {
    let parsed = file.parsed(db);
    let source = db.source_from_index(file.document_source_index(db));
    let source_type = source
        .and_then(|s| s.to_js_file_source())
        .unwrap_or_default();
    semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type))
}

/// Returns the cached semantic model of a parsed JavaScript source or snippet.
///
/// Returns `None` for a snippet that the current parse of its document doesn't
/// contain anymore.
pub fn js_semantic_model<'db>(
    db: &'db dyn LanguageDb,
    file: &AnyParsedSource,
) -> Option<&'db SemanticModel> {
    match file {
        AnyParsedSource::Source(source) => Some(semantic_model_from_source(db, *source)),
        AnyParsedSource::ParsedSnippet(snippet) => snippet
            .resolve(db)
            .map(|snippet| semantic_model_from_snippet(db, snippet)),
    }
}
