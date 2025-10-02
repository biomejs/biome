//! Generated file, do not edit by hand, see `xtask/codegen`

//! Maps GritQL pattern names to Biome's internal syntax kinds.
use biome_js_syntax as lang;
use biome_rowan::AstNode;
use lang::JsSyntaxKind;

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
        kind: JsSyntaxKind::JS_REFERENCE_IDENTIFIER,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "string",
        kind: JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "number",
        kind: JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "property_identifier",
        kind: JsSyntaxKind::JS_LITERAL_MEMBER_NAME,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "call_expression",
        kind: JsSyntaxKind::JS_CALL_EXPRESSION,
        slots: &[("function", 0), ("arguments", 3)],
    },
    LegacyTreeSitterPattern {
        name: "member_expression",
        kind: JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION,
        slots: &[("object", 0), ("property", 2)],
    },
    LegacyTreeSitterPattern {
        name: "subscript_expression",
        kind: JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION,
        slots: &[("object", 0), ("index", 3)],
    },
    LegacyTreeSitterPattern {
        name: "binary_expression",
        kind: JsSyntaxKind::JS_BINARY_EXPRESSION,
        slots: &[("left", 0), ("right", 2)],
    },
    LegacyTreeSitterPattern {
        name: "assignment_expression",
        kind: JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION,
        slots: &[("left", 0), ("right", 2)],
    },
    LegacyTreeSitterPattern {
        name: "conditional_expression",
        kind: JsSyntaxKind::JS_CONDITIONAL_EXPRESSION,
        slots: &[("condition", 0), ("consequence", 2), ("alternative", 4)],
    },
    LegacyTreeSitterPattern {
        name: "arrow_function",
        kind: JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
        slots: &[("body", 5)],
    },
    LegacyTreeSitterPattern {
        name: "object",
        kind: JsSyntaxKind::JS_OBJECT_EXPRESSION,
        slots: &[("properties", 1)],
    },
    LegacyTreeSitterPattern {
        name: "array",
        kind: JsSyntaxKind::JS_ARRAY_EXPRESSION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "pair",
        kind: JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER,
        slots: &[("key", 0), ("value", 2)],
    },
    LegacyTreeSitterPattern {
        name: "if_statement",
        kind: JsSyntaxKind::JS_IF_STATEMENT,
        slots: &[("condition", 2), ("consequence", 4)],
    },
    LegacyTreeSitterPattern {
        name: "for_statement",
        kind: JsSyntaxKind::JS_FOR_STATEMENT,
        slots: &[("initializer", 2), ("condition", 4), ("body", 8)],
    },
    LegacyTreeSitterPattern {
        name: "while_statement",
        kind: JsSyntaxKind::JS_WHILE_STATEMENT,
        slots: &[("condition", 2), ("body", 4)],
    },
    LegacyTreeSitterPattern {
        name: "function_declaration",
        kind: JsSyntaxKind::JS_FUNCTION_DECLARATION,
        slots: &[("name", 2), ("body", 7)],
    },
    LegacyTreeSitterPattern {
        name: "return_statement",
        kind: JsSyntaxKind::JS_RETURN_STATEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "variable_declaration",
        kind: JsSyntaxKind::JS_VARIABLE_DECLARATION,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "expression_statement",
        kind: JsSyntaxKind::JS_EXPRESSION_STATEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_expression",
        kind: JsSyntaxKind::JSX_EXPRESSION_CHILD,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_attribute",
        kind: JsSyntaxKind::JSX_ATTRIBUTE,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_element",
        kind: JsSyntaxKind::JSX_ELEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_self_closing_element",
        kind: JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_opening_element",
        kind: JsSyntaxKind::JSX_OPENING_ELEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_closing_element",
        kind: JsSyntaxKind::JSX_CLOSING_ELEMENT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_text",
        kind: JsSyntaxKind::JSX_TEXT,
        slots: &[],
    },
    LegacyTreeSitterPattern {
        name: "jsx_namespace_name",
        kind: JsSyntaxKind::JSX_NAMESPACE_NAME,
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
    match node_name {
        // Legacy TreeSitter patterns
        "identifier" => Some(JsSyntaxKind::JS_REFERENCE_IDENTIFIER),
        "string" => Some(JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION),
        "number" => Some(JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION),
        "property_identifier" => Some(JsSyntaxKind::JS_LITERAL_MEMBER_NAME),
        "call_expression" => Some(JsSyntaxKind::JS_CALL_EXPRESSION),
        "member_expression" => Some(JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION),
        "subscript_expression" => Some(JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION),
        "binary_expression" => Some(JsSyntaxKind::JS_BINARY_EXPRESSION),
        "assignment_expression" => Some(JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION),
        "conditional_expression" => Some(JsSyntaxKind::JS_CONDITIONAL_EXPRESSION),
        "arrow_function" => Some(JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION),
        "object" => Some(JsSyntaxKind::JS_OBJECT_EXPRESSION),
        "array" => Some(JsSyntaxKind::JS_ARRAY_EXPRESSION),
        "pair" => Some(JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER),
        "if_statement" => Some(JsSyntaxKind::JS_IF_STATEMENT),
        "for_statement" => Some(JsSyntaxKind::JS_FOR_STATEMENT),
        "while_statement" => Some(JsSyntaxKind::JS_WHILE_STATEMENT),
        "function_declaration" => Some(JsSyntaxKind::JS_FUNCTION_DECLARATION),
        "return_statement" => Some(JsSyntaxKind::JS_RETURN_STATEMENT),
        "variable_declaration" => Some(JsSyntaxKind::JS_VARIABLE_DECLARATION),
        "expression_statement" => Some(JsSyntaxKind::JS_EXPRESSION_STATEMENT),
        "jsx_expression" => Some(JsSyntaxKind::JSX_EXPRESSION_CHILD),
        "jsx_attribute" => Some(JsSyntaxKind::JSX_ATTRIBUTE),
        "jsx_element" => Some(JsSyntaxKind::JSX_ELEMENT),
        "jsx_self_closing_element" => Some(JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT),
        "jsx_opening_element" => Some(JsSyntaxKind::JSX_OPENING_ELEMENT),
        "jsx_closing_element" => Some(JsSyntaxKind::JSX_CLOSING_ELEMENT),
        "jsx_text" => Some(JsSyntaxKind::JSX_TEXT),
        "jsx_namespace_name" => Some(JsSyntaxKind::JSX_NAMESPACE_NAME),

        // Native Biome AST patterns
        "JsAccessorModifier" => lang::JsAccessorModifier::KIND_SET.iter().next(),
        "JsArrayAssignmentPattern" => lang::JsArrayAssignmentPattern::KIND_SET.iter().next(),
        "JsArrayAssignmentPatternElement" => lang::JsArrayAssignmentPatternElement::KIND_SET
            .iter()
            .next(),
        "JsArrayAssignmentPatternRestElement" => {
            lang::JsArrayAssignmentPatternRestElement::KIND_SET
                .iter()
                .next()
        }
        "JsArrayBindingPattern" => lang::JsArrayBindingPattern::KIND_SET.iter().next(),
        "JsArrayBindingPatternElement" => {
            lang::JsArrayBindingPatternElement::KIND_SET.iter().next()
        }
        "JsArrayBindingPatternRestElement" => lang::JsArrayBindingPatternRestElement::KIND_SET
            .iter()
            .next(),
        "JsArrayExpression" => lang::JsArrayExpression::KIND_SET.iter().next(),
        "JsArrayHole" => lang::JsArrayHole::KIND_SET.iter().next(),
        "JsArrowFunctionExpression" => lang::JsArrowFunctionExpression::KIND_SET.iter().next(),
        "JsAssignmentExpression" => lang::JsAssignmentExpression::KIND_SET.iter().next(),
        "JsAwaitExpression" => lang::JsAwaitExpression::KIND_SET.iter().next(),
        "JsBigintLiteralExpression" => lang::JsBigintLiteralExpression::KIND_SET.iter().next(),
        "JsBinaryExpression" => lang::JsBinaryExpression::KIND_SET.iter().next(),
        "JsBlockStatement" => lang::JsBlockStatement::KIND_SET.iter().next(),
        "JsBooleanLiteralExpression" => lang::JsBooleanLiteralExpression::KIND_SET.iter().next(),
        "JsBreakStatement" => lang::JsBreakStatement::KIND_SET.iter().next(),
        "JsCallArguments" => lang::JsCallArguments::KIND_SET.iter().next(),
        "JsCallExpression" => lang::JsCallExpression::KIND_SET.iter().next(),
        "JsCaseClause" => lang::JsCaseClause::KIND_SET.iter().next(),
        "JsCatchClause" => lang::JsCatchClause::KIND_SET.iter().next(),
        "JsCatchDeclaration" => lang::JsCatchDeclaration::KIND_SET.iter().next(),
        "JsClassDeclaration" => lang::JsClassDeclaration::KIND_SET.iter().next(),
        "JsClassExportDefaultDeclaration" => lang::JsClassExportDefaultDeclaration::KIND_SET
            .iter()
            .next(),
        "JsClassExpression" => lang::JsClassExpression::KIND_SET.iter().next(),
        "JsComputedMemberAssignment" => lang::JsComputedMemberAssignment::KIND_SET.iter().next(),
        "JsComputedMemberExpression" => lang::JsComputedMemberExpression::KIND_SET.iter().next(),
        "JsComputedMemberName" => lang::JsComputedMemberName::KIND_SET.iter().next(),
        "JsConditionalExpression" => lang::JsConditionalExpression::KIND_SET.iter().next(),
        "JsConstructorClassMember" => lang::JsConstructorClassMember::KIND_SET.iter().next(),
        "JsConstructorParameters" => lang::JsConstructorParameters::KIND_SET.iter().next(),
        "JsContinueStatement" => lang::JsContinueStatement::KIND_SET.iter().next(),
        "JsDebuggerStatement" => lang::JsDebuggerStatement::KIND_SET.iter().next(),
        "JsDecorator" => lang::JsDecorator::KIND_SET.iter().next(),
        "JsDefaultClause" => lang::JsDefaultClause::KIND_SET.iter().next(),
        "JsDefaultImportSpecifier" => lang::JsDefaultImportSpecifier::KIND_SET.iter().next(),
        "JsDirective" => lang::JsDirective::KIND_SET.iter().next(),
        "JsDoWhileStatement" => lang::JsDoWhileStatement::KIND_SET.iter().next(),
        "JsElseClause" => lang::JsElseClause::KIND_SET.iter().next(),
        "JsEmptyClassMember" => lang::JsEmptyClassMember::KIND_SET.iter().next(),
        "JsEmptyStatement" => lang::JsEmptyStatement::KIND_SET.iter().next(),
        "JsExport" => lang::JsExport::KIND_SET.iter().next(),
        "JsExportAsClause" => lang::JsExportAsClause::KIND_SET.iter().next(),
        "JsExportDefaultDeclarationClause" => lang::JsExportDefaultDeclarationClause::KIND_SET
            .iter()
            .next(),
        "JsExportDefaultExpressionClause" => lang::JsExportDefaultExpressionClause::KIND_SET
            .iter()
            .next(),
        "JsExportFromClause" => lang::JsExportFromClause::KIND_SET.iter().next(),
        "JsExportNamedClause" => lang::JsExportNamedClause::KIND_SET.iter().next(),
        "JsExportNamedFromClause" => lang::JsExportNamedFromClause::KIND_SET.iter().next(),
        "JsExportNamedFromSpecifier" => lang::JsExportNamedFromSpecifier::KIND_SET.iter().next(),
        "JsExportNamedShorthandSpecifier" => lang::JsExportNamedShorthandSpecifier::KIND_SET
            .iter()
            .next(),
        "JsExportNamedSpecifier" => lang::JsExportNamedSpecifier::KIND_SET.iter().next(),
        "JsExpressionSnipped" => lang::JsExpressionSnipped::KIND_SET.iter().next(),
        "JsExpressionStatement" => lang::JsExpressionStatement::KIND_SET.iter().next(),
        "JsExtendsClause" => lang::JsExtendsClause::KIND_SET.iter().next(),
        "JsFinallyClause" => lang::JsFinallyClause::KIND_SET.iter().next(),
        "JsForInStatement" => lang::JsForInStatement::KIND_SET.iter().next(),
        "JsForOfStatement" => lang::JsForOfStatement::KIND_SET.iter().next(),
        "JsForStatement" => lang::JsForStatement::KIND_SET.iter().next(),
        "JsForVariableDeclaration" => lang::JsForVariableDeclaration::KIND_SET.iter().next(),
        "JsFormalParameter" => lang::JsFormalParameter::KIND_SET.iter().next(),
        "JsFunctionBody" => lang::JsFunctionBody::KIND_SET.iter().next(),
        "JsFunctionDeclaration" => lang::JsFunctionDeclaration::KIND_SET.iter().next(),
        "JsFunctionExportDefaultDeclaration" => lang::JsFunctionExportDefaultDeclaration::KIND_SET
            .iter()
            .next(),
        "JsFunctionExpression" => lang::JsFunctionExpression::KIND_SET.iter().next(),
        "JsGetterClassMember" => lang::JsGetterClassMember::KIND_SET.iter().next(),
        "JsGetterObjectMember" => lang::JsGetterObjectMember::KIND_SET.iter().next(),
        "JsIdentifierAssignment" => lang::JsIdentifierAssignment::KIND_SET.iter().next(),
        "JsIdentifierBinding" => lang::JsIdentifierBinding::KIND_SET.iter().next(),
        "JsIdentifierExpression" => lang::JsIdentifierExpression::KIND_SET.iter().next(),
        "JsIfStatement" => lang::JsIfStatement::KIND_SET.iter().next(),
        "JsImport" => lang::JsImport::KIND_SET.iter().next(),
        "JsImportAssertion" => lang::JsImportAssertion::KIND_SET.iter().next(),
        "JsImportAssertionEntry" => lang::JsImportAssertionEntry::KIND_SET.iter().next(),
        "JsImportBareClause" => lang::JsImportBareClause::KIND_SET.iter().next(),
        "JsImportCallExpression" => lang::JsImportCallExpression::KIND_SET.iter().next(),
        "JsImportCombinedClause" => lang::JsImportCombinedClause::KIND_SET.iter().next(),
        "JsImportDefaultClause" => lang::JsImportDefaultClause::KIND_SET.iter().next(),
        "JsImportMetaExpression" => lang::JsImportMetaExpression::KIND_SET.iter().next(),
        "JsImportNamedClause" => lang::JsImportNamedClause::KIND_SET.iter().next(),
        "JsImportNamespaceClause" => lang::JsImportNamespaceClause::KIND_SET.iter().next(),
        "JsInExpression" => lang::JsInExpression::KIND_SET.iter().next(),
        "JsInitializerClause" => lang::JsInitializerClause::KIND_SET.iter().next(),
        "JsInstanceofExpression" => lang::JsInstanceofExpression::KIND_SET.iter().next(),
        "JsLabel" => lang::JsLabel::KIND_SET.iter().next(),
        "JsLabeledStatement" => lang::JsLabeledStatement::KIND_SET.iter().next(),
        "JsLiteralExportName" => lang::JsLiteralExportName::KIND_SET.iter().next(),
        "JsLiteralMemberName" => lang::JsLiteralMemberName::KIND_SET.iter().next(),
        "JsLogicalExpression" => lang::JsLogicalExpression::KIND_SET.iter().next(),
        "JsMetavariable" => lang::JsMetavariable::KIND_SET.iter().next(),
        "JsMethodClassMember" => lang::JsMethodClassMember::KIND_SET.iter().next(),
        "JsMethodObjectMember" => lang::JsMethodObjectMember::KIND_SET.iter().next(),
        "JsModule" => lang::JsModule::KIND_SET.iter().next(),
        "JsModuleSource" => lang::JsModuleSource::KIND_SET.iter().next(),
        "JsName" => lang::JsName::KIND_SET.iter().next(),
        "JsNamedImportSpecifier" => lang::JsNamedImportSpecifier::KIND_SET.iter().next(),
        "JsNamedImportSpecifiers" => lang::JsNamedImportSpecifiers::KIND_SET.iter().next(),
        "JsNamespaceImportSpecifier" => lang::JsNamespaceImportSpecifier::KIND_SET.iter().next(),
        "JsNewExpression" => lang::JsNewExpression::KIND_SET.iter().next(),
        "JsNewTargetExpression" => lang::JsNewTargetExpression::KIND_SET.iter().next(),
        "JsNullLiteralExpression" => lang::JsNullLiteralExpression::KIND_SET.iter().next(),
        "JsNumberLiteralExpression" => lang::JsNumberLiteralExpression::KIND_SET.iter().next(),
        "JsObjectAssignmentPattern" => lang::JsObjectAssignmentPattern::KIND_SET.iter().next(),
        "JsObjectAssignmentPatternProperty" => lang::JsObjectAssignmentPatternProperty::KIND_SET
            .iter()
            .next(),
        "JsObjectAssignmentPatternRest" => {
            lang::JsObjectAssignmentPatternRest::KIND_SET.iter().next()
        }
        "JsObjectAssignmentPatternShorthandProperty" => {
            lang::JsObjectAssignmentPatternShorthandProperty::KIND_SET
                .iter()
                .next()
        }
        "JsObjectBindingPattern" => lang::JsObjectBindingPattern::KIND_SET.iter().next(),
        "JsObjectBindingPatternProperty" => {
            lang::JsObjectBindingPatternProperty::KIND_SET.iter().next()
        }
        "JsObjectBindingPatternRest" => lang::JsObjectBindingPatternRest::KIND_SET.iter().next(),
        "JsObjectBindingPatternShorthandProperty" => {
            lang::JsObjectBindingPatternShorthandProperty::KIND_SET
                .iter()
                .next()
        }
        "JsObjectExpression" => lang::JsObjectExpression::KIND_SET.iter().next(),
        "JsParameters" => lang::JsParameters::KIND_SET.iter().next(),
        "JsParenthesizedAssignment" => lang::JsParenthesizedAssignment::KIND_SET.iter().next(),
        "JsParenthesizedExpression" => lang::JsParenthesizedExpression::KIND_SET.iter().next(),
        "JsPostUpdateExpression" => lang::JsPostUpdateExpression::KIND_SET.iter().next(),
        "JsPreUpdateExpression" => lang::JsPreUpdateExpression::KIND_SET.iter().next(),
        "JsPrivateClassMemberName" => lang::JsPrivateClassMemberName::KIND_SET.iter().next(),
        "JsPrivateName" => lang::JsPrivateName::KIND_SET.iter().next(),
        "JsPropertyClassMember" => lang::JsPropertyClassMember::KIND_SET.iter().next(),
        "JsPropertyObjectMember" => lang::JsPropertyObjectMember::KIND_SET.iter().next(),
        "JsReferenceIdentifier" => lang::JsReferenceIdentifier::KIND_SET.iter().next(),
        "JsRegexLiteralExpression" => lang::JsRegexLiteralExpression::KIND_SET.iter().next(),
        "JsRestParameter" => lang::JsRestParameter::KIND_SET.iter().next(),
        "JsReturnStatement" => lang::JsReturnStatement::KIND_SET.iter().next(),
        "JsScript" => lang::JsScript::KIND_SET.iter().next(),
        "JsSequenceExpression" => lang::JsSequenceExpression::KIND_SET.iter().next(),
        "JsSetterClassMember" => lang::JsSetterClassMember::KIND_SET.iter().next(),
        "JsSetterObjectMember" => lang::JsSetterObjectMember::KIND_SET.iter().next(),
        "JsShorthandNamedImportSpecifier" => lang::JsShorthandNamedImportSpecifier::KIND_SET
            .iter()
            .next(),
        "JsShorthandPropertyObjectMember" => lang::JsShorthandPropertyObjectMember::KIND_SET
            .iter()
            .next(),
        "JsSpread" => lang::JsSpread::KIND_SET.iter().next(),
        "JsStaticInitializationBlockClassMember" => {
            lang::JsStaticInitializationBlockClassMember::KIND_SET
                .iter()
                .next()
        }
        "JsStaticMemberAssignment" => lang::JsStaticMemberAssignment::KIND_SET.iter().next(),
        "JsStaticMemberExpression" => lang::JsStaticMemberExpression::KIND_SET.iter().next(),
        "JsStaticModifier" => lang::JsStaticModifier::KIND_SET.iter().next(),
        "JsStringLiteralExpression" => lang::JsStringLiteralExpression::KIND_SET.iter().next(),
        "JsSuperExpression" => lang::JsSuperExpression::KIND_SET.iter().next(),
        "JsSwitchStatement" => lang::JsSwitchStatement::KIND_SET.iter().next(),
        "JsTemplateChunkElement" => lang::JsTemplateChunkElement::KIND_SET.iter().next(),
        "JsTemplateElement" => lang::JsTemplateElement::KIND_SET.iter().next(),
        "JsTemplateExpression" => lang::JsTemplateExpression::KIND_SET.iter().next(),
        "JsThisExpression" => lang::JsThisExpression::KIND_SET.iter().next(),
        "JsThrowStatement" => lang::JsThrowStatement::KIND_SET.iter().next(),
        "JsTryFinallyStatement" => lang::JsTryFinallyStatement::KIND_SET.iter().next(),
        "JsTryStatement" => lang::JsTryStatement::KIND_SET.iter().next(),
        "JsUnaryExpression" => lang::JsUnaryExpression::KIND_SET.iter().next(),
        "JsVariableDeclaration" => lang::JsVariableDeclaration::KIND_SET.iter().next(),
        "JsVariableDeclarationClause" => lang::JsVariableDeclarationClause::KIND_SET.iter().next(),
        "JsVariableDeclarator" => lang::JsVariableDeclarator::KIND_SET.iter().next(),
        "JsVariableStatement" => lang::JsVariableStatement::KIND_SET.iter().next(),
        "JsWhileStatement" => lang::JsWhileStatement::KIND_SET.iter().next(),
        "JsWithStatement" => lang::JsWithStatement::KIND_SET.iter().next(),
        "JsYieldArgument" => lang::JsYieldArgument::KIND_SET.iter().next(),
        "JsYieldExpression" => lang::JsYieldExpression::KIND_SET.iter().next(),
        "JsxAttribute" => lang::JsxAttribute::KIND_SET.iter().next(),
        "JsxAttributeInitializerClause" => {
            lang::JsxAttributeInitializerClause::KIND_SET.iter().next()
        }
        "JsxClosingElement" => lang::JsxClosingElement::KIND_SET.iter().next(),
        "JsxClosingFragment" => lang::JsxClosingFragment::KIND_SET.iter().next(),
        "JsxElement" => lang::JsxElement::KIND_SET.iter().next(),
        "JsxExpressionAttributeValue" => lang::JsxExpressionAttributeValue::KIND_SET.iter().next(),
        "JsxExpressionChild" => lang::JsxExpressionChild::KIND_SET.iter().next(),
        "JsxFragment" => lang::JsxFragment::KIND_SET.iter().next(),
        "JsxMemberName" => lang::JsxMemberName::KIND_SET.iter().next(),
        "JsxName" => lang::JsxName::KIND_SET.iter().next(),
        "JsxNamespaceName" => lang::JsxNamespaceName::KIND_SET.iter().next(),
        "JsxOpeningElement" => lang::JsxOpeningElement::KIND_SET.iter().next(),
        "JsxOpeningFragment" => lang::JsxOpeningFragment::KIND_SET.iter().next(),
        "JsxReferenceIdentifier" => lang::JsxReferenceIdentifier::KIND_SET.iter().next(),
        "JsxSelfClosingElement" => lang::JsxSelfClosingElement::KIND_SET.iter().next(),
        "JsxSpreadAttribute" => lang::JsxSpreadAttribute::KIND_SET.iter().next(),
        "JsxSpreadChild" => lang::JsxSpreadChild::KIND_SET.iter().next(),
        "JsxString" => lang::JsxString::KIND_SET.iter().next(),
        "JsxTagExpression" => lang::JsxTagExpression::KIND_SET.iter().next(),
        "JsxText" => lang::JsxText::KIND_SET.iter().next(),
        "TsAbstractModifier" => lang::TsAbstractModifier::KIND_SET.iter().next(),
        "TsAccessibilityModifier" => lang::TsAccessibilityModifier::KIND_SET.iter().next(),
        "TsAnyType" => lang::TsAnyType::KIND_SET.iter().next(),
        "TsArrayType" => lang::TsArrayType::KIND_SET.iter().next(),
        "TsAsAssignment" => lang::TsAsAssignment::KIND_SET.iter().next(),
        "TsAsExpression" => lang::TsAsExpression::KIND_SET.iter().next(),
        "TsAssertsCondition" => lang::TsAssertsCondition::KIND_SET.iter().next(),
        "TsAssertsReturnType" => lang::TsAssertsReturnType::KIND_SET.iter().next(),
        "TsBigintLiteralType" => lang::TsBigintLiteralType::KIND_SET.iter().next(),
        "TsBigintType" => lang::TsBigintType::KIND_SET.iter().next(),
        "TsBooleanLiteralType" => lang::TsBooleanLiteralType::KIND_SET.iter().next(),
        "TsBooleanType" => lang::TsBooleanType::KIND_SET.iter().next(),
        "TsCallSignatureTypeMember" => lang::TsCallSignatureTypeMember::KIND_SET.iter().next(),
        "TsConditionalType" => lang::TsConditionalType::KIND_SET.iter().next(),
        "TsConstModifier" => lang::TsConstModifier::KIND_SET.iter().next(),
        "TsConstructSignatureTypeMember" => {
            lang::TsConstructSignatureTypeMember::KIND_SET.iter().next()
        }
        "TsConstructorSignatureClassMember" => lang::TsConstructorSignatureClassMember::KIND_SET
            .iter()
            .next(),
        "TsConstructorType" => lang::TsConstructorType::KIND_SET.iter().next(),
        "TsDeclarationModule" => lang::TsDeclarationModule::KIND_SET.iter().next(),
        "TsDeclareFunctionDeclaration" => {
            lang::TsDeclareFunctionDeclaration::KIND_SET.iter().next()
        }
        "TsDeclareFunctionExportDefaultDeclaration" => {
            lang::TsDeclareFunctionExportDefaultDeclaration::KIND_SET
                .iter()
                .next()
        }
        "TsDeclareModifier" => lang::TsDeclareModifier::KIND_SET.iter().next(),
        "TsDeclareStatement" => lang::TsDeclareStatement::KIND_SET.iter().next(),
        "TsDefaultTypeClause" => lang::TsDefaultTypeClause::KIND_SET.iter().next(),
        "TsDefinitePropertyAnnotation" => {
            lang::TsDefinitePropertyAnnotation::KIND_SET.iter().next()
        }
        "TsDefiniteVariableAnnotation" => {
            lang::TsDefiniteVariableAnnotation::KIND_SET.iter().next()
        }
        "TsEmptyExternalModuleDeclarationBody" => {
            lang::TsEmptyExternalModuleDeclarationBody::KIND_SET
                .iter()
                .next()
        }
        "TsEnumDeclaration" => lang::TsEnumDeclaration::KIND_SET.iter().next(),
        "TsEnumMember" => lang::TsEnumMember::KIND_SET.iter().next(),
        "TsExportAsNamespaceClause" => lang::TsExportAsNamespaceClause::KIND_SET.iter().next(),
        "TsExportAssignmentClause" => lang::TsExportAssignmentClause::KIND_SET.iter().next(),
        "TsExportDeclareClause" => lang::TsExportDeclareClause::KIND_SET.iter().next(),
        "TsExtendsClause" => lang::TsExtendsClause::KIND_SET.iter().next(),
        "TsExternalModuleDeclaration" => lang::TsExternalModuleDeclaration::KIND_SET.iter().next(),
        "TsExternalModuleReference" => lang::TsExternalModuleReference::KIND_SET.iter().next(),
        "TsFunctionType" => lang::TsFunctionType::KIND_SET.iter().next(),
        "TsGetterSignatureClassMember" => {
            lang::TsGetterSignatureClassMember::KIND_SET.iter().next()
        }
        "TsGetterSignatureTypeMember" => lang::TsGetterSignatureTypeMember::KIND_SET.iter().next(),
        "TsGlobalDeclaration" => lang::TsGlobalDeclaration::KIND_SET.iter().next(),
        "TsIdentifierBinding" => lang::TsIdentifierBinding::KIND_SET.iter().next(),
        "TsImplementsClause" => lang::TsImplementsClause::KIND_SET.iter().next(),
        "TsImportEqualsDeclaration" => lang::TsImportEqualsDeclaration::KIND_SET.iter().next(),
        "TsImportType" => lang::TsImportType::KIND_SET.iter().next(),
        "TsImportTypeArguments" => lang::TsImportTypeArguments::KIND_SET.iter().next(),
        "TsImportTypeAssertion" => lang::TsImportTypeAssertion::KIND_SET.iter().next(),
        "TsImportTypeAssertionBlock" => lang::TsImportTypeAssertionBlock::KIND_SET.iter().next(),
        "TsImportTypeQualifier" => lang::TsImportTypeQualifier::KIND_SET.iter().next(),
        "TsInModifier" => lang::TsInModifier::KIND_SET.iter().next(),
        "TsIndexSignatureClassMember" => lang::TsIndexSignatureClassMember::KIND_SET.iter().next(),
        "TsIndexSignatureParameter" => lang::TsIndexSignatureParameter::KIND_SET.iter().next(),
        "TsIndexSignatureTypeMember" => lang::TsIndexSignatureTypeMember::KIND_SET.iter().next(),
        "TsIndexedAccessType" => lang::TsIndexedAccessType::KIND_SET.iter().next(),
        "TsInferType" => lang::TsInferType::KIND_SET.iter().next(),
        "TsInitializedPropertySignatureClassMember" => {
            lang::TsInitializedPropertySignatureClassMember::KIND_SET
                .iter()
                .next()
        }
        "TsInstantiationExpression" => lang::TsInstantiationExpression::KIND_SET.iter().next(),
        "TsInterfaceDeclaration" => lang::TsInterfaceDeclaration::KIND_SET.iter().next(),
        "TsIntersectionType" => lang::TsIntersectionType::KIND_SET.iter().next(),
        "TsLiteralEnumMemberName" => lang::TsLiteralEnumMemberName::KIND_SET.iter().next(),
        "TsMappedType" => lang::TsMappedType::KIND_SET.iter().next(),
        "TsMappedTypeAsClause" => lang::TsMappedTypeAsClause::KIND_SET.iter().next(),
        "TsMappedTypeOptionalModifierClause" => lang::TsMappedTypeOptionalModifierClause::KIND_SET
            .iter()
            .next(),
        "TsMappedTypeReadonlyModifierClause" => lang::TsMappedTypeReadonlyModifierClause::KIND_SET
            .iter()
            .next(),
        "TsMethodSignatureClassMember" => {
            lang::TsMethodSignatureClassMember::KIND_SET.iter().next()
        }
        "TsMethodSignatureTypeMember" => lang::TsMethodSignatureTypeMember::KIND_SET.iter().next(),
        "TsModuleBlock" => lang::TsModuleBlock::KIND_SET.iter().next(),
        "TsModuleDeclaration" => lang::TsModuleDeclaration::KIND_SET.iter().next(),
        "TsNamedTupleTypeElement" => lang::TsNamedTupleTypeElement::KIND_SET.iter().next(),
        "TsNeverType" => lang::TsNeverType::KIND_SET.iter().next(),
        "TsNonNullAssertionAssignment" => {
            lang::TsNonNullAssertionAssignment::KIND_SET.iter().next()
        }
        "TsNonNullAssertionExpression" => {
            lang::TsNonNullAssertionExpression::KIND_SET.iter().next()
        }
        "TsNonPrimitiveType" => lang::TsNonPrimitiveType::KIND_SET.iter().next(),
        "TsNullLiteralType" => lang::TsNullLiteralType::KIND_SET.iter().next(),
        "TsNumberLiteralType" => lang::TsNumberLiteralType::KIND_SET.iter().next(),
        "TsNumberType" => lang::TsNumberType::KIND_SET.iter().next(),
        "TsObjectType" => lang::TsObjectType::KIND_SET.iter().next(),
        "TsOptionalPropertyAnnotation" => {
            lang::TsOptionalPropertyAnnotation::KIND_SET.iter().next()
        }
        "TsOptionalTupleTypeElement" => lang::TsOptionalTupleTypeElement::KIND_SET.iter().next(),
        "TsOutModifier" => lang::TsOutModifier::KIND_SET.iter().next(),
        "TsOverrideModifier" => lang::TsOverrideModifier::KIND_SET.iter().next(),
        "TsParenthesizedType" => lang::TsParenthesizedType::KIND_SET.iter().next(),
        "TsPredicateReturnType" => lang::TsPredicateReturnType::KIND_SET.iter().next(),
        "TsPropertyParameter" => lang::TsPropertyParameter::KIND_SET.iter().next(),
        "TsPropertySignatureClassMember" => {
            lang::TsPropertySignatureClassMember::KIND_SET.iter().next()
        }
        "TsPropertySignatureTypeMember" => {
            lang::TsPropertySignatureTypeMember::KIND_SET.iter().next()
        }
        "TsQualifiedModuleName" => lang::TsQualifiedModuleName::KIND_SET.iter().next(),
        "TsQualifiedName" => lang::TsQualifiedName::KIND_SET.iter().next(),
        "TsReadonlyModifier" => lang::TsReadonlyModifier::KIND_SET.iter().next(),
        "TsReferenceType" => lang::TsReferenceType::KIND_SET.iter().next(),
        "TsRestTupleTypeElement" => lang::TsRestTupleTypeElement::KIND_SET.iter().next(),
        "TsReturnTypeAnnotation" => lang::TsReturnTypeAnnotation::KIND_SET.iter().next(),
        "TsSatisfiesAssignment" => lang::TsSatisfiesAssignment::KIND_SET.iter().next(),
        "TsSatisfiesExpression" => lang::TsSatisfiesExpression::KIND_SET.iter().next(),
        "TsSetterSignatureClassMember" => {
            lang::TsSetterSignatureClassMember::KIND_SET.iter().next()
        }
        "TsSetterSignatureTypeMember" => lang::TsSetterSignatureTypeMember::KIND_SET.iter().next(),
        "TsStringLiteralType" => lang::TsStringLiteralType::KIND_SET.iter().next(),
        "TsStringType" => lang::TsStringType::KIND_SET.iter().next(),
        "TsSymbolType" => lang::TsSymbolType::KIND_SET.iter().next(),
        "TsTemplateChunkElement" => lang::TsTemplateChunkElement::KIND_SET.iter().next(),
        "TsTemplateElement" => lang::TsTemplateElement::KIND_SET.iter().next(),
        "TsTemplateLiteralType" => lang::TsTemplateLiteralType::KIND_SET.iter().next(),
        "TsThisParameter" => lang::TsThisParameter::KIND_SET.iter().next(),
        "TsThisType" => lang::TsThisType::KIND_SET.iter().next(),
        "TsTupleType" => lang::TsTupleType::KIND_SET.iter().next(),
        "TsTypeAliasDeclaration" => lang::TsTypeAliasDeclaration::KIND_SET.iter().next(),
        "TsTypeAnnotation" => lang::TsTypeAnnotation::KIND_SET.iter().next(),
        "TsTypeArguments" => lang::TsTypeArguments::KIND_SET.iter().next(),
        "TsTypeAssertionAssignment" => lang::TsTypeAssertionAssignment::KIND_SET.iter().next(),
        "TsTypeAssertionExpression" => lang::TsTypeAssertionExpression::KIND_SET.iter().next(),
        "TsTypeConstraintClause" => lang::TsTypeConstraintClause::KIND_SET.iter().next(),
        "TsTypeOperatorType" => lang::TsTypeOperatorType::KIND_SET.iter().next(),
        "TsTypeParameter" => lang::TsTypeParameter::KIND_SET.iter().next(),
        "TsTypeParameterName" => lang::TsTypeParameterName::KIND_SET.iter().next(),
        "TsTypeParameters" => lang::TsTypeParameters::KIND_SET.iter().next(),
        "TsTypeofType" => lang::TsTypeofType::KIND_SET.iter().next(),
        "TsUndefinedType" => lang::TsUndefinedType::KIND_SET.iter().next(),
        "TsUnionType" => lang::TsUnionType::KIND_SET.iter().next(),
        "TsUnknownType" => lang::TsUnknownType::KIND_SET.iter().next(),
        "TsVoidType" => lang::TsVoidType::KIND_SET.iter().next(),
        _ => None,
    }
}
