//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::JsSyntaxNode::kind(&node) {
                $crate::JsSyntaxKind::JS_ACCESSOR_MODIFIER => {
                    let $pattern = unsafe { $crate::JsAccessorModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN => {
                    let $pattern = unsafe { $crate::JsArrayAssignmentPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::JsArrayAssignmentPatternElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::JsArrayAssignmentPatternRestElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => {
                    let $pattern = unsafe { $crate::JsArrayBindingPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::JsArrayBindingPatternElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::JsArrayBindingPatternRestElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_HOLE => {
                    let $pattern = unsafe { $crate::JsArrayHole::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsArrowFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsAssignmentExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_AWAIT_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsAwaitExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsBigintLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BLOCK_STATEMENT => {
                    let $pattern = unsafe { $crate::JsBlockStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BREAK_STATEMENT => {
                    let $pattern = unsafe { $crate::JsBreakStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CALL_ARGUMENTS => {
                    let $pattern = unsafe { $crate::JsCallArguments::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CASE_CLAUSE => {
                    let $pattern = unsafe { $crate::JsCaseClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CATCH_CLAUSE => {
                    let $pattern = unsafe { $crate::JsCatchClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CATCH_DECLARATION => {
                    let $pattern = unsafe { $crate::JsCatchDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_DECLARATION => {
                    let $pattern = unsafe { $crate::JsClassDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::JsClassExportDefaultDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsClassExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::JsComputedMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsComputedMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::JsComputedMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsConditionalExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::JsConstructorClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => {
                    let $pattern = unsafe { $crate::JsConstructorParameters::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONTINUE_STATEMENT => {
                    let $pattern = unsafe { $crate::JsContinueStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DEBUGGER_STATEMENT => {
                    let $pattern = unsafe { $crate::JsDebuggerStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DECORATOR => {
                    let $pattern = unsafe { $crate::JsDecorator::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { $crate::JsDefaultClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => {
                    let $pattern = unsafe { $crate::JsDefaultImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DIRECTIVE => {
                    let $pattern = unsafe { $crate::JsDirective::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::JsDoWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::JsElseClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EMPTY_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::JsEmptyClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EMPTY_STATEMENT => {
                    let $pattern = unsafe { $crate::JsEmptyStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT => {
                    let $pattern = unsafe { $crate::JsExport::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                    let $pattern = unsafe { $crate::JsExportAsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::JsExportDefaultDeclarationClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::JsExportDefaultExpressionClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::JsExportFromClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE => {
                    let $pattern = unsafe { $crate::JsExportNamedClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::JsExportNamedFromClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::JsExportNamedFromSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::JsExportNamedShorthandSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => {
                    let $pattern = unsafe { $crate::JsExportNamedSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPRESSION_SNIPPED => {
                    let $pattern = unsafe { $crate::JsExpressionSnipped::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                    let $pattern = unsafe { $crate::JsExpressionStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { $crate::JsExtendsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FINALLY_CLAUSE => {
                    let $pattern = unsafe { $crate::JsFinallyClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_IN_STATEMENT => {
                    let $pattern = unsafe { $crate::JsForInStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_OF_STATEMENT => {
                    let $pattern = unsafe { $crate::JsForOfStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_STATEMENT => {
                    let $pattern = unsafe { $crate::JsForStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => {
                    let $pattern = unsafe { $crate::JsForVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FORMAL_PARAMETER => {
                    let $pattern = unsafe { $crate::JsFormalParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_BODY => {
                    let $pattern = unsafe { $crate::JsFunctionBody::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                    let $pattern = unsafe { $crate::JsFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::JsFunctionExportDefaultDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_GETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::JsGetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_GETTER_OBJECT_MEMBER => {
                    let $pattern = unsafe { $crate::JsGetterObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::JsIdentifierAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { $crate::JsIdentifierBinding::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsIdentifierExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IF_STATEMENT => {
                    let $pattern = unsafe { $crate::JsIfStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT => {
                    let $pattern = unsafe { $crate::JsImport::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_ASSERTION => {
                    let $pattern = unsafe { $crate::JsImportAssertion::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => {
                    let $pattern = unsafe { $crate::JsImportAssertionEntry::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
                    let $pattern = unsafe { $crate::JsImportBareClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsImportCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => {
                    let $pattern = unsafe { $crate::JsImportCombinedClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                    let $pattern = unsafe { $crate::JsImportDefaultClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_META_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsImportMetaExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                    let $pattern = unsafe { $crate::JsImportNamedClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                    let $pattern = unsafe { $crate::JsImportNamespaceClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IN_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsInExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                    let $pattern = unsafe { $crate::JsInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsInstanceofExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LABEL => {
                    let $pattern = unsafe { $crate::JsLabel::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LABELED_STATEMENT => {
                    let $pattern = unsafe { $crate::JsLabeledStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LITERAL_EXPORT_NAME => {
                    let $pattern = unsafe { $crate::JsLiteralExportName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::JsLiteralMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_METAVARIABLE => {
                    let $pattern = unsafe { $crate::JsMetavariable::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::JsMethodClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
                    let $pattern = unsafe { $crate::JsMethodObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_MODULE => {
                    let $pattern = unsafe { $crate::JsModule::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_MODULE_SOURCE => {
                    let $pattern = unsafe { $crate::JsModuleSource::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAME => {
                    let $pattern = unsafe { $crate::JsName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => {
                    let $pattern = unsafe { $crate::JsNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => {
                    let $pattern = unsafe { $crate::JsNamedImportSpecifiers::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::JsNamespaceImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NEW_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsNewExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NEW_TARGET_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsNewTargetExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN => {
                    let $pattern =
                        unsafe { $crate::JsObjectAssignmentPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                    let $pattern =
                        unsafe { $crate::JsObjectAssignmentPatternProperty::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                    let $pattern =
                        unsafe { $crate::JsObjectAssignmentPatternRest::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                    let $pattern = unsafe {
                        $crate::JsObjectAssignmentPatternShorthandProperty::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => {
                    let $pattern = unsafe { $crate::JsObjectBindingPattern::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                    let $pattern =
                        unsafe { $crate::JsObjectBindingPatternProperty::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => {
                    let $pattern =
                        unsafe { $crate::JsObjectBindingPatternRest::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                    let $pattern = unsafe {
                        $crate::JsObjectBindingPatternShorthandProperty::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsObjectExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARAMETERS => {
                    let $pattern = unsafe { $crate::JsParameters::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::JsParenthesizedAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsPostUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsPreUpdateExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::JsPrivateClassMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PRIVATE_NAME => {
                    let $pattern = unsafe { $crate::JsPrivateName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::JsPropertyClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                    let $pattern = unsafe { $crate::JsPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                    let $pattern = unsafe { $crate::JsReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsRegexLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_REST_PARAMETER => {
                    let $pattern = unsafe { $crate::JsRestParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_RETURN_STATEMENT => {
                    let $pattern = unsafe { $crate::JsReturnStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SCRIPT => {
                    let $pattern = unsafe { $crate::JsScript::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsSequenceExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SETTER_CLASS_MEMBER => {
                    let $pattern = unsafe { $crate::JsSetterClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
                    let $pattern = unsafe { $crate::JsSetterObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::JsShorthandNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                    let $pattern =
                        unsafe { $crate::JsShorthandPropertyObjectMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SPREAD => {
                    let $pattern = unsafe { $crate::JsSpread::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                    let $pattern = unsafe {
                        $crate::JsStaticInitializationBlockClassMember::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::JsStaticMemberAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsStaticMemberExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATIC_MODIFIER => {
                    let $pattern = unsafe { $crate::JsStaticModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::JsStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SUPER_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsSuperExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SWITCH_STATEMENT => {
                    let $pattern = unsafe { $crate::JsSwitchStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => {
                    let $pattern = unsafe { $crate::JsTemplateChunkElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
                    let $pattern = unsafe { $crate::JsTemplateElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsTemplateExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_THIS_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsThisExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_THROW_STATEMENT => {
                    let $pattern = unsafe { $crate::JsThrowStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TRY_FINALLY_STATEMENT => {
                    let $pattern = unsafe { $crate::JsTryFinallyStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TRY_STATEMENT => {
                    let $pattern = unsafe { $crate::JsTryStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATION => {
                    let $pattern = unsafe { $crate::JsVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::JsVariableDeclarationClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                    let $pattern = unsafe { $crate::JsVariableDeclarator::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::JsVariableStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::JsWhileStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_WITH_STATEMENT => {
                    let $pattern = unsafe { $crate::JsWithStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_YIELD_ARGUMENT => {
                    let $pattern = unsafe { $crate::JsYieldArgument::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_YIELD_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsYieldExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::JsxAttribute::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::JsxAttributeInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::JsxClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_CLOSING_FRAGMENT => {
                    let $pattern = unsafe { $crate::JsxClosingFragment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_ELEMENT => {
                    let $pattern = unsafe { $crate::JsxElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => {
                    let $pattern =
                        unsafe { $crate::JsxExpressionAttributeValue::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                    let $pattern = unsafe { $crate::JsxExpressionChild::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_FRAGMENT => {
                    let $pattern = unsafe { $crate::JsxFragment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::JsxMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_NAME => {
                    let $pattern = unsafe { $crate::JsxName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_NAMESPACE_NAME => {
                    let $pattern = unsafe { $crate::JsxNamespaceName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_OPENING_ELEMENT => {
                    let $pattern = unsafe { $crate::JsxOpeningElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_OPENING_FRAGMENT => {
                    let $pattern = unsafe { $crate::JsxOpeningFragment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => {
                    let $pattern = unsafe { $crate::JsxReferenceIdentifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::JsxSelfClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_SPREAD_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::JsxSpreadAttribute::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_SPREAD_CHILD => {
                    let $pattern = unsafe { $crate::JsxSpreadChild::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_STRING => {
                    let $pattern = unsafe { $crate::JsxString::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_TAG_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsxTagExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_TEXT => {
                    let $pattern = unsafe { $crate::JsxText::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ABSTRACT_MODIFIER => {
                    let $pattern = unsafe { $crate::TsAbstractModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER => {
                    let $pattern = unsafe { $crate::TsAccessibilityModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ANY_TYPE => {
                    let $pattern = unsafe { $crate::TsAnyType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ARRAY_TYPE => {
                    let $pattern = unsafe { $crate::TsArrayType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_AS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::TsAsAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_AS_EXPRESSION => {
                    let $pattern = unsafe { $crate::TsAsExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ASSERTS_CONDITION => {
                    let $pattern = unsafe { $crate::TsAssertsCondition::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ASSERTS_RETURN_TYPE => {
                    let $pattern = unsafe { $crate::TsAssertsReturnType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BIGINT_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::TsBigintLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BIGINT_TYPE => {
                    let $pattern = unsafe { $crate::TsBigintType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::TsBooleanLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BOOLEAN_TYPE => {
                    let $pattern = unsafe { $crate::TsBooleanType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsCallSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                    let $pattern = unsafe { $crate::TsConditionalType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONST_MODIFIER => {
                    let $pattern = unsafe { $crate::TsConstModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsConstructSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsConstructorSignatureClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_CONSTRUCTOR_TYPE => {
                    let $pattern = unsafe { $crate::TsConstructorType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARATION_MODULE => {
                    let $pattern = unsafe { $crate::TsDeclarationModule::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::TsDeclareFunctionDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                    let $pattern = unsafe {
                        $crate::TsDeclareFunctionExportDefaultDeclaration::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARE_MODIFIER => {
                    let $pattern = unsafe { $crate::TsDeclareModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DECLARE_STATEMENT => {
                    let $pattern = unsafe { $crate::TsDeclareStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                    let $pattern = unsafe { $crate::TsDefaultTypeClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::TsDefinitePropertyAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::TsDefiniteVariableAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                    let $pattern = unsafe {
                        $crate::TsEmptyExternalModuleDeclarationBody::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::TS_ENUM_DECLARATION => {
                    let $pattern = unsafe { $crate::TsEnumDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ENUM_MEMBER => {
                    let $pattern = unsafe { $crate::TsEnumMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::TsExportAsNamespaceClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE => {
                    let $pattern = unsafe { $crate::TsExportAssignmentClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE => {
                    let $pattern = unsafe { $crate::TsExportDeclareClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXTENDS_CLAUSE => {
                    let $pattern = unsafe { $crate::TsExtendsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::TsExternalModuleDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE => {
                    let $pattern =
                        unsafe { $crate::TsExternalModuleReference::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_FUNCTION_TYPE => {
                    let $pattern = unsafe { $crate::TsFunctionType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsGetterSignatureClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsGetterSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                    let $pattern = unsafe { $crate::TsGlobalDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                    let $pattern = unsafe { $crate::TsIdentifierBinding::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPLEMENTS_CLAUSE => {
                    let $pattern = unsafe { $crate::TsImplementsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::TsImportEqualsDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE => {
                    let $pattern = unsafe { $crate::TsImportType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { $crate::TsImportTypeArguments::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION => {
                    let $pattern = unsafe { $crate::TsImportTypeAssertion::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK => {
                    let $pattern =
                        unsafe { $crate::TsImportTypeAssertionBlock::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER => {
                    let $pattern = unsafe { $crate::TsImportTypeQualifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_IN_MODIFIER => {
                    let $pattern = unsafe { $crate::TsInModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsIndexSignatureClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER => {
                    let $pattern =
                        unsafe { $crate::TsIndexSignatureParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsIndexSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                    let $pattern = unsafe { $crate::TsIndexedAccessType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INFER_TYPE => {
                    let $pattern = unsafe { $crate::TsInferType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                    let $pattern = unsafe {
                        $crate::TsInitializedPropertySignatureClassMember::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::TS_INSTANTIATION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::TsInstantiationExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INTERFACE_DECLARATION => {
                    let $pattern = unsafe { $crate::TsInterfaceDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INTERSECTION_TYPE => {
                    let $pattern = unsafe { $crate::TsIntersectionType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME => {
                    let $pattern = unsafe { $crate::TsLiteralEnumMemberName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE => {
                    let $pattern = unsafe { $crate::TsMappedType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE => {
                    let $pattern = unsafe { $crate::TsMappedTypeAsClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::TsMappedTypeOptionalModifierClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::TsMappedTypeReadonlyModifierClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsMethodSignatureClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsMethodSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MODULE_BLOCK => {
                    let $pattern = unsafe { $crate::TsModuleBlock::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_MODULE_DECLARATION => {
                    let $pattern = unsafe { $crate::TsModuleDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT => {
                    let $pattern = unsafe { $crate::TsNamedTupleTypeElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NEVER_TYPE => {
                    let $pattern = unsafe { $crate::TsNeverType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::TsNonNullAssertionAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::TsNonNullAssertionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NON_PRIMITIVE_TYPE => {
                    let $pattern = unsafe { $crate::TsNonPrimitiveType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NULL_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::TsNullLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NUMBER_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::TsNumberLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_NUMBER_TYPE => {
                    let $pattern = unsafe { $crate::TsNumberType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OBJECT_TYPE => {
                    let $pattern = unsafe { $crate::TsObjectType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION => {
                    let $pattern =
                        unsafe { $crate::TsOptionalPropertyAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => {
                    let $pattern =
                        unsafe { $crate::TsOptionalTupleTypeElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OUT_MODIFIER => {
                    let $pattern = unsafe { $crate::TsOutModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_OVERRIDE_MODIFIER => {
                    let $pattern = unsafe { $crate::TsOverrideModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PARENTHESIZED_TYPE => {
                    let $pattern = unsafe { $crate::TsParenthesizedType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PREDICATE_RETURN_TYPE => {
                    let $pattern = unsafe { $crate::TsPredicateReturnType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_PARAMETER => {
                    let $pattern = unsafe { $crate::TsPropertyParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsPropertySignatureClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsPropertySignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_QUALIFIED_MODULE_NAME => {
                    let $pattern = unsafe { $crate::TsQualifiedModuleName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_QUALIFIED_NAME => {
                    let $pattern = unsafe { $crate::TsQualifiedName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_READONLY_MODIFIER => {
                    let $pattern = unsafe { $crate::TsReadonlyModifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_REFERENCE_TYPE => {
                    let $pattern = unsafe { $crate::TsReferenceType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => {
                    let $pattern = unsafe { $crate::TsRestTupleTypeElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => {
                    let $pattern = unsafe { $crate::TsReturnTypeAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SATISFIES_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::TsSatisfiesAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SATISFIES_EXPRESSION => {
                    let $pattern = unsafe { $crate::TsSatisfiesExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsSetterSignatureClassMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                    let $pattern =
                        unsafe { $crate::TsSetterSignatureTypeMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_STRING_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::TsStringLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_STRING_TYPE => {
                    let $pattern = unsafe { $crate::TsStringType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_SYMBOL_TYPE => {
                    let $pattern = unsafe { $crate::TsSymbolType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT => {
                    let $pattern = unsafe { $crate::TsTemplateChunkElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_ELEMENT => {
                    let $pattern = unsafe { $crate::TsTemplateElement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE => {
                    let $pattern = unsafe { $crate::TsTemplateLiteralType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_THIS_PARAMETER => {
                    let $pattern = unsafe { $crate::TsThisParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_THIS_TYPE => {
                    let $pattern = unsafe { $crate::TsThisType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TUPLE_TYPE => {
                    let $pattern = unsafe { $crate::TsTupleType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
                    let $pattern = unsafe { $crate::TsTypeAliasDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ANNOTATION => {
                    let $pattern = unsafe { $crate::TsTypeAnnotation::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { $crate::TsTypeArguments::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT => {
                    let $pattern =
                        unsafe { $crate::TsTypeAssertionAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::TsTypeAssertionExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE => {
                    let $pattern = unsafe { $crate::TsTypeConstraintClause::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_OPERATOR_TYPE => {
                    let $pattern = unsafe { $crate::TsTypeOperatorType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER => {
                    let $pattern = unsafe { $crate::TsTypeParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
                    let $pattern = unsafe { $crate::TsTypeParameterName::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETERS => {
                    let $pattern = unsafe { $crate::TsTypeParameters::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPEOF_TYPE => {
                    let $pattern = unsafe { $crate::TsTypeofType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNDEFINED_TYPE => {
                    let $pattern = unsafe { $crate::TsUndefinedType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNION_TYPE => {
                    let $pattern = unsafe { $crate::TsUnionType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNKNOWN_TYPE => {
                    let $pattern = unsafe { $crate::TsUnknownType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_VOID_TYPE => {
                    let $pattern = unsafe { $crate::TsVoidType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS => {
                    let $pattern = unsafe { $crate::JsBogus::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::JsBogusAssignment::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_BINDING => {
                    let $pattern = unsafe { $crate::JsBogusBinding::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::JsBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_IMPORT_ASSERTION_ENTRY => {
                    let $pattern =
                        unsafe { $crate::JsBogusImportAssertionEntry::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::JsBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_NAMED_IMPORT_SPECIFIER => {
                    let $pattern =
                        unsafe { $crate::JsBogusNamedImportSpecifier::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::JsBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::JsBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_BOGUS_TYPE => {
                    let $pattern = unsafe { $crate::TsBogusType::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::JsArrayAssignmentPatternElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::JsArrayBindingPatternElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_ARRAY_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::JsArrayElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CALL_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::JsCallArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CLASS_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::JsClassMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::JsConstructorModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { $crate::JsConstructorParameterList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DECORATOR_LIST => {
                    let $pattern = unsafe { $crate::JsDecoratorList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_DIRECTIVE_LIST => {
                    let $pattern = unsafe { $crate::JsDirectiveList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::JsExportNamedFromSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::JsExportNamedSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY_LIST => {
                    let $pattern =
                        unsafe { $crate::JsImportAssertionEntryList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_METHOD_MODIFIER_LIST => {
                    let $pattern = unsafe { $crate::JsMethodModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_MODULE_ITEM_LIST => {
                    let $pattern = unsafe { $crate::JsModuleItemList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::JsNamedImportSpecifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST => {
                    let $pattern = unsafe {
                        $crate::JsObjectAssignmentPatternPropertyList::new_unchecked(node)
                    };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST => {
                    let $pattern =
                        unsafe { $crate::JsObjectBindingPatternPropertyList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_OBJECT_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::JsObjectMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::JsParameterList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_PROPERTY_MODIFIER_LIST => {
                    let $pattern = unsafe { $crate::JsPropertyModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::JsStatementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_SWITCH_CASE_LIST => {
                    let $pattern = unsafe { $crate::JsSwitchCaseList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_TEMPLATE_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::JsTemplateElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JS_VARIABLE_DECLARATOR_LIST => {
                    let $pattern = unsafe { $crate::JsVariableDeclaratorList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::JsxAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::JSX_CHILD_LIST => {
                    let $pattern = unsafe { $crate::JsxChildList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_ENUM_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::TsEnumMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INDEX_SIGNATURE_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::TsIndexSignatureModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
                    let $pattern =
                        unsafe { $crate::TsIntersectionTypeElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_METHOD_SIGNATURE_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::TsMethodSignatureModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_PARAMETER_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::TsPropertyParameterModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_PROPERTY_SIGNATURE_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::TsPropertySignatureModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TEMPLATE_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::TsTemplateElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TUPLE_TYPE_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::TsTupleTypeElementList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::TsTypeArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_LIST => {
                    let $pattern = unsafe { $crate::TsTypeList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_MEMBER_LIST => {
                    let $pattern = unsafe { $crate::TsTypeMemberList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER_LIST => {
                    let $pattern = unsafe { $crate::TsTypeParameterList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_TYPE_PARAMETER_MODIFIER_LIST => {
                    let $pattern =
                        unsafe { $crate::TsTypeParameterModifierList::new_unchecked(node) };
                    $body
                }
                $crate::JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
                    let $pattern = unsafe { $crate::TsUnionTypeVariantList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
