use crate::comments::collect_comments;
use crate::error::{ReactCompilerError, Result};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsBinding, AnyJsBindingPattern, AnyJsCallArgument,
    AnyJsCombinedSpecifier, AnyJsDeclarationClause, AnyJsExportClause,
    AnyJsExportDefaultDeclaration, AnyJsExpression, AnyJsFunctionBody, AnyJsImportClause,
    AnyJsLiteralExportName, AnyJsLiteralExpression, AnyJsModuleItem, AnyJsModuleSource, AnyJsName,
    AnyJsObjectBindingPatternMember, AnyJsObjectMember, AnyJsObjectMemberName, AnyJsRoot,
    AnyJsStatement, AnyJsTemplateElement, AnyJsxAttribute, AnyJsxAttributeName,
    AnyJsxAttributeValue, AnyJsxChild, AnyJsxElementName, AnyJsxObjectName, AnyJsxTag,
    JsClassDeclaration, JsClassExportDefaultDeclaration, JsClassExpression, JsDirective, JsExport,
    JsFileSource, JsFunctionBody, JsFunctionDeclaration, JsFunctionExpression, JsIdentifierBinding,
    JsIdentifierExpression, JsImport, JsNamedImportSpecifiers, JsObjectBindingPattern,
    JsPostUpdateExpression, JsPreUpdateExpression, JsReturnStatement, JsScript, JsSpread,
    JsSyntaxNode, JsTemplateExpression, JsVariableStatement, JsxAttribute, JsxElement,
    JsxExpressionAttributeValue, JsxExpressionChild, JsxFragment, JsxMemberName, JsxName,
    JsxSelfClosingElement, JsxSpreadAttribute, JsxString, JsxText, inner_string_text,
};
use biome_rowan::AstNode;
use react_compiler_ast::declarations::{
    Declaration, ExportAllDeclaration, ExportDefaultDecl, ExportDefaultDeclaration, ExportKind,
    ExportNamedDeclaration, ExportNamespaceSpecifierData, ExportSpecifier, ExportSpecifierData,
    ImportDeclaration, ImportDefaultSpecifierData, ImportKind, ImportNamespaceSpecifierData,
    ImportSpecifier, ImportSpecifierData, ModuleExportName,
};
use react_compiler_ast::expressions::{
    ArrayExpression, ArrowFunctionBody, ArrowFunctionExpression, AssignmentExpression,
    AwaitExpression, BinaryExpression, CallExpression, ClassBody, ClassExpression,
    ConditionalExpression, Expression, FunctionExpression, Identifier, Import, LogicalExpression,
    MemberExpression, MetaProperty, NewExpression, ObjectExpression, ObjectExpressionProperty,
    ObjectMethod, ObjectMethodKind, ObjectProperty, OptionalCallExpression,
    OptionalMemberExpression, ParenthesizedExpression, SequenceExpression, SpreadElement, Super,
    TSAsExpression, TSInstantiationExpression, TSNonNullExpression, TSSatisfiesExpression,
    TSTypeAssertion, TaggedTemplateExpression, TemplateLiteral, UnaryExpression, UpdateExpression,
    YieldExpression,
};
use react_compiler_ast::jsx::{
    JSXAttribute, JSXAttributeItem, JSXAttributeName, JSXAttributeValue, JSXChild,
    JSXClosingElement, JSXClosingFragment, JSXElement, JSXElementName, JSXEmptyExpression,
    JSXExpressionContainer, JSXExpressionContainerExpr, JSXFragment, JSXIdentifier,
    JSXMemberExprObject, JSXMemberExpression, JSXNamespacedName, JSXOpeningElement,
    JSXOpeningFragment, JSXSpreadAttribute, JSXSpreadChild, JSXText,
};
use react_compiler_ast::literals::{
    BigIntLiteral, BooleanLiteral, NullLiteral, NumericLiteral, NumericLiteralExtra, RegExpLiteral,
    StringLiteral, TemplateElement, TemplateElementValue,
};
use react_compiler_ast::operators::{
    AssignmentOperator, BinaryOperator, LogicalOperator, UnaryOperator, UpdateOperator,
};
use react_compiler_ast::patterns::{
    ArrayPattern, AssignmentPattern, ObjectPattern, ObjectPatternProp, ObjectPatternProperty,
    PatternLike, RestElement,
};
use react_compiler_ast::statements::{
    BlockStatement, BreakStatement, CatchClause, ClassDeclaration, ContinueStatement,
    DebuggerStatement, Directive, DirectiveLiteral, DoWhileStatement, EmptyStatement,
    ExpressionStatement, ForInOfLeft, ForInStatement, ForInit, ForOfStatement, ForStatement,
    FunctionDeclaration, IfStatement, LabeledStatement, ReturnStatement, Statement, SwitchCase,
    SwitchStatement, ThrowStatement, TryStatement, VariableDeclarationKind, VariableDeclarator,
    WhileStatement, WithStatement,
};
use react_compiler_ast::{File, Program, SourceType};

mod ctx;
mod directives;
mod errors;
mod expressions;
mod identifiers;
mod jsx;
mod literals;
mod modules;
mod operators;
mod statements;

pub(crate) use ctx::ConvertCtx;

use directives::*;
use errors::*;
use expressions::*;
use identifiers::*;
use jsx::*;
use literals::*;
use modules::*;
use operators::*;
use statements::*;

pub(crate) fn convert_file(
    root: &AnyJsRoot,
    source: &str,
    source_type: JsFileSource,
) -> Result<File> {
    let ctx = ConvertCtx::new(root, source);
    let base = ctx.base(root.syntax().text_range_with_trivia());
    let (body, directives) = match root {
        AnyJsRoot::JsModule(module) => {
            let body = module
                .items()
                .into_iter()
                .map(|item| convert_module_item(&ctx, item))
                .collect::<Result<Vec<_>>>()?;
            let directives = module
                .directives()
                .into_iter()
                .map(|directive| convert_directive(&ctx, directive))
                .collect::<Result<Vec<_>>>()?;
            (body, directives)
        }
        AnyJsRoot::JsScript(script) => convert_script(&ctx, script)?,
        _ => return Err(unsupported(root.syntax())),
    };

    Ok(File {
        base: base.clone(),
        program: Program {
            base,
            body,
            directives,
            source_type: if source_type.is_module() {
                SourceType::Module
            } else {
                SourceType::Script
            },
            interpreter: None,
            source_file: None,
        },
        comments: collect_comments(&ctx),
        errors: Vec::new(),
    })
}
