use crate::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_db::{AnyParsedSource, ParsedSnippet, ParsedSource};
use biome_languages::db::LanguageDb;
use biome_languages::js::JsFileSource;

#[salsa::db]
pub trait JsSemanticDb: biome_db::Db + LanguageDb {}

#[salsa::tracked]
pub fn semantic_model_from_source(db: &dyn JsSemanticDb, file: ParsedSource) -> SemanticModel {
    let parsed = file.parsed(db);
    let path = file.path(db);
    let source = db.source_from_index(file.document_source_index(db));
    let source_type = source
        .map(|s| s.to_js_file_source())
        .unwrap_or(JsFileSource::try_from(path.as_path()).ok())
        .unwrap_or_default();
    semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type))
}

#[salsa::tracked]
pub(crate) fn semantic_model_from_snippet(
    db: &dyn JsSemanticDb,
    file: ParsedSnippet,
) -> SemanticModel {
    let parsed = file.parsed(db);
    let source = db.source_from_index(file.document_source_index(db));
    let source_type = source
        .and_then(|s| s.to_js_file_source())
        .unwrap_or_default();
    semantic_model(&parsed.tree(), SemanticModelOptions::from(&source_type))
}

pub fn js_semantic_model<Db>(db: &Db, file: AnyParsedSource) -> SemanticModel
where
    Db: JsSemanticDb,
{
    match file {
        AnyParsedSource::ParsedSource(source) => semantic_model_from_source(db, source),
        AnyParsedSource::ParsedSnippet(snippet) => semantic_model_from_snippet(db, snippet),
    }
}
