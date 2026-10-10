mod builder;
mod component;
mod db;
mod layout;
mod model;
mod references;
mod snippets;

pub use db::vue_model_from_source;
pub use model::*;
pub use snippets::SfcSnippet;

use biome_html_syntax::HtmlRoot;
use biome_js_semantic::SemanticModel as JsSemanticModel;
use biome_js_syntax::AnyJsRoot;
use biome_languages::JsFileSource;
use biome_rowan::TextSize;
use builder::ModelBuilder;
use component::{ScriptInput, ScriptOutput, collect_components};
use references::{
    TemplateScopes, add_builtins_scope, collect_markup_references, collect_script_references,
    collect_template_references, collect_template_symbols,
};

/// Builds the model of the components a JavaScript or TypeScript tree
/// defines.
///
/// `js` must be the semantic model of `root`, and `source` must describe how
/// `root` was parsed.
pub fn component_model(
    root: &AnyJsRoot,
    js: &JsSemanticModel,
    source: JsFileSource,
) -> SemanticModel {
    let mut builder = ModelBuilder::default();
    let input = ScriptInput {
        root,
        js,
        source,
        offset: TextSize::from(0),
        snippet: None,
        block: None,
        setup_component: None,
    };
    let output = collect_components(&mut builder, &input);
    collect_script_references(&mut builder, &input, &output.components);
    builder.finish()
}

/// Builds the model of a single-file component.
///
/// `host` is the tree of the `.vue` file. `snippets` are the JavaScript trees
/// embedded in it: the content of its script blocks, its template
/// expressions, and the `v-bind()` expressions of its style blocks.
pub fn sfc_model(host: &HtmlRoot, snippets: &[SfcSnippet]) -> SemanticModel {
    let mut builder = ModelBuilder::default();
    let mut layout = layout::collect_layout(&mut builder, host);
    let ids = snippets::collect_snippets(&mut builder, &mut layout, snippets);

    // Script blocks, `<script setup>` first: the plain block next to it adds
    // to its component instead of defining one.
    let mut scripts: Vec<(usize, BlockId)> = snippets
        .iter()
        .enumerate()
        .filter_map(
            |(index, _)| match builder.data.snippets[ids[index].index()].host {
                SnippetHost::Script(block) => Some((index, block)),
                _ => None,
            },
        )
        .collect();
    scripts.sort_by_key(|(index, _)| !snippets[*index].source.as_embedding_kind().is_vue_setup());

    let mut setup: Option<(ComponentId, ScriptOutput)> = None;
    let mut plain: Option<ScriptOutput> = None;
    let mut inputs = Vec::new();
    for (index, block) in scripts {
        let snippet = &snippets[index];
        let input = ScriptInput {
            root: snippet.root,
            js: snippet.js,
            source: snippet.source,
            offset: snippet.content_offset,
            snippet: Some(ids[index]),
            block: Some(block),
            setup_component: setup.as_ref().map(|(component, _)| *component),
        };
        let output = collect_components(&mut builder, &input);
        let components = output.components.clone();
        if snippet.source.as_embedding_kind().is_vue_setup() {
            if let Some(component) = output.components.first().copied() {
                setup = Some((component, output));
            }
        } else if plain.is_none() {
            plain = Some(output);
        }
        inputs.push((input, components));
    }

    // With `<script setup>`, Vue merges the default export of the plain block
    // into the component, so that export stays a component of its own tree
    // and the setup component points at it.
    let merged = plain.as_ref().and_then(|output| output.default_export);
    let template_component = setup.as_ref().map(|(component, _)| *component).or(merged);
    let has_external_script = builder
        .data
        .blocks
        .iter()
        .any(|block| block.kind == BlockKind::Script && block.has_src);
    match template_component {
        Some(component) => {
            builder.component_mut(component).has_template = true;
            if setup.is_some() {
                builder.component_mut(component).merged = merged;
            }
            // A `<script src>` block declares things the model cannot read.
            if has_external_script {
                builder.open_all(component, OpenReasons::EXTERNAL_SRC);
            }
        }
        None => builder.has_unread_script = has_external_script,
    }

    // The scope chain outside the template. With `<script setup>` the
    // template sees the top-level bindings of both script blocks, then the
    // component instance. Without it, the template sees the instance only.
    let builtins = add_builtins_scope(&mut builder);
    let instance = template_component.map(|component| {
        let instance = builder.data.components[component.index()].instance;
        let merged_instance = builder.data.components[component.index()]
            .merged
            .map(|merged| builder.data.components[merged.index()].instance);
        if let Some(merged_instance) = merged_instance {
            builder.data.scopes[merged_instance.index()].parent = Some(builtins);
        }
        builder.data.scopes[instance.index()].parent = Some(merged_instance.unwrap_or(builtins));
        instance
    });
    let mut head = instance.unwrap_or(builtins);
    let mut modules = Vec::new();
    if let Some((_, setup_output)) = &setup {
        let plain_scope = plain.as_ref().and_then(|output| output.module_scope);
        if let Some(scope) = plain_scope {
            builder.data.scopes[scope.index()].parent = Some(head);
            head = scope;
        }
        if let Some(scope) = setup_output.module_scope {
            builder.data.scopes[scope.index()].parent = Some(head);
            head = scope;
            modules.push(scope);
        }
        modules.extend(plain_scope);
    }
    for block in &builder.data.blocks {
        if block.kind == BlockKind::Template {
            let scope = builder.data.elements[block.element.index()].scope;
            builder.data.scopes[scope.index()].parent = Some(head);
        }
    }
    let scopes = TemplateScopes {
        component: template_component,
        head,
        modules,
        builtins,
    };

    collect_template_symbols(&mut builder, &layout, snippets, &ids, &scopes);
    collect_template_references(&mut builder, snippets, &ids, &scopes);
    for (input, components) in &inputs {
        collect_script_references(&mut builder, input, components);
    }
    collect_markup_references(&mut builder, &scopes);

    // Classes were collected by two passes. Store them grouped by element.
    layout.class_entries.sort_by_key(|(element, _)| *element);
    for (element, entry) in layout.class_entries {
        let index = builder.data.class_entries.len() as u32;
        let classes = &mut builder.data.elements[element.index()].classes;
        if classes.start == classes.end {
            *classes = index..index + 1;
        } else {
            classes.end = index + 1;
        }
        builder.data.class_entries.push(entry);
    }

    builder.finish()
}
