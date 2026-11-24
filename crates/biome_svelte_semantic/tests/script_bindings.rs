use biome_js_parser::{JsParserOptions, parse};
use biome_js_semantic::{JsDeclarationKind, SemanticModelOptions, semantic_model};
use biome_languages::JsFileSource;
use biome_languages::javascript::{JsEmbeddingKind, SvelteEmbeddingKind, SvelteFileKind};
use biome_rowan::{TextRange, TextSize};
use biome_svelte_semantic::{
    BindingUses, Resolution, SvelteSemanticModel, SvelteSnippet, SvelteTemplateNames,
};

fn snippet(offset: u32, source: JsFileSource, text: &str) -> SvelteSnippet {
    let root = parse(text, source, JsParserOptions::default()).tree();
    SvelteSnippet {
        offset: TextSize::from(offset),
        source,
        js: semantic_model(&root, SemanticModelOptions::from(&source)),
    }
}

fn source(embedding_kind: SvelteEmbeddingKind, is_module_script: bool) -> JsFileSource {
    JsFileSource::ts().with_embedding_kind(JsEmbeddingKind::Svelte {
        embedding_kind,
        is_module_script,
        is_class_attribute: false,
        file_kind: SvelteFileKind::Component,
    })
}

#[test]
fn opaque_instance_can_shadow_a_module_declaration_but_not_an_isolated_script() {
    let snippets = [
        snippet(
            0,
            source(SvelteEmbeddingKind::Source, true),
            "const count = 1;",
        ),
        snippet(
            32,
            source(SvelteEmbeddingKind::Expression, false),
            "count++",
        ),
        snippet(48, JsFileSource::js_module(), "count++"),
    ];
    let mut template = SvelteTemplateNames::default();
    template.unknown.instance_script = true;
    let model = SvelteSemanticModel::new(&snippets, std::iter::empty(), template);
    let range = |start: u32, end: u32| TextRange::new(start.into(), end.into());
    assert_eq!(model.resolution(range(32, 37)), Some(Resolution::Unknown));
    assert!(!model.is_const_write(range(32, 37)));
    assert_eq!(
        model.resolution(range(48, 53)),
        Some(Resolution::Unresolved)
    );
    assert_eq!(model.binding_uses(0.into(), range(6, 11)), None);
}

#[test]
fn type_and_value_uses_resolve_independently() {
    let snippets = [
        snippet(
            0,
            source(SvelteEmbeddingKind::Source, true),
            "const Item = 1;",
        ),
        snippet(
            40,
            source(SvelteEmbeddingKind::Source, false),
            "type Item = string;",
        ),
        snippet(
            80,
            source(SvelteEmbeddingKind::TextInterpolation, false),
            "Item as Item",
        ),
    ];
    let model = SvelteSemanticModel::new(&snippets, std::iter::empty(), Default::default());
    let range = |start: u32, end: u32| TextRange::new(start.into(), end.into());
    assert_eq!(
        model.declaration_for_reference(range(80, 84)),
        Some(range(6, 10))
    );
    assert_eq!(
        model.declaration_for_reference(range(88, 92)),
        Some(range(45, 49))
    );
    assert_eq!(
        model.binding_uses(0.into(), range(6, 10)),
        Some(BindingUses {
            value_read: true,
            ..BindingUses::default()
        })
    );
    assert_eq!(
        model.binding_uses(40.into(), range(5, 9)),
        Some(BindingUses {
            type_read: true,
            ..BindingUses::default()
        })
    );
}

#[test]
fn cross_snippet_writes_retain_the_script_declaration_kind() {
    let snippets = [
        snippet(
            0,
            source(SvelteEmbeddingKind::Source, false),
            "import value from 'pkg'; function update() {} class Widget {}",
        ),
        snippet(
            80,
            source(SvelteEmbeddingKind::Expression, false),
            "() => { value = 1; update = () => {}; Widget = class {}; }",
        ),
    ];
    let model = SvelteSemanticModel::new(&snippets, std::iter::empty(), Default::default());
    let range = |start: u32, end: u32| TextRange::new(start.into(), end.into());
    assert_eq!(
        model.declaration_kind_for_reference(range(88, 93)),
        Some(JsDeclarationKind::Import)
    );
    assert_eq!(
        model.declaration_kind_for_reference(range(99, 105)),
        Some(JsDeclarationKind::Function)
    );
    assert_eq!(
        model.declaration_kind_for_reference(range(118, 124)),
        Some(JsDeclarationKind::Class)
    );
}

#[test]
fn updating_a_subscription_reads_the_store_but_does_not_reassign_it() {
    let snippets = [
        snippet(
            0,
            source(SvelteEmbeddingKind::Source, false),
            "const store = writable(0); let sum = 0;",
        ),
        snippet(
            64,
            source(SvelteEmbeddingKind::Expression, false),
            "() => { $store = 1; sum++; }",
        ),
    ];
    let model = SvelteSemanticModel::new(&snippets, std::iter::empty(), Default::default());
    let uses = |name| {
        model
            .binding_uses(0.into(), model.declaration(64.into(), name).unwrap())
            .unwrap()
    };
    assert_eq!(
        uses("store"),
        BindingUses {
            value_read: true,
            ..BindingUses::default()
        }
    );
    assert_eq!(
        uses("sum"),
        BindingUses {
            value_read: true,
            write: true,
            ..BindingUses::default()
        }
    );
}

#[test]
fn erased_import_is_visible_in_a_type_query_but_not_a_runtime_expression() {
    let snippets = [
        snippet(
            0,
            source(SvelteEmbeddingKind::Source, false),
            "import { type Item } from './types';",
        ),
        snippet(
            40,
            source(SvelteEmbeddingKind::Source, true),
            "export type T = typeof Item; Item();",
        ),
    ];
    let model = SvelteSemanticModel::new(&snippets, std::iter::empty(), Default::default());
    let range = |start: u32, end: u32| TextRange::new(start.into(), end.into());
    assert_eq!(
        model.declaration_for_reference(range(63, 67)),
        Some(range(14, 18))
    );
    assert!(model.is_unresolved_reference(range(69, 73)));
}

#[test]
fn template_reads_the_instance_declaration_even_when_the_module_comes_last() {
    let snippets = [
        snippet(
            8,
            source(SvelteEmbeddingKind::Source, false),
            "const name = shared;",
        ),
        snippet(
            42,
            source(SvelteEmbeddingKind::TextInterpolation, false),
            "name",
        ),
        snippet(
            72,
            source(SvelteEmbeddingKind::Source, true),
            "const name = 1, shared = 2;",
        ),
    ];
    let model = SvelteSemanticModel::new(&snippets, std::iter::empty(), Default::default());
    assert_eq!(
        model.declaration(42.into(), "name"),
        Some(TextRange::new(14.into(), 18.into()))
    );
    assert_eq!(
        model.declaration(72.into(), "name"),
        Some(TextRange::new(78.into(), 82.into()))
    );
    assert_eq!(
        model.declaration_for_reference(TextRange::new(42.into(), 46.into())),
        Some(TextRange::new(14.into(), 18.into()))
    );
    assert_eq!(
        model.declaration_for_reference(TextRange::new(21.into(), 27.into())),
        Some(TextRange::new(88.into(), 94.into()))
    );
    let name = TextRange::new(6.into(), 10.into());
    assert_eq!(model.script_binding_used(8.into(), name), Some(true));
    assert_eq!(model.script_binding_used(72.into(), name), Some(false));
    assert_eq!(
        model.script_binding_used(72.into(), TextRange::new(16.into(), 22.into())),
        Some(true)
    );
}
