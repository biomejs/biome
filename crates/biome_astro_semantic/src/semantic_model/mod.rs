mod builder;
mod client;
mod db;
mod define_vars;
mod javascript;
mod model;
mod server;
mod snippets;
mod style;
mod table;

pub use db::{astro_model_from_source, astro_model_from_source_with_options};
pub use model::*;
pub use snippets::AstroSnippet;

use biome_html_syntax::{
    AnyAstroDirective, AnyHtmlAttribute, AnyHtmlTagName, AstroDefineDirective,
    AstroEmbeddedContent, HtmlAttributeList, HtmlElement, HtmlEmbeddedContent, HtmlRoot,
    HtmlSelfClosingElement, ScriptType,
};
use biome_rowan::{AstNode, AstNodeList, TextRange, WalkEvent};
use builder::{ModelBuilder, NewBlock, NewSymbol, synthetic_name};

/// Additional inputs used to build an Astro semantic model.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AstroSemanticModelOptions {
    /// Configured globals available to embedded JavaScript and TypeScript.
    pub globals: Vec<Box<str>>,
}

/// Builds the semantic model of one Astro document.
pub fn astro_model(host: &HtmlRoot, snippets: &[AstroSnippet]) -> AstroSemanticModel {
    astro_model_with_options(host, snippets, AstroSemanticModelOptions::default())
}

/// Builds the semantic model of one Astro document with the supplied options.
pub fn astro_model_with_options(
    host: &HtmlRoot,
    snippets: &[AstroSnippet],
    options: AstroSemanticModelOptions,
) -> AstroSemanticModel {
    let mut builder = ModelBuilder::default();
    let builtins = builder.add_scope(ScopeKind::Builtins, None);
    let host_start = host.range().start();
    builder.add_symbol(NewSymbol {
        name: synthetic_name("Astro"),
        semantic_name: "Astro".into(),
        range: TextRange::empty(host_start),
        declaration: TextRange::empty(host_start),
        snippet: None,
        scope: builtins,
        namespace: Namespace::Value,
        kind: SymbolKind::Builtin,
        origin: Origin::AstroBuiltin,
        declares_value: true,
        declares_type: false,
        declares_namespace: false,
    });

    let frontmatter = host.frontmatter().map(|frontmatter| {
        let scope = builder.add_scope(ScopeKind::Module(None), Some(builtins));
        let content = frontmatter
            .syntax()
            .preorder_of_kinds(AstroEmbeddedContent::KIND_SET)
            .find_map(|event| match event {
                WalkEvent::Enter(node) => AstroEmbeddedContent::cast(node),
                WalkEvent::Leave(_) => None,
            })
            .and_then(|content| content.content_token())
            .map(|token| token.text_trimmed_range());
        let block = builder.add_block(NewBlock {
            kind: BlockKind::Frontmatter,
            range: frontmatter.range(),
            content,
            supported: true,
            globally_scoped: false,
            external: false,
            scope: Some(scope),
        });
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

    let server_scope = frontmatter
        .and_then(|block| builder.data.blocks[block].scope)
        .unwrap_or(builtins);
    let mut snippet_order: Vec<_> = (0..snippets.len()).collect();
    snippet_order.sort_by_key(|index| snippets[*index].content_range.start());
    let mut snippet_ids = vec![SnippetId::new(0); snippets.len()];
    for index in snippet_order {
        let snippet = &snippets[index];
        let embedding = snippet.source.as_embedding_kind();
        let snippet_host = if embedding.is_astro_frontmatter() {
            frontmatter.map_or(SnippetHost::Unknown, SnippetHost::Frontmatter)
        } else if embedding.is_astro_template() {
            match define_vars_target(host, &builder, snippet.content_range) {
                Some(DefineVarsTarget::Script(block)) => SnippetHost::ScriptDefineVars(block),
                Some(DefineVarsTarget::Style(block)) => SnippetHost::StyleDefineVars(block),
                Some(DefineVarsTarget::Ignored) => SnippetHost::Unknown,
                None => SnippetHost::Expression,
            }
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
        let scope = match snippet_host {
            SnippetHost::Frontmatter(block) | SnippetHost::Script(block) => {
                builder.data.blocks[block].scope
            }
            SnippetHost::Expression if snippet.has_errors => {
                Some(builder.add_scope(ScopeKind::Expression, Some(server_scope)))
            }
            SnippetHost::ScriptDefineVars(_)
            | SnippetHost::StyleDefineVars(_)
            | SnippetHost::Expression => Some(server_scope),
            SnippetHost::Unknown => None,
        };
        if snippet.has_errors
            && let Some(scope) = scope
        {
            for namespace in [Namespace::Value, Namespace::Type, Namespace::Namespace] {
                builder.open(scope, namespace, OpenReason::ParseError);
            }
        }
        snippet_ids[index] = builder.add_snippet(
            snippet.content_range,
            snippet.content_offset,
            snippet.source,
            snippet_host,
            scope,
        );
    }

    server::collect_server_semantics(
        &mut builder,
        host,
        snippets,
        &snippet_ids,
        server_scope,
        &options.globals,
    );
    client::collect_client_semantics(&mut builder, host, snippets, &snippet_ids, &options.globals);
    style::collect_style_semantics(&mut builder, host, snippets, &snippet_ids);
    builder.finish()
}

#[derive(Clone, Copy)]
enum DefineVarsTarget {
    Script(BlockId),
    Style(BlockId),
    Ignored,
}

fn define_vars_target(
    host: &HtmlRoot,
    builder: &ModelBuilder,
    content_range: TextRange,
) -> Option<DefineVarsTarget> {
    for event in host
        .syntax()
        .preorder_of_kinds(AstroDefineDirective::KIND_SET)
    {
        let element_range = if let WalkEvent::Enter(node) = event
            && let Some(directive) = AstroDefineDirective::cast_ref(&node)
            && let Ok(value) = directive.value()
            && let Some(name) = value.name().ok().and_then(|name| name.value_token().ok())
            && name.text_trimmed() == "vars"
            && let Some(expression) = value
                .initializer()
                .and_then(|initializer| initializer.value().ok())
                .and_then(|value| value.as_html_attribute_single_text_expression().cloned())
                .and_then(|expression| expression.expression().ok())
            && let Ok(token) = expression.html_literal_token()
            && token.text_range() == content_range
            && let Some(element_range) = directive.syntax().ancestors().find_map(|node| {
                if let Some(element) = HtmlElement::cast_ref(&node) {
                    Some(element.range())
                } else {
                    HtmlSelfClosingElement::cast_ref(&node).map(|element| element.range())
                }
            }) {
            element_range
        } else {
            continue;
        };
        let block = builder
            .data
            .blocks
            .iter()
            .position(|block| block.range == element_range)
            .map(BlockId::new)?;
        let data = &builder.data.blocks[block];
        return Some(match data.kind {
            BlockKind::Script if data.supported && !data.external => {
                DefineVarsTarget::Script(block)
            }
            BlockKind::Style if data.supported && !data.external => DefineVarsTarget::Style(block),
            BlockKind::Script | BlockKind::Style => DefineVarsTarget::Ignored,
            BlockKind::Frontmatter => return None,
        });
    }
    None
}

fn add_element_block(builder: &mut ModelBuilder, element: &HtmlElement) {
    let kind = if element.is_script_tag() {
        BlockKind::Script
    } else if element.is_style_tag() {
        BlockKind::Style
    } else {
        return;
    };
    let content = element
        .children()
        .syntax()
        .preorder_of_kinds(HtmlEmbeddedContent::KIND_SET)
        .find_map(|event| match event {
            WalkEvent::Enter(node) => HtmlEmbeddedContent::cast(node),
            WalkEvent::Leave(_) => None,
        })
        .and_then(|content| content.value_token().ok())
        .map(|token| token.text_range());
    let module_scope =
        kind == BlockKind::Script && element.is_javascript_tag() && !element.has_astro_is_raw();
    let supported = if kind == BlockKind::Script {
        module_scope
    } else {
        element.is_supported_style_tag()
    };
    let globally_scoped = kind == BlockKind::Style
        && element
            .opening_element()
            .is_ok_and(|opening| has_astro_is_value(&opening.attributes(), "global"));
    let external = if kind == BlockKind::Script {
        element
            .opening_element()
            .is_ok_and(|opening| classify_script(&opening.attributes()) == ScriptKind::External)
    } else {
        element.find_attribute_by_name("src").is_some()
    };
    add_block(
        builder,
        NewBlock {
            kind,
            range: element.range(),
            content,
            supported,
            globally_scoped,
            external,
            scope: None,
        },
        module_scope,
    );
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
    let globally_scoped =
        kind == BlockKind::Style && has_astro_is_value(&element.attributes(), "global");
    let external = if kind == BlockKind::Script {
        classify_script(&element.attributes()) == ScriptKind::External
    } else {
        element.find_attribute_by_name("src").is_some()
    };
    add_block(
        builder,
        NewBlock {
            kind,
            range: element.range(),
            content: None,
            supported: module_scope,
            globally_scoped,
            external,
            scope: None,
        },
        module_scope,
    );
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum ScriptKind {
    Inline,
    External,
}

fn classify_script(attributes: &HtmlAttributeList) -> ScriptKind {
    let has_src = attributes.find_attribute_by_name("src").is_some();
    let is_inline = attributes.iter().any(|attribute| {
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
            .is_some_and(|token| token.text_trimmed() == "inline")
    });

    if has_src && !is_inline {
        ScriptKind::External
    } else {
        ScriptKind::Inline
    }
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
    has_astro_is_value(&element.attributes(), "raw")
}

fn has_astro_is_value(attributes: &HtmlAttributeList, value: &str) -> bool {
    attributes.iter().any(|attribute| {
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
            .is_some_and(|token| token.text_trimmed() == value)
    })
}

fn add_block(builder: &mut ModelBuilder, mut data: NewBlock, module_scope: bool) {
    let scope = module_scope.then(|| builder.add_scope(ScopeKind::Module(None), None));
    data.scope = scope;
    let block = builder.add_block(data);
    if let Some(scope) = scope {
        builder.data.scopes[scope].kind = ScopeKind::Module(Some(block));
    }
}
