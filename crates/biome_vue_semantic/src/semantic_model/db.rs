//! Database queries that build a [`SemanticModel`] from a stored file.

use super::{SemanticModel, SfcSnippet, component_model, sfc_model};
use biome_db::ParsedSource;
use biome_html_syntax::HtmlRoot;
use biome_js_semantic::{semantic_model_from_snippet, semantic_model_from_source};
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};

/// Returns the Vue semantic model of a stored file.
///
/// For a `.vue` file the model covers the whole single-file component. For a
/// JavaScript or TypeScript file it covers the components the file defines.
/// For any other file the model is empty.
#[salsa::tracked]
pub fn vue_model_from_source(db: &dyn LanguageDb, file: ParsedSource) -> SemanticModel {
    let source = db.source_from_index(file.document_source_index(db));
    match source {
        Some(DocumentFileSource::Html(html)) if html.is_vue() => {
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
                .map(|(snippet, root, source)| SfcSnippet {
                    root,
                    js: semantic_model_from_snippet(db, **snippet),
                    source: *source,
                    content_range: snippet.content_range(db),
                    content_offset: snippet.content_offset(db),
                })
                .collect();
            sfc_model(&host, &snippets)
        }
        Some(DocumentFileSource::Js(_)) | None => {
            let Some(js_source) = source
                .and_then(|source| source.to_js_file_source())
                .or_else(|| JsFileSource::try_from(file.path(db).as_path()).ok())
            else {
                return SemanticModel::default();
            };
            let root: AnyJsRoot = file.parsed(db).tree();
            component_model(&root, semantic_model_from_source(db, file), js_source)
        }
        _ => SemanticModel::default(),
    }
}
