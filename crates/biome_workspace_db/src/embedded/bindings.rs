use crate::embedded::EmbeddedDb;
use biome_rowan::{TextRange, TokenText};

#[salsa::input]
#[derive(Debug)]
pub struct EmbeddedBinding {
    /// The range of the binding
    #[returns(clone)]
    pub range: TextRange,
    /// The text of the binding
    #[returns(clone)]
    pub text: TokenText,
    /// Optionally, the source of the binding. It represents the path of the import/dynamic import.
    #[returns(ref)]
    pub source: Option<TokenText>,
}

#[salsa::interned]
pub struct InternedBinding {
    #[returns(ref)]
    name: TokenText,
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_by_name<'db>(
    db: &'db dyn EmbeddedDb,
    binding_name: InternedBinding<'db>,
) -> Option<EmbeddedBinding> {
    for bindings in db.bindings() {
        for binding in bindings {
            if binding.text(db).text() == *binding_name.name(db) {
                return Some(*binding);
            }
        }
    }
    None
}

#[salsa::tracked(returns(ref))]
pub fn get_binding_with_source<'db>(
    db: &'db dyn EmbeddedDb,
    binding_name: InternedBinding<'db>,
) -> Option<EmbeddedBinding> {
    for bindings in db.bindings() {
        for binding in bindings {
            if binding.text(db).text() == *binding_name.name(db) && binding.source(db).is_some() {
                return Some(*binding);
            }
        }
    }
    None
}
#[salsa::tracked(returns(ref))]
pub fn bindings_without_source<'db>(db: &'db dyn EmbeddedDb) -> Vec<Vec<(TextRange, TokenText)>> {
    db.bindings()
        .into_iter()
        .map(|bindings| {
            bindings
                .into_iter()
                .map(|b| (b.range(db), b.text(db)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

//
// #[cfg(test)]
// mod tests {
//     use crate::embed::types::{EmbedBlockKind, SvelteBlockKind};
//     use biome_js_parser::JsParserOptions;
//     use biome_js_syntax::AnyJsRoot;
//     use biome_languages::html::HtmlFileSource;
//     use biome_languages::js::JsFileSource;
//
//     fn parse_js(source: &str) -> AnyJsRoot {
//         let result = biome_js_parser::parse(source, JsFileSource::ts(), JsParserOptions::default());
//         result.tree()
//     }
//
//     fn visit_js_root(
//         service: &mut EmbeddedBuilder,
//         root: &AnyJsRoot,
//         html_file_source: HtmlFileSource,
//     ) {
//         service.visit_js_source_snippet(root, &html_file_source, Some(&EmbedBlockKind::default()));
//     }
//
//     fn visit_snippet_header(service: &mut EmbeddedBuilder, source: &str) {
//         service.visit_js_source_snippet(
//             &parse_js(source),
//             &HtmlFileSource::svelte(),
//             Some(&EmbedBlockKind::Svelte(SvelteBlockKind::Snippet)),
//         );
//     }
//
//     fn visit_render_block(service: &mut EmbeddedBuilder, source: &str) {
//         service.visit_js_source_snippet(
//             &parse_js(source),
//             &HtmlFileSource::svelte(),
//             Some(&EmbedBlockKind::Svelte(SvelteBlockKind::Render)),
//         );
//     }
//
//     fn contains_binding(service: &EmbeddedExportedBindings, binding: &str) -> bool {
//         for bindings in service.bindings.iter() {
//             if bindings.iter().any(|b| b.token_text().text() == binding) {
//                 return true;
//             }
//         }
//         false
//     }
//
//     fn visit_html_root(service: &mut EmbeddedBuilder, source: &str) {
//         let parsed = biome_html_parser::parse_html(
//             source,
//             biome_html_parser::HtmlParserOptions::default().with_vue(),
//         );
//         service.visit_html_root(&parsed.tree());
//     }
//
//     #[test]
//     fn tracks_import_and_let_js_bindings() {
//         let source = r#"import { Component } from "somewhere";
// import Component2 from "component.astro"
//
// let variable = "salut";
//  "#;
//
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//
//         service.finish(builder);
//
//         assert!(contains_binding(&service, "Component"));
//         assert!(contains_binding(&service, "Component2"));
//         assert!(contains_binding(&service, "variable"));
//     }
//
//     #[test]
//     fn tracks_import_and_binding_patterns() {
//         let source = r#"import { Component } from "somewhere";
// import Component2 from "component.astro"
//
// let {variable, foo: bar} = {};
// let [arr, ...rest] = [];
//
//  "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//
//         assert!(contains_binding(&service, "Component"));
//         assert!(contains_binding(&service, "Component2"));
//         assert!(contains_binding(&service, "variable"));
//         assert!(contains_binding(&service, "bar"));
//         assert!(contains_binding(&service, "arr"));
//         assert!(contains_binding(&service, "rest"));
//     }
//
//     #[test]
//     fn tracks_multiple_snippets() {
//         let source = r#"import { Component } from "somewhere";
// import Component2 from "component.astro"
//
// let {variable, foo: bar} = {};
// let [arr, ...rest] = [];
//
//  "#;
//
//         let source_2 = r#"import { Alas } from "somewhere";
// import Alas2 from "component.astro"
//
// let lorem = "";
//  "#;
//
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         visit_js_root(&mut builder, &parse_js(source_2), HtmlFileSource::vue());
//         service.finish(builder);
//
//         assert!(contains_binding(&service, "Component"));
//         assert!(contains_binding(&service, "Component2"));
//         assert!(contains_binding(&service, "variable"));
//         assert!(contains_binding(&service, "bar"));
//         assert!(contains_binding(&service, "arr"));
//         assert!(contains_binding(&service, "rest"));
//         assert!(contains_binding(&service, "Alas"));
//         assert!(contains_binding(&service, "Alas2"));
//         assert!(contains_binding(&service, "lorem"));
//     }
//
//     #[test]
//     fn tracks_function_declarations() {
//         let source = r#"
// function buildLink(base: string, path: string): string { return base + path; }
// async function fetchData() {}
// function* generator() {}
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "buildLink"));
//         assert!(contains_binding(&service, "fetchData"));
//         assert!(contains_binding(&service, "generator"));
//     }
//
//     #[test]
//     fn tracks_class_declarations() {
//         let source = r#"
// class MyService {}
// abstract class BaseHandler {}
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "MyService"));
//         assert!(contains_binding(&service, "BaseHandler"));
//     }
//
//     #[test]
//     fn tracks_typescript_declarations() {
//         let source = r#"
// type UserId = string;
// interface UserProfile { name: string }
// enum Direction { Up, Down }
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "UserId"));
//         assert!(contains_binding(&service, "UserProfile"));
//         assert!(contains_binding(&service, "Direction"));
//     }
//
//     #[test]
//     fn tracks_namespace_imports() {
//         let source = r#"import * as Vue from "vue";"#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "Vue"));
//     }
//
//     #[test]
//     fn tracks_vue_options_api_props_object() {
//         let source = r#"
// export default {
//   props: {
//     loading: Boolean,
//     disabled: Boolean,
//   },
// }
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "loading"));
//         assert!(contains_binding(&service, "disabled"));
//     }
//
//     #[test]
//     fn tracks_vue_options_api_props_array() {
//         let source = r#"
// export default {
//   props: ['loading', 'disabled'],
// }
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "loading"));
//         assert!(contains_binding(&service, "disabled"));
//     }
//
//     #[test]
//     fn tracks_define_props_runtime_object() {
//         // defineProps({ title: String, likes: Number })
//         let source = r#"
// defineProps({
//   title: String,
//   likes: Number,
// })
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "title"));
//         assert!(contains_binding(&service, "likes"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_plain_identifier_params() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure(image)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "figure"));
//         assert!(contains_binding(&service, "image"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_object_destructured_params() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure({ src, caption })");
//         service.finish(builder);
//         assert!(contains_binding(&service, "figure"));
//         assert!(contains_binding(&service, "src"));
//         assert!(contains_binding(&service, "caption"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_array_destructured_params() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure([first, second])");
//         service.finish(builder);
//         assert!(contains_binding(&service, "first"));
//         assert!(contains_binding(&service, "second"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_rest_params() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure(...rest)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "rest"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_nested_destructured_params() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure({ a: [b, c], ...d })");
//         service.finish(builder);
//         assert!(contains_binding(&service, "b"));
//         assert!(contains_binding(&service, "c"));
//         assert!(contains_binding(&service, "d"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_default_value_params() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure(image = fallback)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "figure"));
//         assert!(contains_binding(&service, "image"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_object_destructure_default() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure({ src, caption } = fallback)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "figure"));
//         assert!(contains_binding(&service, "src"));
//         assert!(contains_binding(&service, "caption"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_array_destructure_default() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure([first, second] = fallback)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "figure"));
//         assert!(contains_binding(&service, "first"));
//         assert!(contains_binding(&service, "second"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_nested_object_destructure_default() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure({ item: { src } = fallback })");
//         service.finish(builder);
//         assert!(contains_binding(&service, "src"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_nested_array_destructure_default() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure([{ id } = fallback])");
//         service.finish(builder);
//         assert!(contains_binding(&service, "id"));
//     }
//
//     #[test]
//     fn tracks_multiple_svelte_snippet_headers_with_destructured_defaults() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "withPlainDefault(image = fallback)");
//         visit_snippet_header(
//             &mut builder,
//             "withObjectDefault({ src, caption } = fallback)",
//         );
//         visit_snippet_header(
//             &mut builder,
//             "withArrayDefault([first, second] = emptyList)",
//         );
//         service.finish(builder);
//         assert!(contains_binding(&service, "withPlainDefault"));
//         assert!(contains_binding(&service, "withObjectDefault"));
//         assert!(contains_binding(&service, "withArrayDefault"));
//     }
//
//     #[test]
//     fn tracks_svelte_snippet_object_rest_default() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_snippet_header(&mut builder, "figure({ src, ...rest } = fallback)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "src"));
//         assert!(contains_binding(&service, "rest"));
//     }
//
//     #[test]
//     fn tracks_svelte_render_block_callee_only() {
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_render_block(&mut builder, "figure(img)");
//         service.finish(builder);
//         assert!(contains_binding(&service, "figure"));
//         assert!(!contains_binding(&service, "img"));
//     }
//
//     #[test]
//     fn tracks_define_props_runtime_array() {
//         // const props = defineProps(['foo'])
//         let source = r#"
// const props = defineProps(['foo'])
// "#;
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_js_root(&mut builder, &parse_js(source), HtmlFileSource::vue());
//         service.finish(builder);
//         assert!(contains_binding(&service, "foo"));
//     }
//
//     #[test]
//     fn tracks_vue_v_for_bindings() {
//         let source = r#"
// <template>
//   <div v-for="item in items">{{ item }}</div>
//   <div v-for="(value, key, index) of record">{{ value }} {{ key }} {{ index }}</div>
//   <div v-for="({ id, meta: { label }, ...rest }, idx) in rows">{{ id }} {{ label }} {{ rest }} {{ idx }}</div>
//   <div v-for="([first, , ...tail]) in nested">{{ first }} {{ tail }}</div>
// </template>
// "#;
//
//         let mut service = EmbeddedExportedBindings::default();
//         let mut builder = service.builder();
//         visit_html_root(&mut builder, source);
//         service.finish(builder);
//
//         assert!(contains_binding(&service, "item"));
//         assert!(contains_binding(&service, "value"));
//         assert!(contains_binding(&service, "key"));
//         assert!(contains_binding(&service, "index"));
//         assert!(contains_binding(&service, "id"));
//         assert!(contains_binding(&service, "label"));
//         assert!(contains_binding(&service, "rest"));
//         assert!(contains_binding(&service, "idx"));
//         assert!(contains_binding(&service, "first"));
//         assert!(contains_binding(&service, "tail"));
//     }
// }
