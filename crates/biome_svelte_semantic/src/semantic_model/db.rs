//! Database queries that build a [`SvelteSemanticModel`] from a stored file.

use super::{SvelteSemanticModel, SvelteSnippet, component_model, module_model};
use biome_db::Source;
use biome_html_syntax::HtmlRoot;
use biome_js_semantic::{semantic_model_from_snippet, semantic_model_from_source};
use biome_js_syntax::AnyJsRoot;
use biome_languages::{DocumentFileSource, JsFileSource, LanguageDb};
use biome_parser::db::SourceExt;

/// Returns the Svelte semantic model of a stored file.
///
/// For a `.svelte` file the model covers the whole component. For a
/// `.svelte.js` or `.svelte.ts` module it covers the module. For any other
/// file the model is empty.
#[salsa::tracked]
pub fn svelte_model_from_source(db: &dyn LanguageDb, file: Source) -> SvelteSemanticModel {
    let source = db.source_from_index(file.document_source_index(db));
    let path = file.path(db);
    match source {
        Some(DocumentFileSource::Html(html)) if html.is_svelte() => {
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
                .map(|(snippet, root, source)| SvelteSnippet {
                    root,
                    js: semantic_model_from_snippet(db, **snippet),
                    source: *source,
                    content_range: snippet.content_range(db),
                    content_offset: snippet.content_offset(db),
                })
                .collect();
            component_model(&host, &snippets, path)
        }
        Some(DocumentFileSource::Js(_)) | None => {
            // Svelte gives a module runes semantics by its name, whatever
            // file source the database records for it.
            let is_module = JsFileSource::is_svelte_source_module_path(path)
                || source
                    .and_then(|source| source.to_js_file_source())
                    .is_some_and(|source| source.is_svelte_source_module());
            if !is_module {
                return SvelteSemanticModel::default();
            }
            let root: AnyJsRoot = file.parsed(db).tree();
            module_model(&root, semantic_model_from_source(db, file), path)
        }
        _ => SvelteSemanticModel::default(),
    }
}
