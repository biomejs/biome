mod builder;
mod model;
mod snippets;
mod table;

pub use model::*;
pub use snippets::AstroSnippet;

use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, AnyHtmlTagName, AstroEmbeddedContent, HtmlElement,
    HtmlEmbeddedContent, HtmlRoot, HtmlSelfClosingElement, ScriptType,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, WalkEvent};
use builder::{ModelBuilder, NewSymbol, synthetic_name};

/// Builds the semantic model of one Astro document.
pub fn astro_model(host: &HtmlRoot, snippets: &[AstroSnippet]) -> AstroSemanticModel {
    let mut builder = ModelBuilder::default();
    let builtins = builder.add_scope(ScopeKind::Builtins, None);
    let host_start = host.range().start();
    builder.add_symbol(NewSymbol {
        name: synthetic_name("Astro"),
        range: TextRange::empty(host_start),
        declaration: TextRange::empty(host_start),
        snippet: None,
        scope: builtins,
        namespace: Namespace::Binding,
        kind: SymbolKind::Builtin,
        origin: Origin::AstroBuiltin,
        type_only: false,
    });

    let frontmatter = host.frontmatter().map(|frontmatter| {
        let scope = builder.add_scope(ScopeKind::Module(None), Some(builtins));
        let content = frontmatter
            .syntax()
            .descendants()
            .find_map(AstroEmbeddedContent::cast)
            .and_then(|content| content.content_token())
            .map(|token| token.text_trimmed_range());
        let block = builder.add_block(
            BlockKind::Frontmatter,
            frontmatter.range(),
            content,
            Some(scope),
        );
        builder.data.scopes[scope].kind = ScopeKind::Module(Some(block));
        block
    });

    let block_kinds = HtmlElement::KIND_SET.union(HtmlSelfClosingElement::KIND_SET);
    for event in host.syntax().preorder_of_kinds(block_kinds) {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if let Some(element) = HtmlElement::cast_ref(&node) {
            add_element_block(&mut builder, &element);
        } else if let Some(element) = HtmlSelfClosingElement::cast_ref(&node) {
            add_self_closing_block(&mut builder, &element);
        }
    }

    let mut snippet_order: Vec<_> = (0..snippets.len()).collect();
    snippet_order.sort_by_key(|index| snippets[*index].content_range.start());
    for index in snippet_order {
        let snippet = &snippets[index];
        let embedding = snippet.source.as_embedding_kind();
        let host = if embedding.is_astro_frontmatter() {
            frontmatter.map_or(SnippetHost::Unknown, SnippetHost::Frontmatter)
        } else if embedding.is_astro_template() {
            SnippetHost::Expression
        } else {
            builder
                .data
                .blocks
                .iter()
                .enumerate()
                .find(|(_, block)| {
                    block.kind == BlockKind::Script
                        && block
                            .content
                            .is_some_and(|range| range.contains_range(snippet.content_range))
                })
                .map_or(SnippetHost::Unknown, |(index, _)| {
                    SnippetHost::Script(BlockId::new(index))
                })
        };
        builder.add_snippet(
            snippet.content_range,
            snippet.content_offset,
            snippet.source,
            host,
        );
    }

    builder.finish()
}

fn add_element_block(builder: &mut ModelBuilder, element: &HtmlElement) {
    let kind = if element.is_script_tag() {
        BlockKind::Script
    } else if element.is_style_tag() {
        BlockKind::Style
    } else {
        return;
    };
    let content = element.children().syntax().descendants().find_map(|node| {
        HtmlEmbeddedContent::cast(node)
            .and_then(|content| content.value_token().ok())
            .map(|token| token.text_range())
    });
    let module_scope =
        kind == BlockKind::Script && element.is_javascript_tag() && !element.has_astro_is_raw();
    add_block(builder, kind, element.range(), content, module_scope);
}

fn add_self_closing_block(builder: &mut ModelBuilder, element: &HtmlSelfClosingElement) {
    let Ok(AnyHtmlTagName::HtmlTagName(name)) = element.name() else {
        return;
    };
    let Ok(name) = name.value_token() else {
        return;
    };
    let kind = if name.text_trimmed().eq_ignore_ascii_case("script") {
        BlockKind::Script
    } else if name.text_trimmed().eq_ignore_ascii_case("style") {
        BlockKind::Style
    } else {
        return;
    };
    let module_scope = kind == BlockKind::Script && is_javascript_self_closing_script(element);
    add_block(builder, kind, element.range(), None, module_scope);
}

fn is_javascript_self_closing_script(element: &HtmlSelfClosingElement) -> bool {
    element
        .find_attribute_by_name("type")
        .and_then(|attribute| {
            let attribute = attribute.as_html_attribute()?;
            let initializer = attribute.initializer()?;
            let value = initializer.value().ok()?.as_static_value()?;
            Some(ScriptType::from_type_value(value.text()))
        })
        .unwrap_or_default()
        .is_javascript()
        && !has_astro_is_raw(element)
}

fn has_astro_is_raw(element: &HtmlSelfClosingElement) -> bool {
    element.attributes().iter().any(|attribute| {
        let AnyHtmlAttribute::AnyAstroDirective(AnyAstroDirective::AstroIsDirective(directive)) =
            attribute
        else {
            return false;
        };
        directive
            .value()
            .ok()
            .and_then(|value| value.name().ok())
            .and_then(|name| name.value_token().ok())
            .is_some_and(|token| token.text_trimmed() == "raw")
    })
}

fn add_block(
    builder: &mut ModelBuilder,
    kind: BlockKind,
    range: TextRange,
    content: Option<TextRange>,
    module_scope: bool,
) {
    let scope = module_scope.then(|| builder.add_scope(ScopeKind::Module(None), None));
    let block = builder.add_block(kind, range, content, scope);
    if let Some(scope) = scope {
        builder.data.scopes[scope].kind = ScopeKind::Module(Some(block));
    }
}
