// Generated file, do not edit by hand, see `xtask/codegen`

use super::*;
use biome_js_syntax::*;
impl JsAstNode {
    pub(super) fn create_generated_prototype(
        kind: JsSyntaxKind,
        base_prototype: JsObject,
        context: &mut Context,
    ) -> JsObject {
        let mut prototype =
            ObjectInitializer::with_native_data_and_proto(OrdinaryObject, base_prototype, context);
        match kind {
            JsSyntaxKind::JS_ACCESSOR_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ACCESSOR_MODIFIER,
                    JsAccessorModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN,
                    JsArrayAssignmentPattern,
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("elements", |node, context| Self::wrap_node_list(
                        node.elements().into_iter().flatten(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT,
                    JsArrayAssignmentPatternElement,
                    ("pattern", |node, context| Self::wrap_optional_node(
                        node.pattern().ok(),
                        context
                    )),
                    ("init", |node, context| Self::wrap_optional_node(
                        node.init(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT,
                    JsArrayAssignmentPatternRestElement,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("pattern", |node, context| Self::wrap_optional_node(
                        node.pattern().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_BINDING_PATTERN,
                    JsArrayBindingPattern,
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("elements", |node, context| Self::wrap_node_list(
                        node.elements().into_iter().flatten(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT,
                    JsArrayBindingPatternElement,
                    ("pattern", |node, context| Self::wrap_optional_node(
                        node.pattern().ok(),
                        context
                    )),
                    ("init", |node, context| Self::wrap_optional_node(
                        node.init(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT,
                    JsArrayBindingPatternRestElement,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("pattern", |node, context| Self::wrap_optional_node(
                        node.pattern().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARRAY_EXPRESSION,
                    JsArrayExpression,
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("elements", |node, context| Self::wrap_node_list(
                        node.elements().into_iter().flatten(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_ARRAY_HOLE => {
                register_js_ast_fields!(prototype, JsSyntaxKind::JS_ARRAY_HOLE, JsArrayHole,);
            }
            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
                    JsArrowFunctionExpression,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("fatArrowToken", |node, context| Self::wrap_token(
                        node.fat_arrow_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION,
                    JsAssignmentExpression,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_AWAIT_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_AWAIT_EXPRESSION,
                    JsAwaitExpression,
                    ("awaitToken", |node, context| Self::wrap_token(
                        node.await_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION,
                    JsBigintLiteralExpression,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_BINARY_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_BINARY_EXPRESSION,
                    JsBinaryExpression,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_BLOCK_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_BLOCK_STATEMENT,
                    JsBlockStatement,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("statements", |node, context| Self::wrap_node_list(
                        node.statements(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION,
                    JsBooleanLiteralExpression,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_BREAK_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_BREAK_STATEMENT,
                    JsBreakStatement,
                    ("breakToken", |node, context| Self::wrap_token(
                        node.break_token().ok()
                    )),
                    ("label", |node, context| Self::wrap_optional_node(
                        node.label(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_CALL_ARGUMENTS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CALL_ARGUMENTS,
                    JsCallArguments,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("args", |node, context| Self::wrap_node_list(
                        node.args().into_iter().flatten(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_CALL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CALL_EXPRESSION,
                    JsCallExpression,
                    ("callee", |node, context| Self::wrap_optional_node(
                        node.callee().ok(),
                        context
                    )),
                    ("optionalChainToken", |node, context| Self::wrap_token(
                        node.optional_chain_token()
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                    ("arguments", |node, context| Self::wrap_optional_node(
                        node.arguments().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_CASE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CASE_CLAUSE,
                    JsCaseClause,
                    ("caseToken", |node, context| Self::wrap_token(
                        node.case_token().ok()
                    )),
                    ("test", |node, context| Self::wrap_optional_node(
                        node.test().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("consequent", |node, context| Self::wrap_node_list(
                        node.consequent(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_CATCH_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CATCH_CLAUSE,
                    JsCatchClause,
                    ("catchToken", |node, context| Self::wrap_token(
                        node.catch_token().ok()
                    )),
                    ("declaration", |node, context| Self::wrap_optional_node(
                        node.declaration(),
                        context
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_CATCH_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CATCH_DECLARATION,
                    JsCatchDeclaration,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("binding", |node, context| Self::wrap_optional_node(
                        node.binding().ok(),
                        context
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_CLASS_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CLASS_DECLARATION,
                    JsClassDeclaration,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("abstractToken", |node, context| Self::wrap_token(
                        node.abstract_token()
                    )),
                    ("classToken", |node, context| Self::wrap_token(
                        node.class_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("extendsClause", |node, context| Self::wrap_optional_node(
                        node.extends_clause(),
                        context
                    )),
                    (
                        "implementsClause",
                        |node, context| Self::wrap_optional_node(node.implements_clause(), context)
                    ),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION,
                    JsClassExportDefaultDeclaration,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("abstractToken", |node, context| Self::wrap_token(
                        node.abstract_token()
                    )),
                    ("classToken", |node, context| Self::wrap_token(
                        node.class_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("extendsClause", |node, context| Self::wrap_optional_node(
                        node.extends_clause(),
                        context
                    )),
                    (
                        "implementsClause",
                        |node, context| Self::wrap_optional_node(node.implements_clause(), context)
                    ),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_CLASS_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CLASS_EXPRESSION,
                    JsClassExpression,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("classToken", |node, context| Self::wrap_token(
                        node.class_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("extendsClause", |node, context| Self::wrap_optional_node(
                        node.extends_clause(),
                        context
                    )),
                    (
                        "implementsClause",
                        |node, context| Self::wrap_optional_node(node.implements_clause(), context)
                    ),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT,
                    JsComputedMemberAssignment,
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION,
                    JsComputedMemberExpression,
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                    ("optionalChainToken", |node, context| Self::wrap_token(
                        node.optional_chain_token()
                    )),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_COMPUTED_MEMBER_NAME,
                    JsComputedMemberName,
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CONDITIONAL_EXPRESSION,
                    JsConditionalExpression,
                    ("test", |node, context| Self::wrap_optional_node(
                        node.test().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token().ok()
                    )),
                    ("consequent", |node, context| Self::wrap_optional_node(
                        node.consequent().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("alternate", |node, context| Self::wrap_optional_node(
                        node.alternate().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER,
                    JsConstructorClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS,
                    JsConstructorParameters,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("parameters", |node, context| Self::wrap_node_list(
                        node.parameters().into_iter().flatten(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_CONTINUE_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_CONTINUE_STATEMENT,
                    JsContinueStatement,
                    ("continueToken", |node, context| Self::wrap_token(
                        node.continue_token().ok()
                    )),
                    ("label", |node, context| Self::wrap_optional_node(
                        node.label(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_DEBUGGER_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_DEBUGGER_STATEMENT,
                    JsDebuggerStatement,
                    ("debuggerToken", |node, context| Self::wrap_token(
                        node.debugger_token().ok()
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_DECORATOR => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_DECORATOR,
                    JsDecorator,
                    ("atToken", |node, context| Self::wrap_token(
                        node.at_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_DEFAULT_CLAUSE,
                    JsDefaultClause,
                    ("defaultToken", |node, context| Self::wrap_token(
                        node.default_token().ok()
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("consequent", |node, context| Self::wrap_node_list(
                        node.consequent(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER,
                    JsDefaultImportSpecifier,
                    ("localName", |node, context| Self::wrap_optional_node(
                        node.local_name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_DIRECTIVE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_DIRECTIVE,
                    JsDirective,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_DO_WHILE_STATEMENT,
                    JsDoWhileStatement,
                    ("doToken", |node, context| Self::wrap_token(
                        node.do_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                    ("whileToken", |node, context| Self::wrap_token(
                        node.while_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("test", |node, context| Self::wrap_optional_node(
                        node.test().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_ELSE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_ELSE_CLAUSE,
                    JsElseClause,
                    ("elseToken", |node, context| Self::wrap_token(
                        node.else_token().ok()
                    )),
                    ("alternate", |node, context| Self::wrap_optional_node(
                        node.alternate().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_EMPTY_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EMPTY_CLASS_MEMBER,
                    JsEmptyClassMember,
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_EMPTY_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EMPTY_STATEMENT,
                    JsEmptyStatement,
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT,
                    JsExport,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("exportToken", |node, context| Self::wrap_token(
                        node.export_token().ok()
                    )),
                    ("exportClause", |node, context| Self::wrap_optional_node(
                        node.export_clause().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_AS_CLAUSE,
                    JsExportAsClause,
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("exportedName", |node, context| Self::wrap_optional_node(
                        node.exported_name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE,
                    JsExportDefaultDeclarationClause,
                    ("defaultToken", |node, context| Self::wrap_token(
                        node.default_token().ok()
                    )),
                    ("declaration", |node, context| Self::wrap_optional_node(
                        node.declaration().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE,
                    JsExportDefaultExpressionClause,
                    ("defaultToken", |node, context| Self::wrap_token(
                        node.default_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_FROM_CLAUSE,
                    JsExportFromClause,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token().ok()
                    )),
                    ("exportAs", |node, context| Self::wrap_optional_node(
                        node.export_as(),
                        context
                    )),
                    ("fromToken", |node, context| Self::wrap_token(
                        node.from_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE,
                    JsExportNamedClause,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("specifiers", |node, context| Self::wrap_node_list(
                        node.specifiers().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE,
                    JsExportNamedFromClause,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("specifiers", |node, context| Self::wrap_node_list(
                        node.specifiers().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                    ("fromToken", |node, context| Self::wrap_token(
                        node.from_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER,
                    JsExportNamedFromSpecifier,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("sourceName", |node, context| Self::wrap_optional_node(
                        node.source_name().ok(),
                        context
                    )),
                    ("exportAs", |node, context| Self::wrap_optional_node(
                        node.export_as(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER,
                    JsExportNamedShorthandSpecifier,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER,
                    JsExportNamedSpecifier,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("localName", |node, context| Self::wrap_optional_node(
                        node.local_name().ok(),
                        context
                    )),
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("exportedName", |node, context| Self::wrap_optional_node(
                        node.exported_name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_EXPRESSION_SNIPPET => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPRESSION_SNIPPET,
                    JsExpressionSnippet,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("eofToken", |node, context| Self::wrap_token(
                        node.eof_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPRESSION_STATEMENT,
                    JsExpressionStatement,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT,
                    JsExpressionTemplateRoot,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("eofToken", |node, context| Self::wrap_token(
                        node.eof_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_EXTENDS_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_EXTENDS_CLAUSE,
                    JsExtendsClause,
                    ("extendsToken", |node, context| Self::wrap_token(
                        node.extends_token().ok()
                    )),
                    ("superClass", |node, context| Self::wrap_optional_node(
                        node.super_class().ok(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FINALLY_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FINALLY_CLAUSE,
                    JsFinallyClause,
                    ("finallyToken", |node, context| Self::wrap_token(
                        node.finally_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FOR_IN_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FOR_IN_STATEMENT,
                    JsForInStatement,
                    ("forToken", |node, context| Self::wrap_token(
                        node.for_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer().ok(),
                        context
                    )),
                    ("inToken", |node, context| Self::wrap_token(
                        node.in_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FOR_OF_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FOR_OF_STATEMENT,
                    JsForOfStatement,
                    ("forToken", |node, context| Self::wrap_token(
                        node.for_token().ok()
                    )),
                    ("awaitToken", |node, context| Self::wrap_token(
                        node.await_token()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer().ok(),
                        context
                    )),
                    ("ofToken", |node, context| Self::wrap_token(
                        node.of_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FOR_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FOR_STATEMENT,
                    JsForStatement,
                    ("forToken", |node, context| Self::wrap_token(
                        node.for_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer(),
                        context
                    )),
                    ("firstSemiToken", |node, context| Self::wrap_token(
                        node.first_semi_token().ok()
                    )),
                    ("test", |node, context| Self::wrap_optional_node(
                        node.test(),
                        context
                    )),
                    ("secondSemiToken", |node, context| Self::wrap_token(
                        node.second_semi_token().ok()
                    )),
                    ("update", |node, context| Self::wrap_optional_node(
                        node.update(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION,
                    JsForVariableDeclaration,
                    ("awaitToken", |node, context| Self::wrap_token(
                        node.await_token()
                    )),
                    ("kindToken", |node, context| Self::wrap_token(
                        node.kind_token().ok()
                    )),
                    ("declarator", |node, context| Self::wrap_optional_node(
                        node.declarator().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FORMAL_PARAMETER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FORMAL_PARAMETER,
                    JsFormalParameter,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("binding", |node, context| Self::wrap_optional_node(
                        node.binding().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FUNCTION_BODY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FUNCTION_BODY,
                    JsFunctionBody,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("directives", |node, context| Self::wrap_node_list(
                        node.directives(),
                        context
                    )),
                    ("statements", |node, context| Self::wrap_node_list(
                        node.statements(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FUNCTION_DECLARATION,
                    JsFunctionDeclaration,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("functionToken", |node, context| Self::wrap_token(
                        node.function_token().ok()
                    )),
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION,
                    JsFunctionExportDefaultDeclaration,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("functionToken", |node, context| Self::wrap_token(
                        node.function_token().ok()
                    )),
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_FUNCTION_EXPRESSION,
                    JsFunctionExpression,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("functionToken", |node, context| Self::wrap_token(
                        node.function_token().ok()
                    )),
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_GETTER_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_GETTER_CLASS_MEMBER,
                    JsGetterClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("getToken", |node, context| Self::wrap_token(
                        node.get_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("returnType", |node, context| Self::wrap_optional_node(
                        node.return_type(),
                        context
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_GETTER_OBJECT_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_GETTER_OBJECT_MEMBER,
                    JsGetterObjectMember,
                    ("getToken", |node, context| Self::wrap_token(
                        node.get_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("returnType", |node, context| Self::wrap_optional_node(
                        node.return_type(),
                        context
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT,
                    JsIdentifierAssignment,
                    ("nameToken", |node, context| Self::wrap_token(
                        node.name_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IDENTIFIER_BINDING,
                    JsIdentifierBinding,
                    ("nameToken", |node, context| Self::wrap_token(
                        node.name_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IDENTIFIER_EXPRESSION,
                    JsIdentifierExpression,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IF_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IF_STATEMENT,
                    JsIfStatement,
                    ("ifToken", |node, context| Self::wrap_token(
                        node.if_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("test", |node, context| Self::wrap_optional_node(
                        node.test().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("consequent", |node, context| Self::wrap_optional_node(
                        node.consequent().ok(),
                        context
                    )),
                    ("elseClause", |node, context| Self::wrap_optional_node(
                        node.else_clause(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT,
                    JsImport,
                    ("importToken", |node, context| Self::wrap_token(
                        node.import_token().ok()
                    )),
                    ("importClause", |node, context| Self::wrap_optional_node(
                        node.import_clause().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_ASSERTION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_ASSERTION,
                    JsImportAssertion,
                    ("withToken", |node, context| Self::wrap_token(
                        node.with_token().ok()
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("assertions", |node, context| Self::wrap_node_list(
                        node.assertions().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY,
                    JsImportAssertionEntry,
                    ("key", |node, context| Self::wrap_token(node.key().ok())),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_BARE_CLAUSE,
                    JsImportBareClause,
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION,
                    JsImportCallExpression,
                    ("importToken", |node, context| Self::wrap_token(
                        node.import_token().ok()
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token()
                    )),
                    ("phase", |node, context| Self::wrap_token(node.phase())),
                    ("arguments", |node, context| Self::wrap_optional_node(
                        node.arguments().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE,
                    JsImportCombinedClause,
                    (
                        "defaultSpecifier",
                        |node, context| Self::wrap_optional_node(
                            node.default_specifier().ok(),
                            context
                        )
                    ),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token().ok()
                    )),
                    ("specifier", |node, context| Self::wrap_optional_node(
                        node.specifier().ok(),
                        context
                    )),
                    ("fromToken", |node, context| Self::wrap_token(
                        node.from_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE,
                    JsImportDefaultClause,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("phaseToken", |node, context| Self::wrap_token(
                        node.phase_token()
                    )),
                    (
                        "defaultSpecifier",
                        |node, context| Self::wrap_optional_node(
                            node.default_specifier().ok(),
                            context
                        )
                    ),
                    ("fromToken", |node, context| Self::wrap_token(
                        node.from_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_META_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_META_EXPRESSION,
                    JsImportMetaExpression,
                    ("importToken", |node, context| Self::wrap_token(
                        node.import_token().ok()
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("metaToken", |node, context| Self::wrap_token(
                        node.meta_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE,
                    JsImportNamedClause,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("namedSpecifiers", |node, context| Self::wrap_optional_node(
                        node.named_specifiers().ok(),
                        context
                    )),
                    ("fromToken", |node, context| Self::wrap_token(
                        node.from_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE,
                    JsImportNamespaceClause,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("phaseToken", |node, context| Self::wrap_token(
                        node.phase_token()
                    )),
                    ("namespaceSpecifier", |node, context| {
                        Self::wrap_optional_node(node.namespace_specifier().ok(), context)
                    }),
                    ("fromToken", |node, context| Self::wrap_token(
                        node.from_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("assertion", |node, context| Self::wrap_optional_node(
                        node.assertion(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_IN_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_IN_EXPRESSION,
                    JsInExpression,
                    ("property", |node, context| Self::wrap_optional_node(
                        node.property().ok(),
                        context
                    )),
                    ("inToken", |node, context| Self::wrap_token(
                        node.in_token().ok()
                    )),
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_INITIALIZER_CLAUSE,
                    JsInitializerClause,
                    ("eqToken", |node, context| Self::wrap_token(
                        node.eq_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_INSTANCEOF_EXPRESSION,
                    JsInstanceofExpression,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("instanceofToken", |node, context| Self::wrap_token(
                        node.instanceof_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_LABEL => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_LABEL,
                    JsLabel,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_LABELED_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_LABELED_STATEMENT,
                    JsLabeledStatement,
                    ("label", |node, context| Self::wrap_optional_node(
                        node.label().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_LITERAL_EXPORT_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_LITERAL_EXPORT_NAME,
                    JsLiteralExportName,
                    ("value", |node, context| Self::wrap_token(node.value().ok())),
                );
            }
            JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_LITERAL_MEMBER_NAME,
                    JsLiteralMemberName,
                    ("value", |node, context| Self::wrap_token(node.value().ok())),
                );
            }
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_LOGICAL_EXPRESSION,
                    JsLogicalExpression,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_METAVARIABLE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_METAVARIABLE,
                    JsMetavariable,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_METHOD_CLASS_MEMBER,
                    JsMethodClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token()
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_METHOD_OBJECT_MEMBER,
                    JsMethodObjectMember,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_MODULE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_MODULE,
                    JsModule,
                    ("bomToken", |node, context| Self::wrap_token(
                        node.bom_token()
                    )),
                    ("interpreterToken", |node, context| Self::wrap_token(
                        node.interpreter_token()
                    )),
                    ("directives", |node, context| Self::wrap_node_list(
                        node.directives(),
                        context
                    )),
                    ("items", |node, context| Self::wrap_node_list(
                        node.items(),
                        context
                    )),
                    ("eofToken", |node, context| Self::wrap_token(
                        node.eof_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_MODULE_SOURCE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_MODULE_SOURCE,
                    JsModuleSource,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NAME,
                    JsName,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER,
                    JsNamedImportSpecifier,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("localName", |node, context| Self::wrap_optional_node(
                        node.local_name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS,
                    JsNamedImportSpecifiers,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("specifiers", |node, context| Self::wrap_node_list(
                        node.specifiers().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER,
                    JsNamespaceImportSpecifier,
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token().ok()
                    )),
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("localName", |node, context| Self::wrap_optional_node(
                        node.local_name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_NEW_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NEW_EXPRESSION,
                    JsNewExpression,
                    ("newToken", |node, context| Self::wrap_token(
                        node.new_token().ok()
                    )),
                    ("callee", |node, context| Self::wrap_optional_node(
                        node.callee().ok(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                    ("arguments", |node, context| Self::wrap_optional_node(
                        node.arguments(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_NEW_TARGET_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NEW_TARGET_EXPRESSION,
                    JsNewTargetExpression,
                    ("newToken", |node, context| Self::wrap_token(
                        node.new_token().ok()
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("targetToken", |node, context| Self::wrap_token(
                        node.target_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
                    JsNullLiteralExpression,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
                    JsNumberLiteralExpression,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN,
                    JsObjectAssignmentPattern,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("properties", |node, context| Self::wrap_node_list(
                        node.properties().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY,
                    JsObjectAssignmentPatternProperty,
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("pattern", |node, context| Self::wrap_optional_node(
                        node.pattern().ok(),
                        context
                    )),
                    ("init", |node, context| Self::wrap_optional_node(
                        node.init(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST,
                    JsObjectAssignmentPatternRest,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("target", |node, context| Self::wrap_optional_node(
                        node.target().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY,
                    JsObjectAssignmentPatternShorthandProperty,
                    ("identifier", |node, context| Self::wrap_optional_node(
                        node.identifier().ok(),
                        context
                    )),
                    ("init", |node, context| Self::wrap_optional_node(
                        node.init(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN,
                    JsObjectBindingPattern,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("properties", |node, context| Self::wrap_node_list(
                        node.properties().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY,
                    JsObjectBindingPatternProperty,
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("pattern", |node, context| Self::wrap_optional_node(
                        node.pattern().ok(),
                        context
                    )),
                    ("init", |node, context| Self::wrap_optional_node(
                        node.init(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST,
                    JsObjectBindingPatternRest,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("binding", |node, context| Self::wrap_optional_node(
                        node.binding().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY,
                    JsObjectBindingPatternShorthandProperty,
                    ("identifier", |node, context| Self::wrap_optional_node(
                        node.identifier().ok(),
                        context
                    )),
                    ("init", |node, context| Self::wrap_optional_node(
                        node.init(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_OBJECT_EXPRESSION,
                    JsObjectExpression,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_PARAMETERS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PARAMETERS,
                    JsParameters,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("items", |node, context| Self::wrap_node_list(
                        node.items().into_iter().flatten(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT,
                    JsParenthesizedAssignment,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("assignment", |node, context| Self::wrap_optional_node(
                        node.assignment().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION,
                    JsParenthesizedExpression,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_POST_UPDATE_EXPRESSION,
                    JsPostUpdateExpression,
                    ("operand", |node, context| Self::wrap_optional_node(
                        node.operand().ok(),
                        context
                    )),
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION,
                    JsPreUpdateExpression,
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("operand", |node, context| Self::wrap_optional_node(
                        node.operand().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME,
                    JsPrivateClassMemberName,
                    ("hashToken", |node, context| Self::wrap_token(
                        node.hash_token().ok()
                    )),
                    ("idToken", |node, context| Self::wrap_token(
                        node.id_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_PRIVATE_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PRIVATE_NAME,
                    JsPrivateName,
                    ("hashToken", |node, context| Self::wrap_token(
                        node.hash_token().ok()
                    )),
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER,
                    JsPropertyClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("propertyAnnotation", |node, context| {
                        Self::wrap_optional_node(node.property_annotation(), context)
                    }),
                    ("value", |node, context| Self::wrap_optional_node(
                        node.value(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER,
                    JsPropertyObjectMember,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("value", |node, context| Self::wrap_optional_node(
                        node.value().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_REFERENCE_IDENTIFIER,
                    JsReferenceIdentifier,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION,
                    JsRegexLiteralExpression,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_REST_PARAMETER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_REST_PARAMETER,
                    JsRestParameter,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("binding", |node, context| Self::wrap_optional_node(
                        node.binding().ok(),
                        context
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_RETURN_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_RETURN_STATEMENT,
                    JsReturnStatement,
                    ("returnToken", |node, context| Self::wrap_token(
                        node.return_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_SCRIPT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SCRIPT,
                    JsScript,
                    ("bomToken", |node, context| Self::wrap_token(
                        node.bom_token()
                    )),
                    ("interpreterToken", |node, context| Self::wrap_token(
                        node.interpreter_token()
                    )),
                    ("directives", |node, context| Self::wrap_node_list(
                        node.directives(),
                        context
                    )),
                    ("statements", |node, context| Self::wrap_node_list(
                        node.statements(),
                        context
                    )),
                    ("eofToken", |node, context| Self::wrap_token(
                        node.eof_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SEQUENCE_EXPRESSION,
                    JsSequenceExpression,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_SETTER_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SETTER_CLASS_MEMBER,
                    JsSetterClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("setToken", |node, context| Self::wrap_token(
                        node.set_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("parameter", |node, context| Self::wrap_optional_node(
                        node.parameter().ok(),
                        context
                    )),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SETTER_OBJECT_MEMBER,
                    JsSetterObjectMember,
                    ("setToken", |node, context| Self::wrap_token(
                        node.set_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("parameter", |node, context| Self::wrap_optional_node(
                        node.parameter().ok(),
                        context
                    )),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER,
                    JsShorthandNamedImportSpecifier,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("localName", |node, context| Self::wrap_optional_node(
                        node.local_name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER,
                    JsShorthandPropertyObjectMember,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_SPREAD => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SPREAD,
                    JsSpread,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER,
                    JsStaticInitializationBlockClassMember,
                    ("staticToken", |node, context| Self::wrap_token(
                        node.static_token().ok()
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("statements", |node, context| Self::wrap_node_list(
                        node.statements(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT,
                    JsStaticMemberAssignment,
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION,
                    JsStaticMemberExpression,
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_STATIC_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_STATIC_MODIFIER,
                    JsStaticModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
                    JsStringLiteralExpression,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_SUPER_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SUPER_EXPRESSION,
                    JsSuperExpression,
                    ("superToken", |node, context| Self::wrap_token(
                        node.super_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_SVELTE_SNIPPET_ROOT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SVELTE_SNIPPET_ROOT,
                    JsSvelteSnippetRoot,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("eofToken", |node, context| Self::wrap_token(
                        node.eof_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_SWITCH_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_SWITCH_STATEMENT,
                    JsSwitchStatement,
                    ("switchToken", |node, context| Self::wrap_token(
                        node.switch_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("discriminant", |node, context| Self::wrap_optional_node(
                        node.discriminant().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("cases", |node, context| Self::wrap_node_list(
                        node.cases(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT,
                    JsTemplateChunkElement,
                    ("templateChunkToken", |node, context| Self::wrap_token(
                        node.template_chunk_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_TEMPLATE_ELEMENT,
                    JsTemplateElement,
                    ("dollarCurlyToken", |node, context| Self::wrap_token(
                        node.dollar_curly_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_TEMPLATE_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_TEMPLATE_EXPRESSION,
                    JsTemplateExpression,
                    ("tag", |node, context| Self::wrap_optional_node(
                        node.tag(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                    ("lTickToken", |node, context| Self::wrap_token(
                        node.l_tick_token().ok()
                    )),
                    ("elements", |node, context| Self::wrap_node_list(
                        node.elements(),
                        context
                    )),
                    ("rTickToken", |node, context| Self::wrap_token(
                        node.r_tick_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_THIS_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_THIS_EXPRESSION,
                    JsThisExpression,
                    ("thisToken", |node, context| Self::wrap_token(
                        node.this_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JS_THROW_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_THROW_STATEMENT,
                    JsThrowStatement,
                    ("throwToken", |node, context| Self::wrap_token(
                        node.throw_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_TRY_FINALLY_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_TRY_FINALLY_STATEMENT,
                    JsTryFinallyStatement,
                    ("tryToken", |node, context| Self::wrap_token(
                        node.try_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                    ("catchClause", |node, context| Self::wrap_optional_node(
                        node.catch_clause(),
                        context
                    )),
                    ("finallyClause", |node, context| Self::wrap_optional_node(
                        node.finally_clause().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_TRY_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_TRY_STATEMENT,
                    JsTryStatement,
                    ("tryToken", |node, context| Self::wrap_token(
                        node.try_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                    ("catchClause", |node, context| Self::wrap_optional_node(
                        node.catch_clause().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_UNARY_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_UNARY_EXPRESSION,
                    JsUnaryExpression,
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_VARIABLE_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_VARIABLE_DECLARATION,
                    JsVariableDeclaration,
                    ("awaitToken", |node, context| Self::wrap_token(
                        node.await_token()
                    )),
                    ("kindToken", |node, context| Self::wrap_token(
                        node.kind().ok()
                    )),
                    ("declarators", |node, context| Self::wrap_node_list(
                        node.declarators().into_iter().flatten(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE,
                    JsVariableDeclarationClause,
                    ("declaration", |node, context| Self::wrap_optional_node(
                        node.declaration().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_VARIABLE_DECLARATOR,
                    JsVariableDeclarator,
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("variableAnnotation", |node, context| {
                        Self::wrap_optional_node(node.variable_annotation(), context)
                    }),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_VARIABLE_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_VARIABLE_STATEMENT,
                    JsVariableStatement,
                    ("declaration", |node, context| Self::wrap_optional_node(
                        node.declaration().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::JS_WHILE_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_WHILE_STATEMENT,
                    JsWhileStatement,
                    ("whileToken", |node, context| Self::wrap_token(
                        node.while_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("test", |node, context| Self::wrap_optional_node(
                        node.test().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_WITH_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_WITH_STATEMENT,
                    JsWithStatement,
                    ("withToken", |node, context| Self::wrap_token(
                        node.with_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_YIELD_ARGUMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_YIELD_ARGUMENT,
                    JsYieldArgument,
                    ("starToken", |node, context| Self::wrap_token(
                        node.star_token()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JS_YIELD_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JS_YIELD_EXPRESSION,
                    JsYieldExpression,
                    ("yieldToken", |node, context| Self::wrap_token(
                        node.yield_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_ATTRIBUTE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_ATTRIBUTE,
                    JsxAttribute,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE,
                    JsxAttributeInitializerClause,
                    ("eqToken", |node, context| Self::wrap_token(
                        node.eq_token().ok()
                    )),
                    ("value", |node, context| Self::wrap_optional_node(
                        node.value().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_CLOSING_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_CLOSING_ELEMENT,
                    JsxClosingElement,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("slashToken", |node, context| Self::wrap_token(
                        node.slash_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_CLOSING_FRAGMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_CLOSING_FRAGMENT,
                    JsxClosingFragment,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("slashToken", |node, context| Self::wrap_token(
                        node.slash_token().ok()
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_ELEMENT,
                    JsxElement,
                    ("openingElement", |node, context| Self::wrap_optional_node(
                        node.opening_element().ok(),
                        context
                    )),
                    ("children", |node, context| Self::wrap_node_list(
                        node.children(),
                        context
                    )),
                    ("closingElement", |node, context| Self::wrap_optional_node(
                        node.closing_element().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE,
                    JsxExpressionAttributeValue,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_EXPRESSION_CHILD,
                    JsxExpressionChild,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_FRAGMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_FRAGMENT,
                    JsxFragment,
                    ("openingFragment", |node, context| Self::wrap_optional_node(
                        node.opening_fragment().ok(),
                        context
                    )),
                    ("children", |node, context| Self::wrap_node_list(
                        node.children(),
                        context
                    )),
                    ("closingFragment", |node, context| Self::wrap_optional_node(
                        node.closing_fragment().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_MEMBER_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_MEMBER_NAME,
                    JsxMemberName,
                    ("object", |node, context| Self::wrap_optional_node(
                        node.object().ok(),
                        context
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("member", |node, context| Self::wrap_optional_node(
                        node.member().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_NAME,
                    JsxName,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_NAMESPACE_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_NAMESPACE_NAME,
                    JsxNamespaceName,
                    ("namespace", |node, context| Self::wrap_optional_node(
                        node.namespace().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_OPENING_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_OPENING_ELEMENT,
                    JsxOpeningElement,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                    ("attributes", |node, context| Self::wrap_node_list(
                        node.attributes(),
                        context
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_OPENING_FRAGMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_OPENING_FRAGMENT,
                    JsxOpeningFragment,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_REFERENCE_IDENTIFIER,
                    JsxReferenceIdentifier,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT,
                    JsxSelfClosingElement,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                    ("attributes", |node, context| Self::wrap_node_list(
                        node.attributes(),
                        context
                    )),
                    ("slashToken", |node, context| Self::wrap_token(
                        node.slash_token().ok()
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_SHORTHAND_ATTRIBUTE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_SHORTHAND_ATTRIBUTE,
                    JsxShorthandAttribute,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_SPREAD_ATTRIBUTE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_SPREAD_ATTRIBUTE,
                    JsxSpreadAttribute,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_SPREAD_CHILD => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_SPREAD_CHILD,
                    JsxSpreadChild,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_STRING => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_STRING,
                    JsxString,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::JSX_TAG_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_TAG_EXPRESSION,
                    JsxTagExpression,
                    ("tag", |node, context| Self::wrap_optional_node(
                        node.tag().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::JSX_TEXT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::JSX_TEXT,
                    JsxText,
                    ("valueToken", |node, context| Self::wrap_token(
                        node.value_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_ABSTRACT_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ABSTRACT_MODIFIER,
                    TsAbstractModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER,
                    TsAccessibilityModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_ANY_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ANY_TYPE,
                    TsAnyType,
                    ("anyToken", |node, context| Self::wrap_token(
                        node.any_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_ARRAY_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ARRAY_TYPE,
                    TsArrayType,
                    ("elementType", |node, context| Self::wrap_optional_node(
                        node.element_type().ok(),
                        context
                    )),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_AS_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_AS_ASSIGNMENT,
                    TsAsAssignment,
                    ("assignment", |node, context| Self::wrap_optional_node(
                        node.assignment().ok(),
                        context
                    )),
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_AS_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_AS_EXPRESSION,
                    TsAsExpression,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_ASSERTS_CONDITION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ASSERTS_CONDITION,
                    TsAssertsCondition,
                    ("isToken", |node, context| Self::wrap_token(
                        node.is_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_ASSERTS_RETURN_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ASSERTS_RETURN_TYPE,
                    TsAssertsReturnType,
                    ("assertsToken", |node, context| Self::wrap_token(
                        node.asserts_token().ok()
                    )),
                    ("parameterName", |node, context| Self::wrap_optional_node(
                        node.parameter_name().ok(),
                        context
                    )),
                    ("predicate", |node, context| Self::wrap_optional_node(
                        node.predicate(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_BIGINT_LITERAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_BIGINT_LITERAL_TYPE,
                    TsBigintLiteralType,
                    ("minusToken", |node, context| Self::wrap_token(
                        node.minus_token()
                    )),
                    ("literalToken", |node, context| Self::wrap_token(
                        node.literal_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_BIGINT_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_BIGINT_TYPE,
                    TsBigintType,
                    ("bigintToken", |node, context| Self::wrap_token(
                        node.bigint_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE,
                    TsBooleanLiteralType,
                    ("literal", |node, context| Self::wrap_token(
                        node.literal().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_BOOLEAN_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_BOOLEAN_TYPE,
                    TsBooleanType,
                    ("booleanToken", |node, context| Self::wrap_token(
                        node.boolean_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER,
                    TsCallSignatureTypeMember,
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_CONDITIONAL_TYPE,
                    TsConditionalType,
                    ("checkType", |node, context| Self::wrap_optional_node(
                        node.check_type().ok(),
                        context
                    )),
                    ("extendsToken", |node, context| Self::wrap_token(
                        node.extends_token().ok()
                    )),
                    ("extendsType", |node, context| Self::wrap_optional_node(
                        node.extends_type().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token().ok()
                    )),
                    ("trueType", |node, context| Self::wrap_optional_node(
                        node.true_type().ok(),
                        context
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("falseType", |node, context| Self::wrap_optional_node(
                        node.false_type().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_CONST_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_CONST_MODIFIER,
                    TsConstModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER,
                    TsConstructSignatureTypeMember,
                    ("newToken", |node, context| Self::wrap_token(
                        node.new_token().ok()
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER,
                    TsConstructorSignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_CONSTRUCTOR_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_CONSTRUCTOR_TYPE,
                    TsConstructorType,
                    ("abstractToken", |node, context| Self::wrap_token(
                        node.abstract_token()
                    )),
                    ("newToken", |node, context| Self::wrap_token(
                        node.new_token().ok()
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("fatArrowToken", |node, context| Self::wrap_token(
                        node.fat_arrow_token().ok()
                    )),
                    ("returnType", |node, context| Self::wrap_optional_node(
                        node.return_type().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_DECLARATION_MODULE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DECLARATION_MODULE,
                    TsDeclarationModule,
                    ("bomToken", |node, context| Self::wrap_token(
                        node.bom_token()
                    )),
                    ("interpreterToken", |node, context| Self::wrap_token(
                        node.interpreter_token()
                    )),
                    ("directives", |node, context| Self::wrap_node_list(
                        node.directives(),
                        context
                    )),
                    ("items", |node, context| Self::wrap_node_list(
                        node.items(),
                        context
                    )),
                    ("eofToken", |node, context| Self::wrap_token(
                        node.eof_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION,
                    TsDeclareFunctionDeclaration,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("functionToken", |node, context| Self::wrap_token(
                        node.function_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION,
                    TsDeclareFunctionExportDefaultDeclaration,
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("functionToken", |node, context| Self::wrap_token(
                        node.function_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_DECLARE_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DECLARE_MODIFIER,
                    TsDeclareModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_DECLARE_STATEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DECLARE_STATEMENT,
                    TsDeclareStatement,
                    ("declareToken", |node, context| Self::wrap_token(
                        node.declare_token().ok()
                    )),
                    ("declaration", |node, context| Self::wrap_optional_node(
                        node.declaration().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE,
                    TsDefaultTypeClause,
                    ("eqToken", |node, context| Self::wrap_token(
                        node.eq_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION,
                    TsDefinitePropertyAnnotation,
                    ("exclToken", |node, context| Self::wrap_token(
                        node.excl_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION,
                    TsDefiniteVariableAnnotation,
                    ("exclToken", |node, context| Self::wrap_token(
                        node.excl_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY,
                    TsEmptyExternalModuleDeclarationBody,
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_ENUM_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ENUM_DECLARATION,
                    TsEnumDeclaration,
                    ("constToken", |node, context| Self::wrap_token(
                        node.const_token()
                    )),
                    ("enumToken", |node, context| Self::wrap_token(
                        node.enum_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_ENUM_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_ENUM_MEMBER,
                    TsEnumMember,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("initializer", |node, context| Self::wrap_optional_node(
                        node.initializer(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE,
                    TsExportAsNamespaceClause,
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("namespaceToken", |node, context| Self::wrap_token(
                        node.namespace_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE,
                    TsExportAssignmentClause,
                    ("eqToken", |node, context| Self::wrap_token(
                        node.eq_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE,
                    TsExportDeclareClause,
                    ("declareToken", |node, context| Self::wrap_token(
                        node.declare_token().ok()
                    )),
                    ("declaration", |node, context| Self::wrap_optional_node(
                        node.declaration().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_EXTENDS_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EXTENDS_CLAUSE,
                    TsExtendsClause,
                    ("extendsToken", |node, context| Self::wrap_token(
                        node.extends_token().ok()
                    )),
                    ("types", |node, context| Self::wrap_node_list(
                        node.types().into_iter().flatten(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION,
                    TsExternalModuleDeclaration,
                    ("moduleToken", |node, context| Self::wrap_token(
                        node.module_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE,
                    TsExternalModuleReference,
                    ("requireToken", |node, context| Self::wrap_token(
                        node.require_token().ok()
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("source", |node, context| Self::wrap_optional_node(
                        node.source().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_FUNCTION_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_FUNCTION_TYPE,
                    TsFunctionType,
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("fatArrowToken", |node, context| Self::wrap_token(
                        node.fat_arrow_token().ok()
                    )),
                    ("returnType", |node, context| Self::wrap_optional_node(
                        node.return_type().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER,
                    TsGetterSignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("getToken", |node, context| Self::wrap_token(
                        node.get_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("returnType", |node, context| Self::wrap_optional_node(
                        node.return_type(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER,
                    TsGetterSignatureTypeMember,
                    ("getToken", |node, context| Self::wrap_token(
                        node.get_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_GLOBAL_DECLARATION,
                    TsGlobalDeclaration,
                    ("globalToken", |node, context| Self::wrap_token(
                        node.global_token().ok()
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IDENTIFIER_BINDING,
                    TsIdentifierBinding,
                    ("nameToken", |node, context| Self::wrap_token(
                        node.name_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_IMPLEMENTS_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPLEMENTS_CLAUSE,
                    TsImplementsClause,
                    ("implementsToken", |node, context| Self::wrap_token(
                        node.implements_token().ok()
                    )),
                    ("types", |node, context| Self::wrap_node_list(
                        node.types().into_iter().flatten(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION,
                    TsImportEqualsDeclaration,
                    ("importToken", |node, context| Self::wrap_token(
                        node.import_token().ok()
                    )),
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("eqToken", |node, context| Self::wrap_token(
                        node.eq_token().ok()
                    )),
                    ("moduleReference", |node, context| Self::wrap_optional_node(
                        node.module_reference().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_IMPORT_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPORT_TYPE,
                    TsImportType,
                    ("typeofToken", |node, context| Self::wrap_token(
                        node.typeof_token()
                    )),
                    ("importToken", |node, context| Self::wrap_token(
                        node.import_token().ok()
                    )),
                    ("arguments", |node, context| Self::wrap_optional_node(
                        node.arguments().ok(),
                        context
                    )),
                    ("qualifierClause", |node, context| Self::wrap_optional_node(
                        node.qualifier_clause(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS,
                    TsImportTypeArguments,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("argument", |node, context| Self::wrap_optional_node(
                        node.argument().ok(),
                        context
                    )),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token()
                    )),
                    ("tsImportTypeAssertionBlock", |node, context| {
                        Self::wrap_optional_node(node.ts_import_type_assertion_block(), context)
                    }),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION,
                    TsImportTypeAssertion,
                    ("withToken", |node, context| Self::wrap_token(
                        node.with_token().ok()
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("assertions", |node, context| Self::wrap_node_list(
                        node.assertions().into_iter().flatten(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK,
                    TsImportTypeAssertionBlock,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("typeAssertion", |node, context| Self::wrap_optional_node(
                        node.type_assertion().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER,
                    TsImportTypeQualifier,
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_IN_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_IN_MODIFIER,
                    TsInModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER,
                    TsIndexSignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("parameter", |node, context| Self::wrap_optional_node(
                        node.parameter().ok(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER,
                    TsIndexSignatureParameter,
                    ("binding", |node, context| Self::wrap_optional_node(
                        node.binding().ok(),
                        context
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER,
                    TsIndexSignatureTypeMember,
                    ("readonlyToken", |node, context| Self::wrap_token(
                        node.readonly_token()
                    )),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("parameter", |node, context| Self::wrap_optional_node(
                        node.parameter().ok(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation().ok(),
                        context
                    )),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INDEXED_ACCESS_TYPE,
                    TsIndexedAccessType,
                    ("objectType", |node, context| Self::wrap_optional_node(
                        node.object_type().ok(),
                        context
                    )),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("indexType", |node, context| Self::wrap_optional_node(
                        node.index_type().ok(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_INFER_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INFER_TYPE,
                    TsInferType,
                    ("inferToken", |node, context| Self::wrap_token(
                        node.infer_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("constraint", |node, context| Self::wrap_optional_node(
                        node.constraint(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER,
                    TsInitializedPropertySignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token()
                    )),
                    ("value", |node, context| Self::wrap_optional_node(
                        node.value().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_INSTANTIATION_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INSTANTIATION_EXPRESSION,
                    TsInstantiationExpression,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("arguments", |node, context| Self::wrap_optional_node(
                        node.arguments().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_INTERFACE_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INTERFACE_DECLARATION,
                    TsInterfaceDeclaration,
                    ("interfaceToken", |node, context| Self::wrap_token(
                        node.interface_token().ok()
                    )),
                    ("id", |node, context| Self::wrap_optional_node(
                        node.id().ok(),
                        context
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("extendsClause", |node, context| Self::wrap_optional_node(
                        node.extends_clause(),
                        context
                    )),
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_INTERSECTION_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_INTERSECTION_TYPE,
                    TsIntersectionType,
                    ("leadingSeparatorToken", |node, context| Self::wrap_token(
                        node.leading_separator_token()
                    )),
                    ("types", |node, context| Self::wrap_node_list(
                        node.types().into_iter().flatten(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME,
                    TsLiteralEnumMemberName,
                    ("value", |node, context| Self::wrap_token(node.value().ok())),
                );
            }
            JsSyntaxKind::TS_MAPPED_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_MAPPED_TYPE,
                    TsMappedType,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    (
                        "readonlyModifier",
                        |node, context| Self::wrap_optional_node(node.readonly_modifier(), context)
                    ),
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("propertyName", |node, context| Self::wrap_optional_node(
                        node.property_name().ok(),
                        context
                    )),
                    ("inToken", |node, context| Self::wrap_token(
                        node.in_token().ok()
                    )),
                    ("keysType", |node, context| Self::wrap_optional_node(
                        node.keys_type().ok(),
                        context
                    )),
                    ("asClause", |node, context| Self::wrap_optional_node(
                        node.as_clause(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                    (
                        "optionalModifier",
                        |node, context| Self::wrap_optional_node(node.optional_modifier(), context)
                    ),
                    ("mappedType", |node, context| Self::wrap_optional_node(
                        node.mapped_type(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE,
                    TsMappedTypeAsClause,
                    ("asToken", |node, context| Self::wrap_token(
                        node.as_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE,
                    TsMappedTypeOptionalModifierClause,
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token()
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE,
                    TsMappedTypeReadonlyModifierClause,
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token()
                    )),
                    ("readonlyToken", |node, context| Self::wrap_token(
                        node.readonly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER,
                    TsMethodSignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("asyncToken", |node, context| Self::wrap_token(
                        node.async_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token()
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER,
                    TsMethodSignatureTypeMember,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("optionalToken", |node, context| Self::wrap_token(
                        node.optional_token()
                    )),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("parameters", |node, context| Self::wrap_optional_node(
                        node.parameters().ok(),
                        context
                    )),
                    ("returnTypeAnnotation", |node, context| {
                        Self::wrap_optional_node(node.return_type_annotation(), context)
                    }),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_MODULE_BLOCK => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_MODULE_BLOCK,
                    TsModuleBlock,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("items", |node, context| Self::wrap_node_list(
                        node.items(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_MODULE_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_MODULE_DECLARATION,
                    TsModuleDeclaration,
                    ("moduleOrNamespace", |node, context| Self::wrap_token(
                        node.module_or_namespace().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("body", |node, context| Self::wrap_optional_node(
                        node.body().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT,
                    TsNamedTupleTypeElement,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token()
                    )),
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_NEVER_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NEVER_TYPE,
                    TsNeverType,
                    ("neverToken", |node, context| Self::wrap_token(
                        node.never_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT,
                    TsNonNullAssertionAssignment,
                    ("assignment", |node, context| Self::wrap_optional_node(
                        node.assignment().ok(),
                        context
                    )),
                    ("exclToken", |node, context| Self::wrap_token(
                        node.excl_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION,
                    TsNonNullAssertionExpression,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("exclToken", |node, context| Self::wrap_token(
                        node.excl_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_NON_PRIMITIVE_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NON_PRIMITIVE_TYPE,
                    TsNonPrimitiveType,
                    ("objectToken", |node, context| Self::wrap_token(
                        node.object_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_NULL_LITERAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NULL_LITERAL_TYPE,
                    TsNullLiteralType,
                    ("literalToken", |node, context| Self::wrap_token(
                        node.literal_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_NUMBER_LITERAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NUMBER_LITERAL_TYPE,
                    TsNumberLiteralType,
                    ("minusToken", |node, context| Self::wrap_token(
                        node.minus_token()
                    )),
                    ("literalToken", |node, context| Self::wrap_token(
                        node.literal_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_NUMBER_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_NUMBER_TYPE,
                    TsNumberType,
                    ("numberToken", |node, context| Self::wrap_token(
                        node.number_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_OBJECT_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_OBJECT_TYPE,
                    TsObjectType,
                    ("lCurlyToken", |node, context| Self::wrap_token(
                        node.l_curly_token().ok()
                    )),
                    ("members", |node, context| Self::wrap_node_list(
                        node.members(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION,
                    TsOptionalPropertyAnnotation,
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT,
                    TsOptionalTupleTypeElement,
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                    ("questionMarkToken", |node, context| Self::wrap_token(
                        node.question_mark_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_OUT_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_OUT_MODIFIER,
                    TsOutModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_OVERRIDE_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_OVERRIDE_MODIFIER,
                    TsOverrideModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_PARENTHESIZED_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_PARENTHESIZED_TYPE,
                    TsParenthesizedType,
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_PREDICATE_RETURN_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_PREDICATE_RETURN_TYPE,
                    TsPredicateReturnType,
                    ("parameterName", |node, context| Self::wrap_optional_node(
                        node.parameter_name().ok(),
                        context
                    )),
                    ("isToken", |node, context| Self::wrap_token(
                        node.is_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_PROPERTY_PARAMETER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_PROPERTY_PARAMETER,
                    TsPropertyParameter,
                    ("decorators", |node, context| Self::wrap_node_list(
                        node.decorators(),
                        context
                    )),
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("formalParameter", |node, context| Self::wrap_optional_node(
                        node.formal_parameter().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER,
                    TsPropertySignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("propertyAnnotation", |node, context| {
                        Self::wrap_optional_node(node.property_annotation(), context)
                    }),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER,
                    TsPropertySignatureTypeMember,
                    ("readonlyToken", |node, context| Self::wrap_token(
                        node.readonly_token()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("optionalToken", |node, context| Self::wrap_token(
                        node.optional_token()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_QUALIFIED_MODULE_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_QUALIFIED_MODULE_NAME,
                    TsQualifiedModuleName,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_QUALIFIED_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_QUALIFIED_NAME,
                    TsQualifiedName,
                    ("left", |node, context| Self::wrap_optional_node(
                        node.left().ok(),
                        context
                    )),
                    ("dotToken", |node, context| Self::wrap_token(
                        node.dot_token().ok()
                    )),
                    ("right", |node, context| Self::wrap_optional_node(
                        node.right().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_READONLY_MODIFIER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_READONLY_MODIFIER,
                    TsReadonlyModifier,
                    ("modifierToken", |node, context| Self::wrap_token(
                        node.modifier_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_REFERENCE_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_REFERENCE_TYPE,
                    TsReferenceType,
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT,
                    TsRestTupleTypeElement,
                    ("dotdotdotToken", |node, context| Self::wrap_token(
                        node.dotdotdot_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION,
                    TsReturnTypeAnnotation,
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_SATISFIES_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_SATISFIES_ASSIGNMENT,
                    TsSatisfiesAssignment,
                    ("assignment", |node, context| Self::wrap_optional_node(
                        node.assignment().ok(),
                        context
                    )),
                    ("satisfiesToken", |node, context| Self::wrap_token(
                        node.satisfies_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_SATISFIES_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_SATISFIES_EXPRESSION,
                    TsSatisfiesExpression,
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                    ("satisfiesToken", |node, context| Self::wrap_token(
                        node.satisfies_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER,
                    TsSetterSignatureClassMember,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("setToken", |node, context| Self::wrap_token(
                        node.set_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("parameter", |node, context| Self::wrap_optional_node(
                        node.parameter().ok(),
                        context
                    )),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER,
                    TsSetterSignatureTypeMember,
                    ("setToken", |node, context| Self::wrap_token(
                        node.set_token().ok()
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("lParenToken", |node, context| Self::wrap_token(
                        node.l_paren_token().ok()
                    )),
                    ("parameter", |node, context| Self::wrap_optional_node(
                        node.parameter().ok(),
                        context
                    )),
                    ("commaToken", |node, context| Self::wrap_token(
                        node.comma_token()
                    )),
                    ("rParenToken", |node, context| Self::wrap_token(
                        node.r_paren_token().ok()
                    )),
                    ("separatorToken", |node, context| Self::wrap_token(
                        node.separator_token()
                    )),
                );
            }
            JsSyntaxKind::TS_STRING_LITERAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_STRING_LITERAL_TYPE,
                    TsStringLiteralType,
                    ("literalToken", |node, context| Self::wrap_token(
                        node.literal_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_STRING_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_STRING_TYPE,
                    TsStringType,
                    ("stringToken", |node, context| Self::wrap_token(
                        node.string_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_SYMBOL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_SYMBOL_TYPE,
                    TsSymbolType,
                    ("symbolToken", |node, context| Self::wrap_token(
                        node.symbol_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT,
                    TsTemplateChunkElement,
                    ("templateChunkToken", |node, context| Self::wrap_token(
                        node.template_chunk_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TEMPLATE_ELEMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TEMPLATE_ELEMENT,
                    TsTemplateElement,
                    ("dollarCurlyToken", |node, context| Self::wrap_token(
                        node.dollar_curly_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                    ("rCurlyToken", |node, context| Self::wrap_token(
                        node.r_curly_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE,
                    TsTemplateLiteralType,
                    ("lTickToken", |node, context| Self::wrap_token(
                        node.l_tick_token().ok()
                    )),
                    ("elements", |node, context| Self::wrap_node_list(
                        node.elements(),
                        context
                    )),
                    ("rTickToken", |node, context| Self::wrap_token(
                        node.r_tick_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_THIS_PARAMETER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_THIS_PARAMETER,
                    TsThisParameter,
                    ("thisToken", |node, context| Self::wrap_token(
                        node.this_token().ok()
                    )),
                    ("typeAnnotation", |node, context| Self::wrap_optional_node(
                        node.type_annotation(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_THIS_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_THIS_TYPE,
                    TsThisType,
                    ("thisToken", |node, context| Self::wrap_token(
                        node.this_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TUPLE_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TUPLE_TYPE,
                    TsTupleType,
                    ("lBrackToken", |node, context| Self::wrap_token(
                        node.l_brack_token().ok()
                    )),
                    ("elements", |node, context| Self::wrap_node_list(
                        node.elements().into_iter().flatten(),
                        context
                    )),
                    ("rBrackToken", |node, context| Self::wrap_token(
                        node.r_brack_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION,
                    TsTypeAliasDeclaration,
                    ("typeToken", |node, context| Self::wrap_token(
                        node.type_token().ok()
                    )),
                    ("bindingIdentifier", |node, context| {
                        Self::wrap_optional_node(node.binding_identifier().ok(), context)
                    }),
                    ("typeParameters", |node, context| Self::wrap_optional_node(
                        node.type_parameters(),
                        context
                    )),
                    ("eqToken", |node, context| Self::wrap_token(
                        node.eq_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                    ("semicolonToken", |node, context| Self::wrap_token(
                        node.semicolon_token()
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_ANNOTATION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_ANNOTATION,
                    TsTypeAnnotation,
                    ("colonToken", |node, context| Self::wrap_token(
                        node.colon_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_ARGUMENTS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_ARGUMENTS,
                    TsTypeArguments,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("tsTypeArgumentList", |node, context| Self::wrap_node_list(
                        node.ts_type_argument_list().into_iter().flatten(),
                        context
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT,
                    TsTypeAssertionAssignment,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                    ("assignment", |node, context| Self::wrap_optional_node(
                        node.assignment().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION,
                    TsTypeAssertionExpression,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                    ("expression", |node, context| Self::wrap_optional_node(
                        node.expression().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE,
                    TsTypeConstraintClause,
                    ("extendsToken", |node, context| Self::wrap_token(
                        node.extends_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_OPERATOR_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_OPERATOR_TYPE,
                    TsTypeOperatorType,
                    ("operatorToken", |node, context| Self::wrap_token(
                        node.operator_token().ok()
                    )),
                    ("ty", |node, context| Self::wrap_optional_node(
                        node.ty().ok(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_PARAMETER => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_PARAMETER,
                    TsTypeParameter,
                    ("modifiers", |node, context| Self::wrap_node_list(
                        node.modifiers(),
                        context
                    )),
                    ("name", |node, context| Self::wrap_optional_node(
                        node.name().ok(),
                        context
                    )),
                    ("constraint", |node, context| Self::wrap_optional_node(
                        node.constraint(),
                        context
                    )),
                    ("default", |node, context| Self::wrap_optional_node(
                        node.default(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_PARAMETER_NAME,
                    TsTypeParameterName,
                    ("identToken", |node, context| Self::wrap_token(
                        node.ident_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TYPE_PARAMETERS => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPE_PARAMETERS,
                    TsTypeParameters,
                    ("lAngleToken", |node, context| Self::wrap_token(
                        node.l_angle_token().ok()
                    )),
                    ("items", |node, context| Self::wrap_node_list(
                        node.items().into_iter().flatten(),
                        context
                    )),
                    ("rAngleToken", |node, context| Self::wrap_token(
                        node.r_angle_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_TYPEOF_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_TYPEOF_TYPE,
                    TsTypeofType,
                    ("typeofToken", |node, context| Self::wrap_token(
                        node.typeof_token().ok()
                    )),
                    ("expressionName", |node, context| Self::wrap_optional_node(
                        node.expression_name().ok(),
                        context
                    )),
                    ("typeArguments", |node, context| Self::wrap_optional_node(
                        node.type_arguments(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_UNDEFINED_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_UNDEFINED_TYPE,
                    TsUndefinedType,
                    ("undefinedToken", |node, context| Self::wrap_token(
                        node.undefined_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_UNION_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_UNION_TYPE,
                    TsUnionType,
                    ("leadingSeparatorToken", |node, context| Self::wrap_token(
                        node.leading_separator_token()
                    )),
                    ("types", |node, context| Self::wrap_node_list(
                        node.types().into_iter().flatten(),
                        context
                    )),
                );
            }
            JsSyntaxKind::TS_UNKNOWN_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_UNKNOWN_TYPE,
                    TsUnknownType,
                    ("unknownToken", |node, context| Self::wrap_token(
                        node.unknown_token().ok()
                    )),
                );
            }
            JsSyntaxKind::TS_VOID_TYPE => {
                register_js_ast_fields!(
                    prototype,
                    JsSyntaxKind::TS_VOID_TYPE,
                    TsVoidType,
                    ("voidToken", |node, context| Self::wrap_token(
                        node.void_token().ok()
                    )),
                );
            }
            _ => {}
        }
        prototype.build()
    }
}
