use super::builder::{ModelBuilder, NewSymbol, to_host};
use super::model::*;
use super::snippets::AstroSnippet;
use biome_html_syntax::{
    AnyHtmlComponentObjectName, AnyHtmlTagName, HtmlElement, HtmlRoot, HtmlSelfClosingElement,
};
use biome_js_semantic::{Binding, JsDeclarationKind};
use biome_js_syntax::{
    AnyJsIdentifierReference, AnyJsStatement, JsImport, JsSyntaxKind, TsIdentifierBinding,
    TsImportEqualsDeclaration, unescape_js_identifier,
};
use biome_rowan::{AstNode, WalkEvent};

pub(crate) fn collect_server_semantics(
    builder: &mut ModelBuilder,
    host: &HtmlRoot,
    snippets: &[AstroSnippet],
    ids: &[SnippetId],
    server_scope: ScopeId,
) {
    if let Some(index) = ids
        .iter()
        .position(|id| matches!(builder.data.snippets[*id].host, SnippetHost::Frontmatter(_)))
    {
        let snippet = &snippets[index];
        let id = ids[index];
        let scope = builder.data.snippets[id].scope.unwrap_or(server_scope);
        collect_frontmatter_bindings(builder, snippet, id, scope);
        add_props_use(builder, scope);
        collect_free_references(builder, snippet, id, scope);
    }

    for (snippet, id) in snippets.iter().zip(ids) {
        if matches!(builder.data.snippets[*id].host, SnippetHost::Expression) {
            let scope = builder.data.snippets[*id].scope.unwrap_or(server_scope);
            collect_free_references(builder, snippet, *id, scope);
        }
    }

    collect_component_references(builder, host, server_scope);
}

fn collect_frontmatter_bindings(
    builder: &mut ModelBuilder,
    snippet: &AstroSnippet,
    snippet_id: SnippetId,
    scope: ScopeId,
) {
    if snippet.root.syntax().text_trimmed_range().is_empty() {
        return;
    }
    let module_scope = snippet.js.scope(snippet.root.syntax());
    let scopes = std::iter::once(module_scope.clone()).chain(
        module_scope
            .children()
            .filter(|child| child.syntax() == *snippet.root.syntax()),
    );
    let mut seen = Vec::new();
    for js_scope in scopes {
        for binding in js_scope.bindings() {
            let syntax = binding.syntax();
            let Some(name_token) = syntax.first_token() else {
                continue;
            };
            let local_range = name_token.text_trimmed_range();
            if seen.contains(&local_range) {
                continue;
            }
            seen.push(local_range);
            let range = to_host(local_range, snippet.content_offset);
            let declaration = syntax
                .ancestors()
                .find(|ancestor| {
                    AnyJsStatement::can_cast(ancestor.kind()) || JsImport::can_cast(ancestor.kind())
                })
                .map_or(range, |node| {
                    to_host(node.text_trimmed_range(), snippet.content_offset)
                });
            let declaration_kind = binding.declaration_kind();
            let type_only = is_type_only_binding(&binding);
            let declares_value = !type_only
                && (declaration_kind.declares_value()
                    || declaration_kind == JsDeclarationKind::Module);
            let declares_type = type_only || declaration_kind.declares_type();
            let declares_namespace = declaration_kind.declares_namespace()
                || matches!(
                    declaration_kind,
                    JsDeclarationKind::Enum
                        | JsDeclarationKind::Import
                        | JsDeclarationKind::ImportType
                        | JsDeclarationKind::Module
                        | JsDeclarationKind::Unknown
                );
            let symbol = builder.add_symbol(NewSymbol {
                name: name_token.token_text_trimmed(),
                range,
                declaration,
                snippet: Some(snippet_id),
                scope,
                namespace: if declares_value {
                    Namespace::Value
                } else if declares_type {
                    Namespace::Type
                } else {
                    Namespace::Namespace
                },
                kind: if binding.is_imported() {
                    SymbolKind::Import
                } else {
                    SymbolKind::Local
                },
                origin: Origin::None,
                declares_value,
                declares_type,
                declares_namespace,
            });
            collect_bound_references(builder, snippet, snippet_id, symbol, &binding);
        }
    }
}

fn collect_bound_references(
    builder: &mut ModelBuilder,
    snippet: &AstroSnippet,
    snippet_id: SnippetId,
    symbol: SymbolId,
    binding: &Binding,
) {
    for reference in binding.all_references() {
        let Some(reference_node) = AnyJsIdentifierReference::cast(reference.syntax()) else {
            continue;
        };
        let Ok(token) = reference_node.value_token() else {
            continue;
        };
        builder.add_reference(ReferenceData {
            range: to_host(token.text_trimmed_range(), snippet.content_offset),
            name: token.token_text_trimmed(),
            snippet: Some(snippet_id),
            site: RefSite::Identifier,
            namespace: reference_namespace(&reference_node),
            scope: None,
            base: RefBase::Bare,
            access: access_of(&reference_node, reference.is_write()),
            resolution: Resolution::Symbol(symbol),
        });
    }
}

fn collect_free_references(
    builder: &mut ModelBuilder,
    snippet: &AstroSnippet,
    snippet_id: SnippetId,
    scope: ScopeId,
) {
    for event in snippet
        .root
        .syntax()
        .preorder_of_kinds(AnyJsIdentifierReference::KIND_SET)
    {
        let (reference, token) = if let WalkEvent::Enter(node) = event
            && let Some(reference) = AnyJsIdentifierReference::cast_ref(&node)
            && snippet.js.binding(&reference).is_none()
            && let Ok(token) = reference.value_token()
        {
            (reference, token)
        } else {
            continue;
        };
        let name = token.token_text_trimmed();
        let namespace = reference_namespace(&reference);
        let resolution = match lookup(builder, scope, namespace, name.text()) {
            Lookup::Symbol(symbol) => Resolution::Symbol(symbol),
            Lookup::Unknowable => Resolution::Unknowable,
            Lookup::Missing
                if namespace == Namespace::Value && snippet.js.is_global_reference(&reference) =>
            {
                Resolution::Global
            }
            Lookup::Missing if namespace != Namespace::Value => Resolution::Unknowable,
            Lookup::Missing => Resolution::Unresolved,
        };
        builder.add_reference(ReferenceData {
            range: to_host(token.text_trimmed_range(), snippet.content_offset),
            name,
            snippet: Some(snippet_id),
            site: RefSite::Identifier,
            namespace,
            scope: Some(scope),
            base: RefBase::Bare,
            access: access_of(&reference, false),
            resolution,
        });
    }
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
            names_equal(symbol.name.text(), "Props")
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

enum Lookup {
    Symbol(SymbolId),
    Missing,
    Unknowable,
}

fn lookup(builder: &ModelBuilder, mut scope: ScopeId, namespace: Namespace, name: &str) -> Lookup {
    let name = unescape_js_identifier(name);
    loop {
        let data = &builder.data.scopes[scope];
        if let Some(symbol) = data.symbols.iter().copied().find(|id| {
            let symbol = &builder.data.symbols[*id];
            symbol.declares_in(namespace)
                && unescape_js_identifier(symbol.name.text()).as_ref() == name.as_ref()
        }) {
            return Lookup::Symbol(symbol);
        }
        if !data.open[namespace.index()].is_empty() {
            return Lookup::Unknowable;
        }
        let Some(parent) = data.parent else {
            return Lookup::Missing;
        };
        scope = parent;
    }
}

fn names_equal(left: &str, right: &str) -> bool {
    unescape_js_identifier(left) == unescape_js_identifier(right)
}

fn reference_namespace(reference: &AnyJsIdentifierReference) -> Namespace {
    let outer = reference
        .syntax()
        .ancestors()
        .skip(1)
        .find(|node| node.kind() != JsSyntaxKind::TS_QUALIFIED_NAME);
    if outer.is_some_and(|node| node.kind() == JsSyntaxKind::TS_TYPEOF_TYPE) {
        return Namespace::Value;
    }
    if reference
        .syntax()
        .parent()
        .is_some_and(|parent| parent.kind() == JsSyntaxKind::TS_QUALIFIED_NAME)
    {
        return Namespace::Namespace;
    }
    if reference.is_only_type() {
        Namespace::Type
    } else {
        Namespace::Value
    }
}

fn access_of(reference: &AnyJsIdentifierReference, is_write: bool) -> Access {
    if reference.is_only_type() {
        Access::Type
    } else if is_write
        || matches!(
            reference,
            AnyJsIdentifierReference::JsIdentifierAssignment(_)
        )
    {
        Access::Write
    } else {
        Access::Read
    }
}

fn is_type_only_binding(binding: &Binding) -> bool {
    let syntax = binding.syntax();
    if TsIdentifierBinding::can_cast(syntax.kind()) {
        return syntax.parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION | JsSyntaxKind::TS_INTERFACE_DECLARATION
            )
        });
    }
    syntax.ancestors().skip(1).any(|ancestor| {
        if TsImportEqualsDeclaration::cast_ref(&ancestor)
            .is_some_and(|declaration| declaration.type_token().is_some())
        {
            return true;
        }
        match ancestor.kind() {
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER
            | JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
            | JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE
            | JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE
            | JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => ancestor
                .children_with_tokens()
                .filter_map(|element| element.into_token())
                .any(|token| token.kind() == JsSyntaxKind::TYPE_KW),
            _ => false,
        }
    })
}
