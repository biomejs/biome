//! Generated file, do not edit by hand, see `xtask/codegen`

//! Maps GritQL pattern names to Biome's internal syntax kinds.
use biome_js_syntax::JsSyntaxKind;
use biome_js_syntax::JsSyntaxKind::*;
use biome_rowan::AstNode;

/// A legacy TreeSitter pattern for backward compatibility.
pub struct LegacyTreeSitterPattern {
    pub name: &'static str,
    pub kind: JsSyntaxKind,
    pub slots: &'static [(&'static str, u32)],
}

/// A list of legacy TreeSitter patterns for compatibility.
pub const LEGACY_TREESITTER_COMPATIBILITY_PATTERNS: &[LegacyTreeSitterPattern] = &[
    LegacyTreeSitterPattern {
        name: "identifier",
        kind: JS_REFERENCE_IDENTIFIER,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "string",
        kind: JS_STRING_LITERAL_EXPRESSION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "number",
        kind: JS_NUMBER_LITERAL_EXPRESSION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "property_identifier",
        kind: JS_REFERENCE_IDENTIFIER,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "call_expression",
        kind: JS_CALL_EXPRESSION,
        slots: &[("function", 0), ("arguments", 3)],
    },
    LegacyTreeSitterPattern {
        name: "member_expression",
        kind: JS_STATIC_MEMBER_EXPRESSION,
        slots: &[("object", 0), ("property", 2)],
    },
    LegacyTreeSitterPattern {
        name: "binary_expression",
        kind: JS_BINARY_EXPRESSION,
        slots: &[("left", 0), ("right", 2)],
    },
    LegacyTreeSitterPattern {
        name: "assignment_expression",
        kind: JS_ASSIGNMENT_EXPRESSION,
        slots: &[("left", 0), ("right", 2)],
    },
    LegacyTreeSitterPattern {
        name: "conditional_expression",
        kind: JS_CONDITIONAL_EXPRESSION,
        slots: &[("condition", 0), ("consequence", 2), ("alternative", 4)],
    },
    LegacyTreeSitterPattern {
        name: "arrow_function",
        kind: JS_ARROW_FUNCTION_EXPRESSION,
        slots: &[("body", 5)],
    },
    LegacyTreeSitterPattern {
        name: "object",
        kind: JS_OBJECT_EXPRESSION,
        slots: &[("properties", 1)],
    },
    LegacyTreeSitterPattern {
        name: "array",
        kind: JS_ARRAY_EXPRESSION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "pair",
        kind: JS_PROPERTY_OBJECT_MEMBER,
        slots: &[("key", 0), ("value", 2)],
    },
    LegacyTreeSitterPattern {
        name: "if_statement",
        kind: JS_IF_STATEMENT,
        slots: &[("condition", 2), ("consequence", 4)],
    },
    LegacyTreeSitterPattern {
        name: "for_statement",
        kind: JS_FOR_STATEMENT,
        slots: &[("initializer", 2), ("condition", 4), ("body", 8)],
    },
    LegacyTreeSitterPattern {
        name: "while_statement",
        kind: JS_WHILE_STATEMENT,
        slots: &[("condition", 2), ("body", 4)],
    },
    LegacyTreeSitterPattern {
        name: "function_declaration",
        kind: JS_FUNCTION_DECLARATION,
        slots: &[("name", 2), ("body", 7)],
    },
    LegacyTreeSitterPattern {
        name: "return_statement",
        kind: JS_RETURN_STATEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "variable_declaration",
        kind: JS_VARIABLE_DECLARATION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "expression_statement",
        kind: JS_EXPRESSION_STATEMENT,
        slots: &[],
    },
];

/// Returns the snake_case name for a syntax kind if it's part of the legacy set.
pub fn legacy_treesitter_name_for_kind(kind: JsSyntaxKind) -> Option<&'static str> {
    LEGACY_TREESITTER_COMPATIBILITY_PATTERNS
        .iter()
        .find(|p| p.kind == kind)
        .map(|p| p.name)
}

/// Returns the slot mappings for a syntax kind if it's part of the legacy set.
pub fn legacy_treesitter_slots_for_kind(kind: JsSyntaxKind) -> &'static [(&'static str, u32)] {
    LEGACY_TREESITTER_COMPATIBILITY_PATTERNS
        .iter()
        .find(|p| p.kind == kind)
        .map_or(&[], |p| p.slots)
}

/// Returns the syntax kind for a legacy or native node name.
pub fn kind_by_name(node_name: &str) -> Option<JsSyntaxKind> {
    use JsSyntaxKind::*;
    use biome_js_syntax::*;
    match node_name {
        // Legacy TreeSitter patterns
        "identifier" => Some(JS_REFERENCE_IDENTIFIER),
        "string" => Some(JS_STRING_LITERAL_EXPRESSION),
        "number" => Some(JS_NUMBER_LITERAL_EXPRESSION),
        "property_identifier" => Some(JS_REFERENCE_IDENTIFIER),
        "call_expression" => Some(JS_CALL_EXPRESSION),
        "member_expression" => Some(JS_STATIC_MEMBER_EXPRESSION),
        "binary_expression" => Some(JS_BINARY_EXPRESSION),
        "assignment_expression" => Some(JS_ASSIGNMENT_EXPRESSION),
        "conditional_expression" => Some(JS_CONDITIONAL_EXPRESSION),
        "arrow_function" => Some(JS_ARROW_FUNCTION_EXPRESSION),
        "object" => Some(JS_OBJECT_EXPRESSION),
        "array" => Some(JS_ARRAY_EXPRESSION),
        "pair" => Some(JS_PROPERTY_OBJECT_MEMBER),
        "if_statement" => Some(JS_IF_STATEMENT),
        "for_statement" => Some(JS_FOR_STATEMENT),
        "while_statement" => Some(JS_WHILE_STATEMENT),
        "function_declaration" => Some(JS_FUNCTION_DECLARATION),
        "return_statement" => Some(JS_RETURN_STATEMENT),
        "variable_declaration" => Some(JS_VARIABLE_DECLARATION),
        "expression_statement" => Some(JS_EXPRESSION_STATEMENT),

        // Native Biome AST patterns
        "JsAccessorModifier" => JsAccessorModifier::KIND_SET.iter().next(),
        "JsArrayAssignmentPattern" => JsArrayAssignmentPattern::KIND_SET.iter().next(),
        "JsArrayAssignmentPatternElement" => {
            JsArrayAssignmentPatternElement::KIND_SET.iter().next()
        }
        "JsArrayAssignmentPatternRestElement" => {
            JsArrayAssignmentPatternRestElement::KIND_SET.iter().next()
        }
        "JsArrayBindingPattern" => JsArrayBindingPattern::KIND_SET.iter().next(),
        "JsArrayBindingPatternElement" => JsArrayBindingPatternElement::KIND_SET.iter().next(),
        "JsArrayBindingPatternRestElement" => {
            JsArrayBindingPatternRestElement::KIND_SET.iter().next()
        }
        "JsArrayExpression" => JsArrayExpression::KIND_SET.iter().next(),
        "JsArrayHole" => JsArrayHole::KIND_SET.iter().next(),
        "JsArrowFunctionExpression" => JsArrowFunctionExpression::KIND_SET.iter().next(),
        "JsAssignmentExpression" => JsAssignmentExpression::KIND_SET.iter().next(),
        "JsAwaitExpression" => JsAwaitExpression::KIND_SET.iter().next(),
        "JsBigintLiteralExpression" => JsBigintLiteralExpression::KIND_SET.iter().next(),
        "JsBinaryExpression" => JsBinaryExpression::KIND_SET.iter().next(),
        "JsBlockStatement" => JsBlockStatement::KIND_SET.iter().next(),
        "JsBooleanLiteralExpression" => JsBooleanLiteralExpression::KIND_SET.iter().next(),
        "JsBreakStatement" => JsBreakStatement::KIND_SET.iter().next(),
        "JsCallArguments" => JsCallArguments::KIND_SET.iter().next(),
        "JsCallExpression" => JsCallExpression::KIND_SET.iter().next(),
        "JsCaseClause" => JsCaseClause::KIND_SET.iter().next(),
        "JsCatchClause" => JsCatchClause::KIND_SET.iter().next(),
        "JsCatchDeclaration" => JsCatchDeclaration::KIND_SET.iter().next(),
        "JsClassDeclaration" => JsClassDeclaration::KIND_SET.iter().next(),
        "JsClassExportDefaultDeclaration" => {
            JsClassExportDefaultDeclaration::KIND_SET.iter().next()
        }
        "JsClassExpression" => JsClassExpression::KIND_SET.iter().next(),
        "JsComputedMemberAssignment" => JsComputedMemberAssignment::KIND_SET.iter().next(),
        "JsComputedMemberExpression" => JsComputedMemberExpression::KIND_SET.iter().next(),
        "JsComputedMemberName" => JsComputedMemberName::KIND_SET.iter().next(),
        "JsConditionalExpression" => JsConditionalExpression::KIND_SET.iter().next(),
        "JsConstructorClassMember" => JsConstructorClassMember::KIND_SET.iter().next(),
        "JsConstructorParameters" => JsConstructorParameters::KIND_SET.iter().next(),
        "JsContinueStatement" => JsContinueStatement::KIND_SET.iter().next(),
        "JsDebuggerStatement" => JsDebuggerStatement::KIND_SET.iter().next(),
        "JsDecorator" => JsDecorator::KIND_SET.iter().next(),
        "JsDefaultClause" => JsDefaultClause::KIND_SET.iter().next(),
        "JsDefaultImportSpecifier" => JsDefaultImportSpecifier::KIND_SET.iter().next(),
        "JsDirective" => JsDirective::KIND_SET.iter().next(),
        "JsDoWhileStatement" => JsDoWhileStatement::KIND_SET.iter().next(),
        "JsElseClause" => JsElseClause::KIND_SET.iter().next(),
        "JsEmptyClassMember" => JsEmptyClassMember::KIND_SET.iter().next(),
        "JsEmptyStatement" => JsEmptyStatement::KIND_SET.iter().next(),
        "JsExport" => JsExport::KIND_SET.iter().next(),
        "JsExportAsClause" => JsExportAsClause::KIND_SET.iter().next(),
        "JsExportDefaultDeclarationClause" => {
            JsExportDefaultDeclarationClause::KIND_SET.iter().next()
        }
        "JsExportDefaultExpressionClause" => {
            JsExportDefaultExpressionClause::KIND_SET.iter().next()
        }
        "JsExportFromClause" => JsExportFromClause::KIND_SET.iter().next(),
        "JsExportNamedClause" => JsExportNamedClause::KIND_SET.iter().next(),
        "JsExportNamedFromClause" => JsExportNamedFromClause::KIND_SET.iter().next(),
        "JsExportNamedFromSpecifier" => JsExportNamedFromSpecifier::KIND_SET.iter().next(),
        "JsExportNamedShorthandSpecifier" => {
            JsExportNamedShorthandSpecifier::KIND_SET.iter().next()
        }
        "JsExportNamedSpecifier" => JsExportNamedSpecifier::KIND_SET.iter().next(),
        "JsExpressionSnipped" => JsExpressionSnipped::KIND_SET.iter().next(),
        "JsExpressionStatement" => JsExpressionStatement::KIND_SET.iter().next(),
        "JsExtendsClause" => JsExtendsClause::KIND_SET.iter().next(),
        "JsFinallyClause" => JsFinallyClause::KIND_SET.iter().next(),
        "JsForInStatement" => JsForInStatement::KIND_SET.iter().next(),
        "JsForOfStatement" => JsForOfStatement::KIND_SET.iter().next(),
        "JsForStatement" => JsForStatement::KIND_SET.iter().next(),
        "JsForVariableDeclaration" => JsForVariableDeclaration::KIND_SET.iter().next(),
        "JsFormalParameter" => JsFormalParameter::KIND_SET.iter().next(),
        "JsFunctionBody" => JsFunctionBody::KIND_SET.iter().next(),
        "JsFunctionDeclaration" => JsFunctionDeclaration::KIND_SET.iter().next(),
        "JsFunctionExportDefaultDeclaration" => {
            JsFunctionExportDefaultDeclaration::KIND_SET.iter().next()
        }
        "JsFunctionExpression" => JsFunctionExpression::KIND_SET.iter().next(),
        "JsGetterClassMember" => JsGetterClassMember::KIND_SET.iter().next(),
        "JsGetterObjectMember" => JsGetterObjectMember::KIND_SET.iter().next(),
        "JsIdentifierAssignment" => JsIdentifierAssignment::KIND_SET.iter().next(),
        "JsIdentifierBinding" => JsIdentifierBinding::KIND_SET.iter().next(),
        "JsIdentifierExpression" => JsIdentifierExpression::KIND_SET.iter().next(),
        "JsIfStatement" => JsIfStatement::KIND_SET.iter().next(),
        "JsImport" => JsImport::KIND_SET.iter().next(),
        "JsImportAssertion" => JsImportAssertion::KIND_SET.iter().next(),
        "JsImportAssertionEntry" => JsImportAssertionEntry::KIND_SET.iter().next(),
        "JsImportBareClause" => JsImportBareClause::KIND_SET.iter().next(),
        "JsImportCallExpression" => JsImportCallExpression::KIND_SET.iter().next(),
        "JsImportCombinedClause" => JsImportCombinedClause::KIND_SET.iter().next(),
        "JsImportDefaultClause" => JsImportDefaultClause::KIND_SET.iter().next(),
        "JsImportMetaExpression" => JsImportMetaExpression::KIND_SET.iter().next(),
        "JsImportNamedClause" => JsImportNamedClause::KIND_SET.iter().next(),
        "JsImportNamespaceClause" => JsImportNamespaceClause::KIND_SET.iter().next(),
        "JsInExpression" => JsInExpression::KIND_SET.iter().next(),
        "JsInitializerClause" => JsInitializerClause::KIND_SET.iter().next(),
        "JsInstanceofExpression" => JsInstanceofExpression::KIND_SET.iter().next(),
        "JsLabel" => JsLabel::KIND_SET.iter().next(),
        "JsLabeledStatement" => JsLabeledStatement::KIND_SET.iter().next(),
        "JsLiteralExportName" => JsLiteralExportName::KIND_SET.iter().next(),
        "JsLiteralMemberName" => JsLiteralMemberName::KIND_SET.iter().next(),
        "JsLogicalExpression" => JsLogicalExpression::KIND_SET.iter().next(),
        "JsMetavariable" => JsMetavariable::KIND_SET.iter().next(),
        "JsMethodClassMember" => JsMethodClassMember::KIND_SET.iter().next(),
        "JsMethodObjectMember" => JsMethodObjectMember::KIND_SET.iter().next(),
        "JsModule" => JsModule::KIND_SET.iter().next(),
        "JsModuleSource" => JsModuleSource::KIND_SET.iter().next(),
        "JsName" => JsName::KIND_SET.iter().next(),
        "JsNamedImportSpecifier" => JsNamedImportSpecifier::KIND_SET.iter().next(),
        "JsNamedImportSpecifiers" => JsNamedImportSpecifiers::KIND_SET.iter().next(),
        "JsNamespaceImportSpecifier" => JsNamespaceImportSpecifier::KIND_SET.iter().next(),
        "JsNewExpression" => JsNewExpression::KIND_SET.iter().next(),
        "JsNewTargetExpression" => JsNewTargetExpression::KIND_SET.iter().next(),
        "JsNullLiteralExpression" => JsNullLiteralExpression::KIND_SET.iter().next(),
        "JsNumberLiteralExpression" => JsNumberLiteralExpression::KIND_SET.iter().next(),
        "JsObjectAssignmentPattern" => JsObjectAssignmentPattern::KIND_SET.iter().next(),
        "JsObjectAssignmentPatternProperty" => {
            JsObjectAssignmentPatternProperty::KIND_SET.iter().next()
        }
        "JsObjectAssignmentPatternRest" => JsObjectAssignmentPatternRest::KIND_SET.iter().next(),
        "JsObjectAssignmentPatternShorthandProperty" => {
            JsObjectAssignmentPatternShorthandProperty::KIND_SET
                .iter()
                .next()
        }
        "JsObjectBindingPattern" => JsObjectBindingPattern::KIND_SET.iter().next(),
        "JsObjectBindingPatternProperty" => JsObjectBindingPatternProperty::KIND_SET.iter().next(),
        "JsObjectBindingPatternRest" => JsObjectBindingPatternRest::KIND_SET.iter().next(),
        "JsObjectBindingPatternShorthandProperty" => {
            JsObjectBindingPatternShorthandProperty::KIND_SET
                .iter()
                .next()
        }
        "JsObjectExpression" => JsObjectExpression::KIND_SET.iter().next(),
        "JsParameters" => JsParameters::KIND_SET.iter().next(),
        "JsParenthesizedAssignment" => JsParenthesizedAssignment::KIND_SET.iter().next(),
        "JsParenthesizedExpression" => JsParenthesizedExpression::KIND_SET.iter().next(),
        "JsPostUpdateExpression" => JsPostUpdateExpression::KIND_SET.iter().next(),
        "JsPreUpdateExpression" => JsPreUpdateExpression::KIND_SET.iter().next(),
        "JsPrivateClassMemberName" => JsPrivateClassMemberName::KIND_SET.iter().next(),
        "JsPrivateName" => JsPrivateName::KIND_SET.iter().next(),
        "JsPropertyClassMember" => JsPropertyClassMember::KIND_SET.iter().next(),
        "JsPropertyObjectMember" => JsPropertyObjectMember::KIND_SET.iter().next(),
        "JsReferenceIdentifier" => JsReferenceIdentifier::KIND_SET.iter().next(),
        "JsRegexLiteralExpression" => JsRegexLiteralExpression::KIND_SET.iter().next(),
        "JsRestParameter" => JsRestParameter::KIND_SET.iter().next(),
        "JsReturnStatement" => JsReturnStatement::KIND_SET.iter().next(),
        "JsScript" => JsScript::KIND_SET.iter().next(),
        "JsSequenceExpression" => JsSequenceExpression::KIND_SET.iter().next(),
        "JsSetterClassMember" => JsSetterClassMember::KIND_SET.iter().next(),
        "JsSetterObjectMember" => JsSetterObjectMember::KIND_SET.iter().next(),
        "JsShorthandNamedImportSpecifier" => {
            JsShorthandNamedImportSpecifier::KIND_SET.iter().next()
        }
        "JsShorthandPropertyObjectMember" => {
            JsShorthandPropertyObjectMember::KIND_SET.iter().next()
        }
        "JsSpread" => JsSpread::KIND_SET.iter().next(),
        "JsStaticInitializationBlockClassMember" => {
            JsStaticInitializationBlockClassMember::KIND_SET
                .iter()
                .next()
        }
        "JsStaticMemberAssignment" => JsStaticMemberAssignment::KIND_SET.iter().next(),
        "JsStaticMemberExpression" => JsStaticMemberExpression::KIND_SET.iter().next(),
        "JsStaticModifier" => JsStaticModifier::KIND_SET.iter().next(),
        "JsStringLiteralExpression" => JsStringLiteralExpression::KIND_SET.iter().next(),
        "JsSuperExpression" => JsSuperExpression::KIND_SET.iter().next(),
        "JsSwitchStatement" => JsSwitchStatement::KIND_SET.iter().next(),
        "JsTemplateChunkElement" => JsTemplateChunkElement::KIND_SET.iter().next(),
        "JsTemplateElement" => JsTemplateElement::KIND_SET.iter().next(),
        "JsTemplateExpression" => JsTemplateExpression::KIND_SET.iter().next(),
        "JsThisExpression" => JsThisExpression::KIND_SET.iter().next(),
        "JsThrowStatement" => JsThrowStatement::KIND_SET.iter().next(),
        "JsTryFinallyStatement" => JsTryFinallyStatement::KIND_SET.iter().next(),
        "JsTryStatement" => JsTryStatement::KIND_SET.iter().next(),
        "JsUnaryExpression" => JsUnaryExpression::KIND_SET.iter().next(),
        "JsVariableDeclaration" => JsVariableDeclaration::KIND_SET.iter().next(),
        "JsVariableDeclarationClause" => JsVariableDeclarationClause::KIND_SET.iter().next(),
        "JsVariableDeclarator" => JsVariableDeclarator::KIND_SET.iter().next(),
        "JsVariableStatement" => JsVariableStatement::KIND_SET.iter().next(),
        "JsWhileStatement" => JsWhileStatement::KIND_SET.iter().next(),
        "JsWithStatement" => JsWithStatement::KIND_SET.iter().next(),
        "JsYieldArgument" => JsYieldArgument::KIND_SET.iter().next(),
        "JsYieldExpression" => JsYieldExpression::KIND_SET.iter().next(),
        "JsxAttribute" => JsxAttribute::KIND_SET.iter().next(),
        "JsxAttributeInitializerClause" => JsxAttributeInitializerClause::KIND_SET.iter().next(),
        "JsxClosingElement" => JsxClosingElement::KIND_SET.iter().next(),
        "JsxClosingFragment" => JsxClosingFragment::KIND_SET.iter().next(),
        "JsxElement" => JsxElement::KIND_SET.iter().next(),
        "JsxExpressionAttributeValue" => JsxExpressionAttributeValue::KIND_SET.iter().next(),
        "JsxExpressionChild" => JsxExpressionChild::KIND_SET.iter().next(),
        "JsxFragment" => JsxFragment::KIND_SET.iter().next(),
        "JsxMemberName" => JsxMemberName::KIND_SET.iter().next(),
        "JsxName" => JsxName::KIND_SET.iter().next(),
        "JsxNamespaceName" => JsxNamespaceName::KIND_SET.iter().next(),
        "JsxOpeningElement" => JsxOpeningElement::KIND_SET.iter().next(),
        "JsxOpeningFragment" => JsxOpeningFragment::KIND_SET.iter().next(),
        "JsxReferenceIdentifier" => JsxReferenceIdentifier::KIND_SET.iter().next(),
        "JsxSelfClosingElement" => JsxSelfClosingElement::KIND_SET.iter().next(),
        "JsxSpreadAttribute" => JsxSpreadAttribute::KIND_SET.iter().next(),
        "JsxSpreadChild" => JsxSpreadChild::KIND_SET.iter().next(),
        "JsxString" => JsxString::KIND_SET.iter().next(),
        "JsxTagExpression" => JsxTagExpression::KIND_SET.iter().next(),
        "JsxText" => JsxText::KIND_SET.iter().next(),
        "TsAbstractModifier" => TsAbstractModifier::KIND_SET.iter().next(),
        "TsAccessibilityModifier" => TsAccessibilityModifier::KIND_SET.iter().next(),
        "TsAnyType" => TsAnyType::KIND_SET.iter().next(),
        "TsArrayType" => TsArrayType::KIND_SET.iter().next(),
        "TsAsAssignment" => TsAsAssignment::KIND_SET.iter().next(),
        "TsAsExpression" => TsAsExpression::KIND_SET.iter().next(),
        "TsAssertsCondition" => TsAssertsCondition::KIND_SET.iter().next(),
        "TsAssertsReturnType" => TsAssertsReturnType::KIND_SET.iter().next(),
        "TsBigintLiteralType" => TsBigintLiteralType::KIND_SET.iter().next(),
        "TsBigintType" => TsBigintType::KIND_SET.iter().next(),
        "TsBooleanLiteralType" => TsBooleanLiteralType::KIND_SET.iter().next(),
        "TsBooleanType" => TsBooleanType::KIND_SET.iter().next(),
        "TsCallSignatureTypeMember" => TsCallSignatureTypeMember::KIND_SET.iter().next(),
        "TsConditionalType" => TsConditionalType::KIND_SET.iter().next(),
        "TsConstModifier" => TsConstModifier::KIND_SET.iter().next(),
        "TsConstructSignatureTypeMember" => TsConstructSignatureTypeMember::KIND_SET.iter().next(),
        "TsConstructorSignatureClassMember" => {
            TsConstructorSignatureClassMember::KIND_SET.iter().next()
        }
        "TsConstructorType" => TsConstructorType::KIND_SET.iter().next(),
        "TsDeclarationModule" => TsDeclarationModule::KIND_SET.iter().next(),
        "TsDeclareFunctionDeclaration" => TsDeclareFunctionDeclaration::KIND_SET.iter().next(),
        "TsDeclareFunctionExportDefaultDeclaration" => {
            TsDeclareFunctionExportDefaultDeclaration::KIND_SET
                .iter()
                .next()
        }
        "TsDeclareModifier" => TsDeclareModifier::KIND_SET.iter().next(),
        "TsDeclareStatement" => TsDeclareStatement::KIND_SET.iter().next(),
        "TsDefaultTypeClause" => TsDefaultTypeClause::KIND_SET.iter().next(),
        "TsDefinitePropertyAnnotation" => TsDefinitePropertyAnnotation::KIND_SET.iter().next(),
        "TsDefiniteVariableAnnotation" => TsDefiniteVariableAnnotation::KIND_SET.iter().next(),
        "TsEmptyExternalModuleDeclarationBody" => {
            TsEmptyExternalModuleDeclarationBody::KIND_SET.iter().next()
        }
        "TsEnumDeclaration" => TsEnumDeclaration::KIND_SET.iter().next(),
        "TsEnumMember" => TsEnumMember::KIND_SET.iter().next(),
        "TsExportAsNamespaceClause" => TsExportAsNamespaceClause::KIND_SET.iter().next(),
        "TsExportAssignmentClause" => TsExportAssignmentClause::KIND_SET.iter().next(),
        "TsExportDeclareClause" => TsExportDeclareClause::KIND_SET.iter().next(),
        "TsExtendsClause" => TsExtendsClause::KIND_SET.iter().next(),
        "TsExternalModuleDeclaration" => TsExternalModuleDeclaration::KIND_SET.iter().next(),
        "TsExternalModuleReference" => TsExternalModuleReference::KIND_SET.iter().next(),
        "TsFunctionType" => TsFunctionType::KIND_SET.iter().next(),
        "TsGetterSignatureClassMember" => TsGetterSignatureClassMember::KIND_SET.iter().next(),
        "TsGetterSignatureTypeMember" => TsGetterSignatureTypeMember::KIND_SET.iter().next(),
        "TsGlobalDeclaration" => TsGlobalDeclaration::KIND_SET.iter().next(),
        "TsIdentifierBinding" => TsIdentifierBinding::KIND_SET.iter().next(),
        "TsImplementsClause" => TsImplementsClause::KIND_SET.iter().next(),
        "TsImportEqualsDeclaration" => TsImportEqualsDeclaration::KIND_SET.iter().next(),
        "TsImportType" => TsImportType::KIND_SET.iter().next(),
        "TsImportTypeArguments" => TsImportTypeArguments::KIND_SET.iter().next(),
        "TsImportTypeAssertion" => TsImportTypeAssertion::KIND_SET.iter().next(),
        "TsImportTypeAssertionBlock" => TsImportTypeAssertionBlock::KIND_SET.iter().next(),
        "TsImportTypeQualifier" => TsImportTypeQualifier::KIND_SET.iter().next(),
        "TsInModifier" => TsInModifier::KIND_SET.iter().next(),
        "TsIndexSignatureClassMember" => TsIndexSignatureClassMember::KIND_SET.iter().next(),
        "TsIndexSignatureParameter" => TsIndexSignatureParameter::KIND_SET.iter().next(),
        "TsIndexSignatureTypeMember" => TsIndexSignatureTypeMember::KIND_SET.iter().next(),
        "TsIndexedAccessType" => TsIndexedAccessType::KIND_SET.iter().next(),
        "TsInferType" => TsInferType::KIND_SET.iter().next(),
        "TsInitializedPropertySignatureClassMember" => {
            TsInitializedPropertySignatureClassMember::KIND_SET
                .iter()
                .next()
        }
        "TsInstantiationExpression" => TsInstantiationExpression::KIND_SET.iter().next(),
        "TsInterfaceDeclaration" => TsInterfaceDeclaration::KIND_SET.iter().next(),
        "TsIntersectionType" => TsIntersectionType::KIND_SET.iter().next(),
        "TsLiteralEnumMemberName" => TsLiteralEnumMemberName::KIND_SET.iter().next(),
        "TsMappedType" => TsMappedType::KIND_SET.iter().next(),
        "TsMappedTypeAsClause" => TsMappedTypeAsClause::KIND_SET.iter().next(),
        "TsMappedTypeOptionalModifierClause" => {
            TsMappedTypeOptionalModifierClause::KIND_SET.iter().next()
        }
        "TsMappedTypeReadonlyModifierClause" => {
            TsMappedTypeReadonlyModifierClause::KIND_SET.iter().next()
        }
        "TsMethodSignatureClassMember" => TsMethodSignatureClassMember::KIND_SET.iter().next(),
        "TsMethodSignatureTypeMember" => TsMethodSignatureTypeMember::KIND_SET.iter().next(),
        "TsModuleBlock" => TsModuleBlock::KIND_SET.iter().next(),
        "TsModuleDeclaration" => TsModuleDeclaration::KIND_SET.iter().next(),
        "TsNamedTupleTypeElement" => TsNamedTupleTypeElement::KIND_SET.iter().next(),
        "TsNeverType" => TsNeverType::KIND_SET.iter().next(),
        "TsNonNullAssertionAssignment" => TsNonNullAssertionAssignment::KIND_SET.iter().next(),
        "TsNonNullAssertionExpression" => TsNonNullAssertionExpression::KIND_SET.iter().next(),
        "TsNonPrimitiveType" => TsNonPrimitiveType::KIND_SET.iter().next(),
        "TsNullLiteralType" => TsNullLiteralType::KIND_SET.iter().next(),
        "TsNumberLiteralType" => TsNumberLiteralType::KIND_SET.iter().next(),
        "TsNumberType" => TsNumberType::KIND_SET.iter().next(),
        "TsObjectType" => TsObjectType::KIND_SET.iter().next(),
        "TsOptionalPropertyAnnotation" => TsOptionalPropertyAnnotation::KIND_SET.iter().next(),
        "TsOptionalTupleTypeElement" => TsOptionalTupleTypeElement::KIND_SET.iter().next(),
        "TsOutModifier" => TsOutModifier::KIND_SET.iter().next(),
        "TsOverrideModifier" => TsOverrideModifier::KIND_SET.iter().next(),
        "TsParenthesizedType" => TsParenthesizedType::KIND_SET.iter().next(),
        "TsPredicateReturnType" => TsPredicateReturnType::KIND_SET.iter().next(),
        "TsPropertyParameter" => TsPropertyParameter::KIND_SET.iter().next(),
        "TsPropertySignatureClassMember" => TsPropertySignatureClassMember::KIND_SET.iter().next(),
        "TsPropertySignatureTypeMember" => TsPropertySignatureTypeMember::KIND_SET.iter().next(),
        "TsQualifiedModuleName" => TsQualifiedModuleName::KIND_SET.iter().next(),
        "TsQualifiedName" => TsQualifiedName::KIND_SET.iter().next(),
        "TsReadonlyModifier" => TsReadonlyModifier::KIND_SET.iter().next(),
        "TsReferenceType" => TsReferenceType::KIND_SET.iter().next(),
        "TsRestTupleTypeElement" => TsRestTupleTypeElement::KIND_SET.iter().next(),
        "TsReturnTypeAnnotation" => TsReturnTypeAnnotation::KIND_SET.iter().next(),
        "TsSatisfiesAssignment" => TsSatisfiesAssignment::KIND_SET.iter().next(),
        "TsSatisfiesExpression" => TsSatisfiesExpression::KIND_SET.iter().next(),
        "TsSetterSignatureClassMember" => TsSetterSignatureClassMember::KIND_SET.iter().next(),
        "TsSetterSignatureTypeMember" => TsSetterSignatureTypeMember::KIND_SET.iter().next(),
        "TsStringLiteralType" => TsStringLiteralType::KIND_SET.iter().next(),
        "TsStringType" => TsStringType::KIND_SET.iter().next(),
        "TsSymbolType" => TsSymbolType::KIND_SET.iter().next(),
        "TsTemplateChunkElement" => TsTemplateChunkElement::KIND_SET.iter().next(),
        "TsTemplateElement" => TsTemplateElement::KIND_SET.iter().next(),
        "TsTemplateLiteralType" => TsTemplateLiteralType::KIND_SET.iter().next(),
        "TsThisParameter" => TsThisParameter::KIND_SET.iter().next(),
        "TsThisType" => TsThisType::KIND_SET.iter().next(),
        "TsTupleType" => TsTupleType::KIND_SET.iter().next(),
        "TsTypeAliasDeclaration" => TsTypeAliasDeclaration::KIND_SET.iter().next(),
        "TsTypeAnnotation" => TsTypeAnnotation::KIND_SET.iter().next(),
        "TsTypeArguments" => TsTypeArguments::KIND_SET.iter().next(),
        "TsTypeAssertionAssignment" => TsTypeAssertionAssignment::KIND_SET.iter().next(),
        "TsTypeAssertionExpression" => TsTypeAssertionExpression::KIND_SET.iter().next(),
        "TsTypeConstraintClause" => TsTypeConstraintClause::KIND_SET.iter().next(),
        "TsTypeOperatorType" => TsTypeOperatorType::KIND_SET.iter().next(),
        "TsTypeParameter" => TsTypeParameter::KIND_SET.iter().next(),
        "TsTypeParameterName" => TsTypeParameterName::KIND_SET.iter().next(),
        "TsTypeParameters" => TsTypeParameters::KIND_SET.iter().next(),
        "TsTypeofType" => TsTypeofType::KIND_SET.iter().next(),
        "TsUndefinedType" => TsUndefinedType::KIND_SET.iter().next(),
        "TsUnionType" => TsUnionType::KIND_SET.iter().next(),
        "TsUnknownType" => TsUnknownType::KIND_SET.iter().next(),
        "TsVoidType" => TsVoidType::KIND_SET.iter().next(),
        _ => None,
    }
}
