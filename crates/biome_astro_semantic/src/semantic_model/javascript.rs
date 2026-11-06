use super::builder::{ModelBuilder, NewSymbol, to_host};
use super::model::*;
use super::snippets::AstroSnippet;
use biome_js_semantic::{Binding, JsDeclarationKind};
use biome_js_syntax::{
    AnyJsIdentifierReference, AnyJsStatement, JsAssignmentExpression, JsAssignmentOperator,
    JsImport, JsSyntaxKind, TsIdentifierBinding, TsImportEqualsDeclaration, unescape_js_identifier,
};
use biome_rowan::{AstNode, Text, TokenText, WalkEvent};

pub(crate) fn collect_module_bindings(
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
            let name = name_token.token_text_trimmed();
            let semantic_name = identifier_semantic_name(name.clone());
            let local_range = name_token.text_trimmed_range();
            if seen.contains(&local_range) {
                continue;
            }
            seen.push(local_range);
            let range = to_host(local_range, snippet.range_offset());
            let declaration = syntax
                .ancestors()
                .find(|ancestor| {
                    AnyJsStatement::can_cast(ancestor.kind()) || JsImport::can_cast(ancestor.kind())
                })
                .map_or(range, |node| {
                    to_host(node.text_trimmed_range(), snippet.range_offset())
                });
            let capabilities = binding_capabilities(&binding);
            let symbol = builder.add_symbol(NewSymbol {
                name,
                semantic_name,
                range,
                declaration,
                snippet: Some(snippet_id),
                scope,
                namespace: if capabilities.value {
                    Namespace::Value
                } else if capabilities.ty {
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
                declares_value: capabilities.value,
                declares_type: capabilities.ty,
                declares_namespace: capabilities.namespace,
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
        let namespace = reference_namespace(&reference_node);
        let access = access_of(&reference_node, reference.is_write());
        let symbol_data = &builder.data.symbols[symbol];
        let resolves = symbol_data.declares_in(namespace)
            || (access == Access::Type
                && namespace == Namespace::Value
                && symbol_data.kind == SymbolKind::Import
                && symbol_data.declares_type);
        if !resolves {
            continue;
        }
        builder.add_reference(ReferenceData {
            range: to_host(token.text_trimmed_range(), snippet.range_offset()),
            name: token.token_text_trimmed(),
            snippet: Some(snippet_id),
            site: RefSite::Identifier,
            namespace,
            scope: None,
            base: RefBase::Bare,
            access,
            resolution: Resolution::Symbol(symbol),
        });
    }
}

pub(crate) fn collect_free_references(
    builder: &mut ModelBuilder,
    snippet: &AstroSnippet,
    snippet_id: SnippetId,
    scope: ScopeId,
    additional_globals: &[Box<str>],
) {
    for event in snippet
        .root
        .syntax()
        .preorder_of_kinds(AnyJsIdentifierReference::KIND_SET)
    {
        let (reference, token) = if let WalkEvent::Enter(node) = event
            && let Some(reference) = AnyJsIdentifierReference::cast_ref(&node)
            && let Ok(token) = reference.value_token()
        {
            (reference, token)
        } else {
            continue;
        };
        let name = token.token_text_trimmed();
        let namespace = reference_namespace(&reference);
        let access = access_of(&reference, false);
        let resolution = if let Some(binding) = snippet.js.binding(&reference) {
            let capabilities = binding_capabilities(&binding);
            let resolves = capabilities.declares_in(namespace)
                || (access == Access::Type
                    && namespace == Namespace::Value
                    && binding.is_imported()
                    && capabilities.ty);
            if resolves {
                continue;
            }
            if namespace == Namespace::Value {
                Resolution::Unresolved
            } else {
                Resolution::Unknowable
            }
        } else {
            match lookup(builder, scope, namespace, name.text(), access) {
                Lookup::Symbol(symbol) => Resolution::Symbol(symbol),
                Lookup::Unknowable => Resolution::Unknowable,
                Lookup::Missing
                    if namespace == Namespace::Value
                        && (snippet.js.is_global_reference(&reference)
                            || additional_globals.iter().any(|global| {
                                unescape_js_identifier(name.text()).as_ref() == global.as_ref()
                            })) =>
                {
                    Resolution::Global
                }
                Lookup::Missing if namespace != Namespace::Value => Resolution::Unknowable,
                Lookup::Missing => Resolution::Unresolved,
            }
        };
        builder.add_reference(ReferenceData {
            range: to_host(token.text_trimmed_range(), snippet.range_offset()),
            name,
            snippet: Some(snippet_id),
            site: RefSite::Identifier,
            namespace,
            scope: Some(scope),
            base: RefBase::Bare,
            access,
            resolution,
        });
    }
}

pub(crate) enum Lookup {
    Symbol(SymbolId),
    Missing,
    Unknowable,
}

pub(crate) fn lookup(
    builder: &ModelBuilder,
    mut scope: ScopeId,
    namespace: Namespace,
    name: &str,
    access: Access,
) -> Lookup {
    let name = unescape_js_identifier(name);
    loop {
        let data = &builder.data.scopes[scope];
        if let Some(symbol) = data
            .symbols
            .iter()
            .copied()
            .filter(|id| {
                let symbol = &builder.data.symbols[*id];
                let resolves = symbol.declares_in(namespace)
                    || (access == Access::Type
                        && namespace == Namespace::Value
                        && symbol.kind == SymbolKind::Import
                        && symbol.declares_type);
                resolves && symbol.semantic_name.text() == name.as_ref()
            })
            .min_by_key(|id| builder.data.symbols[*id].kind == SymbolKind::Synthetic)
        {
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

#[derive(Clone, Copy)]
struct BindingCapabilities {
    value: bool,
    ty: bool,
    namespace: bool,
}

impl BindingCapabilities {
    fn declares_in(self, namespace: Namespace) -> bool {
        match namespace {
            Namespace::Value => self.value,
            Namespace::Type => self.ty,
            Namespace::Namespace => self.namespace,
        }
    }
}

fn binding_capabilities(binding: &Binding) -> BindingCapabilities {
    let declaration_kind = binding.declaration_kind();
    let type_only = is_type_only_binding(binding);
    BindingCapabilities {
        value: !type_only
            && (declaration_kind.declares_value() || declaration_kind == JsDeclarationKind::Module),
        ty: type_only || declaration_kind.declares_type(),
        namespace: declaration_kind.declares_namespace()
            || matches!(
                declaration_kind,
                JsDeclarationKind::Enum
                    | JsDeclarationKind::Import
                    | JsDeclarationKind::ImportType
                    | JsDeclarationKind::Module
                    | JsDeclarationKind::Unknown
            ),
    }
}

pub(crate) fn identifier_semantic_name(name: TokenText) -> Text {
    let decoded = match unescape_js_identifier(name.text()) {
        std::borrow::Cow::Borrowed(_) => None,
        std::borrow::Cow::Owned(decoded) => Some(decoded),
    };
    decoded.map_or_else(|| name.into(), Into::into)
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
        if is_mutating_reference(reference) {
            Access::Mutate
        } else {
            Access::Write
        }
    } else {
        Access::Read
    }
}

fn is_mutating_reference(reference: &AnyJsIdentifierReference) -> bool {
    reference
        .syntax()
        .ancestors()
        .skip(1)
        .find_map(|ancestor| match ancestor.kind() {
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => JsAssignmentExpression::cast(ancestor)
                .and_then(|assignment| assignment.operator().ok())
                .map(|operator| operator != JsAssignmentOperator::Assign),
            JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                Some(true)
            }
            _ => None,
        })
        .unwrap_or(false)
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
