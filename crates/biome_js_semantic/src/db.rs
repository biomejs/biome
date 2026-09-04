use crate::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_db::FileSource;
use biome_languages::JsFileSource;
use biome_languages::LanguageDb;
use biome_parser::AnyParsedSource;

pub fn js_semantic_model(
    db: &dyn LanguageDb,
    file: FileSource,
    parse: &AnyParsedSource,
) -> SemanticModel {
    let path = file.path(db);
    let source_type = db
        .source_from_index(file.document_source_index(db))
        .and_then(|source| source.to_js_file_source())
        .or_else(|| JsFileSource::try_from(path).ok())
        .unwrap_or_default();
    semantic_model(&parse.tree(), SemanticModelOptions::from(&source_type))
}
