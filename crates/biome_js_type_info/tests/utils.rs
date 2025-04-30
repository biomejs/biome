#![allow(unused)]

use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::JsVariableDeclaration;
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsFileSource, JsFunctionDeclaration,
};
use biome_js_type_info::{
    GlobalsResolver, NUM_PREDEFINED_TYPES, Resolvable, ResolvedTypeId, TypeData, TypeId,
    TypeReference, TypeReferenceQualifier, TypeResolver, TypeResolverLevel,
};
use biome_rowan::{AstNode, Text};
use biome_test_utils::dump_registered_types;

pub fn assert_type_data_snapshot(
    source_code: &str,
    ty: TypeData,
    resolver: &dyn TypeResolver,
    test_name: &str,
) {
    let mut content = String::new();

    let source_type = JsFileSource::ts();
    let tree = parse(source_code, source_type, JsParserOptions::default());
    let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax())
        .unwrap()
        .print()
        .unwrap();

    content.push_str("## Input\n\n");
    content.push_str("```ts\n");
    content.push_str(formatted.as_code());
    content.push_str("\n```\n\n");

    content.push_str("## Result\n\n");
    content.push_str("```\n");
    content.push_str(&ty.to_string());
    content.push_str("\n```\n\n");

    dump_registered_types(&mut content, resolver);

    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

pub fn assert_typed_bindings_snapshot(
    source_code: &str,
    typed_bindings: &[(Text, TypeData)],
    resolver: &dyn TypeResolver,
    test_name: &str,
) {
    let mut content = String::new();

    let source_type = JsFileSource::ts();
    let tree = parse(source_code, source_type, JsParserOptions::default());
    let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax())
        .unwrap()
        .print()
        .unwrap();

    content.push_str("## Input\n\n");
    content.push_str("```ts\n");
    content.push_str(formatted.as_code());
    content.push_str("\n```\n\n");

    content.push_str("## Result\n\n");
    content.push_str("```\n");
    for (name, ty) in typed_bindings {
        content.push_str(&format!("{name} => {ty}\n"));
    }
    content.push_str("\n```\n\n");

    dump_registered_types(&mut content, resolver);

    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

/// Test resolver that can resolve a single type with a hardcoded name, but
/// that is still able to register other types and has a fallback for globals.
pub struct HardcodedSymbolResolver {
    name: &'static str,
    globals: GlobalsResolver,
    types: Vec<TypeData>,
}

impl HardcodedSymbolResolver {
    pub fn new(name: &'static str, data: TypeData, globals: GlobalsResolver) -> Self {
        Self {
            name,
            globals,
            types: vec![data],
        }
    }

    pub fn run_inference(&mut self) {
        self.resolve_all();
        self.flatten_all();
    }

    pub fn resolve_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.resolved(self);
            i += 1;
        }
    }

    fn flatten_all(&mut self) {
        let mut i = 0;
        while i < self.types.len() {
            // First take the type to satisfy the borrow checker:
            let ty = std::mem::take(&mut self.types[i]);
            self.types[i] = ty.flattened(self);
            i += 1;
        }
    }
}

impl TypeResolver for HardcodedSymbolResolver {
    fn level(&self) -> TypeResolverLevel {
        TypeResolverLevel::Module
    }

    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types
            .iter()
            .position(|data| data == type_data)
            .map(TypeId::new)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        &self.types[id.index()]
    }

    fn get_by_resolved_id(&self, id: ResolvedTypeId) -> Option<&TypeData> {
        match id.level() {
            TypeResolverLevel::AdHoc => {
                panic!("Ad-hoc references unsupported by resolver")
            }
            TypeResolverLevel::Module => Some(self.get_by_id(id.id())),
            TypeResolverLevel::Import => {
                panic!("Import references unsupported by resolver")
            }
            TypeResolverLevel::Global => Some(self.globals.get_by_id(id.id())),
        }
    }

    fn register_type(&mut self, type_data: TypeData) -> TypeId {
        match self.types.iter().position(|data| data == &type_data) {
            Some(index) => TypeId::new(index),
            None => {
                let id = TypeId::new(self.types.len());
                self.types.push(type_data);
                id
            }
        }
    }

    fn resolve_reference(&self, ty: &TypeReference) -> Option<ResolvedTypeId> {
        match ty {
            TypeReference::Qualifier(qualifier) => self.resolve_qualifier(qualifier),
            TypeReference::Resolved(resolved_id) => Some(*resolved_id),
            TypeReference::Import(_import) => {
                panic!("Project-level references unsupported by resolver")
            }
            TypeReference::Unknown => None,
        }
    }

    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<ResolvedTypeId> {
        if qualifier.path.len() == 1 && qualifier.path[0] == self.name {
            Some(ResolvedTypeId::new(self.level(), TypeId::new(0)))
        } else {
            self.globals.resolve_qualifier(qualifier)
        }
    }

    fn resolve_type_of(&self, identifier: &Text) -> Option<ResolvedTypeId> {
        self.globals.resolve_type_of(identifier)
    }

    fn fallback_resolver(&self) -> Option<&dyn TypeResolver> {
        Some(&self.globals)
    }

    fn registered_types(&self) -> &[TypeData] {
        &self.types
    }
}

pub fn get_function_declaration(root: &AnyJsRoot) -> JsFunctionDeclaration {
    let module = root.as_js_module().unwrap();
    module
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
            _ => None,
        })
        .find_map(|statement| match statement {
            AnyJsStatement::JsFunctionDeclaration(decl) => Some(decl),
            _ => None,
        })
        .expect("cannot find function declaration")
}

pub fn get_variable_declaration(root: &AnyJsRoot) -> JsVariableDeclaration {
    let module = root.as_js_module().unwrap();
    module
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
            _ => None,
        })
        .find_map(|statement| match statement {
            AnyJsStatement::JsVariableStatement(statement) => statement.declaration().ok(),
            _ => None,
        })
        .expect("cannot find variable declaration")
}

pub fn parse_ts(code: &str) -> AnyJsRoot {
    let parsed = parse(code, JsFileSource::ts(), JsParserOptions::default());
    let diagnostics = parsed.diagnostics();
    assert!(
        diagnostics.is_empty(),
        "Unexpected diagnostics: {diagnostics:?}"
    );

    parsed.tree()
}
