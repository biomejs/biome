use crate::Type;
use biome_js_formatter::context::JsFormatOptions;
use biome_js_formatter::format_node;
use biome_rowan::AstNode;

pub fn assert_type_snapshot(source_code: &str, ty: Type, test_name: &str) {
    let mut content = String::new();

    let source_type = JsFileSource::ts();
    let tree = parse(source_code, source_type, JsParserOptions::default());
    let formatted = format_node(JsFormatOptions::default(), tree.tree().syntax())
        .unwrap()
        .print()
        .unwrap();

    content.push_str("```");
    content.push_str("ts");
    content.push('\n');
    content.push_str(formatted.as_code());
    content.push_str("\n```");

    content.push_str("\n\n");
    content.push_str("```\n");
    content.push_str(&ty.to_string());
    content.push_str("\n```\n\n");

    insta::with_settings!({
        snapshot_path => "../snapshots",
        prepend_module_to_snapshot => false,
    }, {
        insta::assert_snapshot!(test_name, content);
    });
}

use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsExpressionStatement, JsFileSource,
    JsFunctionDeclaration,
};
use biome_js_type_info::{TypeReferenceQualifier, TypeResolver};
use biome_rowan::Text;

/// Test resolver that looks up a single hardcoded symbol.
pub struct HardcodedSymbolResolver(pub &'static str, pub Type);

impl TypeResolver for HardcodedSymbolResolver {
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<Type> {
        if qualifier.parts().len() == 1 && qualifier.parts()[0] == self.0 {
            Some(self.1.clone())
        } else {
            PromiseResolver.resolve_qualifier(qualifier)
        }
    }

    fn resolve_type_of(&self, _identifier: &Text) -> Option<Type> {
        None
    }
}

/// Test resolver that does nothing but resolve type references to `Promise`
/// without any proper scope lookups.
pub struct PromiseResolver;

impl TypeResolver for PromiseResolver {
    fn resolve_qualifier(&self, qualifier: &TypeReferenceQualifier) -> Option<Type> {
        qualifier
            .is_promise()
            .then(|| Type::promise_of(Type::unknown()))
    }

    fn resolve_type_of(&self, _identifier: &Text) -> Option<Type> {
        None
    }
}

pub fn get_expression_statement(root: &AnyJsRoot) -> JsExpressionStatement {
    let module = root.as_js_module().unwrap();
    module
        .items()
        .into_iter()
        .filter_map(|item| match item {
            AnyJsModuleItem::AnyJsStatement(statement) => Some(statement),
            _ => None,
        })
        .find_map(|statement| match statement {
            AnyJsStatement::JsExpressionStatement(expr) => Some(expr),
            _ => None,
        })
        .expect("cannot find expression statement")
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
        .expect("cannot find declaration")
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
