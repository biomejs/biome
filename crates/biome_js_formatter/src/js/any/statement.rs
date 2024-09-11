//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_js_syntax::AnyJsStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsStatement;
impl FormatRule<AnyJsStatement> for FormatAnyJsStatement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsStatement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsStatement::JsBlockStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsBogusStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsBreakStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsClassDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::JsContinueStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsDebuggerStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsDoWhileStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsEmptyStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsExpressionStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsForInStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsForOfStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsForStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::JsIfStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsLabeledStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsMetavariable(node) => node.format().fmt(f),
            AnyJsStatement::JsReturnStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsSwitchStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsThrowStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsTryFinallyStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsTryStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsVariableStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsWhileStatement(node) => node.format().fmt(f),
            AnyJsStatement::JsWithStatement(node) => node.format().fmt(f),
            AnyJsStatement::TsDeclareFunctionDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsDeclareStatement(node) => node.format().fmt(f),
            AnyJsStatement::TsEnumDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsExternalModuleDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsGlobalDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsImportEqualsDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsInterfaceDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsModuleDeclaration(node) => node.format().fmt(f),
            AnyJsStatement::TsTypeAliasDeclaration(node) => node.format().fmt(f),
        }
    }
}
