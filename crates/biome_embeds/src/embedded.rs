use biome_db::FileSource;
use biome_parser::{AnyParse, ParsedSnippet};

#[salsa::interned]
#[derive(Debug)]
pub struct EmbeddedSource {
    file_source: FileSource,
    parsed: AnyParse,
    #[returns(ref)]
    snippets: Vec<ParsedSnippet>,
}

#[derive(Clone, Debug)]
pub struct EmbeddedSourceData {
    pub file_source: FileSource,
    pub parsed: AnyParse,
    pub snippets: Vec<ParsedSnippet>,
}

impl EmbeddedSourceData {
    pub fn intern<'db>(&self, db: &'db dyn biome_languages::LanguageDb) -> EmbeddedSource<'db> {
        EmbeddedSource::new(
            db,
            self.file_source,
            self.parsed.clone(),
            self.snippets.clone(),
        )
    }
}
