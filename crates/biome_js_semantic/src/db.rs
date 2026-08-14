use crate::{SemanticModel, SemanticModelOptions, semantic_model};
use biome_db::FileSource;
use biome_languages::JsFileSource;
use biome_languages::LanguageDb;
use biome_parser::{AnyParse, AnyParsedSource};

#[salsa::interned]
#[derive(Debug)]
pub struct SemanticInput {
    file_source: FileSource,
    parsed: AnyParse,
}

#[salsa::tracked(returns(ref))]
pub fn semantic_model_from_source<'db>(
    db: &'db dyn LanguageDb,
    input: SemanticInput<'db>,
) -> SemanticModel {
    let path = input.file_source(db).path(db);
    let source = db.source_from_path(path);
    let source_type = source
        .map_or(JsFileSource::try_from(path).ok(), |s| s.to_js_file_source())
        .unwrap_or_default();
    semantic_model(
        &input.parsed(db).tree(),
        SemanticModelOptions::from(&source_type),
    )
}

pub fn js_semantic_model<'db, Db>(
    db: &'db Db,
    file: FileSource,
    parse: &'db AnyParsedSource,
) -> &'db SemanticModel
where
    Db: LanguageDb,
{
    match parse {
        AnyParsedSource::ParsedSource(source) => {
            semantic_model_from_source(db, SemanticInput::new(db, file, source.clone()))
        }
        AnyParsedSource::ParsedSnippet(snippet) => {
            semantic_model_from_source(db, SemanticInput::new(db, file, snippet.parsed.clone()))
        }
    }
}
