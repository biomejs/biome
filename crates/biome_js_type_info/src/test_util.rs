use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsModuleItem, AnyJsRoot, AnyJsStatement, JsExpressionStatement, JsFileSource,
    JsFunctionDeclaration,
};
use biome_rowan::Text;

use crate::{Type, TypeReferenceQualifier, TypeResolver};

/// Test resolver that looks up a single hardcoded symbol.
pub(crate) struct HardcodedSymbolResolver(pub &'static str, pub Type);

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
pub(crate) struct PromiseResolver;

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

pub(crate) fn get_expression_statement(root: &AnyJsRoot) -> JsExpressionStatement {
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

pub(crate) fn get_function_declaration(root: &AnyJsRoot) -> JsFunctionDeclaration {
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

pub(crate) fn parse_ts(code: &str) -> AnyJsRoot {
    let parsed = parse(code, JsFileSource::ts(), JsParserOptions::default());
    let diagnostics = parsed.diagnostics();
    assert!(
        diagnostics.is_empty(),
        "Unexpected diagnostics: {diagnostics:?}"
    );

    parsed.tree()
}
