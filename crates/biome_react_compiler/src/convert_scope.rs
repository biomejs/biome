use biome_js_semantic::{JsDeclarationKind, SemanticModel};
use indexmap::IndexMap;
use react_compiler_ast::scope::{
    BindingData, BindingId, BindingKind, ScopeData, ScopeId, ScopeInfo, ScopeKind,
};
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
            kind: binding_kind(binding.declaration_kind()),
            scope: ScopeId(0),
            declaration_type: declaration_type(&declaration),
            declaration_start: Some(declaration_start),
            import: None,
        });
    }

    let mut node_to_scope = HashMap::new();
    for scope in model.scopes() {
        let id = ScopeId(scope.id().index() as u32);
        let parent = scope
            .parent()
            .map(|parent| ScopeId(parent.id().index() as u32));
        let mut scope_bindings = HashMap::new();

        for binding in scope.bindings() {
            let declaration = binding.syntax();
            let declaration_start: u32 = declaration.text_trimmed_range().start().into();
            if let Some(&binding_id) = semantic_to_react_binding.get(&declaration_start) {
                scope_bindings.insert(declaration.text_trimmed().to_string(), binding_id);
                bindings[binding_id.0 as usize].scope = id;
            }
        }

        let syntax = scope.syntax();
        node_to_scope.insert(syntax.text_trimmed_range().start().into(), id);
        scopes.push(ScopeData {
            id,
            parent,
            kind: scope_kind(&syntax),
            bindings: scope_bindings,
        });
    }

    let mut reference_to_binding = IndexMap::new();
    for binding in model.all_bindings() {
        let declaration = binding.syntax();
        let declaration_start: u32 = declaration.text_trimmed_range().start().into();
        let Some(&binding_id) = semantic_to_react_binding.get(&declaration_start) else {
            continue;
        };

        if let Some(start) = bindings[binding_id.0 as usize].declaration_start {
            reference_to_binding.entry(start).or_insert(binding_id);
        }

        for reference in binding.all_references() {
            reference_to_binding.insert(reference.range_start().into(), binding_id);
        }
    }

    ScopeInfo {
        scopes,
        bindings,
        node_to_scope,
        reference_to_binding,
        program_scope: ScopeId(model.global_scope().id().index() as u32),
    }
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
        | JS_SETTER_CLASS_MEMBER => ScopeKind::Function,
        JS_CLASS_DECLARATION | JS_CLASS_EXPRESSION => ScopeKind::Class,
        JS_FOR_STATEMENT | JS_FOR_IN_STATEMENT | JS_FOR_OF_STATEMENT => ScopeKind::For,
        JS_SWITCH_STATEMENT => ScopeKind::Switch,
        JS_CATCH_CLAUSE => ScopeKind::Catch,
        _ => ScopeKind::Block,
    }
}
