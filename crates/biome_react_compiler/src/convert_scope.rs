use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use indexmap::IndexMap;
use react_compiler_ast::scope::{
    BindingData, BindingId, BindingKind, ImportBindingData, ImportBindingKind, ScopeData, ScopeId,
    ScopeInfo, ScopeKind,
};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::collections::HashMap;

pub(crate) fn convert_scope_info(model: &SemanticModel) -> ScopeInfo {
    let mut scopes = Vec::new();
    let mut bindings = Vec::new();
    let mut semantic_to_react_binding = HashMap::new();

    for binding in model.all_bindings() {
        let id = BindingId(bindings.len() as u32);
        let declaration = binding.syntax();
        let declaration_start: u32 = declaration.text_trimmed_range().start().into();
        semantic_to_react_binding.insert(declaration_start, id);
        bindings.push(BindingData {
            id,
            name: declaration.text_trimmed().to_string(),
            kind: binding_kind_of(&declaration, binding.declaration_kind()),
            scope: ScopeId(0),
            declaration_type: declaration_type(&declaration),
            declaration_start: Some(declaration_start),
            declaration_node_id: Some(node_id_from_start(declaration_start)),
            import: import_binding_of(&declaration),
        });
    }

    // Biome models a function as multiple nested scopes: a signature scope on
    // the function node (holding parameters) and a separate `JS_FUNCTION_BODY`
    // block scope (holding the body's `let`/`const`/`var`). The React compiler,
    // like Babel/OXC, expects a single function scope holding both. Merge each
    // `JS_FUNCTION_BODY` scope into its parent so bindings, scope kinds, and the
    // local-vs-context-variable classification match the compiler's model.
    //
    // `model.scopes()` yields scopes whose parent always has a smaller id, so a
    // single pass over ids in ascending order resolves canonical ids before they
    // are referenced as parents.
    let mut all_scopes: Vec<_> = model.scopes().collect();
    all_scopes.sort_by_key(|scope| scope.id().index());

    let mut canonical = vec![ScopeId(0); all_scopes.len()];
    let mut merged_away = vec![false; all_scopes.len()];
    let mut next_react_id = 0u32;
    for scope in &all_scopes {
        let idx = scope.id().index();
        let is_function_body =
            scope.syntax().kind() == biome_js_syntax::JsSyntaxKind::JS_FUNCTION_BODY;
        match scope.parent() {
            Some(parent) if is_function_body => {
                merged_away[idx] = true;
                canonical[idx] = canonical[parent.id().index()];
            }
            _ => {
                canonical[idx] = ScopeId(next_react_id);
                next_react_id += 1;
            }
        }
    }

    let mut node_to_scope = FxHashMap::default();
    let mut node_to_scope_end = FxHashMap::default();
    let mut node_id_to_scope = FxHashMap::default();
    let mut scope_parent: Vec<Option<ScopeId>> = vec![None; next_react_id as usize];
    let mut scope_kinds: Vec<ScopeKind> = vec![ScopeKind::Block; next_react_id as usize];
    let mut scope_bindings: Vec<FxHashMap<String, BindingId>> =
        vec![FxHashMap::default(); next_react_id as usize];

    for scope in &all_scopes {
        let idx = scope.id().index();
        let react_id = canonical[idx];

        // Bindings declared in this Biome scope belong to its canonical scope
        // (so a merged function body's bindings land in the function scope).
        for binding in scope.bindings() {
            let declaration = binding.syntax();
            let declaration_start: u32 = declaration.text_trimmed_range().start().into();
            if let Some(&binding_id) = semantic_to_react_binding.get(&declaration_start) {
                scope_bindings[react_id.0 as usize]
                    .insert(declaration.text_trimmed().to_string(), binding_id);
                bindings[binding_id.0 as usize].scope = react_id;
            }
        }

        // Node lookups for both the function node and its merged body resolve to
        // the canonical (function) scope.
        let syntax = scope.syntax();
        let range = syntax.text_trimmed_range();
        let start = range.start().into();
        node_to_scope.insert(start, react_id);
        node_to_scope_end.insert(start, range.end().into());
        node_id_to_scope.insert(node_id_from_start(start), react_id);

        if merged_away[idx] {
            continue;
        }

        let parent = scope.parent().map(|parent| canonical[parent.id().index()]);
        scope_parent[react_id.0 as usize] = parent;
        scope_kinds[react_id.0 as usize] = if parent.is_none() {
            // The root scope is the program scope regardless of which node Biome
            // reports as its host syntax.
            ScopeKind::Program
        } else {
            scope_kind(&syntax)
        };
    }

    for react_id in 0..next_react_id {
        scopes.push(ScopeData {
            id: ScopeId(react_id),
            parent: scope_parent[react_id as usize],
            kind: scope_kinds[react_id as usize].clone(),
            bindings: std::mem::take(&mut scope_bindings[react_id as usize]),
        });
    }

    let mut reference_to_binding = IndexMap::<u32, BindingId, FxBuildHasher>::default();
    let mut ref_node_id_to_binding = IndexMap::<u32, BindingId, FxBuildHasher>::default();
    for binding in model.all_bindings() {
        let declaration = binding.syntax();
        let declaration_start: u32 = declaration.text_trimmed_range().start().into();
        let Some(&binding_id) = semantic_to_react_binding.get(&declaration_start) else {
            continue;
        };

        if let Some(start) = bindings[binding_id.0 as usize].declaration_start {
            reference_to_binding.entry(start).or_insert(binding_id);
            ref_node_id_to_binding
                .entry(node_id_from_start(start))
                .or_insert(binding_id);
        }

        for reference in binding.all_references() {
            let start = reference.range_start().into();
            reference_to_binding.insert(start, binding_id);
            ref_node_id_to_binding.insert(node_id_from_start(start), binding_id);
        }
    }

    ScopeInfo {
        scopes,
        bindings,
        node_to_scope,
        node_to_scope_end,
        reference_to_binding,
        ref_node_id_to_binding,
        node_id_to_scope,
        program_scope: canonical[model.global_scope().id().index()],
    }
}

fn node_id_from_start(start: u32) -> u32 {
    start.saturating_add(1)
}

/// Extract import metadata for a binding declared by an `import` statement.
///
/// The compiler resolves a module-scope reference to a builtin type (e.g.
/// `useEffect`/`useState`/`useRef`) only when the binding carries import info
/// pointing at a known React module. Without it, an imported hook is treated as
/// a generic module-local custom hook and every type-based validation (the
/// effect family, setState/memo-dependency inference, ref access) silently skips
/// it. Returns `None` for non-import bindings.
fn import_binding_of(decl_node: &biome_js_syntax::JsSyntaxNode) -> Option<ImportBindingData> {
    use biome_js_syntax::JsSyntaxKind::*;
    use biome_js_syntax::{JsImport, JsModuleSource, JsNamedImportSpecifier};
    use biome_rowan::AstNode;

    let (kind, imported) = decl_node
        .ancestors()
        .find_map(|ancestor| match ancestor.kind() {
            JS_DEFAULT_IMPORT_SPECIFIER => Some((ImportBindingKind::Default, None)),
            JS_NAMESPACE_IMPORT_SPECIFIER => Some((ImportBindingKind::Namespace, None)),
            // `import { foo }` — the imported name equals the local binding name.
            JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => Some((
                ImportBindingKind::Named,
                Some(decl_node.text_trimmed().to_string()),
            )),
            // `import { foo as bar }` — the imported name is the original `foo`.
            JS_NAMED_IMPORT_SPECIFIER => {
                let imported = JsNamedImportSpecifier::unwrap_cast(ancestor)
                    .name()
                    .ok()
                    .and_then(|name| name.as_js_literal_export_name()?.inner_string_text().ok())
                    .map(|text| text.to_string());
                Some((ImportBindingKind::Named, imported))
            }
            _ => None,
        })?;

    let source = decl_node
        .ancestors()
        .find_map(JsImport::cast)?
        .syntax()
        .descendants()
        .find_map(JsModuleSource::cast)?
        .inner_string_text()
        .ok()?
        .to_string();

    Some(ImportBindingData {
        source,
        kind,
        imported,
    })
}

/// Classify a binding into the compiler's `BindingKind`.
///
/// `JsDeclarationKind` collapses `let`/`const`/`var` into a single `Value`
/// variant and does not single out parameters, but the compiler relies on the
/// const/let/var/param distinction (e.g. a captured `const` is a frozen value,
/// not a reassignable context variable). We recover it by walking the binding's
/// declaration ancestors, which also handles destructured bindings uniformly
/// (the enclosing `const {x} = ...` is found the same way as a plain `const x`).
fn binding_kind_of(
    decl_node: &biome_js_syntax::JsSyntaxNode,
    kind: JsDeclarationKind,
) -> BindingKind {
    use biome_js_syntax::JsSyntaxKind::*;
    use biome_rowan::AstNode;

    for ancestor in decl_node.ancestors() {
        match ancestor.kind() {
            JS_FORMAL_PARAMETER | JS_REST_PARAMETER | JS_BOGUS_PARAMETER => {
                return BindingKind::Param;
            }
            // A named function/class expression's own name lives inside it and
            // must not be mistaken for an outer `const`/`var` declarator.
            JS_FUNCTION_EXPRESSION | JS_CLASS_EXPRESSION => return BindingKind::Local,
            JS_CATCH_DECLARATION => return BindingKind::Let,
            JS_VARIABLE_DECLARATION => {
                let declaration = biome_js_syntax::JsVariableDeclaration::unwrap_cast(ancestor);
                return if declaration.is_const() {
                    BindingKind::Const
                } else if declaration.is_var() {
                    BindingKind::Var
                } else {
                    BindingKind::Let
                };
            }
            JS_FUNCTION_DECLARATION | JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                return BindingKind::Hoisted;
            }
            _ => {}
        }
    }

    binding_kind(kind)
}

fn binding_kind(kind: JsDeclarationKind) -> BindingKind {
    match kind {
        JsDeclarationKind::HoistedValue => BindingKind::Hoisted,
        JsDeclarationKind::Import | JsDeclarationKind::ImportType => BindingKind::Module,
        JsDeclarationKind::Value => BindingKind::Let,
        JsDeclarationKind::Using => BindingKind::Const,
        JsDeclarationKind::Class
        | JsDeclarationKind::Enum
        | JsDeclarationKind::Generic
        | JsDeclarationKind::Interface
        | JsDeclarationKind::Module
        | JsDeclarationKind::Namespace
        | JsDeclarationKind::Type => BindingKind::Local,
        JsDeclarationKind::Unknown => BindingKind::Unknown,
    }
}

fn declaration_type(node: &biome_js_syntax::JsSyntaxNode) -> String {
    format!("{:?}", node.kind())
}

fn scope_kind(node: &biome_js_syntax::JsSyntaxNode) -> ScopeKind {
    use biome_js_syntax::JsSyntaxKind::*;

    match node.kind() {
        JS_MODULE | JS_SCRIPT => ScopeKind::Program,
        JS_FUNCTION_DECLARATION
        | JS_FUNCTION_EXPRESSION
        | JS_ARROW_FUNCTION_EXPRESSION
        | JS_METHOD_CLASS_MEMBER
        | JS_GETTER_CLASS_MEMBER
        | JS_SETTER_CLASS_MEMBER
        | JS_METHOD_OBJECT_MEMBER
        | JS_GETTER_OBJECT_MEMBER
        | JS_SETTER_OBJECT_MEMBER => ScopeKind::Function,
        JS_CLASS_DECLARATION | JS_CLASS_EXPRESSION => ScopeKind::Class,
        JS_FOR_STATEMENT | JS_FOR_IN_STATEMENT | JS_FOR_OF_STATEMENT => ScopeKind::For,
        JS_SWITCH_STATEMENT => ScopeKind::Switch,
        JS_CATCH_CLAUSE => ScopeKind::Catch,
        _ => ScopeKind::Block,
    }
}
