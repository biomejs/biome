use super::{
    AstroSemanticModel, AstroSemanticModelOptions, AstroSnippet, astro_model_with_options,
};
use biome_db::Source;
use biome_html_syntax::HtmlRoot;
use biome_js_semantic::semantic_model_from_snippet;
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, LanguageDb};
use biome_parser::db::SourceExt;

#[salsa::tracked]
pub fn astro_model_from_source(db: &dyn LanguageDb, file: Source) -> AstroSemanticModel {
    astro_model_from_source_with_options(db, file, AstroSemanticModelOptions::default())
}

/// Builds Astro semantics from a parsed workspace source and configured globals.
pub fn astro_model_from_source_with_options(
    db: &dyn LanguageDb,
    file: Source,
    options: AstroSemanticModelOptions,
) -> AstroSemanticModel {
    let source = db.source_from_index(file.document_source_index(db));
    let Some(DocumentFileSource::Html(html)) = source else {
        return AstroSemanticModel::default();
    };
    if !html.is_astro() {
        return AstroSemanticModel::default();
    }

    let host: HtmlRoot = file.parsed(db).tree();
    let roots: Vec<_> = file
        .snippets(db)
        .iter()
        .filter_map(|snippet| {
            let source = db
                .source_from_index(snippet.document_source_index(db))?
                .to_js_file_source()?;
            let root: AnyJsRoot = snippet.parsed(db).tree();
            Some((snippet, root, source))
        })
        .collect();
    let snippets: Vec<_> = roots
        .iter()
        .map(|(snippet, root, source)| AstroSnippet {
            root,
            js: semantic_model_from_snippet(db, **snippet),
            source: *source,
            content_range: snippet.content_range(db),
            content_offset: snippet.content_offset(db),
            has_errors: snippet.parsed(db).has_errors(),
        })
        .collect();
    astro_model_with_options(&host, &snippets, options)
}
