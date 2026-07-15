// Generated file, do not edit by hand, see `xtask/codegen`

#[derive(Debug)]
enum JsAstNodeData {
    JsAccessorModifier(biome_js_syntax::JsAccessorModifier),
    JsArrayAssignmentPattern(biome_js_syntax::JsArrayAssignmentPattern),
    JsArrayAssignmentPatternElement(biome_js_syntax::JsArrayAssignmentPatternElement),
    JsArrayAssignmentPatternRestElement(biome_js_syntax::JsArrayAssignmentPatternRestElement),
    JsArrayBindingPattern(biome_js_syntax::JsArrayBindingPattern),
    JsArrayBindingPatternElement(biome_js_syntax::JsArrayBindingPatternElement),
    JsArrayBindingPatternRestElement(biome_js_syntax::JsArrayBindingPatternRestElement),
    JsArrayExpression(biome_js_syntax::JsArrayExpression),
    JsArrayHole(biome_js_syntax::JsArrayHole),
    JsArrowFunctionExpression(biome_js_syntax::JsArrowFunctionExpression),
    JsAssignmentExpression(biome_js_syntax::JsAssignmentExpression),
    JsAwaitExpression(biome_js_syntax::JsAwaitExpression),
    JsBigintLiteralExpression(biome_js_syntax::JsBigintLiteralExpression),
    JsBinaryExpression(biome_js_syntax::JsBinaryExpression),
    JsBlockStatement(biome_js_syntax::JsBlockStatement),
    JsBooleanLiteralExpression(biome_js_syntax::JsBooleanLiteralExpression),
    JsBreakStatement(biome_js_syntax::JsBreakStatement),
    JsCallArguments(biome_js_syntax::JsCallArguments),
    JsCallExpression(biome_js_syntax::JsCallExpression),
    JsCaseClause(biome_js_syntax::JsCaseClause),
    JsCatchClause(biome_js_syntax::JsCatchClause),
    JsCatchDeclaration(biome_js_syntax::JsCatchDeclaration),
    JsClassDeclaration(biome_js_syntax::JsClassDeclaration),
    JsClassExportDefaultDeclaration(biome_js_syntax::JsClassExportDefaultDeclaration),
    JsClassExpression(biome_js_syntax::JsClassExpression),
    JsComputedMemberAssignment(biome_js_syntax::JsComputedMemberAssignment),
    JsComputedMemberExpression(biome_js_syntax::JsComputedMemberExpression),
    JsComputedMemberName(biome_js_syntax::JsComputedMemberName),
    JsConditionalExpression(biome_js_syntax::JsConditionalExpression),
    JsConstructorClassMember(biome_js_syntax::JsConstructorClassMember),
    JsConstructorParameters(biome_js_syntax::JsConstructorParameters),
    JsContinueStatement(biome_js_syntax::JsContinueStatement),
    JsDebuggerStatement(biome_js_syntax::JsDebuggerStatement),
    JsDecorator(biome_js_syntax::JsDecorator),
    JsDefaultClause(biome_js_syntax::JsDefaultClause),
    JsDefaultImportSpecifier(biome_js_syntax::JsDefaultImportSpecifier),
    JsDirective(biome_js_syntax::JsDirective),
    JsDoWhileStatement(biome_js_syntax::JsDoWhileStatement),
    JsElseClause(biome_js_syntax::JsElseClause),
    JsEmptyClassMember(biome_js_syntax::JsEmptyClassMember),
    JsEmptyStatement(biome_js_syntax::JsEmptyStatement),
    JsExport(biome_js_syntax::JsExport),
    JsExportAsClause(biome_js_syntax::JsExportAsClause),
    JsExportDefaultDeclarationClause(biome_js_syntax::JsExportDefaultDeclarationClause),
    JsExportDefaultExpressionClause(biome_js_syntax::JsExportDefaultExpressionClause),
    JsExportFromClause(biome_js_syntax::JsExportFromClause),
    JsExportNamedClause(biome_js_syntax::JsExportNamedClause),
    JsExportNamedFromClause(biome_js_syntax::JsExportNamedFromClause),
    JsExportNamedFromSpecifier(biome_js_syntax::JsExportNamedFromSpecifier),
    JsExportNamedShorthandSpecifier(biome_js_syntax::JsExportNamedShorthandSpecifier),
    JsExportNamedSpecifier(biome_js_syntax::JsExportNamedSpecifier),
    JsExpressionSnippet(biome_js_syntax::JsExpressionSnippet),
    JsExpressionStatement(biome_js_syntax::JsExpressionStatement),
    JsExpressionTemplateRoot(biome_js_syntax::JsExpressionTemplateRoot),
    JsExtendsClause(biome_js_syntax::JsExtendsClause),
    JsFinallyClause(biome_js_syntax::JsFinallyClause),
    JsForInStatement(biome_js_syntax::JsForInStatement),
    JsForOfStatement(biome_js_syntax::JsForOfStatement),
    JsForStatement(biome_js_syntax::JsForStatement),
    JsForVariableDeclaration(biome_js_syntax::JsForVariableDeclaration),
    JsFormalParameter(biome_js_syntax::JsFormalParameter),
    JsFunctionBody(biome_js_syntax::JsFunctionBody),
    JsFunctionDeclaration(biome_js_syntax::JsFunctionDeclaration),
    JsFunctionExportDefaultDeclaration(biome_js_syntax::JsFunctionExportDefaultDeclaration),
    JsFunctionExpression(biome_js_syntax::JsFunctionExpression),
    JsGetterClassMember(biome_js_syntax::JsGetterClassMember),
    JsGetterObjectMember(biome_js_syntax::JsGetterObjectMember),
    JsIdentifierAssignment(biome_js_syntax::JsIdentifierAssignment),
    JsIdentifierBinding(biome_js_syntax::JsIdentifierBinding),
    JsIdentifierExpression(biome_js_syntax::JsIdentifierExpression),
    JsIfStatement(biome_js_syntax::JsIfStatement),
    JsImport(biome_js_syntax::JsImport),
    JsImportAssertion(biome_js_syntax::JsImportAssertion),
    JsImportAssertionEntry(biome_js_syntax::JsImportAssertionEntry),
    JsImportBareClause(biome_js_syntax::JsImportBareClause),
    JsImportCallExpression(biome_js_syntax::JsImportCallExpression),
    JsImportCombinedClause(biome_js_syntax::JsImportCombinedClause),
    JsImportDefaultClause(biome_js_syntax::JsImportDefaultClause),
    JsImportMetaExpression(biome_js_syntax::JsImportMetaExpression),
    JsImportNamedClause(biome_js_syntax::JsImportNamedClause),
    JsImportNamespaceClause(biome_js_syntax::JsImportNamespaceClause),
    JsInExpression(biome_js_syntax::JsInExpression),
    JsInitializerClause(biome_js_syntax::JsInitializerClause),
    JsInstanceofExpression(biome_js_syntax::JsInstanceofExpression),
    JsLabel(biome_js_syntax::JsLabel),
    JsLabeledStatement(biome_js_syntax::JsLabeledStatement),
    JsLiteralExportName(biome_js_syntax::JsLiteralExportName),
    JsLiteralMemberName(biome_js_syntax::JsLiteralMemberName),
    JsLogicalExpression(biome_js_syntax::JsLogicalExpression),
    JsMetavariable(biome_js_syntax::JsMetavariable),
    JsMethodClassMember(biome_js_syntax::JsMethodClassMember),
    JsMethodObjectMember(biome_js_syntax::JsMethodObjectMember),
    JsModule(biome_js_syntax::JsModule),
    JsModuleSource(biome_js_syntax::JsModuleSource),
    JsName(biome_js_syntax::JsName),
    JsNamedImportSpecifier(biome_js_syntax::JsNamedImportSpecifier),
    JsNamedImportSpecifiers(biome_js_syntax::JsNamedImportSpecifiers),
    JsNamespaceImportSpecifier(biome_js_syntax::JsNamespaceImportSpecifier),
    JsNewExpression(biome_js_syntax::JsNewExpression),
    JsNewTargetExpression(biome_js_syntax::JsNewTargetExpression),
    JsNullLiteralExpression(biome_js_syntax::JsNullLiteralExpression),
    JsNumberLiteralExpression(biome_js_syntax::JsNumberLiteralExpression),
    JsObjectAssignmentPattern(biome_js_syntax::JsObjectAssignmentPattern),
    JsObjectAssignmentPatternProperty(biome_js_syntax::JsObjectAssignmentPatternProperty),
    JsObjectAssignmentPatternRest(biome_js_syntax::JsObjectAssignmentPatternRest),
    JsObjectAssignmentPatternShorthandProperty(
        biome_js_syntax::JsObjectAssignmentPatternShorthandProperty,
    ),
    JsObjectBindingPattern(biome_js_syntax::JsObjectBindingPattern),
    JsObjectBindingPatternProperty(biome_js_syntax::JsObjectBindingPatternProperty),
    JsObjectBindingPatternRest(biome_js_syntax::JsObjectBindingPatternRest),
    JsObjectBindingPatternShorthandProperty(
        biome_js_syntax::JsObjectBindingPatternShorthandProperty,
    ),
    JsObjectExpression(biome_js_syntax::JsObjectExpression),
    JsParameters(biome_js_syntax::JsParameters),
    JsParenthesizedAssignment(biome_js_syntax::JsParenthesizedAssignment),
    JsParenthesizedExpression(biome_js_syntax::JsParenthesizedExpression),
    JsPostUpdateExpression(biome_js_syntax::JsPostUpdateExpression),
    JsPreUpdateExpression(biome_js_syntax::JsPreUpdateExpression),
    JsPrivateClassMemberName(biome_js_syntax::JsPrivateClassMemberName),
    JsPrivateName(biome_js_syntax::JsPrivateName),
    JsPropertyClassMember(biome_js_syntax::JsPropertyClassMember),
    JsPropertyObjectMember(biome_js_syntax::JsPropertyObjectMember),
    JsReferenceIdentifier(biome_js_syntax::JsReferenceIdentifier),
    JsRegexLiteralExpression(biome_js_syntax::JsRegexLiteralExpression),
    JsRestParameter(biome_js_syntax::JsRestParameter),
    JsReturnStatement(biome_js_syntax::JsReturnStatement),
    JsScript(biome_js_syntax::JsScript),
    JsSequenceExpression(biome_js_syntax::JsSequenceExpression),
    JsSetterClassMember(biome_js_syntax::JsSetterClassMember),
    JsSetterObjectMember(biome_js_syntax::JsSetterObjectMember),
    JsShorthandNamedImportSpecifier(biome_js_syntax::JsShorthandNamedImportSpecifier),
    JsShorthandPropertyObjectMember(biome_js_syntax::JsShorthandPropertyObjectMember),
    JsSpread(biome_js_syntax::JsSpread),
    JsStaticInitializationBlockClassMember(biome_js_syntax::JsStaticInitializationBlockClassMember),
    JsStaticMemberAssignment(biome_js_syntax::JsStaticMemberAssignment),
    JsStaticMemberExpression(biome_js_syntax::JsStaticMemberExpression),
    JsStaticModifier(biome_js_syntax::JsStaticModifier),
    JsStringLiteralExpression(biome_js_syntax::JsStringLiteralExpression),
    JsSuperExpression(biome_js_syntax::JsSuperExpression),
    JsSvelteSnippetRoot(biome_js_syntax::JsSvelteSnippetRoot),
    JsSwitchStatement(biome_js_syntax::JsSwitchStatement),
    JsTemplateChunkElement(biome_js_syntax::JsTemplateChunkElement),
    JsTemplateElement(biome_js_syntax::JsTemplateElement),
    JsTemplateExpression(biome_js_syntax::JsTemplateExpression),
    JsThisExpression(biome_js_syntax::JsThisExpression),
    JsThrowStatement(biome_js_syntax::JsThrowStatement),
    JsTryFinallyStatement(biome_js_syntax::JsTryFinallyStatement),
    JsTryStatement(biome_js_syntax::JsTryStatement),
    JsUnaryExpression(biome_js_syntax::JsUnaryExpression),
    JsVariableDeclaration(biome_js_syntax::JsVariableDeclaration),
    JsVariableDeclarationClause(biome_js_syntax::JsVariableDeclarationClause),
    JsVariableDeclarator(biome_js_syntax::JsVariableDeclarator),
    JsVariableStatement(biome_js_syntax::JsVariableStatement),
    JsWhileStatement(biome_js_syntax::JsWhileStatement),
    JsWithStatement(biome_js_syntax::JsWithStatement),
    JsYieldArgument(biome_js_syntax::JsYieldArgument),
    JsYieldExpression(biome_js_syntax::JsYieldExpression),
    JsxAttribute(biome_js_syntax::JsxAttribute),
    JsxAttributeInitializerClause(biome_js_syntax::JsxAttributeInitializerClause),
    JsxClosingElement(biome_js_syntax::JsxClosingElement),
    JsxClosingFragment(biome_js_syntax::JsxClosingFragment),
    JsxElement(biome_js_syntax::JsxElement),
    JsxExpressionAttributeValue(biome_js_syntax::JsxExpressionAttributeValue),
    JsxExpressionChild(biome_js_syntax::JsxExpressionChild),
    JsxFragment(biome_js_syntax::JsxFragment),
    JsxMemberName(biome_js_syntax::JsxMemberName),
    JsxName(biome_js_syntax::JsxName),
    JsxNamespaceName(biome_js_syntax::JsxNamespaceName),
    JsxOpeningElement(biome_js_syntax::JsxOpeningElement),
    JsxOpeningFragment(biome_js_syntax::JsxOpeningFragment),
    JsxReferenceIdentifier(biome_js_syntax::JsxReferenceIdentifier),
    JsxSelfClosingElement(biome_js_syntax::JsxSelfClosingElement),
    JsxShorthandAttribute(biome_js_syntax::JsxShorthandAttribute),
    JsxSpreadAttribute(biome_js_syntax::JsxSpreadAttribute),
    JsxSpreadChild(biome_js_syntax::JsxSpreadChild),
    JsxString(biome_js_syntax::JsxString),
    JsxTagExpression(biome_js_syntax::JsxTagExpression),
    JsxText(biome_js_syntax::JsxText),
    TsAbstractModifier(biome_js_syntax::TsAbstractModifier),
    TsAccessibilityModifier(biome_js_syntax::TsAccessibilityModifier),
    TsAnyType(biome_js_syntax::TsAnyType),
    TsArrayType(biome_js_syntax::TsArrayType),
    TsAsAssignment(biome_js_syntax::TsAsAssignment),
    TsAsExpression(biome_js_syntax::TsAsExpression),
    TsAssertsCondition(biome_js_syntax::TsAssertsCondition),
    TsAssertsReturnType(biome_js_syntax::TsAssertsReturnType),
    TsBigintLiteralType(biome_js_syntax::TsBigintLiteralType),
    TsBigintType(biome_js_syntax::TsBigintType),
    TsBooleanLiteralType(biome_js_syntax::TsBooleanLiteralType),
    TsBooleanType(biome_js_syntax::TsBooleanType),
    TsCallSignatureTypeMember(biome_js_syntax::TsCallSignatureTypeMember),
    TsConditionalType(biome_js_syntax::TsConditionalType),
    TsConstModifier(biome_js_syntax::TsConstModifier),
    TsConstructSignatureTypeMember(biome_js_syntax::TsConstructSignatureTypeMember),
    TsConstructorSignatureClassMember(biome_js_syntax::TsConstructorSignatureClassMember),
    TsConstructorType(biome_js_syntax::TsConstructorType),
    TsDeclarationModule(biome_js_syntax::TsDeclarationModule),
    TsDeclareFunctionDeclaration(biome_js_syntax::TsDeclareFunctionDeclaration),
    TsDeclareFunctionExportDefaultDeclaration(
        biome_js_syntax::TsDeclareFunctionExportDefaultDeclaration,
    ),
    TsDeclareModifier(biome_js_syntax::TsDeclareModifier),
    TsDeclareStatement(biome_js_syntax::TsDeclareStatement),
    TsDefaultTypeClause(biome_js_syntax::TsDefaultTypeClause),
    TsDefinitePropertyAnnotation(biome_js_syntax::TsDefinitePropertyAnnotation),
    TsDefiniteVariableAnnotation(biome_js_syntax::TsDefiniteVariableAnnotation),
    TsEmptyExternalModuleDeclarationBody(biome_js_syntax::TsEmptyExternalModuleDeclarationBody),
    TsEnumDeclaration(biome_js_syntax::TsEnumDeclaration),
    TsEnumMember(biome_js_syntax::TsEnumMember),
    TsExportAsNamespaceClause(biome_js_syntax::TsExportAsNamespaceClause),
    TsExportAssignmentClause(biome_js_syntax::TsExportAssignmentClause),
    TsExportDeclareClause(biome_js_syntax::TsExportDeclareClause),
    TsExtendsClause(biome_js_syntax::TsExtendsClause),
    TsExternalModuleDeclaration(biome_js_syntax::TsExternalModuleDeclaration),
    TsExternalModuleReference(biome_js_syntax::TsExternalModuleReference),
    TsFunctionType(biome_js_syntax::TsFunctionType),
    TsGetterSignatureClassMember(biome_js_syntax::TsGetterSignatureClassMember),
    TsGetterSignatureTypeMember(biome_js_syntax::TsGetterSignatureTypeMember),
    TsGlobalDeclaration(biome_js_syntax::TsGlobalDeclaration),
    TsIdentifierBinding(biome_js_syntax::TsIdentifierBinding),
    TsImplementsClause(biome_js_syntax::TsImplementsClause),
    TsImportEqualsDeclaration(biome_js_syntax::TsImportEqualsDeclaration),
    TsImportType(biome_js_syntax::TsImportType),
    TsImportTypeArguments(biome_js_syntax::TsImportTypeArguments),
    TsImportTypeAssertion(biome_js_syntax::TsImportTypeAssertion),
    TsImportTypeAssertionBlock(biome_js_syntax::TsImportTypeAssertionBlock),
    TsImportTypeQualifier(biome_js_syntax::TsImportTypeQualifier),
    TsInModifier(biome_js_syntax::TsInModifier),
    TsIndexSignatureClassMember(biome_js_syntax::TsIndexSignatureClassMember),
    TsIndexSignatureParameter(biome_js_syntax::TsIndexSignatureParameter),
    TsIndexSignatureTypeMember(biome_js_syntax::TsIndexSignatureTypeMember),
    TsIndexedAccessType(biome_js_syntax::TsIndexedAccessType),
    TsInferType(biome_js_syntax::TsInferType),
    TsInitializedPropertySignatureClassMember(
        biome_js_syntax::TsInitializedPropertySignatureClassMember,
    ),
    TsInstantiationExpression(biome_js_syntax::TsInstantiationExpression),
    TsInterfaceDeclaration(biome_js_syntax::TsInterfaceDeclaration),
    TsIntersectionType(biome_js_syntax::TsIntersectionType),
    TsLiteralEnumMemberName(biome_js_syntax::TsLiteralEnumMemberName),
    TsMappedType(biome_js_syntax::TsMappedType),
    TsMappedTypeAsClause(biome_js_syntax::TsMappedTypeAsClause),
    TsMappedTypeOptionalModifierClause(biome_js_syntax::TsMappedTypeOptionalModifierClause),
    TsMappedTypeReadonlyModifierClause(biome_js_syntax::TsMappedTypeReadonlyModifierClause),
    TsMethodSignatureClassMember(biome_js_syntax::TsMethodSignatureClassMember),
    TsMethodSignatureTypeMember(biome_js_syntax::TsMethodSignatureTypeMember),
    TsModuleBlock(biome_js_syntax::TsModuleBlock),
    TsModuleDeclaration(biome_js_syntax::TsModuleDeclaration),
    TsNamedTupleTypeElement(biome_js_syntax::TsNamedTupleTypeElement),
    TsNeverType(biome_js_syntax::TsNeverType),
    TsNonNullAssertionAssignment(biome_js_syntax::TsNonNullAssertionAssignment),
    TsNonNullAssertionExpression(biome_js_syntax::TsNonNullAssertionExpression),
    TsNonPrimitiveType(biome_js_syntax::TsNonPrimitiveType),
    TsNullLiteralType(biome_js_syntax::TsNullLiteralType),
    TsNumberLiteralType(biome_js_syntax::TsNumberLiteralType),
    TsNumberType(biome_js_syntax::TsNumberType),
    TsObjectType(biome_js_syntax::TsObjectType),
    TsOptionalPropertyAnnotation(biome_js_syntax::TsOptionalPropertyAnnotation),
    TsOptionalTupleTypeElement(biome_js_syntax::TsOptionalTupleTypeElement),
    TsOutModifier(biome_js_syntax::TsOutModifier),
    TsOverrideModifier(biome_js_syntax::TsOverrideModifier),
    TsParenthesizedType(biome_js_syntax::TsParenthesizedType),
    TsPredicateReturnType(biome_js_syntax::TsPredicateReturnType),
    TsPropertyParameter(biome_js_syntax::TsPropertyParameter),
    TsPropertySignatureClassMember(biome_js_syntax::TsPropertySignatureClassMember),
    TsPropertySignatureTypeMember(biome_js_syntax::TsPropertySignatureTypeMember),
    TsQualifiedModuleName(biome_js_syntax::TsQualifiedModuleName),
    TsQualifiedName(biome_js_syntax::TsQualifiedName),
    TsReadonlyModifier(biome_js_syntax::TsReadonlyModifier),
    TsReferenceType(biome_js_syntax::TsReferenceType),
    TsRestTupleTypeElement(biome_js_syntax::TsRestTupleTypeElement),
    TsReturnTypeAnnotation(biome_js_syntax::TsReturnTypeAnnotation),
    TsSatisfiesAssignment(biome_js_syntax::TsSatisfiesAssignment),
    TsSatisfiesExpression(biome_js_syntax::TsSatisfiesExpression),
    TsSetterSignatureClassMember(biome_js_syntax::TsSetterSignatureClassMember),
    TsSetterSignatureTypeMember(biome_js_syntax::TsSetterSignatureTypeMember),
    TsStringLiteralType(biome_js_syntax::TsStringLiteralType),
    TsStringType(biome_js_syntax::TsStringType),
    TsSymbolType(biome_js_syntax::TsSymbolType),
    TsTemplateChunkElement(biome_js_syntax::TsTemplateChunkElement),
    TsTemplateElement(biome_js_syntax::TsTemplateElement),
    TsTemplateLiteralType(biome_js_syntax::TsTemplateLiteralType),
    TsThisParameter(biome_js_syntax::TsThisParameter),
    TsThisType(biome_js_syntax::TsThisType),
    TsTupleType(biome_js_syntax::TsTupleType),
    TsTypeAliasDeclaration(biome_js_syntax::TsTypeAliasDeclaration),
    TsTypeAnnotation(biome_js_syntax::TsTypeAnnotation),
    TsTypeArguments(biome_js_syntax::TsTypeArguments),
    TsTypeAssertionAssignment(biome_js_syntax::TsTypeAssertionAssignment),
    TsTypeAssertionExpression(biome_js_syntax::TsTypeAssertionExpression),
    TsTypeConstraintClause(biome_js_syntax::TsTypeConstraintClause),
    TsTypeOperatorType(biome_js_syntax::TsTypeOperatorType),
    TsTypeParameter(biome_js_syntax::TsTypeParameter),
    TsTypeParameterName(biome_js_syntax::TsTypeParameterName),
    TsTypeParameters(biome_js_syntax::TsTypeParameters),
    TsTypeofType(biome_js_syntax::TsTypeofType),
    TsUndefinedType(biome_js_syntax::TsUndefinedType),
    TsUnionType(biome_js_syntax::TsUnionType),
    TsUnknownType(biome_js_syntax::TsUnknownType),
    TsVoidType(biome_js_syntax::TsVoidType),
    JsBogus(biome_js_syntax::JsBogus),
    JsBogusAssignment(biome_js_syntax::JsBogusAssignment),
    JsBogusBinding(biome_js_syntax::JsBogusBinding),
    JsBogusExpression(biome_js_syntax::JsBogusExpression),
    JsBogusImportAssertionEntry(biome_js_syntax::JsBogusImportAssertionEntry),
    JsBogusMember(biome_js_syntax::JsBogusMember),
    JsBogusNamedImportSpecifier(biome_js_syntax::JsBogusNamedImportSpecifier),
    JsBogusParameter(biome_js_syntax::JsBogusParameter),
    JsBogusStatement(biome_js_syntax::JsBogusStatement),
    TsBogusType(biome_js_syntax::TsBogusType),
}
impl JsAstNodeData {
    fn cast(node: JsSyntaxNode) -> Option<Self> {
        match node.kind() {
            biome_js_syntax::JsSyntaxKind::JS_ACCESSOR_MODIFIER => {
                <biome_js_syntax::JsAccessorModifier as AstNode>::cast(node)
                    .map(Self::JsAccessorModifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN => {
                <biome_js_syntax::JsArrayAssignmentPattern as AstNode>::cast(node)
                    .map(Self::JsArrayAssignmentPattern)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT => {
                <biome_js_syntax::JsArrayAssignmentPatternElement as AstNode>::cast(node)
                    .map(Self::JsArrayAssignmentPatternElement)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT => {
                <biome_js_syntax::JsArrayAssignmentPatternRestElement as AstNode>::cast(node)
                    .map(Self::JsArrayAssignmentPatternRestElement)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN => {
                <biome_js_syntax::JsArrayBindingPattern as AstNode>::cast(node)
                    .map(Self::JsArrayBindingPattern)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_ELEMENT => {
                <biome_js_syntax::JsArrayBindingPatternElement as AstNode>::cast(node)
                    .map(Self::JsArrayBindingPatternElement)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_BINDING_PATTERN_REST_ELEMENT => {
                <biome_js_syntax::JsArrayBindingPatternRestElement as AstNode>::cast(node)
                    .map(Self::JsArrayBindingPatternRestElement)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_EXPRESSION => {
                <biome_js_syntax::JsArrayExpression as AstNode>::cast(node)
                    .map(Self::JsArrayExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARRAY_HOLE => {
                <biome_js_syntax::JsArrayHole as AstNode>::cast(node).map(Self::JsArrayHole)
            }
            biome_js_syntax::JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
                <biome_js_syntax::JsArrowFunctionExpression as AstNode>::cast(node)
                    .map(Self::JsArrowFunctionExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                <biome_js_syntax::JsAssignmentExpression as AstNode>::cast(node)
                    .map(Self::JsAssignmentExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_AWAIT_EXPRESSION => {
                <biome_js_syntax::JsAwaitExpression as AstNode>::cast(node)
                    .map(Self::JsAwaitExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_BIGINT_LITERAL_EXPRESSION => {
                <biome_js_syntax::JsBigintLiteralExpression as AstNode>::cast(node)
                    .map(Self::JsBigintLiteralExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_BINARY_EXPRESSION => {
                <biome_js_syntax::JsBinaryExpression as AstNode>::cast(node)
                    .map(Self::JsBinaryExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_BLOCK_STATEMENT => {
                <biome_js_syntax::JsBlockStatement as AstNode>::cast(node)
                    .map(Self::JsBlockStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
                <biome_js_syntax::JsBooleanLiteralExpression as AstNode>::cast(node)
                    .map(Self::JsBooleanLiteralExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_BREAK_STATEMENT => {
                <biome_js_syntax::JsBreakStatement as AstNode>::cast(node)
                    .map(Self::JsBreakStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_CALL_ARGUMENTS => {
                <biome_js_syntax::JsCallArguments as AstNode>::cast(node).map(Self::JsCallArguments)
            }
            biome_js_syntax::JsSyntaxKind::JS_CALL_EXPRESSION => {
                <biome_js_syntax::JsCallExpression as AstNode>::cast(node)
                    .map(Self::JsCallExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_CASE_CLAUSE => {
                <biome_js_syntax::JsCaseClause as AstNode>::cast(node).map(Self::JsCaseClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_CATCH_CLAUSE => {
                <biome_js_syntax::JsCatchClause as AstNode>::cast(node).map(Self::JsCatchClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_CATCH_DECLARATION => {
                <biome_js_syntax::JsCatchDeclaration as AstNode>::cast(node)
                    .map(Self::JsCatchDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_CLASS_DECLARATION => {
                <biome_js_syntax::JsClassDeclaration as AstNode>::cast(node)
                    .map(Self::JsClassDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_CLASS_EXPORT_DEFAULT_DECLARATION => {
                <biome_js_syntax::JsClassExportDefaultDeclaration as AstNode>::cast(node)
                    .map(Self::JsClassExportDefaultDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_CLASS_EXPRESSION => {
                <biome_js_syntax::JsClassExpression as AstNode>::cast(node)
                    .map(Self::JsClassExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                <biome_js_syntax::JsComputedMemberAssignment as AstNode>::cast(node)
                    .map(Self::JsComputedMemberAssignment)
            }
            biome_js_syntax::JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                <biome_js_syntax::JsComputedMemberExpression as AstNode>::cast(node)
                    .map(Self::JsComputedMemberExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_COMPUTED_MEMBER_NAME => {
                <biome_js_syntax::JsComputedMemberName as AstNode>::cast(node)
                    .map(Self::JsComputedMemberName)
            }
            biome_js_syntax::JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                <biome_js_syntax::JsConditionalExpression as AstNode>::cast(node)
                    .map(Self::JsConditionalExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER => {
                <biome_js_syntax::JsConstructorClassMember as AstNode>::cast(node)
                    .map(Self::JsConstructorClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_CONSTRUCTOR_PARAMETERS => {
                <biome_js_syntax::JsConstructorParameters as AstNode>::cast(node)
                    .map(Self::JsConstructorParameters)
            }
            biome_js_syntax::JsSyntaxKind::JS_CONTINUE_STATEMENT => {
                <biome_js_syntax::JsContinueStatement as AstNode>::cast(node)
                    .map(Self::JsContinueStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_DEBUGGER_STATEMENT => {
                <biome_js_syntax::JsDebuggerStatement as AstNode>::cast(node)
                    .map(Self::JsDebuggerStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_DECORATOR => {
                <biome_js_syntax::JsDecorator as AstNode>::cast(node).map(Self::JsDecorator)
            }
            biome_js_syntax::JsSyntaxKind::JS_DEFAULT_CLAUSE => {
                <biome_js_syntax::JsDefaultClause as AstNode>::cast(node).map(Self::JsDefaultClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_DEFAULT_IMPORT_SPECIFIER => {
                <biome_js_syntax::JsDefaultImportSpecifier as AstNode>::cast(node)
                    .map(Self::JsDefaultImportSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_DIRECTIVE => {
                <biome_js_syntax::JsDirective as AstNode>::cast(node).map(Self::JsDirective)
            }
            biome_js_syntax::JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
                <biome_js_syntax::JsDoWhileStatement as AstNode>::cast(node)
                    .map(Self::JsDoWhileStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_ELSE_CLAUSE => {
                <biome_js_syntax::JsElseClause as AstNode>::cast(node).map(Self::JsElseClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EMPTY_CLASS_MEMBER => {
                <biome_js_syntax::JsEmptyClassMember as AstNode>::cast(node)
                    .map(Self::JsEmptyClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_EMPTY_STATEMENT => {
                <biome_js_syntax::JsEmptyStatement as AstNode>::cast(node)
                    .map(Self::JsEmptyStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT => {
                <biome_js_syntax::JsExport as AstNode>::cast(node).map(Self::JsExport)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_AS_CLAUSE => {
                <biome_js_syntax::JsExportAsClause as AstNode>::cast(node)
                    .map(Self::JsExportAsClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_DECLARATION_CLAUSE => {
                <biome_js_syntax::JsExportDefaultDeclarationClause as AstNode>::cast(node)
                    .map(Self::JsExportDefaultDeclarationClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE => {
                <biome_js_syntax::JsExportDefaultExpressionClause as AstNode>::cast(node)
                    .map(Self::JsExportDefaultExpressionClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_FROM_CLAUSE => {
                <biome_js_syntax::JsExportFromClause as AstNode>::cast(node)
                    .map(Self::JsExportFromClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_CLAUSE => {
                <biome_js_syntax::JsExportNamedClause as AstNode>::cast(node)
                    .map(Self::JsExportNamedClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_FROM_CLAUSE => {
                <biome_js_syntax::JsExportNamedFromClause as AstNode>::cast(node)
                    .map(Self::JsExportNamedFromClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_FROM_SPECIFIER => {
                <biome_js_syntax::JsExportNamedFromSpecifier as AstNode>::cast(node)
                    .map(Self::JsExportNamedFromSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_SHORTHAND_SPECIFIER => {
                <biome_js_syntax::JsExportNamedShorthandSpecifier as AstNode>::cast(node)
                    .map(Self::JsExportNamedShorthandSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPORT_NAMED_SPECIFIER => {
                <biome_js_syntax::JsExportNamedSpecifier as AstNode>::cast(node)
                    .map(Self::JsExportNamedSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPRESSION_SNIPPET => {
                <biome_js_syntax::JsExpressionSnippet as AstNode>::cast(node)
                    .map(Self::JsExpressionSnippet)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                <biome_js_syntax::JsExpressionStatement as AstNode>::cast(node)
                    .map(Self::JsExpressionStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXPRESSION_TEMPLATE_ROOT => {
                <biome_js_syntax::JsExpressionTemplateRoot as AstNode>::cast(node)
                    .map(Self::JsExpressionTemplateRoot)
            }
            biome_js_syntax::JsSyntaxKind::JS_EXTENDS_CLAUSE => {
                <biome_js_syntax::JsExtendsClause as AstNode>::cast(node).map(Self::JsExtendsClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_FINALLY_CLAUSE => {
                <biome_js_syntax::JsFinallyClause as AstNode>::cast(node).map(Self::JsFinallyClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_FOR_IN_STATEMENT => {
                <biome_js_syntax::JsForInStatement as AstNode>::cast(node)
                    .map(Self::JsForInStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_FOR_OF_STATEMENT => {
                <biome_js_syntax::JsForOfStatement as AstNode>::cast(node)
                    .map(Self::JsForOfStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_FOR_STATEMENT => {
                <biome_js_syntax::JsForStatement as AstNode>::cast(node).map(Self::JsForStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_FOR_VARIABLE_DECLARATION => {
                <biome_js_syntax::JsForVariableDeclaration as AstNode>::cast(node)
                    .map(Self::JsForVariableDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_FORMAL_PARAMETER => {
                <biome_js_syntax::JsFormalParameter as AstNode>::cast(node)
                    .map(Self::JsFormalParameter)
            }
            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_BODY => {
                <biome_js_syntax::JsFunctionBody as AstNode>::cast(node).map(Self::JsFunctionBody)
            }
            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                <biome_js_syntax::JsFunctionDeclaration as AstNode>::cast(node)
                    .map(Self::JsFunctionDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                <biome_js_syntax::JsFunctionExportDefaultDeclaration as AstNode>::cast(node)
                    .map(Self::JsFunctionExportDefaultDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_FUNCTION_EXPRESSION => {
                <biome_js_syntax::JsFunctionExpression as AstNode>::cast(node)
                    .map(Self::JsFunctionExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_GETTER_CLASS_MEMBER => {
                <biome_js_syntax::JsGetterClassMember as AstNode>::cast(node)
                    .map(Self::JsGetterClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_GETTER_OBJECT_MEMBER => {
                <biome_js_syntax::JsGetterObjectMember as AstNode>::cast(node)
                    .map(Self::JsGetterObjectMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT => {
                <biome_js_syntax::JsIdentifierAssignment as AstNode>::cast(node)
                    .map(Self::JsIdentifierAssignment)
            }
            biome_js_syntax::JsSyntaxKind::JS_IDENTIFIER_BINDING => {
                <biome_js_syntax::JsIdentifierBinding as AstNode>::cast(node)
                    .map(Self::JsIdentifierBinding)
            }
            biome_js_syntax::JsSyntaxKind::JS_IDENTIFIER_EXPRESSION => {
                <biome_js_syntax::JsIdentifierExpression as AstNode>::cast(node)
                    .map(Self::JsIdentifierExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_IF_STATEMENT => {
                <biome_js_syntax::JsIfStatement as AstNode>::cast(node).map(Self::JsIfStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT => {
                <biome_js_syntax::JsImport as AstNode>::cast(node).map(Self::JsImport)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_ASSERTION => {
                <biome_js_syntax::JsImportAssertion as AstNode>::cast(node)
                    .map(Self::JsImportAssertion)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_ASSERTION_ENTRY => {
                <biome_js_syntax::JsImportAssertionEntry as AstNode>::cast(node)
                    .map(Self::JsImportAssertionEntry)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_BARE_CLAUSE => {
                <biome_js_syntax::JsImportBareClause as AstNode>::cast(node)
                    .map(Self::JsImportBareClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_CALL_EXPRESSION => {
                <biome_js_syntax::JsImportCallExpression as AstNode>::cast(node)
                    .map(Self::JsImportCallExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_COMBINED_CLAUSE => {
                <biome_js_syntax::JsImportCombinedClause as AstNode>::cast(node)
                    .map(Self::JsImportCombinedClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_DEFAULT_CLAUSE => {
                <biome_js_syntax::JsImportDefaultClause as AstNode>::cast(node)
                    .map(Self::JsImportDefaultClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_META_EXPRESSION => {
                <biome_js_syntax::JsImportMetaExpression as AstNode>::cast(node)
                    .map(Self::JsImportMetaExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_NAMED_CLAUSE => {
                <biome_js_syntax::JsImportNamedClause as AstNode>::cast(node)
                    .map(Self::JsImportNamedClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_IMPORT_NAMESPACE_CLAUSE => {
                <biome_js_syntax::JsImportNamespaceClause as AstNode>::cast(node)
                    .map(Self::JsImportNamespaceClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_IN_EXPRESSION => {
                <biome_js_syntax::JsInExpression as AstNode>::cast(node).map(Self::JsInExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                <biome_js_syntax::JsInitializerClause as AstNode>::cast(node)
                    .map(Self::JsInitializerClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_INSTANCEOF_EXPRESSION => {
                <biome_js_syntax::JsInstanceofExpression as AstNode>::cast(node)
                    .map(Self::JsInstanceofExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_LABEL => {
                <biome_js_syntax::JsLabel as AstNode>::cast(node).map(Self::JsLabel)
            }
            biome_js_syntax::JsSyntaxKind::JS_LABELED_STATEMENT => {
                <biome_js_syntax::JsLabeledStatement as AstNode>::cast(node)
                    .map(Self::JsLabeledStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_LITERAL_EXPORT_NAME => {
                <biome_js_syntax::JsLiteralExportName as AstNode>::cast(node)
                    .map(Self::JsLiteralExportName)
            }
            biome_js_syntax::JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
                <biome_js_syntax::JsLiteralMemberName as AstNode>::cast(node)
                    .map(Self::JsLiteralMemberName)
            }
            biome_js_syntax::JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                <biome_js_syntax::JsLogicalExpression as AstNode>::cast(node)
                    .map(Self::JsLogicalExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_METAVARIABLE => {
                <biome_js_syntax::JsMetavariable as AstNode>::cast(node).map(Self::JsMetavariable)
            }
            biome_js_syntax::JsSyntaxKind::JS_METHOD_CLASS_MEMBER => {
                <biome_js_syntax::JsMethodClassMember as AstNode>::cast(node)
                    .map(Self::JsMethodClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_METHOD_OBJECT_MEMBER => {
                <biome_js_syntax::JsMethodObjectMember as AstNode>::cast(node)
                    .map(Self::JsMethodObjectMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_MODULE => {
                <biome_js_syntax::JsModule as AstNode>::cast(node).map(Self::JsModule)
            }
            biome_js_syntax::JsSyntaxKind::JS_MODULE_SOURCE => {
                <biome_js_syntax::JsModuleSource as AstNode>::cast(node).map(Self::JsModuleSource)
            }
            biome_js_syntax::JsSyntaxKind::JS_NAME => {
                <biome_js_syntax::JsName as AstNode>::cast(node).map(Self::JsName)
            }
            biome_js_syntax::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIER => {
                <biome_js_syntax::JsNamedImportSpecifier as AstNode>::cast(node)
                    .map(Self::JsNamedImportSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_NAMED_IMPORT_SPECIFIERS => {
                <biome_js_syntax::JsNamedImportSpecifiers as AstNode>::cast(node)
                    .map(Self::JsNamedImportSpecifiers)
            }
            biome_js_syntax::JsSyntaxKind::JS_NAMESPACE_IMPORT_SPECIFIER => {
                <biome_js_syntax::JsNamespaceImportSpecifier as AstNode>::cast(node)
                    .map(Self::JsNamespaceImportSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_NEW_EXPRESSION => {
                <biome_js_syntax::JsNewExpression as AstNode>::cast(node).map(Self::JsNewExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_NEW_TARGET_EXPRESSION => {
                <biome_js_syntax::JsNewTargetExpression as AstNode>::cast(node)
                    .map(Self::JsNewTargetExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION => {
                <biome_js_syntax::JsNullLiteralExpression as AstNode>::cast(node)
                    .map(Self::JsNullLiteralExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
                <biome_js_syntax::JsNumberLiteralExpression as AstNode>::cast(node)
                    .map(Self::JsNumberLiteralExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN => {
                <biome_js_syntax::JsObjectAssignmentPattern as AstNode>::cast(node)
                    .map(Self::JsObjectAssignmentPattern)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY => {
                <biome_js_syntax::JsObjectAssignmentPatternProperty as AstNode>::cast(node)
                    .map(Self::JsObjectAssignmentPatternProperty)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_REST => {
                <biome_js_syntax::JsObjectAssignmentPatternRest as AstNode>::cast(node)
                    .map(Self::JsObjectAssignmentPatternRest)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY => {
                <biome_js_syntax::JsObjectAssignmentPatternShorthandProperty as AstNode>::cast(node)
                    .map(Self::JsObjectAssignmentPatternShorthandProperty)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN => {
                <biome_js_syntax::JsObjectBindingPattern as AstNode>::cast(node)
                    .map(Self::JsObjectBindingPattern)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_PROPERTY => {
                <biome_js_syntax::JsObjectBindingPatternProperty as AstNode>::cast(node)
                    .map(Self::JsObjectBindingPatternProperty)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_REST => {
                <biome_js_syntax::JsObjectBindingPatternRest as AstNode>::cast(node)
                    .map(Self::JsObjectBindingPatternRest)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY => {
                <biome_js_syntax::JsObjectBindingPatternShorthandProperty as AstNode>::cast(node)
                    .map(Self::JsObjectBindingPatternShorthandProperty)
            }
            biome_js_syntax::JsSyntaxKind::JS_OBJECT_EXPRESSION => {
                <biome_js_syntax::JsObjectExpression as AstNode>::cast(node)
                    .map(Self::JsObjectExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_PARAMETERS => {
                <biome_js_syntax::JsParameters as AstNode>::cast(node).map(Self::JsParameters)
            }
            biome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT => {
                <biome_js_syntax::JsParenthesizedAssignment as AstNode>::cast(node)
                    .map(Self::JsParenthesizedAssignment)
            }
            biome_js_syntax::JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => {
                <biome_js_syntax::JsParenthesizedExpression as AstNode>::cast(node)
                    .map(Self::JsParenthesizedExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_POST_UPDATE_EXPRESSION => {
                <biome_js_syntax::JsPostUpdateExpression as AstNode>::cast(node)
                    .map(Self::JsPostUpdateExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION => {
                <biome_js_syntax::JsPreUpdateExpression as AstNode>::cast(node)
                    .map(Self::JsPreUpdateExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_PRIVATE_CLASS_MEMBER_NAME => {
                <biome_js_syntax::JsPrivateClassMemberName as AstNode>::cast(node)
                    .map(Self::JsPrivateClassMemberName)
            }
            biome_js_syntax::JsSyntaxKind::JS_PRIVATE_NAME => {
                <biome_js_syntax::JsPrivateName as AstNode>::cast(node).map(Self::JsPrivateName)
            }
            biome_js_syntax::JsSyntaxKind::JS_PROPERTY_CLASS_MEMBER => {
                <biome_js_syntax::JsPropertyClassMember as AstNode>::cast(node)
                    .map(Self::JsPropertyClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER => {
                <biome_js_syntax::JsPropertyObjectMember as AstNode>::cast(node)
                    .map(Self::JsPropertyObjectMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
                <biome_js_syntax::JsReferenceIdentifier as AstNode>::cast(node)
                    .map(Self::JsReferenceIdentifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_REGEX_LITERAL_EXPRESSION => {
                <biome_js_syntax::JsRegexLiteralExpression as AstNode>::cast(node)
                    .map(Self::JsRegexLiteralExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_REST_PARAMETER => {
                <biome_js_syntax::JsRestParameter as AstNode>::cast(node).map(Self::JsRestParameter)
            }
            biome_js_syntax::JsSyntaxKind::JS_RETURN_STATEMENT => {
                <biome_js_syntax::JsReturnStatement as AstNode>::cast(node)
                    .map(Self::JsReturnStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_SCRIPT => {
                <biome_js_syntax::JsScript as AstNode>::cast(node).map(Self::JsScript)
            }
            biome_js_syntax::JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                <biome_js_syntax::JsSequenceExpression as AstNode>::cast(node)
                    .map(Self::JsSequenceExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_SETTER_CLASS_MEMBER => {
                <biome_js_syntax::JsSetterClassMember as AstNode>::cast(node)
                    .map(Self::JsSetterClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_SETTER_OBJECT_MEMBER => {
                <biome_js_syntax::JsSetterObjectMember as AstNode>::cast(node)
                    .map(Self::JsSetterObjectMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_SHORTHAND_NAMED_IMPORT_SPECIFIER => {
                <biome_js_syntax::JsShorthandNamedImportSpecifier as AstNode>::cast(node)
                    .map(Self::JsShorthandNamedImportSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                <biome_js_syntax::JsShorthandPropertyObjectMember as AstNode>::cast(node)
                    .map(Self::JsShorthandPropertyObjectMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_SPREAD => {
                <biome_js_syntax::JsSpread as AstNode>::cast(node).map(Self::JsSpread)
            }
            biome_js_syntax::JsSyntaxKind::JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER => {
                <biome_js_syntax::JsStaticInitializationBlockClassMember as AstNode>::cast(node)
                    .map(Self::JsStaticInitializationBlockClassMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT => {
                <biome_js_syntax::JsStaticMemberAssignment as AstNode>::cast(node)
                    .map(Self::JsStaticMemberAssignment)
            }
            biome_js_syntax::JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => {
                <biome_js_syntax::JsStaticMemberExpression as AstNode>::cast(node)
                    .map(Self::JsStaticMemberExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_STATIC_MODIFIER => {
                <biome_js_syntax::JsStaticModifier as AstNode>::cast(node)
                    .map(Self::JsStaticModifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
                <biome_js_syntax::JsStringLiteralExpression as AstNode>::cast(node)
                    .map(Self::JsStringLiteralExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_SUPER_EXPRESSION => {
                <biome_js_syntax::JsSuperExpression as AstNode>::cast(node)
                    .map(Self::JsSuperExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_SVELTE_SNIPPET_ROOT => {
                <biome_js_syntax::JsSvelteSnippetRoot as AstNode>::cast(node)
                    .map(Self::JsSvelteSnippetRoot)
            }
            biome_js_syntax::JsSyntaxKind::JS_SWITCH_STATEMENT => {
                <biome_js_syntax::JsSwitchStatement as AstNode>::cast(node)
                    .map(Self::JsSwitchStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_TEMPLATE_CHUNK_ELEMENT => {
                <biome_js_syntax::JsTemplateChunkElement as AstNode>::cast(node)
                    .map(Self::JsTemplateChunkElement)
            }
            biome_js_syntax::JsSyntaxKind::JS_TEMPLATE_ELEMENT => {
                <biome_js_syntax::JsTemplateElement as AstNode>::cast(node)
                    .map(Self::JsTemplateElement)
            }
            biome_js_syntax::JsSyntaxKind::JS_TEMPLATE_EXPRESSION => {
                <biome_js_syntax::JsTemplateExpression as AstNode>::cast(node)
                    .map(Self::JsTemplateExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_THIS_EXPRESSION => {
                <biome_js_syntax::JsThisExpression as AstNode>::cast(node)
                    .map(Self::JsThisExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_THROW_STATEMENT => {
                <biome_js_syntax::JsThrowStatement as AstNode>::cast(node)
                    .map(Self::JsThrowStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_TRY_FINALLY_STATEMENT => {
                <biome_js_syntax::JsTryFinallyStatement as AstNode>::cast(node)
                    .map(Self::JsTryFinallyStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_TRY_STATEMENT => {
                <biome_js_syntax::JsTryStatement as AstNode>::cast(node).map(Self::JsTryStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_UNARY_EXPRESSION => {
                <biome_js_syntax::JsUnaryExpression as AstNode>::cast(node)
                    .map(Self::JsUnaryExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATION => {
                <biome_js_syntax::JsVariableDeclaration as AstNode>::cast(node)
                    .map(Self::JsVariableDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATION_CLAUSE => {
                <biome_js_syntax::JsVariableDeclarationClause as AstNode>::cast(node)
                    .map(Self::JsVariableDeclarationClause)
            }
            biome_js_syntax::JsSyntaxKind::JS_VARIABLE_DECLARATOR => {
                <biome_js_syntax::JsVariableDeclarator as AstNode>::cast(node)
                    .map(Self::JsVariableDeclarator)
            }
            biome_js_syntax::JsSyntaxKind::JS_VARIABLE_STATEMENT => {
                <biome_js_syntax::JsVariableStatement as AstNode>::cast(node)
                    .map(Self::JsVariableStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_WHILE_STATEMENT => {
                <biome_js_syntax::JsWhileStatement as AstNode>::cast(node)
                    .map(Self::JsWhileStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_WITH_STATEMENT => {
                <biome_js_syntax::JsWithStatement as AstNode>::cast(node).map(Self::JsWithStatement)
            }
            biome_js_syntax::JsSyntaxKind::JS_YIELD_ARGUMENT => {
                <biome_js_syntax::JsYieldArgument as AstNode>::cast(node).map(Self::JsYieldArgument)
            }
            biome_js_syntax::JsSyntaxKind::JS_YIELD_EXPRESSION => {
                <biome_js_syntax::JsYieldExpression as AstNode>::cast(node)
                    .map(Self::JsYieldExpression)
            }
            biome_js_syntax::JsSyntaxKind::JSX_ATTRIBUTE => {
                <biome_js_syntax::JsxAttribute as AstNode>::cast(node).map(Self::JsxAttribute)
            }
            biome_js_syntax::JsSyntaxKind::JSX_ATTRIBUTE_INITIALIZER_CLAUSE => {
                <biome_js_syntax::JsxAttributeInitializerClause as AstNode>::cast(node)
                    .map(Self::JsxAttributeInitializerClause)
            }
            biome_js_syntax::JsSyntaxKind::JSX_CLOSING_ELEMENT => {
                <biome_js_syntax::JsxClosingElement as AstNode>::cast(node)
                    .map(Self::JsxClosingElement)
            }
            biome_js_syntax::JsSyntaxKind::JSX_CLOSING_FRAGMENT => {
                <biome_js_syntax::JsxClosingFragment as AstNode>::cast(node)
                    .map(Self::JsxClosingFragment)
            }
            biome_js_syntax::JsSyntaxKind::JSX_ELEMENT => {
                <biome_js_syntax::JsxElement as AstNode>::cast(node).map(Self::JsxElement)
            }
            biome_js_syntax::JsSyntaxKind::JSX_EXPRESSION_ATTRIBUTE_VALUE => {
                <biome_js_syntax::JsxExpressionAttributeValue as AstNode>::cast(node)
                    .map(Self::JsxExpressionAttributeValue)
            }
            biome_js_syntax::JsSyntaxKind::JSX_EXPRESSION_CHILD => {
                <biome_js_syntax::JsxExpressionChild as AstNode>::cast(node)
                    .map(Self::JsxExpressionChild)
            }
            biome_js_syntax::JsSyntaxKind::JSX_FRAGMENT => {
                <biome_js_syntax::JsxFragment as AstNode>::cast(node).map(Self::JsxFragment)
            }
            biome_js_syntax::JsSyntaxKind::JSX_MEMBER_NAME => {
                <biome_js_syntax::JsxMemberName as AstNode>::cast(node).map(Self::JsxMemberName)
            }
            biome_js_syntax::JsSyntaxKind::JSX_NAME => {
                <biome_js_syntax::JsxName as AstNode>::cast(node).map(Self::JsxName)
            }
            biome_js_syntax::JsSyntaxKind::JSX_NAMESPACE_NAME => {
                <biome_js_syntax::JsxNamespaceName as AstNode>::cast(node)
                    .map(Self::JsxNamespaceName)
            }
            biome_js_syntax::JsSyntaxKind::JSX_OPENING_ELEMENT => {
                <biome_js_syntax::JsxOpeningElement as AstNode>::cast(node)
                    .map(Self::JsxOpeningElement)
            }
            biome_js_syntax::JsSyntaxKind::JSX_OPENING_FRAGMENT => {
                <biome_js_syntax::JsxOpeningFragment as AstNode>::cast(node)
                    .map(Self::JsxOpeningFragment)
            }
            biome_js_syntax::JsSyntaxKind::JSX_REFERENCE_IDENTIFIER => {
                <biome_js_syntax::JsxReferenceIdentifier as AstNode>::cast(node)
                    .map(Self::JsxReferenceIdentifier)
            }
            biome_js_syntax::JsSyntaxKind::JSX_SELF_CLOSING_ELEMENT => {
                <biome_js_syntax::JsxSelfClosingElement as AstNode>::cast(node)
                    .map(Self::JsxSelfClosingElement)
            }
            biome_js_syntax::JsSyntaxKind::JSX_SHORTHAND_ATTRIBUTE => {
                <biome_js_syntax::JsxShorthandAttribute as AstNode>::cast(node)
                    .map(Self::JsxShorthandAttribute)
            }
            biome_js_syntax::JsSyntaxKind::JSX_SPREAD_ATTRIBUTE => {
                <biome_js_syntax::JsxSpreadAttribute as AstNode>::cast(node)
                    .map(Self::JsxSpreadAttribute)
            }
            biome_js_syntax::JsSyntaxKind::JSX_SPREAD_CHILD => {
                <biome_js_syntax::JsxSpreadChild as AstNode>::cast(node).map(Self::JsxSpreadChild)
            }
            biome_js_syntax::JsSyntaxKind::JSX_STRING => {
                <biome_js_syntax::JsxString as AstNode>::cast(node).map(Self::JsxString)
            }
            biome_js_syntax::JsSyntaxKind::JSX_TAG_EXPRESSION => {
                <biome_js_syntax::JsxTagExpression as AstNode>::cast(node)
                    .map(Self::JsxTagExpression)
            }
            biome_js_syntax::JsSyntaxKind::JSX_TEXT => {
                <biome_js_syntax::JsxText as AstNode>::cast(node).map(Self::JsxText)
            }
            biome_js_syntax::JsSyntaxKind::TS_ABSTRACT_MODIFIER => {
                <biome_js_syntax::TsAbstractModifier as AstNode>::cast(node)
                    .map(Self::TsAbstractModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_ACCESSIBILITY_MODIFIER => {
                <biome_js_syntax::TsAccessibilityModifier as AstNode>::cast(node)
                    .map(Self::TsAccessibilityModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_ANY_TYPE => {
                <biome_js_syntax::TsAnyType as AstNode>::cast(node).map(Self::TsAnyType)
            }
            biome_js_syntax::JsSyntaxKind::TS_ARRAY_TYPE => {
                <biome_js_syntax::TsArrayType as AstNode>::cast(node).map(Self::TsArrayType)
            }
            biome_js_syntax::JsSyntaxKind::TS_AS_ASSIGNMENT => {
                <biome_js_syntax::TsAsAssignment as AstNode>::cast(node).map(Self::TsAsAssignment)
            }
            biome_js_syntax::JsSyntaxKind::TS_AS_EXPRESSION => {
                <biome_js_syntax::TsAsExpression as AstNode>::cast(node).map(Self::TsAsExpression)
            }
            biome_js_syntax::JsSyntaxKind::TS_ASSERTS_CONDITION => {
                <biome_js_syntax::TsAssertsCondition as AstNode>::cast(node)
                    .map(Self::TsAssertsCondition)
            }
            biome_js_syntax::JsSyntaxKind::TS_ASSERTS_RETURN_TYPE => {
                <biome_js_syntax::TsAssertsReturnType as AstNode>::cast(node)
                    .map(Self::TsAssertsReturnType)
            }
            biome_js_syntax::JsSyntaxKind::TS_BIGINT_LITERAL_TYPE => {
                <biome_js_syntax::TsBigintLiteralType as AstNode>::cast(node)
                    .map(Self::TsBigintLiteralType)
            }
            biome_js_syntax::JsSyntaxKind::TS_BIGINT_TYPE => {
                <biome_js_syntax::TsBigintType as AstNode>::cast(node).map(Self::TsBigintType)
            }
            biome_js_syntax::JsSyntaxKind::TS_BOOLEAN_LITERAL_TYPE => {
                <biome_js_syntax::TsBooleanLiteralType as AstNode>::cast(node)
                    .map(Self::TsBooleanLiteralType)
            }
            biome_js_syntax::JsSyntaxKind::TS_BOOLEAN_TYPE => {
                <biome_js_syntax::TsBooleanType as AstNode>::cast(node).map(Self::TsBooleanType)
            }
            biome_js_syntax::JsSyntaxKind::TS_CALL_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsCallSignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsCallSignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                <biome_js_syntax::TsConditionalType as AstNode>::cast(node)
                    .map(Self::TsConditionalType)
            }
            biome_js_syntax::JsSyntaxKind::TS_CONST_MODIFIER => {
                <biome_js_syntax::TsConstModifier as AstNode>::cast(node).map(Self::TsConstModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsConstructSignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsConstructSignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsConstructorSignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsConstructorSignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_CONSTRUCTOR_TYPE => {
                <biome_js_syntax::TsConstructorType as AstNode>::cast(node)
                    .map(Self::TsConstructorType)
            }
            biome_js_syntax::JsSyntaxKind::TS_DECLARATION_MODULE => {
                <biome_js_syntax::TsDeclarationModule as AstNode>::cast(node)
                    .map(Self::TsDeclarationModule)
            }
            biome_js_syntax::JsSyntaxKind::TS_DECLARE_FUNCTION_DECLARATION => {
                <biome_js_syntax::TsDeclareFunctionDeclaration as AstNode>::cast(node)
                    .map(Self::TsDeclareFunctionDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION => {
                <biome_js_syntax::TsDeclareFunctionExportDefaultDeclaration as AstNode>::cast(node)
                    .map(Self::TsDeclareFunctionExportDefaultDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_DECLARE_MODIFIER => {
                <biome_js_syntax::TsDeclareModifier as AstNode>::cast(node)
                    .map(Self::TsDeclareModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_DECLARE_STATEMENT => {
                <biome_js_syntax::TsDeclareStatement as AstNode>::cast(node)
                    .map(Self::TsDeclareStatement)
            }
            biome_js_syntax::JsSyntaxKind::TS_DEFAULT_TYPE_CLAUSE => {
                <biome_js_syntax::TsDefaultTypeClause as AstNode>::cast(node)
                    .map(Self::TsDefaultTypeClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_DEFINITE_PROPERTY_ANNOTATION => {
                <biome_js_syntax::TsDefinitePropertyAnnotation as AstNode>::cast(node)
                    .map(Self::TsDefinitePropertyAnnotation)
            }
            biome_js_syntax::JsSyntaxKind::TS_DEFINITE_VARIABLE_ANNOTATION => {
                <biome_js_syntax::TsDefiniteVariableAnnotation as AstNode>::cast(node)
                    .map(Self::TsDefiniteVariableAnnotation)
            }
            biome_js_syntax::JsSyntaxKind::TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY => {
                <biome_js_syntax::TsEmptyExternalModuleDeclarationBody as AstNode>::cast(node)
                    .map(Self::TsEmptyExternalModuleDeclarationBody)
            }
            biome_js_syntax::JsSyntaxKind::TS_ENUM_DECLARATION => {
                <biome_js_syntax::TsEnumDeclaration as AstNode>::cast(node)
                    .map(Self::TsEnumDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_ENUM_MEMBER => {
                <biome_js_syntax::TsEnumMember as AstNode>::cast(node).map(Self::TsEnumMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE => {
                <biome_js_syntax::TsExportAsNamespaceClause as AstNode>::cast(node)
                    .map(Self::TsExportAsNamespaceClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_EXPORT_ASSIGNMENT_CLAUSE => {
                <biome_js_syntax::TsExportAssignmentClause as AstNode>::cast(node)
                    .map(Self::TsExportAssignmentClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_EXPORT_DECLARE_CLAUSE => {
                <biome_js_syntax::TsExportDeclareClause as AstNode>::cast(node)
                    .map(Self::TsExportDeclareClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_EXTENDS_CLAUSE => {
                <biome_js_syntax::TsExtendsClause as AstNode>::cast(node).map(Self::TsExtendsClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_EXTERNAL_MODULE_DECLARATION => {
                <biome_js_syntax::TsExternalModuleDeclaration as AstNode>::cast(node)
                    .map(Self::TsExternalModuleDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_EXTERNAL_MODULE_REFERENCE => {
                <biome_js_syntax::TsExternalModuleReference as AstNode>::cast(node)
                    .map(Self::TsExternalModuleReference)
            }
            biome_js_syntax::JsSyntaxKind::TS_FUNCTION_TYPE => {
                <biome_js_syntax::TsFunctionType as AstNode>::cast(node).map(Self::TsFunctionType)
            }
            biome_js_syntax::JsSyntaxKind::TS_GETTER_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsGetterSignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsGetterSignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_GETTER_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsGetterSignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsGetterSignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_GLOBAL_DECLARATION => {
                <biome_js_syntax::TsGlobalDeclaration as AstNode>::cast(node)
                    .map(Self::TsGlobalDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_IDENTIFIER_BINDING => {
                <biome_js_syntax::TsIdentifierBinding as AstNode>::cast(node)
                    .map(Self::TsIdentifierBinding)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPLEMENTS_CLAUSE => {
                <biome_js_syntax::TsImplementsClause as AstNode>::cast(node)
                    .map(Self::TsImplementsClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPORT_EQUALS_DECLARATION => {
                <biome_js_syntax::TsImportEqualsDeclaration as AstNode>::cast(node)
                    .map(Self::TsImportEqualsDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE => {
                <biome_js_syntax::TsImportType as AstNode>::cast(node).map(Self::TsImportType)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE_ARGUMENTS => {
                <biome_js_syntax::TsImportTypeArguments as AstNode>::cast(node)
                    .map(Self::TsImportTypeArguments)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION => {
                <biome_js_syntax::TsImportTypeAssertion as AstNode>::cast(node)
                    .map(Self::TsImportTypeAssertion)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE_ASSERTION_BLOCK => {
                <biome_js_syntax::TsImportTypeAssertionBlock as AstNode>::cast(node)
                    .map(Self::TsImportTypeAssertionBlock)
            }
            biome_js_syntax::JsSyntaxKind::TS_IMPORT_TYPE_QUALIFIER => {
                <biome_js_syntax::TsImportTypeQualifier as AstNode>::cast(node)
                    .map(Self::TsImportTypeQualifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_IN_MODIFIER => {
                <biome_js_syntax::TsInModifier as AstNode>::cast(node).map(Self::TsInModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsIndexSignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsIndexSignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_PARAMETER => {
                <biome_js_syntax::TsIndexSignatureParameter as AstNode>::cast(node)
                    .map(Self::TsIndexSignatureParameter)
            }
            biome_js_syntax::JsSyntaxKind::TS_INDEX_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsIndexSignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsIndexSignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                <biome_js_syntax::TsIndexedAccessType as AstNode>::cast(node)
                    .map(Self::TsIndexedAccessType)
            }
            biome_js_syntax::JsSyntaxKind::TS_INFER_TYPE => {
                <biome_js_syntax::TsInferType as AstNode>::cast(node).map(Self::TsInferType)
            }
            biome_js_syntax::JsSyntaxKind::TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsInitializedPropertySignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsInitializedPropertySignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_INSTANTIATION_EXPRESSION => {
                <biome_js_syntax::TsInstantiationExpression as AstNode>::cast(node)
                    .map(Self::TsInstantiationExpression)
            }
            biome_js_syntax::JsSyntaxKind::TS_INTERFACE_DECLARATION => {
                <biome_js_syntax::TsInterfaceDeclaration as AstNode>::cast(node)
                    .map(Self::TsInterfaceDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_INTERSECTION_TYPE => {
                <biome_js_syntax::TsIntersectionType as AstNode>::cast(node)
                    .map(Self::TsIntersectionType)
            }
            biome_js_syntax::JsSyntaxKind::TS_LITERAL_ENUM_MEMBER_NAME => {
                <biome_js_syntax::TsLiteralEnumMemberName as AstNode>::cast(node)
                    .map(Self::TsLiteralEnumMemberName)
            }
            biome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE => {
                <biome_js_syntax::TsMappedType as AstNode>::cast(node).map(Self::TsMappedType)
            }
            biome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE_AS_CLAUSE => {
                <biome_js_syntax::TsMappedTypeAsClause as AstNode>::cast(node)
                    .map(Self::TsMappedTypeAsClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE => {
                <biome_js_syntax::TsMappedTypeOptionalModifierClause as AstNode>::cast(node)
                    .map(Self::TsMappedTypeOptionalModifierClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE => {
                <biome_js_syntax::TsMappedTypeReadonlyModifierClause as AstNode>::cast(node)
                    .map(Self::TsMappedTypeReadonlyModifierClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_METHOD_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsMethodSignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsMethodSignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_METHOD_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsMethodSignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsMethodSignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_MODULE_BLOCK => {
                <biome_js_syntax::TsModuleBlock as AstNode>::cast(node).map(Self::TsModuleBlock)
            }
            biome_js_syntax::JsSyntaxKind::TS_MODULE_DECLARATION => {
                <biome_js_syntax::TsModuleDeclaration as AstNode>::cast(node)
                    .map(Self::TsModuleDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_NAMED_TUPLE_TYPE_ELEMENT => {
                <biome_js_syntax::TsNamedTupleTypeElement as AstNode>::cast(node)
                    .map(Self::TsNamedTupleTypeElement)
            }
            biome_js_syntax::JsSyntaxKind::TS_NEVER_TYPE => {
                <biome_js_syntax::TsNeverType as AstNode>::cast(node).map(Self::TsNeverType)
            }
            biome_js_syntax::JsSyntaxKind::TS_NON_NULL_ASSERTION_ASSIGNMENT => {
                <biome_js_syntax::TsNonNullAssertionAssignment as AstNode>::cast(node)
                    .map(Self::TsNonNullAssertionAssignment)
            }
            biome_js_syntax::JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => {
                <biome_js_syntax::TsNonNullAssertionExpression as AstNode>::cast(node)
                    .map(Self::TsNonNullAssertionExpression)
            }
            biome_js_syntax::JsSyntaxKind::TS_NON_PRIMITIVE_TYPE => {
                <biome_js_syntax::TsNonPrimitiveType as AstNode>::cast(node)
                    .map(Self::TsNonPrimitiveType)
            }
            biome_js_syntax::JsSyntaxKind::TS_NULL_LITERAL_TYPE => {
                <biome_js_syntax::TsNullLiteralType as AstNode>::cast(node)
                    .map(Self::TsNullLiteralType)
            }
            biome_js_syntax::JsSyntaxKind::TS_NUMBER_LITERAL_TYPE => {
                <biome_js_syntax::TsNumberLiteralType as AstNode>::cast(node)
                    .map(Self::TsNumberLiteralType)
            }
            biome_js_syntax::JsSyntaxKind::TS_NUMBER_TYPE => {
                <biome_js_syntax::TsNumberType as AstNode>::cast(node).map(Self::TsNumberType)
            }
            biome_js_syntax::JsSyntaxKind::TS_OBJECT_TYPE => {
                <biome_js_syntax::TsObjectType as AstNode>::cast(node).map(Self::TsObjectType)
            }
            biome_js_syntax::JsSyntaxKind::TS_OPTIONAL_PROPERTY_ANNOTATION => {
                <biome_js_syntax::TsOptionalPropertyAnnotation as AstNode>::cast(node)
                    .map(Self::TsOptionalPropertyAnnotation)
            }
            biome_js_syntax::JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => {
                <biome_js_syntax::TsOptionalTupleTypeElement as AstNode>::cast(node)
                    .map(Self::TsOptionalTupleTypeElement)
            }
            biome_js_syntax::JsSyntaxKind::TS_OUT_MODIFIER => {
                <biome_js_syntax::TsOutModifier as AstNode>::cast(node).map(Self::TsOutModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_OVERRIDE_MODIFIER => {
                <biome_js_syntax::TsOverrideModifier as AstNode>::cast(node)
                    .map(Self::TsOverrideModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_PARENTHESIZED_TYPE => {
                <biome_js_syntax::TsParenthesizedType as AstNode>::cast(node)
                    .map(Self::TsParenthesizedType)
            }
            biome_js_syntax::JsSyntaxKind::TS_PREDICATE_RETURN_TYPE => {
                <biome_js_syntax::TsPredicateReturnType as AstNode>::cast(node)
                    .map(Self::TsPredicateReturnType)
            }
            biome_js_syntax::JsSyntaxKind::TS_PROPERTY_PARAMETER => {
                <biome_js_syntax::TsPropertyParameter as AstNode>::cast(node)
                    .map(Self::TsPropertyParameter)
            }
            biome_js_syntax::JsSyntaxKind::TS_PROPERTY_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsPropertySignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsPropertySignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_PROPERTY_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsPropertySignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsPropertySignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_QUALIFIED_MODULE_NAME => {
                <biome_js_syntax::TsQualifiedModuleName as AstNode>::cast(node)
                    .map(Self::TsQualifiedModuleName)
            }
            biome_js_syntax::JsSyntaxKind::TS_QUALIFIED_NAME => {
                <biome_js_syntax::TsQualifiedName as AstNode>::cast(node).map(Self::TsQualifiedName)
            }
            biome_js_syntax::JsSyntaxKind::TS_READONLY_MODIFIER => {
                <biome_js_syntax::TsReadonlyModifier as AstNode>::cast(node)
                    .map(Self::TsReadonlyModifier)
            }
            biome_js_syntax::JsSyntaxKind::TS_REFERENCE_TYPE => {
                <biome_js_syntax::TsReferenceType as AstNode>::cast(node).map(Self::TsReferenceType)
            }
            biome_js_syntax::JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT => {
                <biome_js_syntax::TsRestTupleTypeElement as AstNode>::cast(node)
                    .map(Self::TsRestTupleTypeElement)
            }
            biome_js_syntax::JsSyntaxKind::TS_RETURN_TYPE_ANNOTATION => {
                <biome_js_syntax::TsReturnTypeAnnotation as AstNode>::cast(node)
                    .map(Self::TsReturnTypeAnnotation)
            }
            biome_js_syntax::JsSyntaxKind::TS_SATISFIES_ASSIGNMENT => {
                <biome_js_syntax::TsSatisfiesAssignment as AstNode>::cast(node)
                    .map(Self::TsSatisfiesAssignment)
            }
            biome_js_syntax::JsSyntaxKind::TS_SATISFIES_EXPRESSION => {
                <biome_js_syntax::TsSatisfiesExpression as AstNode>::cast(node)
                    .map(Self::TsSatisfiesExpression)
            }
            biome_js_syntax::JsSyntaxKind::TS_SETTER_SIGNATURE_CLASS_MEMBER => {
                <biome_js_syntax::TsSetterSignatureClassMember as AstNode>::cast(node)
                    .map(Self::TsSetterSignatureClassMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_SETTER_SIGNATURE_TYPE_MEMBER => {
                <biome_js_syntax::TsSetterSignatureTypeMember as AstNode>::cast(node)
                    .map(Self::TsSetterSignatureTypeMember)
            }
            biome_js_syntax::JsSyntaxKind::TS_STRING_LITERAL_TYPE => {
                <biome_js_syntax::TsStringLiteralType as AstNode>::cast(node)
                    .map(Self::TsStringLiteralType)
            }
            biome_js_syntax::JsSyntaxKind::TS_STRING_TYPE => {
                <biome_js_syntax::TsStringType as AstNode>::cast(node).map(Self::TsStringType)
            }
            biome_js_syntax::JsSyntaxKind::TS_SYMBOL_TYPE => {
                <biome_js_syntax::TsSymbolType as AstNode>::cast(node).map(Self::TsSymbolType)
            }
            biome_js_syntax::JsSyntaxKind::TS_TEMPLATE_CHUNK_ELEMENT => {
                <biome_js_syntax::TsTemplateChunkElement as AstNode>::cast(node)
                    .map(Self::TsTemplateChunkElement)
            }
            biome_js_syntax::JsSyntaxKind::TS_TEMPLATE_ELEMENT => {
                <biome_js_syntax::TsTemplateElement as AstNode>::cast(node)
                    .map(Self::TsTemplateElement)
            }
            biome_js_syntax::JsSyntaxKind::TS_TEMPLATE_LITERAL_TYPE => {
                <biome_js_syntax::TsTemplateLiteralType as AstNode>::cast(node)
                    .map(Self::TsTemplateLiteralType)
            }
            biome_js_syntax::JsSyntaxKind::TS_THIS_PARAMETER => {
                <biome_js_syntax::TsThisParameter as AstNode>::cast(node).map(Self::TsThisParameter)
            }
            biome_js_syntax::JsSyntaxKind::TS_THIS_TYPE => {
                <biome_js_syntax::TsThisType as AstNode>::cast(node).map(Self::TsThisType)
            }
            biome_js_syntax::JsSyntaxKind::TS_TUPLE_TYPE => {
                <biome_js_syntax::TsTupleType as AstNode>::cast(node).map(Self::TsTupleType)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION => {
                <biome_js_syntax::TsTypeAliasDeclaration as AstNode>::cast(node)
                    .map(Self::TsTypeAliasDeclaration)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_ANNOTATION => {
                <biome_js_syntax::TsTypeAnnotation as AstNode>::cast(node)
                    .map(Self::TsTypeAnnotation)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_ARGUMENTS => {
                <biome_js_syntax::TsTypeArguments as AstNode>::cast(node).map(Self::TsTypeArguments)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT => {
                <biome_js_syntax::TsTypeAssertionAssignment as AstNode>::cast(node)
                    .map(Self::TsTypeAssertionAssignment)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION => {
                <biome_js_syntax::TsTypeAssertionExpression as AstNode>::cast(node)
                    .map(Self::TsTypeAssertionExpression)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_CONSTRAINT_CLAUSE => {
                <biome_js_syntax::TsTypeConstraintClause as AstNode>::cast(node)
                    .map(Self::TsTypeConstraintClause)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_OPERATOR_TYPE => {
                <biome_js_syntax::TsTypeOperatorType as AstNode>::cast(node)
                    .map(Self::TsTypeOperatorType)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETER => {
                <biome_js_syntax::TsTypeParameter as AstNode>::cast(node).map(Self::TsTypeParameter)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
                <biome_js_syntax::TsTypeParameterName as AstNode>::cast(node)
                    .map(Self::TsTypeParameterName)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPE_PARAMETERS => {
                <biome_js_syntax::TsTypeParameters as AstNode>::cast(node)
                    .map(Self::TsTypeParameters)
            }
            biome_js_syntax::JsSyntaxKind::TS_TYPEOF_TYPE => {
                <biome_js_syntax::TsTypeofType as AstNode>::cast(node).map(Self::TsTypeofType)
            }
            biome_js_syntax::JsSyntaxKind::TS_UNDEFINED_TYPE => {
                <biome_js_syntax::TsUndefinedType as AstNode>::cast(node).map(Self::TsUndefinedType)
            }
            biome_js_syntax::JsSyntaxKind::TS_UNION_TYPE => {
                <biome_js_syntax::TsUnionType as AstNode>::cast(node).map(Self::TsUnionType)
            }
            biome_js_syntax::JsSyntaxKind::TS_UNKNOWN_TYPE => {
                <biome_js_syntax::TsUnknownType as AstNode>::cast(node).map(Self::TsUnknownType)
            }
            biome_js_syntax::JsSyntaxKind::TS_VOID_TYPE => {
                <biome_js_syntax::TsVoidType as AstNode>::cast(node).map(Self::TsVoidType)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS => {
                <biome_js_syntax::JsBogus as AstNode>::cast(node).map(Self::JsBogus)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_ASSIGNMENT => {
                <biome_js_syntax::JsBogusAssignment as AstNode>::cast(node)
                    .map(Self::JsBogusAssignment)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_BINDING => {
                <biome_js_syntax::JsBogusBinding as AstNode>::cast(node).map(Self::JsBogusBinding)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_EXPRESSION => {
                <biome_js_syntax::JsBogusExpression as AstNode>::cast(node)
                    .map(Self::JsBogusExpression)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_IMPORT_ASSERTION_ENTRY => {
                <biome_js_syntax::JsBogusImportAssertionEntry as AstNode>::cast(node)
                    .map(Self::JsBogusImportAssertionEntry)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_MEMBER => {
                <biome_js_syntax::JsBogusMember as AstNode>::cast(node).map(Self::JsBogusMember)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_NAMED_IMPORT_SPECIFIER => {
                <biome_js_syntax::JsBogusNamedImportSpecifier as AstNode>::cast(node)
                    .map(Self::JsBogusNamedImportSpecifier)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_PARAMETER => {
                <biome_js_syntax::JsBogusParameter as AstNode>::cast(node)
                    .map(Self::JsBogusParameter)
            }
            biome_js_syntax::JsSyntaxKind::JS_BOGUS_STATEMENT => {
                <biome_js_syntax::JsBogusStatement as AstNode>::cast(node)
                    .map(Self::JsBogusStatement)
            }
            biome_js_syntax::JsSyntaxKind::TS_BOGUS_TYPE => {
                <biome_js_syntax::TsBogusType as AstNode>::cast(node).map(Self::TsBogusType)
            }
            _ => None,
        }
    }
    fn syntax(&self) -> &JsSyntaxNode {
        match self {
            Self::JsAccessorModifier(node) => node.syntax(),
            Self::JsArrayAssignmentPattern(node) => node.syntax(),
            Self::JsArrayAssignmentPatternElement(node) => node.syntax(),
            Self::JsArrayAssignmentPatternRestElement(node) => node.syntax(),
            Self::JsArrayBindingPattern(node) => node.syntax(),
            Self::JsArrayBindingPatternElement(node) => node.syntax(),
            Self::JsArrayBindingPatternRestElement(node) => node.syntax(),
            Self::JsArrayExpression(node) => node.syntax(),
            Self::JsArrayHole(node) => node.syntax(),
            Self::JsArrowFunctionExpression(node) => node.syntax(),
            Self::JsAssignmentExpression(node) => node.syntax(),
            Self::JsAwaitExpression(node) => node.syntax(),
            Self::JsBigintLiteralExpression(node) => node.syntax(),
            Self::JsBinaryExpression(node) => node.syntax(),
            Self::JsBlockStatement(node) => node.syntax(),
            Self::JsBooleanLiteralExpression(node) => node.syntax(),
            Self::JsBreakStatement(node) => node.syntax(),
            Self::JsCallArguments(node) => node.syntax(),
            Self::JsCallExpression(node) => node.syntax(),
            Self::JsCaseClause(node) => node.syntax(),
            Self::JsCatchClause(node) => node.syntax(),
            Self::JsCatchDeclaration(node) => node.syntax(),
            Self::JsClassDeclaration(node) => node.syntax(),
            Self::JsClassExportDefaultDeclaration(node) => node.syntax(),
            Self::JsClassExpression(node) => node.syntax(),
            Self::JsComputedMemberAssignment(node) => node.syntax(),
            Self::JsComputedMemberExpression(node) => node.syntax(),
            Self::JsComputedMemberName(node) => node.syntax(),
            Self::JsConditionalExpression(node) => node.syntax(),
            Self::JsConstructorClassMember(node) => node.syntax(),
            Self::JsConstructorParameters(node) => node.syntax(),
            Self::JsContinueStatement(node) => node.syntax(),
            Self::JsDebuggerStatement(node) => node.syntax(),
            Self::JsDecorator(node) => node.syntax(),
            Self::JsDefaultClause(node) => node.syntax(),
            Self::JsDefaultImportSpecifier(node) => node.syntax(),
            Self::JsDirective(node) => node.syntax(),
            Self::JsDoWhileStatement(node) => node.syntax(),
            Self::JsElseClause(node) => node.syntax(),
            Self::JsEmptyClassMember(node) => node.syntax(),
            Self::JsEmptyStatement(node) => node.syntax(),
            Self::JsExport(node) => node.syntax(),
            Self::JsExportAsClause(node) => node.syntax(),
            Self::JsExportDefaultDeclarationClause(node) => node.syntax(),
            Self::JsExportDefaultExpressionClause(node) => node.syntax(),
            Self::JsExportFromClause(node) => node.syntax(),
            Self::JsExportNamedClause(node) => node.syntax(),
            Self::JsExportNamedFromClause(node) => node.syntax(),
            Self::JsExportNamedFromSpecifier(node) => node.syntax(),
            Self::JsExportNamedShorthandSpecifier(node) => node.syntax(),
            Self::JsExportNamedSpecifier(node) => node.syntax(),
            Self::JsExpressionSnippet(node) => node.syntax(),
            Self::JsExpressionStatement(node) => node.syntax(),
            Self::JsExpressionTemplateRoot(node) => node.syntax(),
            Self::JsExtendsClause(node) => node.syntax(),
            Self::JsFinallyClause(node) => node.syntax(),
            Self::JsForInStatement(node) => node.syntax(),
            Self::JsForOfStatement(node) => node.syntax(),
            Self::JsForStatement(node) => node.syntax(),
            Self::JsForVariableDeclaration(node) => node.syntax(),
            Self::JsFormalParameter(node) => node.syntax(),
            Self::JsFunctionBody(node) => node.syntax(),
            Self::JsFunctionDeclaration(node) => node.syntax(),
            Self::JsFunctionExportDefaultDeclaration(node) => node.syntax(),
            Self::JsFunctionExpression(node) => node.syntax(),
            Self::JsGetterClassMember(node) => node.syntax(),
            Self::JsGetterObjectMember(node) => node.syntax(),
            Self::JsIdentifierAssignment(node) => node.syntax(),
            Self::JsIdentifierBinding(node) => node.syntax(),
            Self::JsIdentifierExpression(node) => node.syntax(),
            Self::JsIfStatement(node) => node.syntax(),
            Self::JsImport(node) => node.syntax(),
            Self::JsImportAssertion(node) => node.syntax(),
            Self::JsImportAssertionEntry(node) => node.syntax(),
            Self::JsImportBareClause(node) => node.syntax(),
            Self::JsImportCallExpression(node) => node.syntax(),
            Self::JsImportCombinedClause(node) => node.syntax(),
            Self::JsImportDefaultClause(node) => node.syntax(),
            Self::JsImportMetaExpression(node) => node.syntax(),
            Self::JsImportNamedClause(node) => node.syntax(),
            Self::JsImportNamespaceClause(node) => node.syntax(),
            Self::JsInExpression(node) => node.syntax(),
            Self::JsInitializerClause(node) => node.syntax(),
            Self::JsInstanceofExpression(node) => node.syntax(),
            Self::JsLabel(node) => node.syntax(),
            Self::JsLabeledStatement(node) => node.syntax(),
            Self::JsLiteralExportName(node) => node.syntax(),
            Self::JsLiteralMemberName(node) => node.syntax(),
            Self::JsLogicalExpression(node) => node.syntax(),
            Self::JsMetavariable(node) => node.syntax(),
            Self::JsMethodClassMember(node) => node.syntax(),
            Self::JsMethodObjectMember(node) => node.syntax(),
            Self::JsModule(node) => node.syntax(),
            Self::JsModuleSource(node) => node.syntax(),
            Self::JsName(node) => node.syntax(),
            Self::JsNamedImportSpecifier(node) => node.syntax(),
            Self::JsNamedImportSpecifiers(node) => node.syntax(),
            Self::JsNamespaceImportSpecifier(node) => node.syntax(),
            Self::JsNewExpression(node) => node.syntax(),
            Self::JsNewTargetExpression(node) => node.syntax(),
            Self::JsNullLiteralExpression(node) => node.syntax(),
            Self::JsNumberLiteralExpression(node) => node.syntax(),
            Self::JsObjectAssignmentPattern(node) => node.syntax(),
            Self::JsObjectAssignmentPatternProperty(node) => node.syntax(),
            Self::JsObjectAssignmentPatternRest(node) => node.syntax(),
            Self::JsObjectAssignmentPatternShorthandProperty(node) => node.syntax(),
            Self::JsObjectBindingPattern(node) => node.syntax(),
            Self::JsObjectBindingPatternProperty(node) => node.syntax(),
            Self::JsObjectBindingPatternRest(node) => node.syntax(),
            Self::JsObjectBindingPatternShorthandProperty(node) => node.syntax(),
            Self::JsObjectExpression(node) => node.syntax(),
            Self::JsParameters(node) => node.syntax(),
            Self::JsParenthesizedAssignment(node) => node.syntax(),
            Self::JsParenthesizedExpression(node) => node.syntax(),
            Self::JsPostUpdateExpression(node) => node.syntax(),
            Self::JsPreUpdateExpression(node) => node.syntax(),
            Self::JsPrivateClassMemberName(node) => node.syntax(),
            Self::JsPrivateName(node) => node.syntax(),
            Self::JsPropertyClassMember(node) => node.syntax(),
            Self::JsPropertyObjectMember(node) => node.syntax(),
            Self::JsReferenceIdentifier(node) => node.syntax(),
            Self::JsRegexLiteralExpression(node) => node.syntax(),
            Self::JsRestParameter(node) => node.syntax(),
            Self::JsReturnStatement(node) => node.syntax(),
            Self::JsScript(node) => node.syntax(),
            Self::JsSequenceExpression(node) => node.syntax(),
            Self::JsSetterClassMember(node) => node.syntax(),
            Self::JsSetterObjectMember(node) => node.syntax(),
            Self::JsShorthandNamedImportSpecifier(node) => node.syntax(),
            Self::JsShorthandPropertyObjectMember(node) => node.syntax(),
            Self::JsSpread(node) => node.syntax(),
            Self::JsStaticInitializationBlockClassMember(node) => node.syntax(),
            Self::JsStaticMemberAssignment(node) => node.syntax(),
            Self::JsStaticMemberExpression(node) => node.syntax(),
            Self::JsStaticModifier(node) => node.syntax(),
            Self::JsStringLiteralExpression(node) => node.syntax(),
            Self::JsSuperExpression(node) => node.syntax(),
            Self::JsSvelteSnippetRoot(node) => node.syntax(),
            Self::JsSwitchStatement(node) => node.syntax(),
            Self::JsTemplateChunkElement(node) => node.syntax(),
            Self::JsTemplateElement(node) => node.syntax(),
            Self::JsTemplateExpression(node) => node.syntax(),
            Self::JsThisExpression(node) => node.syntax(),
            Self::JsThrowStatement(node) => node.syntax(),
            Self::JsTryFinallyStatement(node) => node.syntax(),
            Self::JsTryStatement(node) => node.syntax(),
            Self::JsUnaryExpression(node) => node.syntax(),
            Self::JsVariableDeclaration(node) => node.syntax(),
            Self::JsVariableDeclarationClause(node) => node.syntax(),
            Self::JsVariableDeclarator(node) => node.syntax(),
            Self::JsVariableStatement(node) => node.syntax(),
            Self::JsWhileStatement(node) => node.syntax(),
            Self::JsWithStatement(node) => node.syntax(),
            Self::JsYieldArgument(node) => node.syntax(),
            Self::JsYieldExpression(node) => node.syntax(),
            Self::JsxAttribute(node) => node.syntax(),
            Self::JsxAttributeInitializerClause(node) => node.syntax(),
            Self::JsxClosingElement(node) => node.syntax(),
            Self::JsxClosingFragment(node) => node.syntax(),
            Self::JsxElement(node) => node.syntax(),
            Self::JsxExpressionAttributeValue(node) => node.syntax(),
            Self::JsxExpressionChild(node) => node.syntax(),
            Self::JsxFragment(node) => node.syntax(),
            Self::JsxMemberName(node) => node.syntax(),
            Self::JsxName(node) => node.syntax(),
            Self::JsxNamespaceName(node) => node.syntax(),
            Self::JsxOpeningElement(node) => node.syntax(),
            Self::JsxOpeningFragment(node) => node.syntax(),
            Self::JsxReferenceIdentifier(node) => node.syntax(),
            Self::JsxSelfClosingElement(node) => node.syntax(),
            Self::JsxShorthandAttribute(node) => node.syntax(),
            Self::JsxSpreadAttribute(node) => node.syntax(),
            Self::JsxSpreadChild(node) => node.syntax(),
            Self::JsxString(node) => node.syntax(),
            Self::JsxTagExpression(node) => node.syntax(),
            Self::JsxText(node) => node.syntax(),
            Self::TsAbstractModifier(node) => node.syntax(),
            Self::TsAccessibilityModifier(node) => node.syntax(),
            Self::TsAnyType(node) => node.syntax(),
            Self::TsArrayType(node) => node.syntax(),
            Self::TsAsAssignment(node) => node.syntax(),
            Self::TsAsExpression(node) => node.syntax(),
            Self::TsAssertsCondition(node) => node.syntax(),
            Self::TsAssertsReturnType(node) => node.syntax(),
            Self::TsBigintLiteralType(node) => node.syntax(),
            Self::TsBigintType(node) => node.syntax(),
            Self::TsBooleanLiteralType(node) => node.syntax(),
            Self::TsBooleanType(node) => node.syntax(),
            Self::TsCallSignatureTypeMember(node) => node.syntax(),
            Self::TsConditionalType(node) => node.syntax(),
            Self::TsConstModifier(node) => node.syntax(),
            Self::TsConstructSignatureTypeMember(node) => node.syntax(),
            Self::TsConstructorSignatureClassMember(node) => node.syntax(),
            Self::TsConstructorType(node) => node.syntax(),
            Self::TsDeclarationModule(node) => node.syntax(),
            Self::TsDeclareFunctionDeclaration(node) => node.syntax(),
            Self::TsDeclareFunctionExportDefaultDeclaration(node) => node.syntax(),
            Self::TsDeclareModifier(node) => node.syntax(),
            Self::TsDeclareStatement(node) => node.syntax(),
            Self::TsDefaultTypeClause(node) => node.syntax(),
            Self::TsDefinitePropertyAnnotation(node) => node.syntax(),
            Self::TsDefiniteVariableAnnotation(node) => node.syntax(),
            Self::TsEmptyExternalModuleDeclarationBody(node) => node.syntax(),
            Self::TsEnumDeclaration(node) => node.syntax(),
            Self::TsEnumMember(node) => node.syntax(),
            Self::TsExportAsNamespaceClause(node) => node.syntax(),
            Self::TsExportAssignmentClause(node) => node.syntax(),
            Self::TsExportDeclareClause(node) => node.syntax(),
            Self::TsExtendsClause(node) => node.syntax(),
            Self::TsExternalModuleDeclaration(node) => node.syntax(),
            Self::TsExternalModuleReference(node) => node.syntax(),
            Self::TsFunctionType(node) => node.syntax(),
            Self::TsGetterSignatureClassMember(node) => node.syntax(),
            Self::TsGetterSignatureTypeMember(node) => node.syntax(),
            Self::TsGlobalDeclaration(node) => node.syntax(),
            Self::TsIdentifierBinding(node) => node.syntax(),
            Self::TsImplementsClause(node) => node.syntax(),
            Self::TsImportEqualsDeclaration(node) => node.syntax(),
            Self::TsImportType(node) => node.syntax(),
            Self::TsImportTypeArguments(node) => node.syntax(),
            Self::TsImportTypeAssertion(node) => node.syntax(),
            Self::TsImportTypeAssertionBlock(node) => node.syntax(),
            Self::TsImportTypeQualifier(node) => node.syntax(),
            Self::TsInModifier(node) => node.syntax(),
            Self::TsIndexSignatureClassMember(node) => node.syntax(),
            Self::TsIndexSignatureParameter(node) => node.syntax(),
            Self::TsIndexSignatureTypeMember(node) => node.syntax(),
            Self::TsIndexedAccessType(node) => node.syntax(),
            Self::TsInferType(node) => node.syntax(),
            Self::TsInitializedPropertySignatureClassMember(node) => node.syntax(),
            Self::TsInstantiationExpression(node) => node.syntax(),
            Self::TsInterfaceDeclaration(node) => node.syntax(),
            Self::TsIntersectionType(node) => node.syntax(),
            Self::TsLiteralEnumMemberName(node) => node.syntax(),
            Self::TsMappedType(node) => node.syntax(),
            Self::TsMappedTypeAsClause(node) => node.syntax(),
            Self::TsMappedTypeOptionalModifierClause(node) => node.syntax(),
            Self::TsMappedTypeReadonlyModifierClause(node) => node.syntax(),
            Self::TsMethodSignatureClassMember(node) => node.syntax(),
            Self::TsMethodSignatureTypeMember(node) => node.syntax(),
            Self::TsModuleBlock(node) => node.syntax(),
            Self::TsModuleDeclaration(node) => node.syntax(),
            Self::TsNamedTupleTypeElement(node) => node.syntax(),
            Self::TsNeverType(node) => node.syntax(),
            Self::TsNonNullAssertionAssignment(node) => node.syntax(),
            Self::TsNonNullAssertionExpression(node) => node.syntax(),
            Self::TsNonPrimitiveType(node) => node.syntax(),
            Self::TsNullLiteralType(node) => node.syntax(),
            Self::TsNumberLiteralType(node) => node.syntax(),
            Self::TsNumberType(node) => node.syntax(),
            Self::TsObjectType(node) => node.syntax(),
            Self::TsOptionalPropertyAnnotation(node) => node.syntax(),
            Self::TsOptionalTupleTypeElement(node) => node.syntax(),
            Self::TsOutModifier(node) => node.syntax(),
            Self::TsOverrideModifier(node) => node.syntax(),
            Self::TsParenthesizedType(node) => node.syntax(),
            Self::TsPredicateReturnType(node) => node.syntax(),
            Self::TsPropertyParameter(node) => node.syntax(),
            Self::TsPropertySignatureClassMember(node) => node.syntax(),
            Self::TsPropertySignatureTypeMember(node) => node.syntax(),
            Self::TsQualifiedModuleName(node) => node.syntax(),
            Self::TsQualifiedName(node) => node.syntax(),
            Self::TsReadonlyModifier(node) => node.syntax(),
            Self::TsReferenceType(node) => node.syntax(),
            Self::TsRestTupleTypeElement(node) => node.syntax(),
            Self::TsReturnTypeAnnotation(node) => node.syntax(),
            Self::TsSatisfiesAssignment(node) => node.syntax(),
            Self::TsSatisfiesExpression(node) => node.syntax(),
            Self::TsSetterSignatureClassMember(node) => node.syntax(),
            Self::TsSetterSignatureTypeMember(node) => node.syntax(),
            Self::TsStringLiteralType(node) => node.syntax(),
            Self::TsStringType(node) => node.syntax(),
            Self::TsSymbolType(node) => node.syntax(),
            Self::TsTemplateChunkElement(node) => node.syntax(),
            Self::TsTemplateElement(node) => node.syntax(),
            Self::TsTemplateLiteralType(node) => node.syntax(),
            Self::TsThisParameter(node) => node.syntax(),
            Self::TsThisType(node) => node.syntax(),
            Self::TsTupleType(node) => node.syntax(),
            Self::TsTypeAliasDeclaration(node) => node.syntax(),
            Self::TsTypeAnnotation(node) => node.syntax(),
            Self::TsTypeArguments(node) => node.syntax(),
            Self::TsTypeAssertionAssignment(node) => node.syntax(),
            Self::TsTypeAssertionExpression(node) => node.syntax(),
            Self::TsTypeConstraintClause(node) => node.syntax(),
            Self::TsTypeOperatorType(node) => node.syntax(),
            Self::TsTypeParameter(node) => node.syntax(),
            Self::TsTypeParameterName(node) => node.syntax(),
            Self::TsTypeParameters(node) => node.syntax(),
            Self::TsTypeofType(node) => node.syntax(),
            Self::TsUndefinedType(node) => node.syntax(),
            Self::TsUnionType(node) => node.syntax(),
            Self::TsUnknownType(node) => node.syntax(),
            Self::TsVoidType(node) => node.syntax(),
            Self::JsBogus(node) => node.syntax(),
            Self::JsBogusAssignment(node) => node.syntax(),
            Self::JsBogusBinding(node) => node.syntax(),
            Self::JsBogusExpression(node) => node.syntax(),
            Self::JsBogusImportAssertionEntry(node) => node.syntax(),
            Self::JsBogusMember(node) => node.syntax(),
            Self::JsBogusNamedImportSpecifier(node) => node.syntax(),
            Self::JsBogusParameter(node) => node.syntax(),
            Self::JsBogusStatement(node) => node.syntax(),
            Self::TsBogusType(node) => node.syntax(),
        }
    }
    fn resolve_field(&self, field: &str, context: &mut Context) -> JsResult<JsValue> {
        match self {
            Self::JsAccessorModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayAssignmentPattern(node) => match field {
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "elements" => {
                    JsAstNode::wrap_node_list(node.elements().into_iter().flatten(), context)
                }
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayAssignmentPatternElement(node) => match field {
                "pattern" => JsAstNode::wrap_optional_node(node.pattern().ok(), context),
                "init" => JsAstNode::wrap_optional_node(node.init(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayAssignmentPatternRestElement(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "pattern" => JsAstNode::wrap_optional_node(node.pattern().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayBindingPattern(node) => match field {
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "elements" => {
                    JsAstNode::wrap_node_list(node.elements().into_iter().flatten(), context)
                }
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayBindingPatternElement(node) => match field {
                "pattern" => JsAstNode::wrap_optional_node(node.pattern().ok(), context),
                "init" => JsAstNode::wrap_optional_node(node.init(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayBindingPatternRestElement(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "pattern" => JsAstNode::wrap_optional_node(node.pattern().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayExpression(node) => match field {
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "elements" => {
                    JsAstNode::wrap_node_list(node.elements().into_iter().flatten(), context)
                }
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsArrayHole(_) => Ok(JsValue::undefined()),
            Self::JsArrowFunctionExpression(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "fatArrowToken" => Ok(JsAstNode::wrap_token(node.fat_arrow_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsAssignmentExpression(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsAwaitExpression(node) => match field {
                "awaitToken" => Ok(JsAstNode::wrap_token(node.await_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsBigintLiteralExpression(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsBinaryExpression(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsBlockStatement(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "statements" => JsAstNode::wrap_node_list(node.statements(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsBooleanLiteralExpression(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsBreakStatement(node) => match field {
                "breakToken" => Ok(JsAstNode::wrap_token(node.break_token().ok())),
                "label" => JsAstNode::wrap_optional_node(node.label(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsCallArguments(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "args" => JsAstNode::wrap_node_list(node.args().into_iter().flatten(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsCallExpression(node) => match field {
                "callee" => JsAstNode::wrap_optional_node(node.callee().ok(), context),
                "optionalChainToken" => Ok(JsAstNode::wrap_token(node.optional_chain_token())),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                "arguments" => JsAstNode::wrap_optional_node(node.arguments().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsCaseClause(node) => match field {
                "caseToken" => Ok(JsAstNode::wrap_token(node.case_token().ok())),
                "test" => JsAstNode::wrap_optional_node(node.test().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "consequent" => JsAstNode::wrap_node_list(node.consequent(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsCatchClause(node) => match field {
                "catchToken" => Ok(JsAstNode::wrap_token(node.catch_token().ok())),
                "declaration" => JsAstNode::wrap_optional_node(node.declaration(), context),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsCatchDeclaration(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "binding" => JsAstNode::wrap_optional_node(node.binding().ok(), context),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsClassDeclaration(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "abstractToken" => Ok(JsAstNode::wrap_token(node.abstract_token())),
                "classToken" => Ok(JsAstNode::wrap_token(node.class_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "extendsClause" => JsAstNode::wrap_optional_node(node.extends_clause(), context),
                "implementsClause" => {
                    JsAstNode::wrap_optional_node(node.implements_clause(), context)
                }
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => JsAstNode::wrap_node_list(node.members(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsClassExportDefaultDeclaration(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "abstractToken" => Ok(JsAstNode::wrap_token(node.abstract_token())),
                "classToken" => Ok(JsAstNode::wrap_token(node.class_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "extendsClause" => JsAstNode::wrap_optional_node(node.extends_clause(), context),
                "implementsClause" => {
                    JsAstNode::wrap_optional_node(node.implements_clause(), context)
                }
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => JsAstNode::wrap_node_list(node.members(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsClassExpression(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "classToken" => Ok(JsAstNode::wrap_token(node.class_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "extendsClause" => JsAstNode::wrap_optional_node(node.extends_clause(), context),
                "implementsClause" => {
                    JsAstNode::wrap_optional_node(node.implements_clause(), context)
                }
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => JsAstNode::wrap_node_list(node.members(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsComputedMemberAssignment(node) => match field {
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsComputedMemberExpression(node) => match field {
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                "optionalChainToken" => Ok(JsAstNode::wrap_token(node.optional_chain_token())),
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsComputedMemberName(node) => match field {
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsConditionalExpression(node) => match field {
                "test" => JsAstNode::wrap_optional_node(node.test().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token().ok())),
                "consequent" => JsAstNode::wrap_optional_node(node.consequent().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "alternate" => JsAstNode::wrap_optional_node(node.alternate().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsConstructorClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsConstructorParameters(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "parameters" => {
                    JsAstNode::wrap_node_list(node.parameters().into_iter().flatten(), context)
                }
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsContinueStatement(node) => match field {
                "continueToken" => Ok(JsAstNode::wrap_token(node.continue_token().ok())),
                "label" => JsAstNode::wrap_optional_node(node.label(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsDebuggerStatement(node) => match field {
                "debuggerToken" => Ok(JsAstNode::wrap_token(node.debugger_token().ok())),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsDecorator(node) => match field {
                "atToken" => Ok(JsAstNode::wrap_token(node.at_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsDefaultClause(node) => match field {
                "defaultToken" => Ok(JsAstNode::wrap_token(node.default_token().ok())),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "consequent" => JsAstNode::wrap_node_list(node.consequent(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsDefaultImportSpecifier(node) => match field {
                "localName" => JsAstNode::wrap_optional_node(node.local_name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsDirective(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsDoWhileStatement(node) => match field {
                "doToken" => Ok(JsAstNode::wrap_token(node.do_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                "whileToken" => Ok(JsAstNode::wrap_token(node.while_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "test" => JsAstNode::wrap_optional_node(node.test().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsElseClause(node) => match field {
                "elseToken" => Ok(JsAstNode::wrap_token(node.else_token().ok())),
                "alternate" => JsAstNode::wrap_optional_node(node.alternate().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsEmptyClassMember(node) => match field {
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsEmptyStatement(node) => match field {
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExport(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "exportToken" => Ok(JsAstNode::wrap_token(node.export_token().ok())),
                "exportClause" => JsAstNode::wrap_optional_node(node.export_clause().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportAsClause(node) => match field {
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "exportedName" => JsAstNode::wrap_optional_node(node.exported_name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportDefaultDeclarationClause(node) => match field {
                "defaultToken" => Ok(JsAstNode::wrap_token(node.default_token().ok())),
                "declaration" => JsAstNode::wrap_optional_node(node.declaration().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportDefaultExpressionClause(node) => match field {
                "defaultToken" => Ok(JsAstNode::wrap_token(node.default_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportFromClause(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token().ok())),
                "exportAs" => JsAstNode::wrap_optional_node(node.export_as(), context),
                "fromToken" => Ok(JsAstNode::wrap_token(node.from_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportNamedClause(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "specifiers" => {
                    JsAstNode::wrap_node_list(node.specifiers().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportNamedFromClause(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "specifiers" => {
                    JsAstNode::wrap_node_list(node.specifiers().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                "fromToken" => Ok(JsAstNode::wrap_token(node.from_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportNamedFromSpecifier(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "sourceName" => JsAstNode::wrap_optional_node(node.source_name().ok(), context),
                "exportAs" => JsAstNode::wrap_optional_node(node.export_as(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportNamedShorthandSpecifier(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExportNamedSpecifier(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "localName" => JsAstNode::wrap_optional_node(node.local_name().ok(), context),
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "exportedName" => JsAstNode::wrap_optional_node(node.exported_name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExpressionSnippet(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "eofToken" => Ok(JsAstNode::wrap_token(node.eof_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExpressionStatement(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExpressionTemplateRoot(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "eofToken" => Ok(JsAstNode::wrap_token(node.eof_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsExtendsClause(node) => match field {
                "extendsToken" => Ok(JsAstNode::wrap_token(node.extends_token().ok())),
                "superClass" => JsAstNode::wrap_optional_node(node.super_class().ok(), context),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsFinallyClause(node) => match field {
                "finallyToken" => Ok(JsAstNode::wrap_token(node.finally_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsForInStatement(node) => match field {
                "forToken" => Ok(JsAstNode::wrap_token(node.for_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "initializer" => JsAstNode::wrap_optional_node(node.initializer().ok(), context),
                "inToken" => Ok(JsAstNode::wrap_token(node.in_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsForOfStatement(node) => match field {
                "forToken" => Ok(JsAstNode::wrap_token(node.for_token().ok())),
                "awaitToken" => Ok(JsAstNode::wrap_token(node.await_token())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "initializer" => JsAstNode::wrap_optional_node(node.initializer().ok(), context),
                "ofToken" => Ok(JsAstNode::wrap_token(node.of_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsForStatement(node) => match field {
                "forToken" => Ok(JsAstNode::wrap_token(node.for_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "initializer" => JsAstNode::wrap_optional_node(node.initializer(), context),
                "firstSemiToken" => Ok(JsAstNode::wrap_token(node.first_semi_token().ok())),
                "test" => JsAstNode::wrap_optional_node(node.test(), context),
                "secondSemiToken" => Ok(JsAstNode::wrap_token(node.second_semi_token().ok())),
                "update" => JsAstNode::wrap_optional_node(node.update(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsForVariableDeclaration(node) => match field {
                "awaitToken" => Ok(JsAstNode::wrap_token(node.await_token())),
                "kindToken" => Ok(JsAstNode::wrap_token(node.kind_token().ok())),
                "declarator" => JsAstNode::wrap_optional_node(node.declarator().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsFormalParameter(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "binding" => JsAstNode::wrap_optional_node(node.binding().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token())),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                "initializer" => JsAstNode::wrap_optional_node(node.initializer(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsFunctionBody(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "directives" => JsAstNode::wrap_node_list(node.directives(), context),
                "statements" => JsAstNode::wrap_node_list(node.statements(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsFunctionDeclaration(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "functionToken" => Ok(JsAstNode::wrap_token(node.function_token().ok())),
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token())),
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsFunctionExportDefaultDeclaration(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "functionToken" => Ok(JsAstNode::wrap_token(node.function_token().ok())),
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token())),
                "id" => JsAstNode::wrap_optional_node(node.id(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsFunctionExpression(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "functionToken" => Ok(JsAstNode::wrap_token(node.function_token().ok())),
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token())),
                "id" => JsAstNode::wrap_optional_node(node.id(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsGetterClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "getToken" => Ok(JsAstNode::wrap_token(node.get_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "returnType" => JsAstNode::wrap_optional_node(node.return_type(), context),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsGetterObjectMember(node) => match field {
                "getToken" => Ok(JsAstNode::wrap_token(node.get_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "returnType" => JsAstNode::wrap_optional_node(node.return_type(), context),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsIdentifierAssignment(node) => match field {
                "nameToken" => Ok(JsAstNode::wrap_token(node.name_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsIdentifierBinding(node) => match field {
                "nameToken" => Ok(JsAstNode::wrap_token(node.name_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsIdentifierExpression(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsIfStatement(node) => match field {
                "ifToken" => Ok(JsAstNode::wrap_token(node.if_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "test" => JsAstNode::wrap_optional_node(node.test().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "consequent" => JsAstNode::wrap_optional_node(node.consequent().ok(), context),
                "elseClause" => JsAstNode::wrap_optional_node(node.else_clause(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImport(node) => match field {
                "importToken" => Ok(JsAstNode::wrap_token(node.import_token().ok())),
                "importClause" => JsAstNode::wrap_optional_node(node.import_clause().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportAssertion(node) => match field {
                "withToken" => Ok(JsAstNode::wrap_token(node.with_token().ok())),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "assertions" => {
                    JsAstNode::wrap_node_list(node.assertions().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportAssertionEntry(node) => match field {
                "key" => Ok(JsAstNode::wrap_token(node.key().ok())),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportBareClause(node) => match field {
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportCallExpression(node) => match field {
                "importToken" => Ok(JsAstNode::wrap_token(node.import_token().ok())),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token())),
                "phase" => Ok(JsAstNode::wrap_token(node.phase())),
                "arguments" => JsAstNode::wrap_optional_node(node.arguments().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportCombinedClause(node) => match field {
                "defaultSpecifier" => {
                    JsAstNode::wrap_optional_node(node.default_specifier().ok(), context)
                }
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token().ok())),
                "specifier" => JsAstNode::wrap_optional_node(node.specifier().ok(), context),
                "fromToken" => Ok(JsAstNode::wrap_token(node.from_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportDefaultClause(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "phaseToken" => Ok(JsAstNode::wrap_token(node.phase_token())),
                "defaultSpecifier" => {
                    JsAstNode::wrap_optional_node(node.default_specifier().ok(), context)
                }
                "fromToken" => Ok(JsAstNode::wrap_token(node.from_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportMetaExpression(node) => match field {
                "importToken" => Ok(JsAstNode::wrap_token(node.import_token().ok())),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "metaToken" => Ok(JsAstNode::wrap_token(node.meta_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportNamedClause(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "namedSpecifiers" => {
                    JsAstNode::wrap_optional_node(node.named_specifiers().ok(), context)
                }
                "fromToken" => Ok(JsAstNode::wrap_token(node.from_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsImportNamespaceClause(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "phaseToken" => Ok(JsAstNode::wrap_token(node.phase_token())),
                "namespaceSpecifier" => {
                    JsAstNode::wrap_optional_node(node.namespace_specifier().ok(), context)
                }
                "fromToken" => Ok(JsAstNode::wrap_token(node.from_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "assertion" => JsAstNode::wrap_optional_node(node.assertion(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsInExpression(node) => match field {
                "property" => JsAstNode::wrap_optional_node(node.property().ok(), context),
                "inToken" => Ok(JsAstNode::wrap_token(node.in_token().ok())),
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsInitializerClause(node) => match field {
                "eqToken" => Ok(JsAstNode::wrap_token(node.eq_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsInstanceofExpression(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "instanceofToken" => Ok(JsAstNode::wrap_token(node.instanceof_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsLabel(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsLabeledStatement(node) => match field {
                "label" => JsAstNode::wrap_optional_node(node.label().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsLiteralExportName(node) => match field {
                "value" => Ok(JsAstNode::wrap_token(node.value().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsLiteralMemberName(node) => match field {
                "value" => Ok(JsAstNode::wrap_token(node.value().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsLogicalExpression(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsMetavariable(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsMethodClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token())),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsMethodObjectMember(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsModule(node) => match field {
                "bomToken" => Ok(JsAstNode::wrap_token(node.bom_token())),
                "interpreterToken" => Ok(JsAstNode::wrap_token(node.interpreter_token())),
                "directives" => JsAstNode::wrap_node_list(node.directives(), context),
                "items" => JsAstNode::wrap_node_list(node.items(), context),
                "eofToken" => Ok(JsAstNode::wrap_token(node.eof_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsModuleSource(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsName(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNamedImportSpecifier(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "localName" => JsAstNode::wrap_optional_node(node.local_name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNamedImportSpecifiers(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "specifiers" => {
                    JsAstNode::wrap_node_list(node.specifiers().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNamespaceImportSpecifier(node) => match field {
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token().ok())),
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "localName" => JsAstNode::wrap_optional_node(node.local_name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNewExpression(node) => match field {
                "newToken" => Ok(JsAstNode::wrap_token(node.new_token().ok())),
                "callee" => JsAstNode::wrap_optional_node(node.callee().ok(), context),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                "arguments" => JsAstNode::wrap_optional_node(node.arguments(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNewTargetExpression(node) => match field {
                "newToken" => Ok(JsAstNode::wrap_token(node.new_token().ok())),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "targetToken" => Ok(JsAstNode::wrap_token(node.target_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNullLiteralExpression(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsNumberLiteralExpression(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectAssignmentPattern(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "properties" => {
                    JsAstNode::wrap_node_list(node.properties().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectAssignmentPatternProperty(node) => match field {
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "pattern" => JsAstNode::wrap_optional_node(node.pattern().ok(), context),
                "init" => JsAstNode::wrap_optional_node(node.init(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectAssignmentPatternRest(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "target" => JsAstNode::wrap_optional_node(node.target().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectAssignmentPatternShorthandProperty(node) => match field {
                "identifier" => JsAstNode::wrap_optional_node(node.identifier().ok(), context),
                "init" => JsAstNode::wrap_optional_node(node.init(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectBindingPattern(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "properties" => {
                    JsAstNode::wrap_node_list(node.properties().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectBindingPatternProperty(node) => match field {
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "pattern" => JsAstNode::wrap_optional_node(node.pattern().ok(), context),
                "init" => JsAstNode::wrap_optional_node(node.init(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectBindingPatternRest(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "binding" => JsAstNode::wrap_optional_node(node.binding().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectBindingPatternShorthandProperty(node) => match field {
                "identifier" => JsAstNode::wrap_optional_node(node.identifier().ok(), context),
                "init" => JsAstNode::wrap_optional_node(node.init(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsObjectExpression(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => {
                    JsAstNode::wrap_node_list(node.members().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsParameters(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "items" => JsAstNode::wrap_node_list(node.items().into_iter().flatten(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsParenthesizedAssignment(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "assignment" => JsAstNode::wrap_optional_node(node.assignment().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsParenthesizedExpression(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsPostUpdateExpression(node) => match field {
                "operand" => JsAstNode::wrap_optional_node(node.operand().ok(), context),
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsPreUpdateExpression(node) => match field {
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "operand" => JsAstNode::wrap_optional_node(node.operand().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsPrivateClassMemberName(node) => match field {
                "hashToken" => Ok(JsAstNode::wrap_token(node.hash_token().ok())),
                "idToken" => Ok(JsAstNode::wrap_token(node.id_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsPrivateName(node) => match field {
                "hashToken" => Ok(JsAstNode::wrap_token(node.hash_token().ok())),
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsPropertyClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "propertyAnnotation" => {
                    JsAstNode::wrap_optional_node(node.property_annotation(), context)
                }
                "value" => JsAstNode::wrap_optional_node(node.value(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsPropertyObjectMember(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "value" => JsAstNode::wrap_optional_node(node.value().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsReferenceIdentifier(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsRegexLiteralExpression(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsRestParameter(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "binding" => JsAstNode::wrap_optional_node(node.binding().ok(), context),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsReturnStatement(node) => match field {
                "returnToken" => Ok(JsAstNode::wrap_token(node.return_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsScript(node) => match field {
                "bomToken" => Ok(JsAstNode::wrap_token(node.bom_token())),
                "interpreterToken" => Ok(JsAstNode::wrap_token(node.interpreter_token())),
                "directives" => JsAstNode::wrap_node_list(node.directives(), context),
                "statements" => JsAstNode::wrap_node_list(node.statements(), context),
                "eofToken" => Ok(JsAstNode::wrap_token(node.eof_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSequenceExpression(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSetterClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "setToken" => Ok(JsAstNode::wrap_token(node.set_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "parameter" => JsAstNode::wrap_optional_node(node.parameter().ok(), context),
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSetterObjectMember(node) => match field {
                "setToken" => Ok(JsAstNode::wrap_token(node.set_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "parameter" => JsAstNode::wrap_optional_node(node.parameter().ok(), context),
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsShorthandNamedImportSpecifier(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "localName" => JsAstNode::wrap_optional_node(node.local_name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsShorthandPropertyObjectMember(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSpread(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsStaticInitializationBlockClassMember(node) => match field {
                "staticToken" => Ok(JsAstNode::wrap_token(node.static_token().ok())),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "statements" => JsAstNode::wrap_node_list(node.statements(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsStaticMemberAssignment(node) => match field {
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsStaticMemberExpression(node) => match field {
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsStaticModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsStringLiteralExpression(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSuperExpression(node) => match field {
                "superToken" => Ok(JsAstNode::wrap_token(node.super_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSvelteSnippetRoot(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "eofToken" => Ok(JsAstNode::wrap_token(node.eof_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsSwitchStatement(node) => match field {
                "switchToken" => Ok(JsAstNode::wrap_token(node.switch_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "discriminant" => JsAstNode::wrap_optional_node(node.discriminant().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "cases" => JsAstNode::wrap_node_list(node.cases(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsTemplateChunkElement(node) => match field {
                "templateChunkToken" => Ok(JsAstNode::wrap_token(node.template_chunk_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsTemplateElement(node) => match field {
                "dollarCurlyToken" => Ok(JsAstNode::wrap_token(node.dollar_curly_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsTemplateExpression(node) => match field {
                "tag" => JsAstNode::wrap_optional_node(node.tag(), context),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                "lTickToken" => Ok(JsAstNode::wrap_token(node.l_tick_token().ok())),
                "elements" => JsAstNode::wrap_node_list(node.elements(), context),
                "rTickToken" => Ok(JsAstNode::wrap_token(node.r_tick_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsThisExpression(node) => match field {
                "thisToken" => Ok(JsAstNode::wrap_token(node.this_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsThrowStatement(node) => match field {
                "throwToken" => Ok(JsAstNode::wrap_token(node.throw_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsTryFinallyStatement(node) => match field {
                "tryToken" => Ok(JsAstNode::wrap_token(node.try_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                "catchClause" => JsAstNode::wrap_optional_node(node.catch_clause(), context),
                "finallyClause" => {
                    JsAstNode::wrap_optional_node(node.finally_clause().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::JsTryStatement(node) => match field {
                "tryToken" => Ok(JsAstNode::wrap_token(node.try_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                "catchClause" => JsAstNode::wrap_optional_node(node.catch_clause().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsUnaryExpression(node) => match field {
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsVariableDeclaration(node) => match field {
                "awaitToken" => Ok(JsAstNode::wrap_token(node.await_token())),
                "kindToken" => Ok(JsAstNode::wrap_token(node.kind().ok())),
                "declarators" => {
                    JsAstNode::wrap_node_list(node.declarators().into_iter().flatten(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::JsVariableDeclarationClause(node) => match field {
                "declaration" => JsAstNode::wrap_optional_node(node.declaration().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsVariableDeclarator(node) => match field {
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "variableAnnotation" => {
                    JsAstNode::wrap_optional_node(node.variable_annotation(), context)
                }
                "initializer" => JsAstNode::wrap_optional_node(node.initializer(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsVariableStatement(node) => match field {
                "declaration" => JsAstNode::wrap_optional_node(node.declaration().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsWhileStatement(node) => match field {
                "whileToken" => Ok(JsAstNode::wrap_token(node.while_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "test" => JsAstNode::wrap_optional_node(node.test().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsWithStatement(node) => match field {
                "withToken" => Ok(JsAstNode::wrap_token(node.with_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsYieldArgument(node) => match field {
                "starToken" => Ok(JsAstNode::wrap_token(node.star_token())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsYieldExpression(node) => match field {
                "yieldToken" => Ok(JsAstNode::wrap_token(node.yield_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxAttribute(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "initializer" => JsAstNode::wrap_optional_node(node.initializer(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxAttributeInitializerClause(node) => match field {
                "eqToken" => Ok(JsAstNode::wrap_token(node.eq_token().ok())),
                "value" => JsAstNode::wrap_optional_node(node.value().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxClosingElement(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "slashToken" => Ok(JsAstNode::wrap_token(node.slash_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxClosingFragment(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "slashToken" => Ok(JsAstNode::wrap_token(node.slash_token().ok())),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxElement(node) => match field {
                "openingElement" => {
                    JsAstNode::wrap_optional_node(node.opening_element().ok(), context)
                }
                "children" => JsAstNode::wrap_node_list(node.children(), context),
                "closingElement" => {
                    JsAstNode::wrap_optional_node(node.closing_element().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxExpressionAttributeValue(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxExpressionChild(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxFragment(node) => match field {
                "openingFragment" => {
                    JsAstNode::wrap_optional_node(node.opening_fragment().ok(), context)
                }
                "children" => JsAstNode::wrap_node_list(node.children(), context),
                "closingFragment" => {
                    JsAstNode::wrap_optional_node(node.closing_fragment().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxMemberName(node) => match field {
                "object" => JsAstNode::wrap_optional_node(node.object().ok(), context),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "member" => JsAstNode::wrap_optional_node(node.member().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxName(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxNamespaceName(node) => match field {
                "namespace" => JsAstNode::wrap_optional_node(node.namespace().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxOpeningElement(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                "attributes" => JsAstNode::wrap_node_list(node.attributes(), context),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxOpeningFragment(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxReferenceIdentifier(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxSelfClosingElement(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                "attributes" => JsAstNode::wrap_node_list(node.attributes(), context),
                "slashToken" => Ok(JsAstNode::wrap_token(node.slash_token().ok())),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxShorthandAttribute(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxSpreadAttribute(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument().ok(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxSpreadChild(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxString(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxTagExpression(node) => match field {
                "tag" => JsAstNode::wrap_optional_node(node.tag().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsxText(node) => match field {
                "valueToken" => Ok(JsAstNode::wrap_token(node.value_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAbstractModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAccessibilityModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAnyType(node) => match field {
                "anyToken" => Ok(JsAstNode::wrap_token(node.any_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsArrayType(node) => match field {
                "elementType" => JsAstNode::wrap_optional_node(node.element_type().ok(), context),
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAsAssignment(node) => match field {
                "assignment" => JsAstNode::wrap_optional_node(node.assignment().ok(), context),
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAsExpression(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAssertsCondition(node) => match field {
                "isToken" => Ok(JsAstNode::wrap_token(node.is_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsAssertsReturnType(node) => match field {
                "assertsToken" => Ok(JsAstNode::wrap_token(node.asserts_token().ok())),
                "parameterName" => {
                    JsAstNode::wrap_optional_node(node.parameter_name().ok(), context)
                }
                "predicate" => JsAstNode::wrap_optional_node(node.predicate(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsBigintLiteralType(node) => match field {
                "minusToken" => Ok(JsAstNode::wrap_token(node.minus_token())),
                "literalToken" => Ok(JsAstNode::wrap_token(node.literal_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsBigintType(node) => match field {
                "bigintToken" => Ok(JsAstNode::wrap_token(node.bigint_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsBooleanLiteralType(node) => match field {
                "literal" => Ok(JsAstNode::wrap_token(node.literal().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsBooleanType(node) => match field {
                "booleanToken" => Ok(JsAstNode::wrap_token(node.boolean_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsCallSignatureTypeMember(node) => match field {
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsConditionalType(node) => match field {
                "checkType" => JsAstNode::wrap_optional_node(node.check_type().ok(), context),
                "extendsToken" => Ok(JsAstNode::wrap_token(node.extends_token().ok())),
                "extendsType" => JsAstNode::wrap_optional_node(node.extends_type().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token().ok())),
                "trueType" => JsAstNode::wrap_optional_node(node.true_type().ok(), context),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "falseType" => JsAstNode::wrap_optional_node(node.false_type().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsConstModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsConstructSignatureTypeMember(node) => match field {
                "newToken" => Ok(JsAstNode::wrap_token(node.new_token().ok())),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsConstructorSignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsConstructorType(node) => match field {
                "abstractToken" => Ok(JsAstNode::wrap_token(node.abstract_token())),
                "newToken" => Ok(JsAstNode::wrap_token(node.new_token().ok())),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "fatArrowToken" => Ok(JsAstNode::wrap_token(node.fat_arrow_token().ok())),
                "returnType" => JsAstNode::wrap_optional_node(node.return_type().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDeclarationModule(node) => match field {
                "bomToken" => Ok(JsAstNode::wrap_token(node.bom_token())),
                "interpreterToken" => Ok(JsAstNode::wrap_token(node.interpreter_token())),
                "directives" => JsAstNode::wrap_node_list(node.directives(), context),
                "items" => JsAstNode::wrap_node_list(node.items(), context),
                "eofToken" => Ok(JsAstNode::wrap_token(node.eof_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDeclareFunctionDeclaration(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "functionToken" => Ok(JsAstNode::wrap_token(node.function_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDeclareFunctionExportDefaultDeclaration(node) => match field {
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "functionToken" => Ok(JsAstNode::wrap_token(node.function_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDeclareModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDeclareStatement(node) => match field {
                "declareToken" => Ok(JsAstNode::wrap_token(node.declare_token().ok())),
                "declaration" => JsAstNode::wrap_optional_node(node.declaration().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDefaultTypeClause(node) => match field {
                "eqToken" => Ok(JsAstNode::wrap_token(node.eq_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDefinitePropertyAnnotation(node) => match field {
                "exclToken" => Ok(JsAstNode::wrap_token(node.excl_token().ok())),
                "typeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.type_annotation().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::TsDefiniteVariableAnnotation(node) => match field {
                "exclToken" => Ok(JsAstNode::wrap_token(node.excl_token().ok())),
                "typeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.type_annotation().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::TsEmptyExternalModuleDeclarationBody(node) => match field {
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsEnumDeclaration(node) => match field {
                "constToken" => Ok(JsAstNode::wrap_token(node.const_token())),
                "enumToken" => Ok(JsAstNode::wrap_token(node.enum_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => {
                    JsAstNode::wrap_node_list(node.members().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsEnumMember(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "initializer" => JsAstNode::wrap_optional_node(node.initializer(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsExportAsNamespaceClause(node) => match field {
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "namespaceToken" => Ok(JsAstNode::wrap_token(node.namespace_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsExportAssignmentClause(node) => match field {
                "eqToken" => Ok(JsAstNode::wrap_token(node.eq_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsExportDeclareClause(node) => match field {
                "declareToken" => Ok(JsAstNode::wrap_token(node.declare_token().ok())),
                "declaration" => JsAstNode::wrap_optional_node(node.declaration().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsExtendsClause(node) => match field {
                "extendsToken" => Ok(JsAstNode::wrap_token(node.extends_token().ok())),
                "types" => JsAstNode::wrap_node_list(node.types().into_iter().flatten(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsExternalModuleDeclaration(node) => match field {
                "moduleToken" => Ok(JsAstNode::wrap_token(node.module_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "body" => JsAstNode::wrap_optional_node(node.body(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsExternalModuleReference(node) => match field {
                "requireToken" => Ok(JsAstNode::wrap_token(node.require_token().ok())),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "source" => JsAstNode::wrap_optional_node(node.source().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsFunctionType(node) => match field {
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "fatArrowToken" => Ok(JsAstNode::wrap_token(node.fat_arrow_token().ok())),
                "returnType" => JsAstNode::wrap_optional_node(node.return_type().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsGetterSignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "getToken" => Ok(JsAstNode::wrap_token(node.get_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "returnType" => JsAstNode::wrap_optional_node(node.return_type(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsGetterSignatureTypeMember(node) => match field {
                "getToken" => Ok(JsAstNode::wrap_token(node.get_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsGlobalDeclaration(node) => match field {
                "globalToken" => Ok(JsAstNode::wrap_token(node.global_token().ok())),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsIdentifierBinding(node) => match field {
                "nameToken" => Ok(JsAstNode::wrap_token(node.name_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImplementsClause(node) => match field {
                "implementsToken" => Ok(JsAstNode::wrap_token(node.implements_token().ok())),
                "types" => JsAstNode::wrap_node_list(node.types().into_iter().flatten(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImportEqualsDeclaration(node) => match field {
                "importToken" => Ok(JsAstNode::wrap_token(node.import_token().ok())),
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token())),
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "eqToken" => Ok(JsAstNode::wrap_token(node.eq_token().ok())),
                "moduleReference" => {
                    JsAstNode::wrap_optional_node(node.module_reference().ok(), context)
                }
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImportType(node) => match field {
                "typeofToken" => Ok(JsAstNode::wrap_token(node.typeof_token())),
                "importToken" => Ok(JsAstNode::wrap_token(node.import_token().ok())),
                "arguments" => JsAstNode::wrap_optional_node(node.arguments().ok(), context),
                "qualifierClause" => {
                    JsAstNode::wrap_optional_node(node.qualifier_clause(), context)
                }
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImportTypeArguments(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "argument" => JsAstNode::wrap_optional_node(node.argument().ok(), context),
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token())),
                "tsImportTypeAssertionBlock" => {
                    JsAstNode::wrap_optional_node(node.ts_import_type_assertion_block(), context)
                }
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImportTypeAssertion(node) => match field {
                "withToken" => Ok(JsAstNode::wrap_token(node.with_token().ok())),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "assertions" => {
                    JsAstNode::wrap_node_list(node.assertions().into_iter().flatten(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImportTypeAssertionBlock(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "typeAssertion" => {
                    JsAstNode::wrap_optional_node(node.type_assertion().ok(), context)
                }
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsImportTypeQualifier(node) => match field {
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsInModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsIndexSignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "parameter" => JsAstNode::wrap_optional_node(node.parameter().ok(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                "typeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.type_annotation().ok(), context)
                }
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsIndexSignatureParameter(node) => match field {
                "binding" => JsAstNode::wrap_optional_node(node.binding().ok(), context),
                "typeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.type_annotation().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::TsIndexSignatureTypeMember(node) => match field {
                "readonlyToken" => Ok(JsAstNode::wrap_token(node.readonly_token())),
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "parameter" => JsAstNode::wrap_optional_node(node.parameter().ok(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                "typeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.type_annotation().ok(), context)
                }
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsIndexedAccessType(node) => match field {
                "objectType" => JsAstNode::wrap_optional_node(node.object_type().ok(), context),
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "indexType" => JsAstNode::wrap_optional_node(node.index_type().ok(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsInferType(node) => match field {
                "inferToken" => Ok(JsAstNode::wrap_token(node.infer_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "constraint" => JsAstNode::wrap_optional_node(node.constraint(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsInitializedPropertySignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token())),
                "value" => JsAstNode::wrap_optional_node(node.value().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsInstantiationExpression(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "arguments" => JsAstNode::wrap_optional_node(node.arguments().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsInterfaceDeclaration(node) => match field {
                "interfaceToken" => Ok(JsAstNode::wrap_token(node.interface_token().ok())),
                "id" => JsAstNode::wrap_optional_node(node.id().ok(), context),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "extendsClause" => JsAstNode::wrap_optional_node(node.extends_clause(), context),
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => JsAstNode::wrap_node_list(node.members(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsIntersectionType(node) => match field {
                "leadingSeparatorToken" => {
                    Ok(JsAstNode::wrap_token(node.leading_separator_token()))
                }
                "types" => JsAstNode::wrap_node_list(node.types().into_iter().flatten(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsLiteralEnumMemberName(node) => match field {
                "value" => Ok(JsAstNode::wrap_token(node.value().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsMappedType(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "readonlyModifier" => {
                    JsAstNode::wrap_optional_node(node.readonly_modifier(), context)
                }
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "propertyName" => JsAstNode::wrap_optional_node(node.property_name().ok(), context),
                "inToken" => Ok(JsAstNode::wrap_token(node.in_token().ok())),
                "keysType" => JsAstNode::wrap_optional_node(node.keys_type().ok(), context),
                "asClause" => JsAstNode::wrap_optional_node(node.as_clause(), context),
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                "optionalModifier" => {
                    JsAstNode::wrap_optional_node(node.optional_modifier(), context)
                }
                "mappedType" => JsAstNode::wrap_optional_node(node.mapped_type(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsMappedTypeAsClause(node) => match field {
                "asToken" => Ok(JsAstNode::wrap_token(node.as_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsMappedTypeOptionalModifierClause(node) => match field {
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token())),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsMappedTypeReadonlyModifierClause(node) => match field {
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token())),
                "readonlyToken" => Ok(JsAstNode::wrap_token(node.readonly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsMethodSignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "asyncToken" => Ok(JsAstNode::wrap_token(node.async_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token())),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsMethodSignatureTypeMember(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "optionalToken" => Ok(JsAstNode::wrap_token(node.optional_token())),
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "parameters" => JsAstNode::wrap_optional_node(node.parameters().ok(), context),
                "returnTypeAnnotation" => {
                    JsAstNode::wrap_optional_node(node.return_type_annotation(), context)
                }
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsModuleBlock(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "items" => JsAstNode::wrap_node_list(node.items(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsModuleDeclaration(node) => match field {
                "moduleOrNamespace" => Ok(JsAstNode::wrap_token(node.module_or_namespace().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "body" => JsAstNode::wrap_optional_node(node.body().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNamedTupleTypeElement(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token())),
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNeverType(node) => match field {
                "neverToken" => Ok(JsAstNode::wrap_token(node.never_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNonNullAssertionAssignment(node) => match field {
                "assignment" => JsAstNode::wrap_optional_node(node.assignment().ok(), context),
                "exclToken" => Ok(JsAstNode::wrap_token(node.excl_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNonNullAssertionExpression(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "exclToken" => Ok(JsAstNode::wrap_token(node.excl_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNonPrimitiveType(node) => match field {
                "objectToken" => Ok(JsAstNode::wrap_token(node.object_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNullLiteralType(node) => match field {
                "literalToken" => Ok(JsAstNode::wrap_token(node.literal_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNumberLiteralType(node) => match field {
                "minusToken" => Ok(JsAstNode::wrap_token(node.minus_token())),
                "literalToken" => Ok(JsAstNode::wrap_token(node.literal_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsNumberType(node) => match field {
                "numberToken" => Ok(JsAstNode::wrap_token(node.number_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsObjectType(node) => match field {
                "lCurlyToken" => Ok(JsAstNode::wrap_token(node.l_curly_token().ok())),
                "members" => JsAstNode::wrap_node_list(node.members(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsOptionalPropertyAnnotation(node) => match field {
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token().ok())),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsOptionalTupleTypeElement(node) => match field {
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                "questionMarkToken" => Ok(JsAstNode::wrap_token(node.question_mark_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsOutModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsOverrideModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsParenthesizedType(node) => match field {
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsPredicateReturnType(node) => match field {
                "parameterName" => {
                    JsAstNode::wrap_optional_node(node.parameter_name().ok(), context)
                }
                "isToken" => Ok(JsAstNode::wrap_token(node.is_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsPropertyParameter(node) => match field {
                "decorators" => JsAstNode::wrap_node_list(node.decorators(), context),
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "formalParameter" => {
                    JsAstNode::wrap_optional_node(node.formal_parameter().ok(), context)
                }
                _ => Ok(JsValue::undefined()),
            },
            Self::TsPropertySignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "propertyAnnotation" => {
                    JsAstNode::wrap_optional_node(node.property_annotation(), context)
                }
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsPropertySignatureTypeMember(node) => match field {
                "readonlyToken" => Ok(JsAstNode::wrap_token(node.readonly_token())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "optionalToken" => Ok(JsAstNode::wrap_token(node.optional_token())),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsQualifiedModuleName(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsQualifiedName(node) => match field {
                "left" => JsAstNode::wrap_optional_node(node.left().ok(), context),
                "dotToken" => Ok(JsAstNode::wrap_token(node.dot_token().ok())),
                "right" => JsAstNode::wrap_optional_node(node.right().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsReadonlyModifier(node) => match field {
                "modifierToken" => Ok(JsAstNode::wrap_token(node.modifier_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsReferenceType(node) => match field {
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsRestTupleTypeElement(node) => match field {
                "dotdotdotToken" => Ok(JsAstNode::wrap_token(node.dotdotdot_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsReturnTypeAnnotation(node) => match field {
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsSatisfiesAssignment(node) => match field {
                "assignment" => JsAstNode::wrap_optional_node(node.assignment().ok(), context),
                "satisfiesToken" => Ok(JsAstNode::wrap_token(node.satisfies_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsSatisfiesExpression(node) => match field {
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                "satisfiesToken" => Ok(JsAstNode::wrap_token(node.satisfies_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsSetterSignatureClassMember(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "setToken" => Ok(JsAstNode::wrap_token(node.set_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "parameter" => JsAstNode::wrap_optional_node(node.parameter().ok(), context),
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsSetterSignatureTypeMember(node) => match field {
                "setToken" => Ok(JsAstNode::wrap_token(node.set_token().ok())),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "lParenToken" => Ok(JsAstNode::wrap_token(node.l_paren_token().ok())),
                "parameter" => JsAstNode::wrap_optional_node(node.parameter().ok(), context),
                "commaToken" => Ok(JsAstNode::wrap_token(node.comma_token())),
                "rParenToken" => Ok(JsAstNode::wrap_token(node.r_paren_token().ok())),
                "separatorToken" => Ok(JsAstNode::wrap_token(node.separator_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsStringLiteralType(node) => match field {
                "literalToken" => Ok(JsAstNode::wrap_token(node.literal_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsStringType(node) => match field {
                "stringToken" => Ok(JsAstNode::wrap_token(node.string_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsSymbolType(node) => match field {
                "symbolToken" => Ok(JsAstNode::wrap_token(node.symbol_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTemplateChunkElement(node) => match field {
                "templateChunkToken" => Ok(JsAstNode::wrap_token(node.template_chunk_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTemplateElement(node) => match field {
                "dollarCurlyToken" => Ok(JsAstNode::wrap_token(node.dollar_curly_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                "rCurlyToken" => Ok(JsAstNode::wrap_token(node.r_curly_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTemplateLiteralType(node) => match field {
                "lTickToken" => Ok(JsAstNode::wrap_token(node.l_tick_token().ok())),
                "elements" => JsAstNode::wrap_node_list(node.elements(), context),
                "rTickToken" => Ok(JsAstNode::wrap_token(node.r_tick_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsThisParameter(node) => match field {
                "thisToken" => Ok(JsAstNode::wrap_token(node.this_token().ok())),
                "typeAnnotation" => JsAstNode::wrap_optional_node(node.type_annotation(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsThisType(node) => match field {
                "thisToken" => Ok(JsAstNode::wrap_token(node.this_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTupleType(node) => match field {
                "lBrackToken" => Ok(JsAstNode::wrap_token(node.l_brack_token().ok())),
                "elements" => {
                    JsAstNode::wrap_node_list(node.elements().into_iter().flatten(), context)
                }
                "rBrackToken" => Ok(JsAstNode::wrap_token(node.r_brack_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeAliasDeclaration(node) => match field {
                "typeToken" => Ok(JsAstNode::wrap_token(node.type_token().ok())),
                "bindingIdentifier" => {
                    JsAstNode::wrap_optional_node(node.binding_identifier().ok(), context)
                }
                "typeParameters" => JsAstNode::wrap_optional_node(node.type_parameters(), context),
                "eqToken" => Ok(JsAstNode::wrap_token(node.eq_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                "semicolonToken" => Ok(JsAstNode::wrap_token(node.semicolon_token())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeAnnotation(node) => match field {
                "colonToken" => Ok(JsAstNode::wrap_token(node.colon_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeArguments(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "tsTypeArgumentList" => JsAstNode::wrap_node_list(
                    node.ts_type_argument_list().into_iter().flatten(),
                    context,
                ),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeAssertionAssignment(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                "assignment" => JsAstNode::wrap_optional_node(node.assignment().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeAssertionExpression(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                "expression" => JsAstNode::wrap_optional_node(node.expression().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeConstraintClause(node) => match field {
                "extendsToken" => Ok(JsAstNode::wrap_token(node.extends_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeOperatorType(node) => match field {
                "operatorToken" => Ok(JsAstNode::wrap_token(node.operator_token().ok())),
                "ty" => JsAstNode::wrap_optional_node(node.ty().ok(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeParameter(node) => match field {
                "modifiers" => JsAstNode::wrap_node_list(node.modifiers(), context),
                "name" => JsAstNode::wrap_optional_node(node.name().ok(), context),
                "constraint" => JsAstNode::wrap_optional_node(node.constraint(), context),
                "default" => JsAstNode::wrap_optional_node(node.default(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeParameterName(node) => match field {
                "identToken" => Ok(JsAstNode::wrap_token(node.ident_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeParameters(node) => match field {
                "lAngleToken" => Ok(JsAstNode::wrap_token(node.l_angle_token().ok())),
                "items" => JsAstNode::wrap_node_list(node.items().into_iter().flatten(), context),
                "rAngleToken" => Ok(JsAstNode::wrap_token(node.r_angle_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsTypeofType(node) => match field {
                "typeofToken" => Ok(JsAstNode::wrap_token(node.typeof_token().ok())),
                "expressionName" => {
                    JsAstNode::wrap_optional_node(node.expression_name().ok(), context)
                }
                "typeArguments" => JsAstNode::wrap_optional_node(node.type_arguments(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsUndefinedType(node) => match field {
                "undefinedToken" => Ok(JsAstNode::wrap_token(node.undefined_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsUnionType(node) => match field {
                "leadingSeparatorToken" => {
                    Ok(JsAstNode::wrap_token(node.leading_separator_token()))
                }
                "types" => JsAstNode::wrap_node_list(node.types().into_iter().flatten(), context),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsUnknownType(node) => match field {
                "unknownToken" => Ok(JsAstNode::wrap_token(node.unknown_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::TsVoidType(node) => match field {
                "voidToken" => Ok(JsAstNode::wrap_token(node.void_token().ok())),
                _ => Ok(JsValue::undefined()),
            },
            Self::JsBogus(_) => Ok(JsValue::undefined()),
            Self::JsBogusAssignment(_) => Ok(JsValue::undefined()),
            Self::JsBogusBinding(_) => Ok(JsValue::undefined()),
            Self::JsBogusExpression(_) => Ok(JsValue::undefined()),
            Self::JsBogusImportAssertionEntry(_) => Ok(JsValue::undefined()),
            Self::JsBogusMember(_) => Ok(JsValue::undefined()),
            Self::JsBogusNamedImportSpecifier(_) => Ok(JsValue::undefined()),
            Self::JsBogusParameter(_) => Ok(JsValue::undefined()),
            Self::JsBogusStatement(_) => Ok(JsValue::undefined()),
            Self::TsBogusType(_) => Ok(JsValue::undefined()),
        }
    }
}
impl JsAstNode {
    fn get_field_abstract_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "abstractToken", context)
    }
    fn get_field_alternate(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "alternate", context)
    }
    fn get_field_any_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "anyToken", context)
    }
    fn get_field_args(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "args", context)
    }
    fn get_field_argument(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "argument", context)
    }
    fn get_field_arguments(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "arguments", context)
    }
    fn get_field_as_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "asClause", context)
    }
    fn get_field_as_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "asToken", context)
    }
    fn get_field_assertion(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "assertion", context)
    }
    fn get_field_assertions(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "assertions", context)
    }
    fn get_field_asserts_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "assertsToken", context)
    }
    fn get_field_assignment(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "assignment", context)
    }
    fn get_field_async_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "asyncToken", context)
    }
    fn get_field_at_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "atToken", context)
    }
    fn get_field_attributes(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "attributes", context)
    }
    fn get_field_await_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "awaitToken", context)
    }
    fn get_field_bigint_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "bigintToken", context)
    }
    fn get_field_binding(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "binding", context)
    }
    fn get_field_binding_identifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "bindingIdentifier", context)
    }
    fn get_field_body(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "body", context)
    }
    fn get_field_bom_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "bomToken", context)
    }
    fn get_field_boolean_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "booleanToken", context)
    }
    fn get_field_break_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "breakToken", context)
    }
    fn get_field_callee(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "callee", context)
    }
    fn get_field_case_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "caseToken", context)
    }
    fn get_field_cases(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "cases", context)
    }
    fn get_field_catch_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "catchClause", context)
    }
    fn get_field_catch_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "catchToken", context)
    }
    fn get_field_check_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "checkType", context)
    }
    fn get_field_children(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "children", context)
    }
    fn get_field_class_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "classToken", context)
    }
    fn get_field_closing_element(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "closingElement", context)
    }
    fn get_field_closing_fragment(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "closingFragment", context)
    }
    fn get_field_colon_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "colonToken", context)
    }
    fn get_field_comma_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "commaToken", context)
    }
    fn get_field_consequent(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "consequent", context)
    }
    fn get_field_const_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "constToken", context)
    }
    fn get_field_constraint(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "constraint", context)
    }
    fn get_field_continue_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "continueToken", context)
    }
    fn get_field_debugger_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "debuggerToken", context)
    }
    fn get_field_declaration(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "declaration", context)
    }
    fn get_field_declarator(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "declarator", context)
    }
    fn get_field_declarators(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "declarators", context)
    }
    fn get_field_declare_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "declareToken", context)
    }
    fn get_field_decorators(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "decorators", context)
    }
    fn get_field_default(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "default", context)
    }
    fn get_field_default_specifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "defaultSpecifier", context)
    }
    fn get_field_default_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "defaultToken", context)
    }
    fn get_field_directives(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "directives", context)
    }
    fn get_field_discriminant(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "discriminant", context)
    }
    fn get_field_do_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "doToken", context)
    }
    fn get_field_dollar_curly_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "dollarCurlyToken", context)
    }
    fn get_field_dot_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "dotToken", context)
    }
    fn get_field_dotdotdot_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "dotdotdotToken", context)
    }
    fn get_field_element_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "elementType", context)
    }
    fn get_field_elements(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "elements", context)
    }
    fn get_field_else_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "elseClause", context)
    }
    fn get_field_else_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "elseToken", context)
    }
    fn get_field_enum_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "enumToken", context)
    }
    fn get_field_eof_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "eofToken", context)
    }
    fn get_field_eq_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "eqToken", context)
    }
    fn get_field_excl_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "exclToken", context)
    }
    fn get_field_export_as(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "exportAs", context)
    }
    fn get_field_export_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "exportClause", context)
    }
    fn get_field_export_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "exportToken", context)
    }
    fn get_field_exported_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "exportedName", context)
    }
    fn get_field_expression(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "expression", context)
    }
    fn get_field_expression_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "expressionName", context)
    }
    fn get_field_extends_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "extendsClause", context)
    }
    fn get_field_extends_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "extendsToken", context)
    }
    fn get_field_extends_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "extendsType", context)
    }
    fn get_field_false_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "falseType", context)
    }
    fn get_field_fat_arrow_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "fatArrowToken", context)
    }
    fn get_field_finally_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "finallyClause", context)
    }
    fn get_field_finally_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "finallyToken", context)
    }
    fn get_field_first_semi_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "firstSemiToken", context)
    }
    fn get_field_for_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "forToken", context)
    }
    fn get_field_formal_parameter(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "formalParameter", context)
    }
    fn get_field_from_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "fromToken", context)
    }
    fn get_field_function_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "functionToken", context)
    }
    fn get_field_get_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "getToken", context)
    }
    fn get_field_global_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "globalToken", context)
    }
    fn get_field_hash_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "hashToken", context)
    }
    fn get_field_id(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Self::resolve_field(this, "id", context)
    }
    fn get_field_id_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "idToken", context)
    }
    fn get_field_ident_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "identToken", context)
    }
    fn get_field_identifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "identifier", context)
    }
    fn get_field_if_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "ifToken", context)
    }
    fn get_field_implements_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "implementsClause", context)
    }
    fn get_field_implements_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "implementsToken", context)
    }
    fn get_field_import_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "importClause", context)
    }
    fn get_field_import_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "importToken", context)
    }
    fn get_field_in_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "inToken", context)
    }
    fn get_field_index_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "indexType", context)
    }
    fn get_field_infer_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "inferToken", context)
    }
    fn get_field_init(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "init", context)
    }
    fn get_field_initializer(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "initializer", context)
    }
    fn get_field_instanceof_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "instanceofToken", context)
    }
    fn get_field_interface_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "interfaceToken", context)
    }
    fn get_field_interpreter_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "interpreterToken", context)
    }
    fn get_field_is_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "isToken", context)
    }
    fn get_field_items(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "items", context)
    }
    fn get_field_key(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "key", context)
    }
    fn get_field_keys_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "keysType", context)
    }
    fn get_field_kind_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "kindToken", context)
    }
    fn get_field_l_angle_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "lAngleToken", context)
    }
    fn get_field_l_brack_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "lBrackToken", context)
    }
    fn get_field_l_curly_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "lCurlyToken", context)
    }
    fn get_field_l_paren_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "lParenToken", context)
    }
    fn get_field_l_tick_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "lTickToken", context)
    }
    fn get_field_label(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "label", context)
    }
    fn get_field_leading_separator_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "leadingSeparatorToken", context)
    }
    fn get_field_left(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "left", context)
    }
    fn get_field_literal(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "literal", context)
    }
    fn get_field_literal_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "literalToken", context)
    }
    fn get_field_local_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "localName", context)
    }
    fn get_field_mapped_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "mappedType", context)
    }
    fn get_field_member(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "member", context)
    }
    fn get_field_members(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "members", context)
    }
    fn get_field_meta_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "metaToken", context)
    }
    fn get_field_minus_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "minusToken", context)
    }
    fn get_field_modifier_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "modifierToken", context)
    }
    fn get_field_modifiers(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "modifiers", context)
    }
    fn get_field_module_or_namespace(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "moduleOrNamespace", context)
    }
    fn get_field_module_reference(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "moduleReference", context)
    }
    fn get_field_module_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "moduleToken", context)
    }
    fn get_field_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "name", context)
    }
    fn get_field_name_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "nameToken", context)
    }
    fn get_field_named_specifiers(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "namedSpecifiers", context)
    }
    fn get_field_namespace(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "namespace", context)
    }
    fn get_field_namespace_specifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "namespaceSpecifier", context)
    }
    fn get_field_namespace_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "namespaceToken", context)
    }
    fn get_field_never_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "neverToken", context)
    }
    fn get_field_new_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "newToken", context)
    }
    fn get_field_number_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "numberToken", context)
    }
    fn get_field_object(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "object", context)
    }
    fn get_field_object_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "objectToken", context)
    }
    fn get_field_object_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "objectType", context)
    }
    fn get_field_of_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "ofToken", context)
    }
    fn get_field_opening_element(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "openingElement", context)
    }
    fn get_field_opening_fragment(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "openingFragment", context)
    }
    fn get_field_operand(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "operand", context)
    }
    fn get_field_operator_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "operatorToken", context)
    }
    fn get_field_optional_chain_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "optionalChainToken", context)
    }
    fn get_field_optional_modifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "optionalModifier", context)
    }
    fn get_field_optional_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "optionalToken", context)
    }
    fn get_field_parameter(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "parameter", context)
    }
    fn get_field_parameter_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "parameterName", context)
    }
    fn get_field_parameters(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "parameters", context)
    }
    fn get_field_pattern(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "pattern", context)
    }
    fn get_field_phase(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "phase", context)
    }
    fn get_field_phase_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "phaseToken", context)
    }
    fn get_field_predicate(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "predicate", context)
    }
    fn get_field_properties(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "properties", context)
    }
    fn get_field_property(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "property", context)
    }
    fn get_field_property_annotation(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "propertyAnnotation", context)
    }
    fn get_field_property_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "propertyName", context)
    }
    fn get_field_qualifier_clause(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "qualifierClause", context)
    }
    fn get_field_question_mark_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "questionMarkToken", context)
    }
    fn get_field_r_angle_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "rAngleToken", context)
    }
    fn get_field_r_brack_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "rBrackToken", context)
    }
    fn get_field_r_curly_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "rCurlyToken", context)
    }
    fn get_field_r_paren_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "rParenToken", context)
    }
    fn get_field_r_tick_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "rTickToken", context)
    }
    fn get_field_readonly_modifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "readonlyModifier", context)
    }
    fn get_field_readonly_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "readonlyToken", context)
    }
    fn get_field_require_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "requireToken", context)
    }
    fn get_field_return_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "returnToken", context)
    }
    fn get_field_return_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "returnType", context)
    }
    fn get_field_return_type_annotation(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "returnTypeAnnotation", context)
    }
    fn get_field_right(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "right", context)
    }
    fn get_field_satisfies_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "satisfiesToken", context)
    }
    fn get_field_second_semi_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "secondSemiToken", context)
    }
    fn get_field_semicolon_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "semicolonToken", context)
    }
    fn get_field_separator_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "separatorToken", context)
    }
    fn get_field_set_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "setToken", context)
    }
    fn get_field_slash_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "slashToken", context)
    }
    fn get_field_source(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "source", context)
    }
    fn get_field_source_name(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "sourceName", context)
    }
    fn get_field_specifier(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "specifier", context)
    }
    fn get_field_specifiers(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "specifiers", context)
    }
    fn get_field_star_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "starToken", context)
    }
    fn get_field_statements(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "statements", context)
    }
    fn get_field_static_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "staticToken", context)
    }
    fn get_field_string_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "stringToken", context)
    }
    fn get_field_super_class(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "superClass", context)
    }
    fn get_field_super_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "superToken", context)
    }
    fn get_field_switch_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "switchToken", context)
    }
    fn get_field_symbol_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "symbolToken", context)
    }
    fn get_field_tag(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "tag", context)
    }
    fn get_field_target(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "target", context)
    }
    fn get_field_target_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "targetToken", context)
    }
    fn get_field_template_chunk_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "templateChunkToken", context)
    }
    fn get_field_test(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "test", context)
    }
    fn get_field_this_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "thisToken", context)
    }
    fn get_field_throw_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "throwToken", context)
    }
    fn get_field_true_type(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "trueType", context)
    }
    fn get_field_try_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "tryToken", context)
    }
    fn get_field_ts_import_type_assertion_block(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "tsImportTypeAssertionBlock", context)
    }
    fn get_field_ts_type_argument_list(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "tsTypeArgumentList", context)
    }
    fn get_field_ty(this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        Self::resolve_field(this, "ty", context)
    }
    fn get_field_type_annotation(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "typeAnnotation", context)
    }
    fn get_field_type_arguments(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "typeArguments", context)
    }
    fn get_field_type_assertion(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "typeAssertion", context)
    }
    fn get_field_type_parameters(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "typeParameters", context)
    }
    fn get_field_type_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "typeToken", context)
    }
    fn get_field_typeof_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "typeofToken", context)
    }
    fn get_field_types(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "types", context)
    }
    fn get_field_undefined_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "undefinedToken", context)
    }
    fn get_field_unknown_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "unknownToken", context)
    }
    fn get_field_update(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "update", context)
    }
    fn get_field_value(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "value", context)
    }
    fn get_field_value_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "valueToken", context)
    }
    fn get_field_variable_annotation(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "variableAnnotation", context)
    }
    fn get_field_void_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "voidToken", context)
    }
    fn get_field_while_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "whileToken", context)
    }
    fn get_field_with_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "withToken", context)
    }
    fn get_field_yield_token(
        this: &JsValue,
        _args: &[JsValue],
        context: &mut Context,
    ) -> JsResult<JsValue> {
        Self::resolve_field(this, "yieldToken", context)
    }
    fn init_generated(class: &mut ClassBuilder<'_>) {
        let getter = NativeFunction::from_fn_ptr(Self::get_field_abstract_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("abstractToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_alternate)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("alternate"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_any_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("anyToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_args)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("args"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_argument)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("argument"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_arguments)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("arguments"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_as_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("asClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_as_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("asToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_assertion)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("assertion"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_assertions)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("assertions"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_asserts_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("assertsToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_assignment)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("assignment"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_async_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("asyncToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_at_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("atToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_attributes)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("attributes"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_await_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("awaitToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_bigint_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("bigintToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_binding)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("binding"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_binding_identifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("bindingIdentifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_body)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("body"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_bom_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("bomToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_boolean_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("booleanToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_break_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("breakToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_callee)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("callee"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_case_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("caseToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_cases)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("cases"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_catch_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("catchClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_catch_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("catchToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_check_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("checkType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_children)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("children"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_class_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("classToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_closing_element)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("closingElement"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_closing_fragment)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("closingFragment"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_colon_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("colonToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_comma_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("commaToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_consequent)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("consequent"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_const_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("constToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_constraint)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("constraint"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_continue_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("continueToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_debugger_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("debuggerToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_declaration)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("declaration"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_declarator)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("declarator"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_declarators)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("declarators"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_declare_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("declareToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_decorators)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("decorators"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_default)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("default"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_default_specifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("defaultSpecifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_default_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("defaultToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_directives)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("directives"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_discriminant)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("discriminant"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_do_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("doToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_dollar_curly_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("dollarCurlyToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_dot_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("dotToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_dotdotdot_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("dotdotdotToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_element_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("elementType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_elements)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("elements"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_else_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("elseClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_else_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("elseToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_enum_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("enumToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_eof_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("eofToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_eq_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("eqToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_excl_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("exclToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_export_as)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("exportAs"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_export_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("exportClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_export_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("exportToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_exported_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("exportedName"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_expression)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("expression"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_expression_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("expressionName"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_extends_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("extendsClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_extends_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("extendsToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_extends_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("extendsType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_false_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("falseType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_fat_arrow_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("fatArrowToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_finally_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("finallyClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_finally_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("finallyToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_first_semi_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("firstSemiToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_for_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("forToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_formal_parameter)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("formalParameter"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_from_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("fromToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_function_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("functionToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_get_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("getToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_global_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("globalToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_hash_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("hashToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter =
            NativeFunction::from_fn_ptr(Self::get_field_id).to_js_function(class.context().realm());
        class.accessor(js_string!("id"), Some(getter), None, Attribute::ENUMERABLE);
        let getter = NativeFunction::from_fn_ptr(Self::get_field_id_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("idToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_ident_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("identToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_identifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("identifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_if_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("ifToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_implements_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("implementsClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_implements_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("implementsToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_import_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("importClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_import_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("importToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_in_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("inToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_index_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("indexType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_infer_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("inferToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_init)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("init"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_initializer)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("initializer"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_instanceof_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("instanceofToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_interface_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("interfaceToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_interpreter_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("interpreterToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_is_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("isToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_items)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("items"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_key)
            .to_js_function(class.context().realm());
        class.accessor(js_string!("key"), Some(getter), None, Attribute::ENUMERABLE);
        let getter = NativeFunction::from_fn_ptr(Self::get_field_keys_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("keysType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_kind_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("kindToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_l_angle_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("lAngleToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_l_brack_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("lBrackToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_l_curly_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("lCurlyToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_l_paren_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("lParenToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_l_tick_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("lTickToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_label)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("label"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_leading_separator_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("leadingSeparatorToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_left)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("left"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_literal)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("literal"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_literal_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("literalToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_local_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("localName"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_mapped_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("mappedType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_member)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("member"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_members)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("members"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_meta_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("metaToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_minus_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("minusToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_modifier_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("modifierToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_modifiers)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("modifiers"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_module_or_namespace)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("moduleOrNamespace"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_module_reference)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("moduleReference"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_module_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("moduleToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("name"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_name_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("nameToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_named_specifiers)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("namedSpecifiers"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_namespace)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("namespace"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_namespace_specifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("namespaceSpecifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_namespace_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("namespaceToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_never_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("neverToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_new_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("newToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_number_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("numberToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_object)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("object"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_object_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("objectToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_object_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("objectType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_of_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("ofToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_opening_element)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("openingElement"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_opening_fragment)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("openingFragment"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_operand)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("operand"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_operator_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("operatorToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_optional_chain_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("optionalChainToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_optional_modifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("optionalModifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_optional_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("optionalToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_parameter)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("parameter"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_parameter_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("parameterName"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_parameters)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("parameters"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_pattern)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("pattern"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_phase)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("phase"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_phase_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("phaseToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_predicate)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("predicate"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_properties)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("properties"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_property)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("property"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_property_annotation)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("propertyAnnotation"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_property_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("propertyName"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_qualifier_clause)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("qualifierClause"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_question_mark_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("questionMarkToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_r_angle_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("rAngleToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_r_brack_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("rBrackToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_r_curly_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("rCurlyToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_r_paren_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("rParenToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_r_tick_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("rTickToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_readonly_modifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("readonlyModifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_readonly_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("readonlyToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_require_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("requireToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_return_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("returnToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_return_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("returnType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_return_type_annotation)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("returnTypeAnnotation"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_right)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("right"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_satisfies_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("satisfiesToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_second_semi_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("secondSemiToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_semicolon_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("semicolonToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_separator_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("separatorToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_set_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("setToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_slash_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("slashToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_source)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("source"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_source_name)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("sourceName"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_specifier)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("specifier"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_specifiers)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("specifiers"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_star_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("starToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_statements)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("statements"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_static_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("staticToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_string_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("stringToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_super_class)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("superClass"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_super_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("superToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_switch_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("switchToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_symbol_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("symbolToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_tag)
            .to_js_function(class.context().realm());
        class.accessor(js_string!("tag"), Some(getter), None, Attribute::ENUMERABLE);
        let getter = NativeFunction::from_fn_ptr(Self::get_field_target)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("target"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_target_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("targetToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_template_chunk_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("templateChunkToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_test)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("test"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_this_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("thisToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_throw_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("throwToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_true_type)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("trueType"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_try_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("tryToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_ts_import_type_assertion_block)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("tsImportTypeAssertionBlock"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_ts_type_argument_list)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("tsTypeArgumentList"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter =
            NativeFunction::from_fn_ptr(Self::get_field_ty).to_js_function(class.context().realm());
        class.accessor(js_string!("ty"), Some(getter), None, Attribute::ENUMERABLE);
        let getter = NativeFunction::from_fn_ptr(Self::get_field_type_annotation)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("typeAnnotation"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_type_arguments)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("typeArguments"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_type_assertion)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("typeAssertion"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_type_parameters)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("typeParameters"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_type_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("typeToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_typeof_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("typeofToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_types)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("types"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_undefined_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("undefinedToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_unknown_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("unknownToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_update)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("update"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_value)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("value"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_value_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("valueToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_variable_annotation)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("variableAnnotation"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_void_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("voidToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_while_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("whileToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_with_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("withToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
        let getter = NativeFunction::from_fn_ptr(Self::get_field_yield_token)
            .to_js_function(class.context().realm());
        class.accessor(
            js_string!("yieldToken"),
            Some(getter),
            None,
            Attribute::ENUMERABLE,
        );
    }
}
