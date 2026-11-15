//! The declaration pass: records what the JavaScript trees declare, the runes
//! they call, the props of the component and the names the scripts export.

use super::builder::{ModelBuilder, NewSymbol, synthetic_name};
use super::layout::Layout;
use super::model::*;
use biome_js_semantic::{JsDeclarationKind, SemanticModel as JsSemanticModel};
use biome_js_syntax::{
    AnyJsBinding, AnyJsBindingPattern, AnyJsClassMemberName, AnyJsDeclarationClause,
    AnyJsExportClause, AnyJsExportNamedSpecifier, AnyJsExpression, AnyJsModuleItem,
    AnyJsObjectBindingPatternMember, AnyJsRoot, AnyJsStatement, AnyJsSvelteDeclaration,
    AnyJsSvelteLegacyConstDeclarator, AnyTsName, AnyTsType, AnyTsTypeMember,
    AnyTsVariableAnnotation, JsCallExpression, JsIdentifierBinding, JsImport, JsSyntaxKind,
    JsSyntaxNode, JsVariableDeclaration, JsVariableDeclarator, TsIdentifierBinding,
    inner_string_text,
};
use biome_rowan::{
    AstNode, AstNodeList, AstSeparatedList, TextRange, TextSize, TokenText, WalkEvent,
};

/// One JavaScript tree of the file.
#[derive(Clone, Copy)]
pub(crate) struct Tree<'a> {
    pub(crate) root: &'a AnyJsRoot,
    pub(crate) js: &'a JsSemanticModel,
    /// The host offset of position zero of the tree.
    pub(crate) offset: TextSize,
    pub(crate) snippet: Option<SnippetId>,
}

impl Tree<'_> {
    pub(crate) fn host(&self, range: TextRange) -> TextRange {
        range + self.offset
    }
}

/// A symbol declared by a JavaScript binding, whose references the reference
/// pass reads from the semantic model of its tree.
pub(crate) struct Declared {
    pub(crate) symbol: SymbolId,
    /// The index of the tree in the list the passes share.
    pub(crate) tree: usize,
    pub(crate) binding: JsSyntaxNode,
}

/// Records the top-level bindings of a script, or of a module.
pub(crate) fn declare_script_bindings(
    builder: &mut ModelBuilder,
    tree: Tree,
    tree_index: usize,
    scope: ScopeId,
    declared: &mut Vec<Declared>,
) {
    // A script that holds only comments has an empty range, which has no
    // scope, and declares nothing.
    if tree.root.syntax().text_trimmed_range().is_empty() {
        return;
    }
    let module_scope = tree.js.scope(tree.root.syntax());
    // The semantic model stores the bindings of a module in a child of the
    // global scope when the root is a module.
    let scopes = std::iter::once(module_scope.clone()).chain(
        module_scope
            .children()
            .filter(|child| child.syntax() == *tree.root.syntax()),
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
            let declaration = syntax
                .ancestors()
                .find(|ancestor| {
                    AnyJsStatement::can_cast(ancestor.kind())
                        || AnyJsModuleItem::can_cast(ancestor.kind())
                })
                .unwrap_or_else(|| syntax.clone());
            let node = builder.add_js_node(tree.snippet, &declaration);
            let kind = if binding.is_imported() {
                SymbolKind::Import
            } else {
                SymbolKind::Local
            };
            let id = builder.add_symbol(NewSymbol {
                name: name_token.token_text_trimmed(),
                range: tree.host(local_range),
                declaration: tree.host(declaration.text_trimmed_range()),
                snippet: tree.snippet,
                scope,
                kind,
                declaration_kind: declaration_kind(binding.declaration_kind(), &syntax),
                node: Some(node),
            });
            let symbol = builder.symbol_mut(id);
            symbol.type_only = is_type_only_binding(&syntax);
            if kind == SymbolKind::Import {
                symbol.import = import_data(&syntax);
            }
            declared.push(Declared {
                symbol: id,
                tree: tree_index,
                binding: syntax,
            });
        }
    }
}

fn declaration_kind(kind: JsDeclarationKind, binding: &JsSyntaxNode) -> DeclarationKind {
    match kind {
        JsDeclarationKind::Class => DeclarationKind::Class,
        JsDeclarationKind::Function => DeclarationKind::Function,
        JsDeclarationKind::HoistedValue => DeclarationKind::Var,
        JsDeclarationKind::Import => DeclarationKind::Import,
        JsDeclarationKind::ImportType
        | JsDeclarationKind::Interface
        | JsDeclarationKind::Type
        | JsDeclarationKind::Generic => DeclarationKind::Type,
        JsDeclarationKind::Value => variable_kind(binding),
        _ => DeclarationKind::Other,
    }
}

/// Returns whether the variable declaration around `binding` uses `const` or
/// `let`.
fn variable_kind(binding: &JsSyntaxNode) -> DeclarationKind {
    let is_const = binding
        .ancestors()
        .find_map(JsVariableDeclaration::cast)
        .and_then(|declaration| declaration.kind().ok())
        .is_some_and(|token| token.kind() == JsSyntaxKind::CONST_KW);
    if is_const {
        DeclarationKind::Const
    } else {
        DeclarationKind::Let
    }
}

fn is_type_only_binding(binding: &JsSyntaxNode) -> bool {
    if TsIdentifierBinding::can_cast(binding.kind()) {
        return binding.parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION | JsSyntaxKind::TS_INTERFACE_DECLARATION
            )
        });
    }
    binding.ancestors().skip(1).any(|ancestor| {
        match ancestor.kind() {
            // import { type Foo } from "./foo"
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER
            | JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER
            // import type Foo from "./foo"
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

/// Reads the module an imported binding comes from, and what it stands for
/// there.
fn import_data(binding: &JsSyntaxNode) -> Option<ImportData> {
    let mut imported = None;
    for ancestor in binding.ancestors().skip(1) {
        match ancestor.kind() {
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => {
                let specifier = biome_js_syntax::JsNamedImportSpecifier::cast(ancestor)?;
                let name = specifier.name().ok()?;
                let token = name.syntax().first_token()?;
                imported = Some(Imported::Named(inner_string_text(&token)));
            }
            JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                let token = binding.first_token()?;
                imported = Some(Imported::Named(token.token_text_trimmed()));
            }
            JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER | JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                imported.get_or_insert(Imported::Default);
            }
            JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER
            | JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                imported.get_or_insert(Imported::Namespace);
            }
            JsSyntaxKind::JS_IMPORT => {
                let import = JsImport::cast(ancestor)?;
                let source = import.source_text().ok()?;
                return Some(ImportData {
                    source,
                    imported: imported?,
                });
            }
            _ => {}
        }
    }
    None
}

/// Returns the bindings a pattern declares, without those of the default
/// values and type annotations written in it.
fn pattern_binding_nodes(pattern: &JsSyntaxNode) -> Vec<JsSyntaxNode> {
    let mut found = Vec::new();
    let mut preorder = pattern.preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if matches!(
            node.kind(),
            JsSyntaxKind::JS_INITIALIZER_CLAUSE
                | JsSyntaxKind::TS_TYPE_ANNOTATION
                | JsSyntaxKind::JS_DECORATOR_LIST
        ) {
            preorder.skip_subtree();
            continue;
        }
        if JsIdentifierBinding::can_cast(node.kind()) {
            found.push(node);
        }
    }
    found
}

/// Records what the template declares in JavaScript: the name and the
/// parameters of each `{#snippet}`, and the variables of `{@const}`, `{let}`
/// and `{const}` tags.
pub(crate) fn declare_template_bindings(
    builder: &mut ModelBuilder,
    layout: &Layout,
    tree: Tree,
    tree_index: usize,
    declared: &mut Vec<Declared>,
) {
    let Some(snippet) = tree.snippet else {
        return;
    };
    let row = &builder.data.snippets[snippet.index()];
    let (host, Some(scope)) = (row.host, row.scope) else {
        return;
    };
    let mut add = |builder: &mut ModelBuilder,
                   binding: JsSyntaxNode,
                   declaration: &JsSyntaxNode,
                   scope: ScopeId,
                   kind: SymbolKind,
                   declaration_kind: DeclarationKind| {
        let Some(token) = binding.first_token() else {
            return;
        };
        let node = builder.add_js_node(Some(snippet), declaration);
        let id = builder.add_symbol(NewSymbol {
            name: token.token_text_trimmed(),
            range: tree.host(token.text_trimmed_range()),
            declaration: tree.host(declaration.text_trimmed_range()),
            snippet: Some(snippet),
            scope,
            kind,
            declaration_kind,
            node: Some(node),
        });
        declared.push(Declared {
            symbol: id,
            tree: tree_index,
            binding,
        });
    };
    match (host, tree.root) {
        (SnippetHost::SnippetSignature(block), AnyJsRoot::JsSvelteSnippetRoot(root)) => {
            let name_scope = layout
                .snippet_name_scopes
                .iter()
                .find(|(id, _)| *id == block)
                .map_or(scope, |(_, scope)| *scope);
            if let Ok(name) = root.name() {
                add(
                    builder,
                    name.syntax().clone(),
                    root.syntax(),
                    name_scope,
                    SymbolKind::Snippet,
                    DeclarationKind::Function,
                );
            }
            if let Ok(parameters) = root.parameters() {
                for binding in pattern_binding_nodes(parameters.syntax()) {
                    add(
                        builder,
                        binding.clone(),
                        &binding,
                        scope,
                        SymbolKind::SnippetParam,
                        DeclarationKind::Parameter,
                    );
                }
            }
        }
        (SnippetHost::ConstTag, AnyJsRoot::JsSvelteLegacyConstRoot(root)) => {
            if let Ok(AnyJsSvelteLegacyConstDeclarator::JsVariableDeclarator(declarator)) =
                root.declarator()
                && let Ok(id) = declarator.id()
            {
                for binding in pattern_binding_nodes(id.syntax()) {
                    add(
                        builder,
                        binding,
                        declarator.syntax(),
                        scope,
                        SymbolKind::ConstTag,
                        DeclarationKind::Const,
                    );
                }
            }
        }
        (SnippetHost::DeclarationTag, AnyJsRoot::JsSvelteDeclarationRoot(root)) => {
            if let Ok(AnyJsSvelteDeclaration::JsVariableDeclaration(declaration)) =
                root.declaration()
            {
                let declaration_kind = match declaration.kind() {
                    Ok(token) if token.kind() == JsSyntaxKind::CONST_KW => DeclarationKind::Const,
                    _ => DeclarationKind::Let,
                };
                for declarator in declaration.declarators().iter().flatten() {
                    let Ok(id) = declarator.id() else {
                        continue;
                    };
                    for binding in pattern_binding_nodes(id.syntax()) {
                        add(
                            builder,
                            binding,
                            declarator.syntax(),
                            scope,
                            SymbolKind::DeclarationTag,
                            declaration_kind,
                        );
                    }
                }
            }
        }
        _ => {}
    }
}

/// Records the names a script exports.
pub(crate) fn collect_exports(builder: &mut ModelBuilder, tree: Tree, scope: ScopeId) {
    let AnyJsRoot::JsModule(module) = tree.root else {
        return;
    };
    let local = |builder: &ModelBuilder, name: &str| {
        builder.data.scopes[scope.index()]
            .symbols
            .iter()
            .copied()
            .find(|id| builder.data.symbols[id.index()].name.text() == name)
    };
    for item in module.items() {
        let AnyJsModuleItem::JsExport(export) = item else {
            continue;
        };
        let Ok(clause) = export.export_clause() else {
            continue;
        };
        let mut exports: Vec<ExportData> = Vec::new();
        let mut push = |name: TokenText,
                        range: TextRange,
                        local: Option<SymbolId>,
                        reexport: bool,
                        type_only: bool| {
            exports.push(ExportData {
                name,
                range: tree.host(range),
                snippet: tree.snippet,
                local,
                reexport,
                type_only,
            });
        };
        match &clause {
            AnyJsExportClause::JsExportNamedClause(named) => {
                let clause_type = named.type_token().is_some();
                for specifier in named.specifiers().iter().flatten() {
                    let (local_name, exported, type_token) = match &specifier {
                        AnyJsExportNamedSpecifier::JsExportNamedShorthandSpecifier(shorthand) => {
                            let Ok(name) = shorthand.name() else {
                                continue;
                            };
                            let Ok(token) = name.value_token() else {
                                continue;
                            };
                            (
                                token.token_text_trimmed(),
                                token.clone(),
                                shorthand.type_token(),
                            )
                        }
                        AnyJsExportNamedSpecifier::JsExportNamedSpecifier(specifier) => {
                            let (Ok(local_name), Ok(exported)) =
                                (specifier.local_name(), specifier.exported_name())
                            else {
                                continue;
                            };
                            let (Ok(local_token), Some(exported_token)) =
                                (local_name.value_token(), exported.syntax().first_token())
                            else {
                                continue;
                            };
                            (
                                local_token.token_text_trimmed(),
                                exported_token,
                                specifier.type_token(),
                            )
                        }
                    };
                    let symbol = local(builder, local_name.text());
                    let type_only = clause_type
                        || type_token.is_some()
                        || symbol.is_some_and(|id| builder.data.symbols[id.index()].type_only);
                    push(
                        inner_string_text(&exported),
                        specifier.syntax().text_trimmed_range(),
                        symbol,
                        false,
                        type_only,
                    );
                }
            }
            AnyJsExportClause::JsExportNamedFromClause(from) => {
                let clause_type = from.type_token().is_some();
                for specifier in from.specifiers().iter().flatten() {
                    let name = specifier
                        .export_as()
                        .and_then(|export_as| export_as.exported_name().ok())
                        .map(|name| name.syntax().clone())
                        .or_else(|| {
                            specifier
                                .source_name()
                                .ok()
                                .map(|name| name.syntax().clone())
                        });
                    let Some(token) = name.and_then(|name| name.first_token()) else {
                        continue;
                    };
                    push(
                        inner_string_text(&token),
                        specifier.syntax().text_trimmed_range(),
                        None,
                        true,
                        clause_type || specifier.type_token().is_some(),
                    );
                }
            }
            AnyJsExportClause::JsExportFromClause(from) => {
                if let Some(token) = from
                    .export_as()
                    .and_then(|export_as| export_as.exported_name().ok())
                    .and_then(|name| name.syntax().first_token())
                {
                    push(
                        inner_string_text(&token),
                        from.syntax().text_trimmed_range(),
                        None,
                        true,
                        from.type_token().is_some(),
                    );
                }
            }
            AnyJsExportClause::JsExportDefaultDeclarationClause(_)
            | AnyJsExportClause::JsExportDefaultExpressionClause(_) => {
                let symbol = clause
                    .syntax()
                    .descendants()
                    .find_map(|node| builder.symbol_at(tree.host(node.text_trimmed_range())));
                push(
                    biome_rowan::TokenText::new_raw(biome_rowan::RawSyntaxKind(0), "default"),
                    clause.syntax().text_trimmed_range(),
                    symbol,
                    false,
                    false,
                );
            }
            AnyJsExportClause::AnyJsDeclarationClause(declaration) => {
                let type_only = matches!(
                    declaration,
                    AnyJsDeclarationClause::TsInterfaceDeclaration(_)
                        | AnyJsDeclarationClause::TsTypeAliasDeclaration(_)
                        | AnyJsDeclarationClause::TsDeclareFunctionDeclaration(_)
                );
                for node in declaration_names(declaration.syntax()) {
                    let Some(symbol) = builder.symbol_at(tree.host(node.text_trimmed_range()))
                    else {
                        continue;
                    };
                    let name = builder.data.symbols[symbol.index()].name.clone();
                    let type_only = type_only || builder.data.symbols[symbol.index()].type_only;
                    push(
                        name,
                        node.text_trimmed_range(),
                        Some(symbol),
                        false,
                        type_only,
                    );
                }
            }
            _ => {}
        }
        for export in exports {
            if let Some(symbol) = export.local {
                builder.symbol_mut(symbol).exported = true;
            }
            builder.data.exports.push(export);
        }
    }
}

/// Returns the name tokens' nodes a declaration declares at its own level,
/// without looking into function bodies, class bodies or initializers.
fn declaration_names(declaration: &JsSyntaxNode) -> Vec<JsSyntaxNode> {
    let mut found = Vec::new();
    let mut preorder = declaration.preorder();
    while let Some(event) = preorder.next() {
        let WalkEvent::Enter(node) = event else {
            continue;
        };
        if matches!(
            node.kind(),
            JsSyntaxKind::JS_INITIALIZER_CLAUSE
                | JsSyntaxKind::JS_FUNCTION_BODY
                | JsSyntaxKind::JS_CLASS_MEMBER_LIST
                | JsSyntaxKind::JS_PARAMETERS
                | JsSyntaxKind::TS_TYPE_ANNOTATION
                | JsSyntaxKind::TS_TYPE_MEMBER_LIST
                | JsSyntaxKind::TS_TYPE_PARAMETERS
        ) {
            preorder.skip_subtree();
            continue;
        }
        if JsIdentifierBinding::can_cast(node.kind()) || TsIdentifierBinding::can_cast(node.kind())
        {
            found.push(node);
            preorder.skip_subtree();
        }
    }
    found
}

/// Returns the rune `call` calls, with the range of its callee, when its
/// callee is a rune that nothing in its tree declares.
pub(crate) fn rune_of_call(
    call: &JsCallExpression,
    js: &JsSemanticModel,
) -> Option<(Rune, TextRange)> {
    let callee = call.callee().ok()?.omit_parentheses();
    let (identifier, member) = match &callee {
        AnyJsExpression::JsIdentifierExpression(identifier) => (identifier.clone(), None),
        AnyJsExpression::JsStaticMemberExpression(member) => {
            let AnyJsExpression::JsIdentifierExpression(identifier) =
                member.object().ok()?.omit_parentheses()
            else {
                return None;
            };
            let name = member.member().ok()?.value_token().ok()?;
            (identifier, Some(name.token_text_trimmed()))
        }
        _ => return None,
    };
    let reference = identifier.name().ok()?;
    if js.binding(&reference).is_some() {
        return None;
    }
    let name = reference.value_token().ok()?;
    let rune = Rune::from_name(name.text_trimmed(), member.as_ref().map(TokenText::text))?;
    Some((rune, callee.syntax().text_trimmed_range()))
}

/// Records every call of a rune in `tree`, and the symbols each call
/// initializes.
pub(crate) fn collect_rune_calls(builder: &mut ModelBuilder, tree: Tree) {
    for node in tree.root.syntax().descendants() {
        let Some(call) = JsCallExpression::cast(node) else {
            continue;
        };
        let Some((rune, callee)) = rune_of_call(&call, tree.js) else {
            continue;
        };
        let (target, declarator) = rune_target(&call);
        let symbols: Box<[_]> = declarator
            .as_ref()
            .and_then(|declarator| declarator.id().ok())
            .map_or_default(|id| {
                pattern_binding_nodes(id.syntax())
                    .iter()
                    .filter_map(|binding| {
                        builder.symbol_at(tree.host(binding.text_trimmed_range()))
                    })
                    .collect()
            });
        let id = RuneCallId::new(builder.data.rune_calls.len());
        for symbol in &symbols {
            builder.symbol_mut(*symbol).rune = Some(id);
        }
        let node = builder.add_js_node(tree.snippet, call.syntax());
        let args = call
            .arguments()
            .map_or(0, |arguments| arguments.args().len() as u8);
        builder.data.rune_calls.push(RuneCallData {
            rune,
            range: tree.host(call.syntax().text_trimmed_range()),
            callee: tree.host(callee),
            snippet: tree.snippet,
            target,
            symbols,
            args,
            node,
        });
    }
}

/// Returns where the value of a rune call is stored, and the declarator that
/// stores it in variables.
fn rune_target(call: &JsCallExpression) -> (RuneTarget, Option<JsVariableDeclarator>) {
    // Parentheses and TypeScript wrappers keep the value: the call in
    // `let map = $state(new SvelteMap()) as Map` still initializes `map`.
    let Some(parent) = AnyJsExpression::JsCallExpression(call.clone())
        .outer_expression()
        .and_then(|value| value.syntax().parent())
    else {
        return (RuneTarget::Other, None);
    };
    match parent.kind() {
        JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
            let Some(owner) = parent.parent() else {
                return (RuneTarget::Other, None);
            };
            if let Some(declarator) = JsVariableDeclarator::cast_ref(&owner) {
                return (RuneTarget::Variable, Some(declarator));
            }
            if let Some(property) = biome_js_syntax::JsPropertyClassMember::cast(owner)
                && let Ok(name) = property.name()
                && let Some(name) = class_member_name(&name)
            {
                return (RuneTarget::ClassField(name.text().into()), None);
            }
            (RuneTarget::Other, None)
        }
        JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
            let Some(assignment) = biome_js_syntax::JsAssignmentExpression::cast(parent) else {
                return (RuneTarget::Other, None);
            };
            if let Ok(biome_js_syntax::AnyJsAssignmentPattern::AnyJsAssignment(
                biome_js_syntax::AnyJsAssignment::JsStaticMemberAssignment(member),
            )) = assignment.left()
                && matches!(member.object(), Ok(AnyJsExpression::JsThisExpression(_)))
                && assignment
                    .syntax()
                    .ancestors()
                    .any(|ancestor| ancestor.kind() == JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER)
                && let Ok(name) = member.member()
            {
                // A private name keeps its `#`, which tells `this.#x` from
                // `this.x`.
                return (
                    RuneTarget::ConstructorField(name.syntax().text_trimmed().to_string().into()),
                    None,
                );
            }
            (RuneTarget::Other, None)
        }
        _ => (RuneTarget::Other, None),
    }
}

fn class_member_name(name: &AnyJsClassMemberName) -> Option<TokenText> {
    match name {
        AnyJsClassMemberName::JsLiteralMemberName(name) => name.name().ok(),
        // A private name keeps its `#`, which tells `#x` from `x`.
        AnyJsClassMemberName::JsPrivateClassMemberName(name) => {
            Some(synthetic_name(&name.syntax().text_trimmed().to_string()))
        }
        _ => None,
    }
}

/// How many type aliases and interfaces are followed to find the members of
/// the type of the props.
const MAX_TYPE_DEPTH: u8 = 16;

/// Records the props the `$props()` call of the instance script declares.
///
/// `scripts` are the trees of the instance script and of `<script module>`,
/// where the type of the props may be declared.
pub(crate) fn collect_props(builder: &mut ModelBuilder, instance: Tree, scripts: &[Tree]) {
    let Some(call_index) = builder.data.rune_calls.iter().position(|call| {
        call.rune == Rune::Props
            && call.snippet == instance.snippet
            && call.target == RuneTarget::Variable
    }) else {
        return;
    };
    let call_id = RuneCallId::new(call_index);
    let local_range = builder.data.rune_calls[call_index].range - instance.offset;
    let Some(declarator) = instance
        .root
        .syntax()
        .covering_element(local_range)
        .into_node()
        .and_then(|node| node.ancestors().find_map(JsVariableDeclarator::cast))
    else {
        return;
    };
    let mut props = PropsData {
        call: call_id,
        object: None,
        rest: None,
        pattern_open: false,
        has_type: false,
        type_open: false,
    };
    let symbol_of = |builder: &ModelBuilder, binding: &AnyJsBinding| {
        builder.symbol_at(instance.host(binding.syntax().text_trimmed_range()))
    };
    let mut found: Vec<PropData> = Vec::new();
    match declarator.id() {
        Ok(AnyJsBindingPattern::AnyJsBinding(binding)) => {
            props.object = symbol_of(builder, &binding);
        }
        Ok(AnyJsBindingPattern::JsObjectBindingPattern(pattern)) => {
            for member in pattern.properties().iter() {
                let Ok(member) = member else {
                    props.pattern_open = true;
                    continue;
                };
                match member {
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternShorthandProperty(
                        property,
                    ) => {
                        let Ok(binding) = property.identifier() else {
                            continue;
                        };
                        let Some(token) = binding.syntax().first_token() else {
                            continue;
                        };
                        let (default, bindable) = default_of(property.init(), instance);
                        found.push(PropData {
                            name: token.token_text_trimmed(),
                            range: instance.host(token.text_trimmed_range()),
                            snippet: instance.snippet,
                            local: symbol_of(builder, &binding),
                            default,
                            bindable,
                            in_type: false,
                        });
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternProperty(property) => {
                        let Some((name, range)) = property.member().ok().and_then(|member| {
                            let token = member.as_js_literal_member_name()?.value().ok()?;
                            Some((inner_string_text(&token), token.text_trimmed_range()))
                        }) else {
                            props.pattern_open = true;
                            continue;
                        };
                        let local = match property.pattern() {
                            Ok(AnyJsBindingPattern::AnyJsBinding(binding)) => {
                                symbol_of(builder, &binding)
                            }
                            _ => None,
                        };
                        let (default, bindable) = default_of(property.init(), instance);
                        found.push(PropData {
                            name,
                            range: instance.host(range),
                            snippet: instance.snippet,
                            local,
                            default,
                            bindable,
                            in_type: false,
                        });
                    }
                    AnyJsObjectBindingPatternMember::JsObjectBindingPatternRest(rest) => {
                        props.rest = rest
                            .binding()
                            .ok()
                            .and_then(|binding| symbol_of(builder, &binding));
                    }
                    _ => props.pattern_open = true,
                }
            }
        }
        _ => props.pattern_open = true,
    }

    if let Some(AnyTsVariableAnnotation::TsTypeAnnotation(annotation)) =
        declarator.variable_annotation()
        && let Ok(ty) = annotation.ty()
    {
        props.has_type = true;
        let mut members = Vec::new();
        let mut reader = TypeReader {
            scripts,
            members: &mut members,
            open: false,
        };
        reader.read(&ty, instance, 0);
        props.type_open = reader.open;
        for (name, range) in members {
            match found
                .iter_mut()
                .find(|prop| prop.name.text() == name.text())
            {
                Some(prop) => prop.in_type = true,
                None => found.push(PropData {
                    name,
                    range,
                    snippet: tree_snippet_of(scripts, range),
                    local: None,
                    default: None,
                    bindable: false,
                    in_type: true,
                }),
            }
        }
    }

    builder.data.props = found;
    builder.data.props_declaration = Some(props);
}

/// Returns the snippet of the script whose tree holds `range`.
fn tree_snippet_of(scripts: &[Tree], range: TextRange) -> Option<SnippetId> {
    scripts
        .iter()
        .find(|tree| {
            tree.host(tree.root.syntax().text_trimmed_range())
                .contains_range(range)
        })
        .and_then(|tree| tree.snippet)
}

/// Returns the range of a default value, and whether it is a call of
/// `$bindable()`.
fn default_of(
    init: Option<biome_js_syntax::JsInitializerClause>,
    tree: Tree,
) -> (Option<TextRange>, bool) {
    let Some(expression) = init.and_then(|init| init.expression().ok()) else {
        return (None, false);
    };
    let bindable = match expression.clone().omit_parentheses() {
        AnyJsExpression::JsCallExpression(call) => {
            rune_of_call(&call, tree.js).is_some_and(|(rune, _)| rune == Rune::Bindable)
        }
        _ => false,
    };
    (
        Some(tree.host(expression.syntax().text_trimmed_range())),
        bindable,
    )
}

/// Lists the members of a type written in the scripts, following
/// interfaces and type aliases declared at their top level.
struct TypeReader<'a, 'b> {
    scripts: &'a [Tree<'b>],
    /// The name and host range of each member found.
    members: &'a mut Vec<(TokenText, TextRange)>,
    /// Whether the type has members the reader cannot name.
    open: bool,
}

impl TypeReader<'_, '_> {
    fn read(&mut self, ty: &AnyTsType, tree: Tree, depth: u8) {
        if depth > MAX_TYPE_DEPTH {
            self.open = true;
            return;
        }
        match ty {
            AnyTsType::TsObjectType(object) => {
                self.read_members(object.members().iter(), tree);
            }
            AnyTsType::TsParenthesizedType(parenthesized) => match parenthesized.ty() {
                Ok(inner) => self.read(&inner, tree, depth + 1),
                Err(_) => self.open = true,
            },
            AnyTsType::TsIntersectionType(intersection) => {
                for inner in intersection.types().iter().flatten() {
                    self.read(&inner, tree, depth + 1);
                }
            }
            AnyTsType::TsReferenceType(reference) => {
                if reference.type_arguments().is_some() {
                    self.open = true;
                    return;
                }
                let Some(name) = reference.name().ok().and_then(|name| match name {
                    AnyTsName::JsReferenceIdentifier(identifier) => identifier.value_token().ok(),
                    AnyTsName::TsQualifiedName(_) => None,
                }) else {
                    self.open = true;
                    return;
                };
                self.read_declaration(name.text_trimmed(), depth);
            }
            _ => self.open = true,
        }
    }

    fn read_members(&mut self, members: impl Iterator<Item = AnyTsTypeMember>, tree: Tree) {
        for member in members {
            let name = match &member {
                AnyTsTypeMember::TsPropertySignatureTypeMember(property) => property.name().ok(),
                AnyTsTypeMember::TsMethodSignatureTypeMember(method) => method.name().ok(),
                _ => {
                    self.open = true;
                    continue;
                }
            };
            let Some(token) = name.and_then(|name| name.as_js_literal_member_name()?.value().ok())
            else {
                self.open = true;
                continue;
            };
            self.members.push((
                inner_string_text(&token),
                tree.host(token.text_trimmed_range()),
            ));
        }
    }

    /// Reads the interface or type alias named `name` declared at the top
    /// level of a script.
    fn read_declaration(&mut self, name: &str, depth: u8) {
        for tree in self.scripts {
            let AnyJsRoot::JsModule(module) = tree.root else {
                continue;
            };
            for item in module.items() {
                let declaration = match item {
                    AnyJsModuleItem::AnyJsStatement(statement) => Some(statement.into_syntax()),
                    AnyJsModuleItem::JsExport(export) => export
                        .export_clause()
                        .ok()
                        .map(|clause| clause.into_syntax()),
                    AnyJsModuleItem::JsImport(_) => None,
                };
                let Some(declaration) = declaration else {
                    continue;
                };
                if let Some(interface) =
                    biome_js_syntax::TsInterfaceDeclaration::cast_ref(&declaration)
                    && interface_name(&interface).as_deref() == Some(name)
                {
                    if interface.type_parameters().is_some() {
                        self.open = true;
                    }
                    if let Some(extends) = interface.extends_clause() {
                        for ty in extends.types().iter().flatten() {
                            self.read(&AnyTsType::TsReferenceType(ty), *tree, depth + 1);
                        }
                    }
                    self.read_members(interface.members().iter(), *tree);
                    return;
                }
                if let Some(alias) = biome_js_syntax::TsTypeAliasDeclaration::cast_ref(&declaration)
                    && alias
                        .binding_identifier()
                        .ok()
                        .and_then(|binding| binding.syntax().first_token())
                        .is_some_and(|token| token.text_trimmed() == name)
                {
                    if alias.type_parameters().is_some() {
                        self.open = true;
                    }
                    match alias.ty() {
                        Ok(ty) => self.read(&ty, *tree, depth + 1),
                        Err(_) => self.open = true,
                    }
                    return;
                }
            }
        }
        // An imported type, or a type the scripts don't declare.
        self.open = true;
    }
}

fn interface_name(interface: &biome_js_syntax::TsInterfaceDeclaration) -> Option<String> {
    let token = interface.id().ok()?.syntax().first_token()?;
    Some(token.text_trimmed().to_string())
}
