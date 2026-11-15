mod builder;
mod db;
mod layout;
mod model;
mod nodes;
mod patterns;
mod references;
mod script;
mod snippets;
mod values;

pub use db::svelte_model_from_source;
pub use model::*;
pub use nodes::{
    AnalyzedSvelteModel, AnySvelteElementNode, HtmlNodeId, JsNodeId, NodeId, SvelteRow,
};
pub use snippets::SvelteSnippet;

use biome_html_syntax::HtmlRoot;
use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::AnyJsRoot;
use biome_rowan::TextSize;
use builder::ModelBuilder;
use camino::Utf8Path;
use script::Tree;

/// Builds the model of a `.svelte` component.
///
/// `host` is the tree of the component. `snippets` are the JavaScript trees
/// embedded in it: the content of its scripts and the expressions of its
/// template. `path` is the path of the file, from which the model reads what
/// SvelteKit makes of it.
pub fn component_model(
    host: &HtmlRoot,
    snippets: &[SvelteSnippet],
    path: &Utf8Path,
) -> SvelteSemanticModel {
    let mut builder = ModelBuilder::default();
    builder.data.file_kind = FileKind::Component;
    builder.data.route_file = route_file(path);
    let layout = layout::collect_layout(&mut builder, host);
    let ids = snippets::collect_snippets(&mut builder, &layout, snippets);
    let trees: Vec<_> = snippets
        .iter()
        .zip(&ids)
        .map(|(snippet, id)| Tree {
            root: snippet.root,
            js: snippet.js,
            offset: snippet.content_offset,
            snippet: Some(*id),
        })
        .collect();
    let script_of = |builder: &ModelBuilder, tree: &Tree| {
        let id = tree.snippet?;
        match builder.data.snippets[id.index()].host {
            SnippetHost::Script(script) => Some(script),
            _ => None,
        }
    };

    let mut declared = Vec::new();
    for (index, tree) in trees.iter().enumerate() {
        if let Some(script) = script_of(&builder, tree) {
            let scope = builder.data.scripts[script.index()].scope;
            script::declare_script_bindings(&mut builder, *tree, index, scope, &mut declared);
        }
    }
    for (index, tree) in trees.iter().enumerate() {
        script::declare_template_bindings(&mut builder, &layout, *tree, index, &mut declared);
    }
    for tree in &trees {
        script::collect_rune_calls(&mut builder, *tree);
    }
    let mut scripts = Vec::new();
    let mut instance = None;
    for tree in &trees {
        if let Some(script) = script_of(&builder, tree) {
            let data = &builder.data.scripts[script.index()];
            let scope = data.scope;
            if data.kind == ScriptKind::Instance {
                instance = Some(*tree);
            }
            scripts.push(*tree);
            script::collect_exports(&mut builder, *tree, scope);
        }
    }
    if let Some(instance) = instance {
        script::collect_props(&mut builder, instance, &scripts);
    }

    references::collect_declared_references(&mut builder, &trees, &declared);
    references::collect_free_references(&mut builder, &trees);
    references::collect_markup_references(&mut builder, &layout);
    references::collect_each_mutations(&mut builder, &layout, &trees);
    values::collect_store_creators(&mut builder, &trees, &declared);
    values::collect_facts(&mut builder, &trees, &declared);
    references::decide_runes_mode(&mut builder, &trees);
    builder.finish()
}

/// Builds the model of a `.svelte.js` or `.svelte.ts` module.
///
/// `js` must be the semantic model of `root`. `path` is the path of the
/// file.
pub fn module_model(
    root: &AnyJsRoot,
    js: &JsSemanticModel,
    path: &Utf8Path,
) -> SvelteSemanticModel {
    let mut builder = ModelBuilder::default();
    builder.data.file_kind = FileKind::Module;
    builder.data.route_file = route_file(path);
    builder.data.runes = true;
    let scope = builder.add_scope(ScopeKind::Module, None);
    builder.data.module_scope = Some(scope);
    let trees = [Tree {
        root,
        js,
        offset: TextSize::from(0),
        snippet: None,
    }];
    let mut declared = Vec::new();
    script::declare_script_bindings(&mut builder, trees[0], 0, scope, &mut declared);
    script::collect_rune_calls(&mut builder, trees[0]);
    script::collect_exports(&mut builder, trees[0], scope);
    references::collect_declared_references(&mut builder, &trees, &declared);
    references::collect_free_references(&mut builder, &trees);
    values::collect_store_creators(&mut builder, &trees, &declared);
    values::collect_facts(&mut builder, &trees, &declared);
    builder.finish()
}

/// Returns the meaning SvelteKit gives the file at `path`, when the file is
/// under `src/routes`.
pub fn route_file(path: &Utf8Path) -> Option<RouteFile> {
    let mut components = path.components().map(|component| component.as_str());
    let in_routes = std::iter::from_fn(|| components.next())
        .collect::<Vec<_>>()
        .windows(2)
        .any(|pair| pair == ["src", "routes"]);
    if !in_routes {
        return None;
    }
    let name = path.file_name()?;
    // A layout reset, as in `+page@.svelte` or `+layout@group.svelte`.
    let without_reset = |base: &str, extension: &str| {
        name.strip_prefix(base)
            .and_then(|rest| rest.strip_suffix(extension))
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('@'))
    };
    Some(match name {
        _ if without_reset("+page", ".svelte") => RouteFile::PageComponent,
        _ if without_reset("+layout", ".svelte") => RouteFile::LayoutComponent,
        "+error.svelte" => RouteFile::ErrorComponent,
        "+page.js" | "+page.ts" => RouteFile::PageModule,
        "+page.server.js" | "+page.server.ts" => RouteFile::PageServerModule,
        "+layout.js" | "+layout.ts" => RouteFile::LayoutModule,
        "+layout.server.js" | "+layout.server.ts" => RouteFile::LayoutServerModule,
        "+server.js" | "+server.ts" => RouteFile::Server,
        _ => return None,
    })
}
