use super::builder::ModelBuilder;
use super::javascript::{Lookup, collect_free_references, collect_module_bindings, lookup};
use super::model::*;
use super::snippets::AstroSnippet;
use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, HtmlElement, HtmlRoot, HtmlSelfClosingElement,
};
use biome_rowan::{AstNode, WalkEvent};

pub(crate) fn collect_server_semantics(
    builder: &mut ModelBuilder,
    host: &HtmlRoot,
    snippets: &[AstroSnippet],
    ids: &[SnippetId],
    server_scope: ScopeId,
    globals: &[Box<str>],
) {
    if let Some(index) = ids
        .iter()
        .position(|id| matches!(builder.data.snippets[*id].host, SnippetHost::Frontmatter(_)))
    {
        let snippet = &snippets[index];
        let id = ids[index];
        let scope = builder.data.snippets[id].scope.unwrap_or(server_scope);
        collect_module_bindings(builder, snippet, id, scope);
        add_props_use(builder, scope);
        collect_free_references(builder, snippet, id, scope, globals);
    }

    for (snippet, id) in snippets.iter().zip(ids) {
        if matches!(
            builder.data.snippets[*id].host,
            SnippetHost::Expression
                | SnippetHost::ScriptDefineVars(_)
                | SnippetHost::StyleDefineVars(_)
        ) {
            let scope = builder.data.snippets[*id].scope.unwrap_or(server_scope);
            collect_free_references(builder, snippet, *id, scope, globals);
        }
    }

    collect_component_references(builder, host, server_scope);
}

fn add_props_use(builder: &mut ModelBuilder, scope: ScopeId) {
    // Astro reads a local `interface Props` or `type Props` as the component's
    // prop contract. Imported types and runtime declarations do not define it.
    let Some(symbol) = builder.data.scopes[scope]
        .symbols
        .iter()
        .copied()
        .find(|id| {
            let symbol = &builder.data.symbols[*id];
            symbol.semantic_name.text() == "Props"
                && symbol.kind == SymbolKind::Local
                && symbol.declares_type
                && !symbol.declares_value
        })
    else {
        return;
    };
    let data = &builder.data.symbols[symbol];
    let range = data.range;
    let name = data.name.clone();
    let snippet = data.snippet;
    builder.add_reference(ReferenceData {
        range,
        name,
        snippet,
        site: RefSite::Implicit,
        namespace: Namespace::Type,
        scope: Some(scope),
        base: RefBase::Bare,
        access: Access::Type,
        resolution: Resolution::Symbol(symbol),
    });
}

fn collect_component_references(builder: &mut ModelBuilder, host: &HtmlRoot, scope: ScopeId) {
    let element_kinds = HtmlElement::KIND_SET.union(HtmlSelfClosingElement::KIND_SET);
    for event in host.syntax().preorder_of_kinds(element_kinds) {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        let name = if let Some(element) = HtmlElement::cast_ref(&node) {
            element
                .opening_element()
                .ok()
                .and_then(|opening| opening.name().ok())
        } else if let Some(element) = HtmlSelfClosingElement::cast_ref(&node) {
            element.name().ok()
        } else {
            None
        };
        let Some(name) = name else {
            continue;
        };
        let Some(token) = component_token(&name) else {
            continue;
        };
        let name = token.token_text_trimmed();
        let resolution = match lookup(builder, scope, Namespace::Value, name.text()) {
            Lookup::Symbol(symbol) => Resolution::Symbol(symbol),
            Lookup::Unknowable => Resolution::Unknowable,
            Lookup::Missing => Resolution::Unresolved,
        };
        builder.add_reference(ReferenceData {
            range: token.text_trimmed_range(),
            name,
            snippet: None,
            site: RefSite::TagName,
            namespace: Namespace::Value,
            scope: Some(scope),
            base: RefBase::Bare,
            access: Access::Read,
            resolution,
        });
    }
}

fn component_token(name: &AnyHtmlTagName) -> Option<biome_html_syntax::HtmlSyntaxToken> {
    match name {
        AnyHtmlTagName::HtmlComponentName(component) => component.value_token().ok(),
        AnyHtmlTagName::HtmlMemberName(member) => component_object_token(&member.object().ok()?),
        AnyHtmlTagName::HtmlTagName(_) => None,
    }
}

fn component_object_token(
    name: &AnyHtmlComponentObjectName,
) -> Option<biome_html_syntax::HtmlSyntaxToken> {
    match name {
        AnyHtmlComponentObjectName::HtmlComponentName(component) => component.value_token().ok(),
        AnyHtmlComponentObjectName::HtmlMemberName(member) => {
            component_object_token(&member.object().ok()?)
        }
        AnyHtmlComponentObjectName::HtmlTagName(tag) => tag.value_token().ok(),
    }
}
