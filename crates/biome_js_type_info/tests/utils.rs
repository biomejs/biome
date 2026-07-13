#![allow(unused)]

use std::borrow::Cow;

use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsExpression, JsVariableDeclaration, TsInterfaceDeclaration, TsTypeAliasDeclaration,
};
use biome_js_syntax::{AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsFunctionDeclaration};
use biome_js_type_info::{
    RawTypeCollector, RawTypeId, ScopeId, TypeData, TypeId, TypeReference, TypeStore,
};
use biome_languages::JsFileSource;
use biome_rowan::{AstNode, Text};
use biome_test_utils::dump_registered_module_types;

pub fn assert_type_data_snapshot(
    source_code: &str,
    ty: &TypeData,
    resolver: &TestTypeCollector,
    test_name: &str,
) {
    let mut content = String::new();

    let source_type = JsFileSource::ts();
    let tree = parse(source_code, source_type, JsParserOptions::default());
    let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax(), Vec::new())
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

    dump_registered_module_types(&mut content, resolver.types.as_references());

    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

pub fn assert_typed_bindings_snapshot(
    source_code: &str,
    typed_bindings: &[(Text, TypeReference)],
    resolver: &TestTypeCollector,
    test_name: &str,
) {
    let mut content = String::new();

    let source_type = JsFileSource::ts();
    let tree = parse(source_code, source_type, JsParserOptions::default());
    let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax(), Vec::new())
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
        let ty = match ty {
            TypeReference::Resolved(RawTypeId::Local(id)) => resolver.get_by_id(*id).clone(),
            ty => TypeData::reference(ty.clone()),
        };
        content.push_str(&format!("{name} => {ty}\n"));
    }
    content.push_str("\n```\n\n");

    dump_registered_module_types(&mut content, resolver.types.as_references());

    insta::with_settings!({
        snapshot_path => "snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

#[derive(Default)]
pub struct TestTypeCollector {
    pub types: TypeStore,
}

impl RawTypeCollector for TestTypeCollector {
    fn find_type(&self, type_data: &TypeData) -> Option<TypeId> {
        self.types.find(type_data)
    }

    fn get_by_id(&self, id: TypeId) -> &TypeData {
        self.types.get_by_id(id)
    }

    fn register_type(&mut self, type_data: Cow<TypeData>) -> TypeId {
        self.types.insert_cow(type_data)
    }

    fn resolve_expression(
        &mut self,
        scope_id: ScopeId,
        expression: &AnyJsExpression,
    ) -> Cow<'_, TypeData> {
        Cow::Owned(TypeData::from_any_js_expression(self, scope_id, expression))
    }
}

pub fn get_expression(root: &AnyJsRoot) -> AnyJsExpression {
    let module = root.as_js_module().unwrap();
    module
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
            _ => None,
        })
        .find_map(|statement| match statement {
            AnyJsStatement::JsExpressionStatement(expr) => expr.expression().ok(),
            _ => None,
        })
        .expect("cannot find expression")
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

pub fn get_interface_declaration(root: &AnyJsRoot) -> TsInterfaceDeclaration {
    let module = root.as_js_module().unwrap();
    module
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
            _ => None,
        })
        .find_map(|statement| match statement {
            AnyJsStatement::TsInterfaceDeclaration(decl) => Some(decl),
            _ => None,
        })
        .expect("cannot find interface declaration")
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
