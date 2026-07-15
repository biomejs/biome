// Generated file, do not edit by hand, see `xtask/codegen`.

export interface JsAstNode {
	readonly kind: string;
	readonly text: string;
}

export interface JsAccessorModifier extends JsAstNode {
	readonly kind: "JS_ACCESSOR_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface JsArrayAssignmentPattern extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN";
	readonly lBrackToken: string | undefined;
	readonly elements: JsArrayAssignmentPatternElementList;
	readonly rBrackToken: string | undefined;
}

export interface JsArrayAssignmentPatternElement extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT";
	readonly pattern: AnyJsAssignmentPattern | undefined;
	readonly init: JsInitializerClause | undefined;
}

export interface JsArrayAssignmentPatternRestElement extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	readonly pattern: AnyJsAssignmentPattern | undefined;
}

export interface JsArrayBindingPattern extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN";
	readonly lBrackToken: string | undefined;
	readonly elements: JsArrayBindingPatternElementList;
	readonly rBrackToken: string | undefined;
}

export interface JsArrayBindingPatternElement extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN_ELEMENT";
	readonly pattern: AnyJsBindingPattern | undefined;
	readonly init: JsInitializerClause | undefined;
}

export interface JsArrayBindingPatternRestElement extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN_REST_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	readonly pattern: AnyJsBindingPattern | undefined;
}

export interface JsArrayExpression extends JsAstNode {
	readonly kind: "JS_ARRAY_EXPRESSION";
	readonly lBrackToken: string | undefined;
	readonly elements: JsArrayElementList;
	readonly rBrackToken: string | undefined;
}

export interface JsArrayHole extends JsAstNode {
	readonly kind: "JS_ARRAY_HOLE";
}

export interface JsArrowFunctionExpression extends JsAstNode {
	readonly kind: "JS_ARROW_FUNCTION_EXPRESSION";
	readonly asyncToken: string | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: AnyJsArrowFunctionParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly fatArrowToken: string | undefined;
	readonly body: AnyJsFunctionBody | undefined;
}

export interface JsAssignmentExpression extends JsAstNode {
	readonly kind: "JS_ASSIGNMENT_EXPRESSION";
	readonly left: AnyJsAssignmentPattern | undefined;
	readonly operatorToken: string | undefined;
	readonly right: AnyJsExpression | undefined;
}

export interface JsAwaitExpression extends JsAstNode {
	readonly kind: "JS_AWAIT_EXPRESSION";
	readonly awaitToken: string | undefined;
	readonly argument: AnyJsExpression | undefined;
}

export interface JsBigintLiteralExpression extends JsAstNode {
	readonly kind: "JS_BIGINT_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
}

export interface JsBinaryExpression extends JsAstNode {
	readonly kind: "JS_BINARY_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	readonly operatorToken: string | undefined;
	readonly right: AnyJsExpression | undefined;
}

export interface JsBlockStatement extends JsAstNode {
	readonly kind: "JS_BLOCK_STATEMENT";
	readonly lCurlyToken: string | undefined;
	readonly statements: JsStatementList;
	readonly rCurlyToken: string | undefined;
}

export interface JsBooleanLiteralExpression extends JsAstNode {
	readonly kind: "JS_BOOLEAN_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
}

export interface JsBreakStatement extends JsAstNode {
	readonly kind: "JS_BREAK_STATEMENT";
	readonly breakToken: string | undefined;
	readonly label: JsLabel | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsCallArguments extends JsAstNode {
	readonly kind: "JS_CALL_ARGUMENTS";
	readonly lParenToken: string | undefined;
	readonly args: JsCallArgumentList;
	readonly rParenToken: string | undefined;
}

export interface JsCallExpression extends JsAstNode {
	readonly kind: "JS_CALL_EXPRESSION";
	readonly callee: AnyJsExpression | undefined;
	readonly optionalChainToken: string | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
	readonly arguments: JsCallArguments | undefined;
}

export interface JsCaseClause extends JsAstNode {
	readonly kind: "JS_CASE_CLAUSE";
	readonly caseToken: string | undefined;
	readonly test: AnyJsExpression | undefined;
	readonly colonToken: string | undefined;
	readonly consequent: JsStatementList;
}

export interface JsCatchClause extends JsAstNode {
	readonly kind: "JS_CATCH_CLAUSE";
	readonly catchToken: string | undefined;
	readonly declaration: JsCatchDeclaration | undefined;
	readonly body: JsBlockStatement | undefined;
}

export interface JsCatchDeclaration extends JsAstNode {
	readonly kind: "JS_CATCH_DECLARATION";
	readonly lParenToken: string | undefined;
	readonly binding: AnyJsBindingPattern | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly rParenToken: string | undefined;
}

export interface JsClassDeclaration extends JsAstNode {
	readonly kind: "JS_CLASS_DECLARATION";
	readonly decorators: JsDecoratorList;
	readonly abstractToken: string | undefined;
	readonly classToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly extendsClause: JsExtendsClause | undefined;
	readonly implementsClause: TsImplementsClause | undefined;
	readonly lCurlyToken: string | undefined;
	readonly members: JsClassMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface JsClassExportDefaultDeclaration extends JsAstNode {
	readonly kind: "JS_CLASS_EXPORT_DEFAULT_DECLARATION";
	readonly decorators: JsDecoratorList;
	readonly abstractToken: string | undefined;
	readonly classToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly extendsClause: JsExtendsClause | undefined;
	readonly implementsClause: TsImplementsClause | undefined;
	readonly lCurlyToken: string | undefined;
	readonly members: JsClassMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface JsClassExpression extends JsAstNode {
	readonly kind: "JS_CLASS_EXPRESSION";
	readonly decorators: JsDecoratorList;
	readonly classToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly extendsClause: JsExtendsClause | undefined;
	readonly implementsClause: TsImplementsClause | undefined;
	readonly lCurlyToken: string | undefined;
	readonly members: JsClassMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface JsComputedMemberAssignment extends JsAstNode {
	readonly kind: "JS_COMPUTED_MEMBER_ASSIGNMENT";
	readonly object: AnyJsExpression | undefined;
	readonly lBrackToken: string | undefined;
	readonly member: AnyJsExpression | undefined;
	readonly rBrackToken: string | undefined;
}

export interface JsComputedMemberExpression extends JsAstNode {
	readonly kind: "JS_COMPUTED_MEMBER_EXPRESSION";
	readonly object: AnyJsExpression | undefined;
	readonly optionalChainToken: string | undefined;
	readonly lBrackToken: string | undefined;
	readonly member: AnyJsExpression | undefined;
	readonly rBrackToken: string | undefined;
}

export interface JsComputedMemberName extends JsAstNode {
	readonly kind: "JS_COMPUTED_MEMBER_NAME";
	readonly lBrackToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rBrackToken: string | undefined;
}

export interface JsConditionalExpression extends JsAstNode {
	readonly kind: "JS_CONDITIONAL_EXPRESSION";
	readonly test: AnyJsExpression | undefined;
	readonly questionMarkToken: string | undefined;
	readonly consequent: AnyJsExpression | undefined;
	readonly colonToken: string | undefined;
	readonly alternate: AnyJsExpression | undefined;
}

export interface JsConstructorClassMember extends JsAstNode {
	readonly kind: "JS_CONSTRUCTOR_CLASS_MEMBER";
	readonly modifiers: JsConstructorModifierList;
	readonly name: JsLiteralMemberName | undefined;
	readonly parameters: JsConstructorParameters | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsConstructorParameters extends JsAstNode {
	readonly kind: "JS_CONSTRUCTOR_PARAMETERS";
	readonly lParenToken: string | undefined;
	readonly parameters: JsConstructorParameterList;
	readonly rParenToken: string | undefined;
}

export interface JsContinueStatement extends JsAstNode {
	readonly kind: "JS_CONTINUE_STATEMENT";
	readonly continueToken: string | undefined;
	readonly label: JsLabel | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsDebuggerStatement extends JsAstNode {
	readonly kind: "JS_DEBUGGER_STATEMENT";
	readonly debuggerToken: string | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsDecorator extends JsAstNode {
	readonly kind: "JS_DECORATOR";
	readonly atToken: string | undefined;
	readonly expression: AnyJsDecorator | undefined;
}

export interface JsDefaultClause extends JsAstNode {
	readonly kind: "JS_DEFAULT_CLAUSE";
	readonly defaultToken: string | undefined;
	readonly colonToken: string | undefined;
	readonly consequent: JsStatementList;
}

export interface JsDefaultImportSpecifier extends JsAstNode {
	readonly kind: "JS_DEFAULT_IMPORT_SPECIFIER";
	readonly localName: AnyJsBinding | undefined;
}

export interface JsDirective extends JsAstNode {
	readonly kind: "JS_DIRECTIVE";
	readonly valueToken: string | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsDoWhileStatement extends JsAstNode {
	readonly kind: "JS_DO_WHILE_STATEMENT";
	readonly doToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
	readonly whileToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly test: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsElseClause extends JsAstNode {
	readonly kind: "JS_ELSE_CLAUSE";
	readonly elseToken: string | undefined;
	readonly alternate: AnyJsStatement | undefined;
}

export interface JsEmptyClassMember extends JsAstNode {
	readonly kind: "JS_EMPTY_CLASS_MEMBER";
	readonly semicolonToken: string | undefined;
}

export interface JsEmptyStatement extends JsAstNode {
	readonly kind: "JS_EMPTY_STATEMENT";
	readonly semicolonToken: string | undefined;
}

export interface JsExport extends JsAstNode {
	readonly kind: "JS_EXPORT";
	readonly decorators: JsDecoratorList;
	readonly exportToken: string | undefined;
	readonly exportClause: AnyJsExportClause | undefined;
}

export interface JsExportAsClause extends JsAstNode {
	readonly kind: "JS_EXPORT_AS_CLAUSE";
	readonly asToken: string | undefined;
	readonly exportedName: AnyJsLiteralExportName | undefined;
}

export interface JsExportDefaultDeclarationClause extends JsAstNode {
	readonly kind: "JS_EXPORT_DEFAULT_DECLARATION_CLAUSE";
	readonly defaultToken: string | undefined;
	readonly declaration: AnyJsExportDefaultDeclaration | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsExportDefaultExpressionClause extends JsAstNode {
	readonly kind: "JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE";
	readonly defaultToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsExportFromClause extends JsAstNode {
	readonly kind: "JS_EXPORT_FROM_CLAUSE";
	readonly typeToken: string | undefined;
	readonly starToken: string | undefined;
	readonly exportAs: JsExportAsClause | undefined;
	readonly fromToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsExportNamedClause extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_CLAUSE";
	readonly typeToken: string | undefined;
	readonly lCurlyToken: string | undefined;
	readonly specifiers: JsExportNamedSpecifierList;
	readonly rCurlyToken: string | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsExportNamedFromClause extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_FROM_CLAUSE";
	readonly typeToken: string | undefined;
	readonly lCurlyToken: string | undefined;
	readonly specifiers: JsExportNamedFromSpecifierList;
	readonly rCurlyToken: string | undefined;
	readonly fromToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsExportNamedFromSpecifier extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_FROM_SPECIFIER";
	readonly typeToken: string | undefined;
	readonly sourceName: AnyJsLiteralExportName | undefined;
	readonly exportAs: JsExportAsClause | undefined;
}

export interface JsExportNamedShorthandSpecifier extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_SHORTHAND_SPECIFIER";
	readonly typeToken: string | undefined;
	readonly name: JsReferenceIdentifier | undefined;
}

export interface JsExportNamedSpecifier extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_SPECIFIER";
	readonly typeToken: string | undefined;
	readonly localName: JsReferenceIdentifier | undefined;
	readonly asToken: string | undefined;
	readonly exportedName: AnyJsLiteralExportName | undefined;
}

export interface JsExpressionSnippet extends JsAstNode {
	readonly kind: "JS_EXPRESSION_SNIPPET";
	readonly expression: AnyJsExpression | undefined;
	readonly eofToken: string | undefined;
}

export interface JsExpressionStatement extends JsAstNode {
	readonly kind: "JS_EXPRESSION_STATEMENT";
	readonly expression: AnyJsExpression | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsExpressionTemplateRoot extends JsAstNode {
	readonly kind: "JS_EXPRESSION_TEMPLATE_ROOT";
	readonly expression: AnyJsExpression | undefined;
	readonly eofToken: string | undefined;
}

export interface JsExtendsClause extends JsAstNode {
	readonly kind: "JS_EXTENDS_CLAUSE";
	readonly extendsToken: string | undefined;
	readonly superClass: AnyJsExpression | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
}

export interface JsFinallyClause extends JsAstNode {
	readonly kind: "JS_FINALLY_CLAUSE";
	readonly finallyToken: string | undefined;
	readonly body: JsBlockStatement | undefined;
}

export interface JsForInStatement extends JsAstNode {
	readonly kind: "JS_FOR_IN_STATEMENT";
	readonly forToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly initializer: AnyJsForInOrOfInitializer | undefined;
	readonly inToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
}

export interface JsForOfStatement extends JsAstNode {
	readonly kind: "JS_FOR_OF_STATEMENT";
	readonly forToken: string | undefined;
	readonly awaitToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly initializer: AnyJsForInOrOfInitializer | undefined;
	readonly ofToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
}

export interface JsForStatement extends JsAstNode {
	readonly kind: "JS_FOR_STATEMENT";
	readonly forToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly initializer: AnyJsForInitializer | undefined;
	readonly firstSemiToken: string | undefined;
	readonly test: AnyJsExpression | undefined;
	readonly secondSemiToken: string | undefined;
	readonly update: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
}

export interface JsForVariableDeclaration extends JsAstNode {
	readonly kind: "JS_FOR_VARIABLE_DECLARATION";
	readonly awaitToken: string | undefined;
	readonly kindToken: string | undefined;
	readonly declarator: JsVariableDeclarator | undefined;
}

export interface JsFormalParameter extends JsAstNode {
	readonly kind: "JS_FORMAL_PARAMETER";
	readonly decorators: JsDecoratorList;
	readonly binding: AnyJsBindingPattern | undefined;
	readonly questionMarkToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly initializer: JsInitializerClause | undefined;
}

export interface JsFunctionBody extends JsAstNode {
	readonly kind: "JS_FUNCTION_BODY";
	readonly lCurlyToken: string | undefined;
	readonly directives: JsDirectiveList;
	readonly statements: JsStatementList;
	readonly rCurlyToken: string | undefined;
}

export interface JsFunctionDeclaration extends JsAstNode {
	readonly kind: "JS_FUNCTION_DECLARATION";
	readonly asyncToken: string | undefined;
	readonly functionToken: string | undefined;
	readonly starToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsFunctionExportDefaultDeclaration extends JsAstNode {
	readonly kind: "JS_FUNCTION_EXPORT_DEFAULT_DECLARATION";
	readonly asyncToken: string | undefined;
	readonly functionToken: string | undefined;
	readonly starToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsFunctionExpression extends JsAstNode {
	readonly kind: "JS_FUNCTION_EXPRESSION";
	readonly asyncToken: string | undefined;
	readonly functionToken: string | undefined;
	readonly starToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsGetterClassMember extends JsAstNode {
	readonly kind: "JS_GETTER_CLASS_MEMBER";
	readonly modifiers: JsMethodModifierList;
	readonly getToken: string | undefined;
	readonly name: AnyJsClassMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly returnType: TsTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsGetterObjectMember extends JsAstNode {
	readonly kind: "JS_GETTER_OBJECT_MEMBER";
	readonly getToken: string | undefined;
	readonly name: AnyJsObjectMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly returnType: TsTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsIdentifierAssignment extends JsAstNode {
	readonly kind: "JS_IDENTIFIER_ASSIGNMENT";
	readonly nameToken: string | undefined;
}

export interface JsIdentifierBinding extends JsAstNode {
	readonly kind: "JS_IDENTIFIER_BINDING";
	readonly nameToken: string | undefined;
}

export interface JsIdentifierExpression extends JsAstNode {
	readonly kind: "JS_IDENTIFIER_EXPRESSION";
	readonly name: JsReferenceIdentifier | undefined;
}

export interface JsIfStatement extends JsAstNode {
	readonly kind: "JS_IF_STATEMENT";
	readonly ifToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly test: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly consequent: AnyJsStatement | undefined;
	readonly elseClause: JsElseClause | undefined;
}

export interface JsImport extends JsAstNode {
	readonly kind: "JS_IMPORT";
	readonly importToken: string | undefined;
	readonly importClause: AnyJsImportClause | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsImportAssertion extends JsAstNode {
	readonly kind: "JS_IMPORT_ASSERTION";
	readonly withToken: string | undefined;
	readonly lCurlyToken: string | undefined;
	readonly assertions: JsImportAssertionEntryList;
	readonly rCurlyToken: string | undefined;
}

export interface JsImportAssertionEntry extends JsAstNode {
	readonly kind: "JS_IMPORT_ASSERTION_ENTRY";
	readonly key: string | undefined;
	readonly colonToken: string | undefined;
	readonly valueToken: string | undefined;
}

export interface JsImportBareClause extends JsAstNode {
	readonly kind: "JS_IMPORT_BARE_CLAUSE";
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
}

export interface JsImportCallExpression extends JsAstNode {
	readonly kind: "JS_IMPORT_CALL_EXPRESSION";
	readonly importToken: string | undefined;
	readonly dotToken: string | undefined;
	readonly phase: string | undefined;
	readonly arguments: JsCallArguments | undefined;
}

export interface JsImportCombinedClause extends JsAstNode {
	readonly kind: "JS_IMPORT_COMBINED_CLAUSE";
	readonly defaultSpecifier: JsDefaultImportSpecifier | undefined;
	readonly commaToken: string | undefined;
	readonly specifier: AnyJsCombinedSpecifier | undefined;
	readonly fromToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
}

export interface JsImportDefaultClause extends JsAstNode {
	readonly kind: "JS_IMPORT_DEFAULT_CLAUSE";
	readonly typeToken: string | undefined;
	readonly phaseToken: string | undefined;
	readonly defaultSpecifier: JsDefaultImportSpecifier | undefined;
	readonly fromToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
}

export interface JsImportMetaExpression extends JsAstNode {
	readonly kind: "JS_IMPORT_META_EXPRESSION";
	readonly importToken: string | undefined;
	readonly dotToken: string | undefined;
	readonly metaToken: string | undefined;
}

export interface JsImportNamedClause extends JsAstNode {
	readonly kind: "JS_IMPORT_NAMED_CLAUSE";
	readonly typeToken: string | undefined;
	readonly namedSpecifiers: JsNamedImportSpecifiers | undefined;
	readonly fromToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
}

export interface JsImportNamespaceClause extends JsAstNode {
	readonly kind: "JS_IMPORT_NAMESPACE_CLAUSE";
	readonly typeToken: string | undefined;
	readonly phaseToken: string | undefined;
	readonly namespaceSpecifier: JsNamespaceImportSpecifier | undefined;
	readonly fromToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly assertion: JsImportAssertion | undefined;
}

export interface JsInExpression extends JsAstNode {
	readonly kind: "JS_IN_EXPRESSION";
	readonly property: AnyJsInProperty | undefined;
	readonly inToken: string | undefined;
	readonly object: AnyJsExpression | undefined;
}

export interface JsInitializerClause extends JsAstNode {
	readonly kind: "JS_INITIALIZER_CLAUSE";
	readonly eqToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
}

export interface JsInstanceofExpression extends JsAstNode {
	readonly kind: "JS_INSTANCEOF_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	readonly instanceofToken: string | undefined;
	readonly right: AnyJsExpression | undefined;
}

export interface JsLabel extends JsAstNode {
	readonly kind: "JS_LABEL";
	readonly valueToken: string | undefined;
}

export interface JsLabeledStatement extends JsAstNode {
	readonly kind: "JS_LABELED_STATEMENT";
	readonly label: JsLabel | undefined;
	readonly colonToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
}

export interface JsLiteralExportName extends JsAstNode {
	readonly kind: "JS_LITERAL_EXPORT_NAME";
	readonly value: string | undefined;
}

export interface JsLiteralMemberName extends JsAstNode {
	readonly kind: "JS_LITERAL_MEMBER_NAME";
	readonly value: string | undefined;
}

export interface JsLogicalExpression extends JsAstNode {
	readonly kind: "JS_LOGICAL_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	readonly operatorToken: string | undefined;
	readonly right: AnyJsExpression | undefined;
}

export interface JsMetavariable extends JsAstNode {
	readonly kind: "JS_METAVARIABLE";
	readonly valueToken: string | undefined;
}

export interface JsMethodClassMember extends JsAstNode {
	readonly kind: "JS_METHOD_CLASS_MEMBER";
	readonly modifiers: JsMethodModifierList;
	readonly asyncToken: string | undefined;
	readonly starToken: string | undefined;
	readonly name: AnyJsClassMemberName | undefined;
	readonly questionMarkToken: string | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsMethodObjectMember extends JsAstNode {
	readonly kind: "JS_METHOD_OBJECT_MEMBER";
	readonly asyncToken: string | undefined;
	readonly starToken: string | undefined;
	readonly name: AnyJsObjectMemberName | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsModule extends JsAstNode {
	readonly kind: "JS_MODULE";
	readonly bomToken: string | undefined;
	readonly interpreterToken: string | undefined;
	readonly directives: JsDirectiveList;
	readonly items: JsModuleItemList;
	readonly eofToken: string | undefined;
}

export interface JsModuleSource extends JsAstNode {
	readonly kind: "JS_MODULE_SOURCE";
	readonly valueToken: string | undefined;
}

export interface JsName extends JsAstNode {
	readonly kind: "JS_NAME";
	readonly valueToken: string | undefined;
}

export interface JsNamedImportSpecifier extends JsAstNode {
	readonly kind: "JS_NAMED_IMPORT_SPECIFIER";
	readonly typeToken: string | undefined;
	readonly name: AnyJsLiteralExportName | undefined;
	readonly asToken: string | undefined;
	readonly localName: AnyJsBinding | undefined;
}

export interface JsNamedImportSpecifiers extends JsAstNode {
	readonly kind: "JS_NAMED_IMPORT_SPECIFIERS";
	readonly lCurlyToken: string | undefined;
	readonly specifiers: JsNamedImportSpecifierList;
	readonly rCurlyToken: string | undefined;
}

export interface JsNamespaceImportSpecifier extends JsAstNode {
	readonly kind: "JS_NAMESPACE_IMPORT_SPECIFIER";
	readonly starToken: string | undefined;
	readonly asToken: string | undefined;
	readonly localName: AnyJsBinding | undefined;
}

export interface JsNewExpression extends JsAstNode {
	readonly kind: "JS_NEW_EXPRESSION";
	readonly newToken: string | undefined;
	readonly callee: AnyJsExpression | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
	readonly arguments: JsCallArguments | undefined;
}

export interface JsNewTargetExpression extends JsAstNode {
	readonly kind: "JS_NEW_TARGET_EXPRESSION";
	readonly newToken: string | undefined;
	readonly dotToken: string | undefined;
	readonly targetToken: string | undefined;
}

export interface JsNullLiteralExpression extends JsAstNode {
	readonly kind: "JS_NULL_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
}

export interface JsNumberLiteralExpression extends JsAstNode {
	readonly kind: "JS_NUMBER_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
}

export interface JsObjectAssignmentPattern extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN";
	readonly lCurlyToken: string | undefined;
	readonly properties: JsObjectAssignmentPatternPropertyList;
	readonly rCurlyToken: string | undefined;
}

export interface JsObjectAssignmentPatternProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY";
	readonly member: AnyJsObjectMemberName | undefined;
	readonly colonToken: string | undefined;
	readonly pattern: AnyJsAssignmentPattern | undefined;
	readonly init: JsInitializerClause | undefined;
}

export interface JsObjectAssignmentPatternRest extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_REST";
	readonly dotdotdotToken: string | undefined;
	readonly target: AnyJsAssignment | undefined;
}

export interface JsObjectAssignmentPatternShorthandProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY";
	readonly identifier: JsIdentifierAssignment | undefined;
	readonly init: JsInitializerClause | undefined;
}

export interface JsObjectBindingPattern extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN";
	readonly lCurlyToken: string | undefined;
	readonly properties: JsObjectBindingPatternPropertyList;
	readonly rCurlyToken: string | undefined;
}

export interface JsObjectBindingPatternProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_PROPERTY";
	readonly member: AnyJsObjectMemberName | undefined;
	readonly colonToken: string | undefined;
	readonly pattern: AnyJsBindingPattern | undefined;
	readonly init: JsInitializerClause | undefined;
}

export interface JsObjectBindingPatternRest extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_REST";
	readonly dotdotdotToken: string | undefined;
	readonly binding: AnyJsBinding | undefined;
}

export interface JsObjectBindingPatternShorthandProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY";
	readonly identifier: AnyJsBinding | undefined;
	readonly init: JsInitializerClause | undefined;
}

export interface JsObjectExpression extends JsAstNode {
	readonly kind: "JS_OBJECT_EXPRESSION";
	readonly lCurlyToken: string | undefined;
	readonly members: JsObjectMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface JsParameters extends JsAstNode {
	readonly kind: "JS_PARAMETERS";
	readonly lParenToken: string | undefined;
	readonly items: JsParameterList;
	readonly rParenToken: string | undefined;
}

export interface JsParenthesizedAssignment extends JsAstNode {
	readonly kind: "JS_PARENTHESIZED_ASSIGNMENT";
	readonly lParenToken: string | undefined;
	readonly assignment: AnyJsAssignment | undefined;
	readonly rParenToken: string | undefined;
}

export interface JsParenthesizedExpression extends JsAstNode {
	readonly kind: "JS_PARENTHESIZED_EXPRESSION";
	readonly lParenToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
}

export interface JsPostUpdateExpression extends JsAstNode {
	readonly kind: "JS_POST_UPDATE_EXPRESSION";
	readonly operand: AnyJsAssignment | undefined;
	readonly operatorToken: string | undefined;
}

export interface JsPreUpdateExpression extends JsAstNode {
	readonly kind: "JS_PRE_UPDATE_EXPRESSION";
	readonly operatorToken: string | undefined;
	readonly operand: AnyJsAssignment | undefined;
}

export interface JsPrivateClassMemberName extends JsAstNode {
	readonly kind: "JS_PRIVATE_CLASS_MEMBER_NAME";
	readonly hashToken: string | undefined;
	readonly idToken: string | undefined;
}

export interface JsPrivateName extends JsAstNode {
	readonly kind: "JS_PRIVATE_NAME";
	readonly hashToken: string | undefined;
	readonly valueToken: string | undefined;
}

export interface JsPropertyClassMember extends JsAstNode {
	readonly kind: "JS_PROPERTY_CLASS_MEMBER";
	readonly modifiers: JsPropertyModifierList;
	readonly name: AnyJsClassMemberName | undefined;
	readonly propertyAnnotation: AnyTsPropertyAnnotation | undefined;
	readonly value: JsInitializerClause | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsPropertyObjectMember extends JsAstNode {
	readonly kind: "JS_PROPERTY_OBJECT_MEMBER";
	readonly name: AnyJsObjectMemberName | undefined;
	readonly colonToken: string | undefined;
	readonly value: AnyJsExpression | undefined;
}

export interface JsReferenceIdentifier extends JsAstNode {
	readonly kind: "JS_REFERENCE_IDENTIFIER";
	readonly valueToken: string | undefined;
}

export interface JsRegexLiteralExpression extends JsAstNode {
	readonly kind: "JS_REGEX_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
}

export interface JsRestParameter extends JsAstNode {
	readonly kind: "JS_REST_PARAMETER";
	readonly decorators: JsDecoratorList;
	readonly dotdotdotToken: string | undefined;
	readonly binding: AnyJsBindingPattern | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
}

export interface JsReturnStatement extends JsAstNode {
	readonly kind: "JS_RETURN_STATEMENT";
	readonly returnToken: string | undefined;
	readonly argument: AnyJsExpression | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsScript extends JsAstNode {
	readonly kind: "JS_SCRIPT";
	readonly bomToken: string | undefined;
	readonly interpreterToken: string | undefined;
	readonly directives: JsDirectiveList;
	readonly statements: JsStatementList;
	readonly eofToken: string | undefined;
}

export interface JsSequenceExpression extends JsAstNode {
	readonly kind: "JS_SEQUENCE_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	readonly commaToken: string | undefined;
	readonly right: AnyJsExpression | undefined;
}

export interface JsSetterClassMember extends JsAstNode {
	readonly kind: "JS_SETTER_CLASS_MEMBER";
	readonly modifiers: JsMethodModifierList;
	readonly setToken: string | undefined;
	readonly name: AnyJsClassMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly parameter: AnyJsFormalParameter | undefined;
	readonly commaToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsSetterObjectMember extends JsAstNode {
	readonly kind: "JS_SETTER_OBJECT_MEMBER";
	readonly setToken: string | undefined;
	readonly name: AnyJsObjectMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly parameter: AnyJsFormalParameter | undefined;
	readonly commaToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly body: JsFunctionBody | undefined;
}

export interface JsShorthandNamedImportSpecifier extends JsAstNode {
	readonly kind: "JS_SHORTHAND_NAMED_IMPORT_SPECIFIER";
	readonly typeToken: string | undefined;
	readonly localName: AnyJsBinding | undefined;
}

export interface JsShorthandPropertyObjectMember extends JsAstNode {
	readonly kind: "JS_SHORTHAND_PROPERTY_OBJECT_MEMBER";
	readonly name: JsReferenceIdentifier | undefined;
}

export interface JsSpread extends JsAstNode {
	readonly kind: "JS_SPREAD";
	readonly dotdotdotToken: string | undefined;
	readonly argument: AnyJsExpression | undefined;
}

export interface JsStaticInitializationBlockClassMember extends JsAstNode {
	readonly kind: "JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER";
	readonly staticToken: string | undefined;
	readonly lCurlyToken: string | undefined;
	readonly statements: JsStatementList;
	readonly rCurlyToken: string | undefined;
}

export interface JsStaticMemberAssignment extends JsAstNode {
	readonly kind: "JS_STATIC_MEMBER_ASSIGNMENT";
	readonly object: AnyJsExpression | undefined;
	readonly dotToken: string | undefined;
	readonly member: AnyJsName | undefined;
}

export interface JsStaticMemberExpression extends JsAstNode {
	readonly kind: "JS_STATIC_MEMBER_EXPRESSION";
	readonly object: AnyJsExpression | undefined;
	readonly operatorToken: string | undefined;
	readonly member: AnyJsName | undefined;
}

export interface JsStaticModifier extends JsAstNode {
	readonly kind: "JS_STATIC_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface JsStringLiteralExpression extends JsAstNode {
	readonly kind: "JS_STRING_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
}

export interface JsSuperExpression extends JsAstNode {
	readonly kind: "JS_SUPER_EXPRESSION";
	readonly superToken: string | undefined;
}

export interface JsSvelteSnippetRoot extends JsAstNode {
	readonly kind: "JS_SVELTE_SNIPPET_ROOT";
	readonly name: AnyJsBinding | undefined;
	readonly parameters: JsParameters | undefined;
	readonly eofToken: string | undefined;
}

export interface JsSwitchStatement extends JsAstNode {
	readonly kind: "JS_SWITCH_STATEMENT";
	readonly switchToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly discriminant: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly lCurlyToken: string | undefined;
	readonly cases: JsSwitchCaseList;
	readonly rCurlyToken: string | undefined;
}

export interface JsTemplateChunkElement extends JsAstNode {
	readonly kind: "JS_TEMPLATE_CHUNK_ELEMENT";
	readonly templateChunkToken: string | undefined;
}

export interface JsTemplateElement extends JsAstNode {
	readonly kind: "JS_TEMPLATE_ELEMENT";
	readonly dollarCurlyToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface JsTemplateExpression extends JsAstNode {
	readonly kind: "JS_TEMPLATE_EXPRESSION";
	readonly tag: AnyJsExpression | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
	readonly lTickToken: string | undefined;
	readonly elements: JsTemplateElementList;
	readonly rTickToken: string | undefined;
}

export interface JsThisExpression extends JsAstNode {
	readonly kind: "JS_THIS_EXPRESSION";
	readonly thisToken: string | undefined;
}

export interface JsThrowStatement extends JsAstNode {
	readonly kind: "JS_THROW_STATEMENT";
	readonly throwToken: string | undefined;
	readonly argument: AnyJsExpression | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsTryFinallyStatement extends JsAstNode {
	readonly kind: "JS_TRY_FINALLY_STATEMENT";
	readonly tryToken: string | undefined;
	readonly body: JsBlockStatement | undefined;
	readonly catchClause: JsCatchClause | undefined;
	readonly finallyClause: JsFinallyClause | undefined;
}

export interface JsTryStatement extends JsAstNode {
	readonly kind: "JS_TRY_STATEMENT";
	readonly tryToken: string | undefined;
	readonly body: JsBlockStatement | undefined;
	readonly catchClause: JsCatchClause | undefined;
}

export interface JsUnaryExpression extends JsAstNode {
	readonly kind: "JS_UNARY_EXPRESSION";
	readonly operatorToken: string | undefined;
	readonly argument: AnyJsExpression | undefined;
}

export interface JsVariableDeclaration extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATION";
	readonly awaitToken: string | undefined;
	readonly kindToken: string | undefined;
	readonly declarators: JsVariableDeclaratorList;
}

export interface JsVariableDeclarationClause extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATION_CLAUSE";
	readonly declaration: JsVariableDeclaration | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsVariableDeclarator extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATOR";
	readonly id: AnyJsBindingPattern | undefined;
	readonly variableAnnotation: AnyTsVariableAnnotation | undefined;
	readonly initializer: JsInitializerClause | undefined;
}

export interface JsVariableStatement extends JsAstNode {
	readonly kind: "JS_VARIABLE_STATEMENT";
	readonly declaration: JsVariableDeclaration | undefined;
	readonly semicolonToken: string | undefined;
}

export interface JsWhileStatement extends JsAstNode {
	readonly kind: "JS_WHILE_STATEMENT";
	readonly whileToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly test: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
}

export interface JsWithStatement extends JsAstNode {
	readonly kind: "JS_WITH_STATEMENT";
	readonly withToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly object: AnyJsExpression | undefined;
	readonly rParenToken: string | undefined;
	readonly body: AnyJsStatement | undefined;
}

export interface JsYieldArgument extends JsAstNode {
	readonly kind: "JS_YIELD_ARGUMENT";
	readonly starToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
}

export interface JsYieldExpression extends JsAstNode {
	readonly kind: "JS_YIELD_EXPRESSION";
	readonly yieldToken: string | undefined;
	readonly argument: JsYieldArgument | undefined;
}

export interface JsxAttribute extends JsAstNode {
	readonly kind: "JSX_ATTRIBUTE";
	readonly name: AnyJsxAttributeName | undefined;
	readonly initializer: JsxAttributeInitializerClause | undefined;
}

export interface JsxAttributeInitializerClause extends JsAstNode {
	readonly kind: "JSX_ATTRIBUTE_INITIALIZER_CLAUSE";
	readonly eqToken: string | undefined;
	readonly value: AnyJsxAttributeValue | undefined;
}

export interface JsxClosingElement extends JsAstNode {
	readonly kind: "JSX_CLOSING_ELEMENT";
	readonly lAngleToken: string | undefined;
	readonly slashToken: string | undefined;
	readonly name: AnyJsxElementName | undefined;
	readonly rAngleToken: string | undefined;
}

export interface JsxClosingFragment extends JsAstNode {
	readonly kind: "JSX_CLOSING_FRAGMENT";
	readonly lAngleToken: string | undefined;
	readonly slashToken: string | undefined;
	readonly rAngleToken: string | undefined;
}

export interface JsxElement extends JsAstNode {
	readonly kind: "JSX_ELEMENT";
	readonly openingElement: JsxOpeningElement | undefined;
	readonly children: JsxChildList;
	readonly closingElement: JsxClosingElement | undefined;
}

export interface JsxExpressionAttributeValue extends JsAstNode {
	readonly kind: "JSX_EXPRESSION_ATTRIBUTE_VALUE";
	readonly lCurlyToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface JsxExpressionChild extends JsAstNode {
	readonly kind: "JSX_EXPRESSION_CHILD";
	readonly lCurlyToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface JsxFragment extends JsAstNode {
	readonly kind: "JSX_FRAGMENT";
	readonly openingFragment: JsxOpeningFragment | undefined;
	readonly children: JsxChildList;
	readonly closingFragment: JsxClosingFragment | undefined;
}

export interface JsxMemberName extends JsAstNode {
	readonly kind: "JSX_MEMBER_NAME";
	readonly object: AnyJsxObjectName | undefined;
	readonly dotToken: string | undefined;
	readonly member: JsName | undefined;
}

export interface JsxName extends JsAstNode {
	readonly kind: "JSX_NAME";
	readonly valueToken: string | undefined;
}

export interface JsxNamespaceName extends JsAstNode {
	readonly kind: "JSX_NAMESPACE_NAME";
	readonly namespace: JsxName | undefined;
	readonly colonToken: string | undefined;
	readonly name: JsxName | undefined;
}

export interface JsxOpeningElement extends JsAstNode {
	readonly kind: "JSX_OPENING_ELEMENT";
	readonly lAngleToken: string | undefined;
	readonly name: AnyJsxElementName | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
	readonly attributes: JsxAttributeList;
	readonly rAngleToken: string | undefined;
}

export interface JsxOpeningFragment extends JsAstNode {
	readonly kind: "JSX_OPENING_FRAGMENT";
	readonly lAngleToken: string | undefined;
	readonly rAngleToken: string | undefined;
}

export interface JsxReferenceIdentifier extends JsAstNode {
	readonly kind: "JSX_REFERENCE_IDENTIFIER";
	readonly valueToken: string | undefined;
}

export interface JsxSelfClosingElement extends JsAstNode {
	readonly kind: "JSX_SELF_CLOSING_ELEMENT";
	readonly lAngleToken: string | undefined;
	readonly name: AnyJsxElementName | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
	readonly attributes: JsxAttributeList;
	readonly slashToken: string | undefined;
	readonly rAngleToken: string | undefined;
}

export interface JsxShorthandAttribute extends JsAstNode {
	readonly kind: "JSX_SHORTHAND_ATTRIBUTE";
	readonly lCurlyToken: string | undefined;
	readonly name: JsReferenceIdentifier | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface JsxSpreadAttribute extends JsAstNode {
	readonly kind: "JSX_SPREAD_ATTRIBUTE";
	readonly lCurlyToken: string | undefined;
	readonly dotdotdotToken: string | undefined;
	readonly argument: AnyJsExpression | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface JsxSpreadChild extends JsAstNode {
	readonly kind: "JSX_SPREAD_CHILD";
	readonly lCurlyToken: string | undefined;
	readonly dotdotdotToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface JsxString extends JsAstNode {
	readonly kind: "JSX_STRING";
	readonly valueToken: string | undefined;
}

export interface JsxTagExpression extends JsAstNode {
	readonly kind: "JSX_TAG_EXPRESSION";
	readonly tag: AnyJsxTag | undefined;
}

export interface JsxText extends JsAstNode {
	readonly kind: "JSX_TEXT";
	readonly valueToken: string | undefined;
}

export interface TsAbstractModifier extends JsAstNode {
	readonly kind: "TS_ABSTRACT_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsAccessibilityModifier extends JsAstNode {
	readonly kind: "TS_ACCESSIBILITY_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsAnyType extends JsAstNode {
	readonly kind: "TS_ANY_TYPE";
	readonly anyToken: string | undefined;
}

export interface TsArrayType extends JsAstNode {
	readonly kind: "TS_ARRAY_TYPE";
	readonly elementType: AnyTsType | undefined;
	readonly lBrackToken: string | undefined;
	readonly rBrackToken: string | undefined;
}

export interface TsAsAssignment extends JsAstNode {
	readonly kind: "TS_AS_ASSIGNMENT";
	readonly assignment: AnyJsAssignment | undefined;
	readonly asToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsAsExpression extends JsAstNode {
	readonly kind: "TS_AS_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	readonly asToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsAssertsCondition extends JsAstNode {
	readonly kind: "TS_ASSERTS_CONDITION";
	readonly isToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsAssertsReturnType extends JsAstNode {
	readonly kind: "TS_ASSERTS_RETURN_TYPE";
	readonly assertsToken: string | undefined;
	readonly parameterName: AnyTsTypePredicateParameterName | undefined;
	readonly predicate: TsAssertsCondition | undefined;
}

export interface TsBigintLiteralType extends JsAstNode {
	readonly kind: "TS_BIGINT_LITERAL_TYPE";
	readonly minusToken: string | undefined;
	readonly literalToken: string | undefined;
}

export interface TsBigintType extends JsAstNode {
	readonly kind: "TS_BIGINT_TYPE";
	readonly bigintToken: string | undefined;
}

export interface TsBooleanLiteralType extends JsAstNode {
	readonly kind: "TS_BOOLEAN_LITERAL_TYPE";
	readonly literal: string | undefined;
}

export interface TsBooleanType extends JsAstNode {
	readonly kind: "TS_BOOLEAN_TYPE";
	readonly booleanToken: string | undefined;
}

export interface TsCallSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_CALL_SIGNATURE_TYPE_MEMBER";
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsConditionalType extends JsAstNode {
	readonly kind: "TS_CONDITIONAL_TYPE";
	readonly checkType: AnyTsType | undefined;
	readonly extendsToken: string | undefined;
	readonly extendsType: AnyTsType | undefined;
	readonly questionMarkToken: string | undefined;
	readonly trueType: AnyTsType | undefined;
	readonly colonToken: string | undefined;
	readonly falseType: AnyTsType | undefined;
}

export interface TsConstModifier extends JsAstNode {
	readonly kind: "TS_CONST_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsConstructSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER";
	readonly newToken: string | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsConstructorSignatureClassMember extends JsAstNode {
	readonly kind: "TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: JsConstructorModifierList;
	readonly name: JsLiteralMemberName | undefined;
	readonly parameters: JsConstructorParameters | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsConstructorType extends JsAstNode {
	readonly kind: "TS_CONSTRUCTOR_TYPE";
	readonly abstractToken: string | undefined;
	readonly newToken: string | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly fatArrowToken: string | undefined;
	readonly returnType: AnyTsType | undefined;
}

export interface TsDeclarationModule extends JsAstNode {
	readonly kind: "TS_DECLARATION_MODULE";
	readonly bomToken: string | undefined;
	readonly interpreterToken: string | undefined;
	readonly directives: JsDirectiveList;
	readonly items: JsModuleItemList;
	readonly eofToken: string | undefined;
}

export interface TsDeclareFunctionDeclaration extends JsAstNode {
	readonly kind: "TS_DECLARE_FUNCTION_DECLARATION";
	readonly asyncToken: string | undefined;
	readonly functionToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsDeclareFunctionExportDefaultDeclaration extends JsAstNode {
	readonly kind: "TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION";
	readonly asyncToken: string | undefined;
	readonly functionToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsDeclareModifier extends JsAstNode {
	readonly kind: "TS_DECLARE_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsDeclareStatement extends JsAstNode {
	readonly kind: "TS_DECLARE_STATEMENT";
	readonly declareToken: string | undefined;
	readonly declaration: AnyJsDeclarationClause | undefined;
}

export interface TsDefaultTypeClause extends JsAstNode {
	readonly kind: "TS_DEFAULT_TYPE_CLAUSE";
	readonly eqToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsDefinitePropertyAnnotation extends JsAstNode {
	readonly kind: "TS_DEFINITE_PROPERTY_ANNOTATION";
	readonly exclToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
}

export interface TsDefiniteVariableAnnotation extends JsAstNode {
	readonly kind: "TS_DEFINITE_VARIABLE_ANNOTATION";
	readonly exclToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
}

export interface TsEmptyExternalModuleDeclarationBody extends JsAstNode {
	readonly kind: "TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY";
	readonly semicolonToken: string | undefined;
}

export interface TsEnumDeclaration extends JsAstNode {
	readonly kind: "TS_ENUM_DECLARATION";
	readonly constToken: string | undefined;
	readonly enumToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly lCurlyToken: string | undefined;
	readonly members: TsEnumMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface TsEnumMember extends JsAstNode {
	readonly kind: "TS_ENUM_MEMBER";
	readonly name: AnyTsEnumMemberName | undefined;
	readonly initializer: JsInitializerClause | undefined;
}

export interface TsExportAsNamespaceClause extends JsAstNode {
	readonly kind: "TS_EXPORT_AS_NAMESPACE_CLAUSE";
	readonly asToken: string | undefined;
	readonly namespaceToken: string | undefined;
	readonly name: JsName | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsExportAssignmentClause extends JsAstNode {
	readonly kind: "TS_EXPORT_ASSIGNMENT_CLAUSE";
	readonly eqToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsExportDeclareClause extends JsAstNode {
	readonly kind: "TS_EXPORT_DECLARE_CLAUSE";
	readonly declareToken: string | undefined;
	readonly declaration: AnyJsDeclarationClause | undefined;
}

export interface TsExtendsClause extends JsAstNode {
	readonly kind: "TS_EXTENDS_CLAUSE";
	readonly extendsToken: string | undefined;
	readonly types: TsTypeList;
}

export interface TsExternalModuleDeclaration extends JsAstNode {
	readonly kind: "TS_EXTERNAL_MODULE_DECLARATION";
	readonly moduleToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly body: AnyTsExternalModuleDeclarationBody | undefined;
}

export interface TsExternalModuleReference extends JsAstNode {
	readonly kind: "TS_EXTERNAL_MODULE_REFERENCE";
	readonly requireToken: string | undefined;
	readonly lParenToken: string | undefined;
	readonly source: AnyJsModuleSource | undefined;
	readonly rParenToken: string | undefined;
}

export interface TsFunctionType extends JsAstNode {
	readonly kind: "TS_FUNCTION_TYPE";
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly fatArrowToken: string | undefined;
	readonly returnType: AnyTsReturnType | undefined;
}

export interface TsGetterSignatureClassMember extends JsAstNode {
	readonly kind: "TS_GETTER_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsMethodSignatureModifierList;
	readonly getToken: string | undefined;
	readonly name: AnyJsClassMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly returnType: TsTypeAnnotation | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsGetterSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_GETTER_SIGNATURE_TYPE_MEMBER";
	readonly getToken: string | undefined;
	readonly name: AnyJsObjectMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsGlobalDeclaration extends JsAstNode {
	readonly kind: "TS_GLOBAL_DECLARATION";
	readonly globalToken: string | undefined;
	readonly body: TsModuleBlock | undefined;
}

export interface TsIdentifierBinding extends JsAstNode {
	readonly kind: "TS_IDENTIFIER_BINDING";
	readonly nameToken: string | undefined;
}

export interface TsImplementsClause extends JsAstNode {
	readonly kind: "TS_IMPLEMENTS_CLAUSE";
	readonly implementsToken: string | undefined;
	readonly types: TsTypeList;
}

export interface TsImportEqualsDeclaration extends JsAstNode {
	readonly kind: "TS_IMPORT_EQUALS_DECLARATION";
	readonly importToken: string | undefined;
	readonly typeToken: string | undefined;
	readonly id: AnyJsBinding | undefined;
	readonly eqToken: string | undefined;
	readonly moduleReference: AnyTsModuleReference | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsImportType extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE";
	readonly typeofToken: string | undefined;
	readonly importToken: string | undefined;
	readonly arguments: TsImportTypeArguments | undefined;
	readonly qualifierClause: TsImportTypeQualifier | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
}

export interface TsImportTypeArguments extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_ARGUMENTS";
	readonly lParenToken: string | undefined;
	readonly argument: AnyTsType | undefined;
	readonly commaToken: string | undefined;
	readonly tsImportTypeAssertionBlock: TsImportTypeAssertionBlock | undefined;
	readonly rParenToken: string | undefined;
}

export interface TsImportTypeAssertion extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_ASSERTION";
	readonly withToken: string | undefined;
	readonly colonToken: string | undefined;
	readonly lCurlyToken: string | undefined;
	readonly assertions: JsImportAssertionEntryList;
	readonly rCurlyToken: string | undefined;
}

export interface TsImportTypeAssertionBlock extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_ASSERTION_BLOCK";
	readonly lCurlyToken: string | undefined;
	readonly typeAssertion: TsImportTypeAssertion | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface TsImportTypeQualifier extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_QUALIFIER";
	readonly dotToken: string | undefined;
	readonly right: AnyTsName | undefined;
}

export interface TsInModifier extends JsAstNode {
	readonly kind: "TS_IN_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsIndexSignatureClassMember extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsIndexSignatureModifierList;
	readonly lBrackToken: string | undefined;
	readonly parameter: TsIndexSignatureParameter | undefined;
	readonly rBrackToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsIndexSignatureParameter extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_PARAMETER";
	readonly binding: JsIdentifierBinding | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
}

export interface TsIndexSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_TYPE_MEMBER";
	readonly readonlyToken: string | undefined;
	readonly lBrackToken: string | undefined;
	readonly parameter: TsIndexSignatureParameter | undefined;
	readonly rBrackToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsIndexedAccessType extends JsAstNode {
	readonly kind: "TS_INDEXED_ACCESS_TYPE";
	readonly objectType: AnyTsType | undefined;
	readonly lBrackToken: string | undefined;
	readonly indexType: AnyTsType | undefined;
	readonly rBrackToken: string | undefined;
}

export interface TsInferType extends JsAstNode {
	readonly kind: "TS_INFER_TYPE";
	readonly inferToken: string | undefined;
	readonly name: TsTypeParameterName | undefined;
	readonly constraint: TsTypeConstraintClause | undefined;
}

export interface TsInitializedPropertySignatureClassMember extends JsAstNode {
	readonly kind: "TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsPropertySignatureModifierList;
	readonly name: AnyJsClassMemberName | undefined;
	readonly questionMarkToken: string | undefined;
	readonly value: JsInitializerClause | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsInstantiationExpression extends JsAstNode {
	readonly kind: "TS_INSTANTIATION_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	readonly arguments: TsTypeArguments | undefined;
}

export interface TsInterfaceDeclaration extends JsAstNode {
	readonly kind: "TS_INTERFACE_DECLARATION";
	readonly interfaceToken: string | undefined;
	readonly id: AnyTsIdentifierBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly extendsClause: TsExtendsClause | undefined;
	readonly lCurlyToken: string | undefined;
	readonly members: TsTypeMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface TsIntersectionType extends JsAstNode {
	readonly kind: "TS_INTERSECTION_TYPE";
	readonly leadingSeparatorToken: string | undefined;
	readonly types: TsIntersectionTypeElementList;
}

export interface TsLiteralEnumMemberName extends JsAstNode {
	readonly kind: "TS_LITERAL_ENUM_MEMBER_NAME";
	readonly value: string | undefined;
}

export interface TsMappedType extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE";
	readonly lCurlyToken: string | undefined;
	readonly readonlyModifier: TsMappedTypeReadonlyModifierClause | undefined;
	readonly lBrackToken: string | undefined;
	readonly propertyName: TsTypeParameterName | undefined;
	readonly inToken: string | undefined;
	readonly keysType: AnyTsType | undefined;
	readonly asClause: TsMappedTypeAsClause | undefined;
	readonly rBrackToken: string | undefined;
	readonly optionalModifier: TsMappedTypeOptionalModifierClause | undefined;
	readonly mappedType: TsTypeAnnotation | undefined;
	readonly semicolonToken: string | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface TsMappedTypeAsClause extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE_AS_CLAUSE";
	readonly asToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsMappedTypeOptionalModifierClause extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE";
	readonly operatorToken: string | undefined;
	readonly questionMarkToken: string | undefined;
}

export interface TsMappedTypeReadonlyModifierClause extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE";
	readonly operatorToken: string | undefined;
	readonly readonlyToken: string | undefined;
}

export interface TsMethodSignatureClassMember extends JsAstNode {
	readonly kind: "TS_METHOD_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsMethodSignatureModifierList;
	readonly asyncToken: string | undefined;
	readonly name: AnyJsClassMemberName | undefined;
	readonly questionMarkToken: string | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsMethodSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_METHOD_SIGNATURE_TYPE_MEMBER";
	readonly name: AnyJsObjectMemberName | undefined;
	readonly optionalToken: string | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly parameters: JsParameters | undefined;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsModuleBlock extends JsAstNode {
	readonly kind: "TS_MODULE_BLOCK";
	readonly lCurlyToken: string | undefined;
	readonly items: JsModuleItemList;
	readonly rCurlyToken: string | undefined;
}

export interface TsModuleDeclaration extends JsAstNode {
	readonly kind: "TS_MODULE_DECLARATION";
	readonly moduleOrNamespace: string | undefined;
	readonly name: AnyTsModuleName | undefined;
	readonly body: TsModuleBlock | undefined;
}

export interface TsNamedTupleTypeElement extends JsAstNode {
	readonly kind: "TS_NAMED_TUPLE_TYPE_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	readonly name: JsName | undefined;
	readonly questionMarkToken: string | undefined;
	readonly colonToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsNeverType extends JsAstNode {
	readonly kind: "TS_NEVER_TYPE";
	readonly neverToken: string | undefined;
}

export interface TsNonNullAssertionAssignment extends JsAstNode {
	readonly kind: "TS_NON_NULL_ASSERTION_ASSIGNMENT";
	readonly assignment: AnyJsAssignment | undefined;
	readonly exclToken: string | undefined;
}

export interface TsNonNullAssertionExpression extends JsAstNode {
	readonly kind: "TS_NON_NULL_ASSERTION_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	readonly exclToken: string | undefined;
}

export interface TsNonPrimitiveType extends JsAstNode {
	readonly kind: "TS_NON_PRIMITIVE_TYPE";
	readonly objectToken: string | undefined;
}

export interface TsNullLiteralType extends JsAstNode {
	readonly kind: "TS_NULL_LITERAL_TYPE";
	readonly literalToken: string | undefined;
}

export interface TsNumberLiteralType extends JsAstNode {
	readonly kind: "TS_NUMBER_LITERAL_TYPE";
	readonly minusToken: string | undefined;
	readonly literalToken: string | undefined;
}

export interface TsNumberType extends JsAstNode {
	readonly kind: "TS_NUMBER_TYPE";
	readonly numberToken: string | undefined;
}

export interface TsObjectType extends JsAstNode {
	readonly kind: "TS_OBJECT_TYPE";
	readonly lCurlyToken: string | undefined;
	readonly members: TsTypeMemberList;
	readonly rCurlyToken: string | undefined;
}

export interface TsOptionalPropertyAnnotation extends JsAstNode {
	readonly kind: "TS_OPTIONAL_PROPERTY_ANNOTATION";
	readonly questionMarkToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
}

export interface TsOptionalTupleTypeElement extends JsAstNode {
	readonly kind: "TS_OPTIONAL_TUPLE_TYPE_ELEMENT";
	readonly ty: AnyTsType | undefined;
	readonly questionMarkToken: string | undefined;
}

export interface TsOutModifier extends JsAstNode {
	readonly kind: "TS_OUT_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsOverrideModifier extends JsAstNode {
	readonly kind: "TS_OVERRIDE_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsParenthesizedType extends JsAstNode {
	readonly kind: "TS_PARENTHESIZED_TYPE";
	readonly lParenToken: string | undefined;
	readonly ty: AnyTsType | undefined;
	readonly rParenToken: string | undefined;
}

export interface TsPredicateReturnType extends JsAstNode {
	readonly kind: "TS_PREDICATE_RETURN_TYPE";
	readonly parameterName: AnyTsTypePredicateParameterName | undefined;
	readonly isToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsPropertyParameter extends JsAstNode {
	readonly kind: "TS_PROPERTY_PARAMETER";
	readonly decorators: JsDecoratorList;
	readonly modifiers: TsPropertyParameterModifierList;
	readonly formalParameter: AnyJsFormalParameter | undefined;
}

export interface TsPropertySignatureClassMember extends JsAstNode {
	readonly kind: "TS_PROPERTY_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsPropertySignatureModifierList;
	readonly name: AnyJsClassMemberName | undefined;
	readonly propertyAnnotation: AnyTsPropertySignatureAnnotation | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsPropertySignatureTypeMember extends JsAstNode {
	readonly kind: "TS_PROPERTY_SIGNATURE_TYPE_MEMBER";
	readonly readonlyToken: string | undefined;
	readonly name: AnyJsObjectMemberName | undefined;
	readonly optionalToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsQualifiedModuleName extends JsAstNode {
	readonly kind: "TS_QUALIFIED_MODULE_NAME";
	readonly left: AnyTsModuleName | undefined;
	readonly dotToken: string | undefined;
	readonly right: JsName | undefined;
}

export interface TsQualifiedName extends JsAstNode {
	readonly kind: "TS_QUALIFIED_NAME";
	readonly left: AnyTsName | undefined;
	readonly dotToken: string | undefined;
	readonly right: JsName | undefined;
}

export interface TsReadonlyModifier extends JsAstNode {
	readonly kind: "TS_READONLY_MODIFIER";
	readonly modifierToken: string | undefined;
}

export interface TsReferenceType extends JsAstNode {
	readonly kind: "TS_REFERENCE_TYPE";
	readonly name: AnyTsName | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
}

export interface TsRestTupleTypeElement extends JsAstNode {
	readonly kind: "TS_REST_TUPLE_TYPE_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsReturnTypeAnnotation extends JsAstNode {
	readonly kind: "TS_RETURN_TYPE_ANNOTATION";
	readonly colonToken: string | undefined;
	readonly ty: AnyTsReturnType | undefined;
}

export interface TsSatisfiesAssignment extends JsAstNode {
	readonly kind: "TS_SATISFIES_ASSIGNMENT";
	readonly assignment: AnyJsAssignment | undefined;
	readonly satisfiesToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsSatisfiesExpression extends JsAstNode {
	readonly kind: "TS_SATISFIES_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	readonly satisfiesToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsSetterSignatureClassMember extends JsAstNode {
	readonly kind: "TS_SETTER_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsMethodSignatureModifierList;
	readonly setToken: string | undefined;
	readonly name: AnyJsClassMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly parameter: AnyJsFormalParameter | undefined;
	readonly commaToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsSetterSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_SETTER_SIGNATURE_TYPE_MEMBER";
	readonly setToken: string | undefined;
	readonly name: AnyJsObjectMemberName | undefined;
	readonly lParenToken: string | undefined;
	readonly parameter: AnyJsFormalParameter | undefined;
	readonly commaToken: string | undefined;
	readonly rParenToken: string | undefined;
	readonly separatorToken: string | undefined;
}

export interface TsStringLiteralType extends JsAstNode {
	readonly kind: "TS_STRING_LITERAL_TYPE";
	readonly literalToken: string | undefined;
}

export interface TsStringType extends JsAstNode {
	readonly kind: "TS_STRING_TYPE";
	readonly stringToken: string | undefined;
}

export interface TsSymbolType extends JsAstNode {
	readonly kind: "TS_SYMBOL_TYPE";
	readonly symbolToken: string | undefined;
}

export interface TsTemplateChunkElement extends JsAstNode {
	readonly kind: "TS_TEMPLATE_CHUNK_ELEMENT";
	readonly templateChunkToken: string | undefined;
}

export interface TsTemplateElement extends JsAstNode {
	readonly kind: "TS_TEMPLATE_ELEMENT";
	readonly dollarCurlyToken: string | undefined;
	readonly ty: AnyTsType | undefined;
	readonly rCurlyToken: string | undefined;
}

export interface TsTemplateLiteralType extends JsAstNode {
	readonly kind: "TS_TEMPLATE_LITERAL_TYPE";
	readonly lTickToken: string | undefined;
	readonly elements: TsTemplateElementList;
	readonly rTickToken: string | undefined;
}

export interface TsThisParameter extends JsAstNode {
	readonly kind: "TS_THIS_PARAMETER";
	readonly thisToken: string | undefined;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
}

export interface TsThisType extends JsAstNode {
	readonly kind: "TS_THIS_TYPE";
	readonly thisToken: string | undefined;
}

export interface TsTupleType extends JsAstNode {
	readonly kind: "TS_TUPLE_TYPE";
	readonly lBrackToken: string | undefined;
	readonly elements: TsTupleTypeElementList;
	readonly rBrackToken: string | undefined;
}

export interface TsTypeAliasDeclaration extends JsAstNode {
	readonly kind: "TS_TYPE_ALIAS_DECLARATION";
	readonly typeToken: string | undefined;
	readonly bindingIdentifier: AnyTsIdentifierBinding | undefined;
	readonly typeParameters: TsTypeParameters | undefined;
	readonly eqToken: string | undefined;
	readonly ty: AnyTsType | undefined;
	readonly semicolonToken: string | undefined;
}

export interface TsTypeAnnotation extends JsAstNode {
	readonly kind: "TS_TYPE_ANNOTATION";
	readonly colonToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsTypeArguments extends JsAstNode {
	readonly kind: "TS_TYPE_ARGUMENTS";
	readonly lAngleToken: string | undefined;
	readonly tsTypeArgumentList: TsTypeArgumentList;
	readonly rAngleToken: string | undefined;
}

export interface TsTypeAssertionAssignment extends JsAstNode {
	readonly kind: "TS_TYPE_ASSERTION_ASSIGNMENT";
	readonly lAngleToken: string | undefined;
	readonly ty: AnyTsType | undefined;
	readonly rAngleToken: string | undefined;
	readonly assignment: AnyJsAssignment | undefined;
}

export interface TsTypeAssertionExpression extends JsAstNode {
	readonly kind: "TS_TYPE_ASSERTION_EXPRESSION";
	readonly lAngleToken: string | undefined;
	readonly ty: AnyTsType | undefined;
	readonly rAngleToken: string | undefined;
	readonly expression: AnyJsExpression | undefined;
}

export interface TsTypeConstraintClause extends JsAstNode {
	readonly kind: "TS_TYPE_CONSTRAINT_CLAUSE";
	readonly extendsToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsTypeOperatorType extends JsAstNode {
	readonly kind: "TS_TYPE_OPERATOR_TYPE";
	readonly operatorToken: string | undefined;
	readonly ty: AnyTsType | undefined;
}

export interface TsTypeParameter extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETER";
	readonly modifiers: TsTypeParameterModifierList;
	readonly name: TsTypeParameterName | undefined;
	readonly constraint: TsTypeConstraintClause | undefined;
	readonly default: TsDefaultTypeClause | undefined;
}

export interface TsTypeParameterName extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETER_NAME";
	readonly identToken: string | undefined;
}

export interface TsTypeParameters extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETERS";
	readonly lAngleToken: string | undefined;
	readonly items: TsTypeParameterList;
	readonly rAngleToken: string | undefined;
}

export interface TsTypeofType extends JsAstNode {
	readonly kind: "TS_TYPEOF_TYPE";
	readonly typeofToken: string | undefined;
	readonly expressionName: AnyTsName | undefined;
	readonly typeArguments: TsTypeArguments | undefined;
}

export interface TsUndefinedType extends JsAstNode {
	readonly kind: "TS_UNDEFINED_TYPE";
	readonly undefinedToken: string | undefined;
}

export interface TsUnionType extends JsAstNode {
	readonly kind: "TS_UNION_TYPE";
	readonly leadingSeparatorToken: string | undefined;
	readonly types: TsUnionTypeVariantList;
}

export interface TsUnknownType extends JsAstNode {
	readonly kind: "TS_UNKNOWN_TYPE";
	readonly unknownToken: string | undefined;
}

export interface TsVoidType extends JsAstNode {
	readonly kind: "TS_VOID_TYPE";
	readonly voidToken: string | undefined;
}

export interface JsBogus extends JsAstNode {
	readonly kind: "JS_BOGUS";
}

export interface JsBogusAssignment extends JsAstNode {
	readonly kind: "JS_BOGUS_ASSIGNMENT";
}

export interface JsBogusBinding extends JsAstNode {
	readonly kind: "JS_BOGUS_BINDING";
}

export interface JsBogusExpression extends JsAstNode {
	readonly kind: "JS_BOGUS_EXPRESSION";
}

export interface JsBogusImportAssertionEntry extends JsAstNode {
	readonly kind: "JS_BOGUS_IMPORT_ASSERTION_ENTRY";
}

export interface JsBogusMember extends JsAstNode {
	readonly kind: "JS_BOGUS_MEMBER";
}

export interface JsBogusNamedImportSpecifier extends JsAstNode {
	readonly kind: "JS_BOGUS_NAMED_IMPORT_SPECIFIER";
}

export interface JsBogusParameter extends JsAstNode {
	readonly kind: "JS_BOGUS_PARAMETER";
}

export interface JsBogusStatement extends JsAstNode {
	readonly kind: "JS_BOGUS_STATEMENT";
}

export interface TsBogusType extends JsAstNode {
	readonly kind: "TS_BOGUS_TYPE";
}

export type AnyJsArrayAssignmentPatternElement = JsArrayAssignmentPatternElement | JsArrayAssignmentPatternRestElement | JsArrayHole;
export type AnyJsArrayBindingPatternElement = JsArrayBindingPatternElement | JsArrayBindingPatternRestElement | JsArrayHole;
export type AnyJsArrayElement = AnyJsExpression | JsArrayHole | JsSpread;
export type AnyJsArrowFunctionParameters = AnyJsBinding | JsParameters;
export type AnyJsAssignment = JsBogusAssignment | JsComputedMemberAssignment | JsIdentifierAssignment | JsParenthesizedAssignment | JsStaticMemberAssignment | TsAsAssignment | TsNonNullAssertionAssignment | TsSatisfiesAssignment | TsTypeAssertionAssignment;
export type AnyJsAssignmentPattern = AnyJsAssignment | JsArrayAssignmentPattern | JsObjectAssignmentPattern;
export type AnyJsBinding = JsBogusBinding | JsIdentifierBinding | JsMetavariable;
export type AnyJsBindingPattern = AnyJsBinding | JsArrayBindingPattern | JsObjectBindingPattern;
export type AnyJsCallArgument = AnyJsExpression | JsSpread;
export type AnyJsClass = JsClassDeclaration | JsClassExportDefaultDeclaration | JsClassExpression;
export type AnyJsClassMember = JsBogusMember | JsConstructorClassMember | JsEmptyClassMember | JsGetterClassMember | JsMetavariable | JsMethodClassMember | JsPropertyClassMember | JsSetterClassMember | JsStaticInitializationBlockClassMember | TsConstructorSignatureClassMember | TsGetterSignatureClassMember | TsIndexSignatureClassMember | TsInitializedPropertySignatureClassMember | TsMethodSignatureClassMember | TsPropertySignatureClassMember | TsSetterSignatureClassMember;
export type AnyJsClassMemberName = JsComputedMemberName | JsLiteralMemberName | JsMetavariable | JsPrivateClassMemberName;
export type AnyJsCombinedSpecifier = JsNamedImportSpecifiers | JsNamespaceImportSpecifier;
export type AnyJsConstructorParameter = AnyJsFormalParameter | JsRestParameter | TsPropertyParameter;
export type AnyJsDeclaration = JsClassDeclaration | JsFunctionDeclaration | JsVariableDeclaration | TsDeclareFunctionDeclaration | TsEnumDeclaration | TsExternalModuleDeclaration | TsGlobalDeclaration | TsImportEqualsDeclaration | TsInterfaceDeclaration | TsModuleDeclaration | TsTypeAliasDeclaration;
export type AnyJsDeclarationClause = JsClassDeclaration | JsFunctionDeclaration | JsVariableDeclarationClause | TsDeclareFunctionDeclaration | TsEnumDeclaration | TsExternalModuleDeclaration | TsGlobalDeclaration | TsImportEqualsDeclaration | TsInterfaceDeclaration | TsModuleDeclaration | TsTypeAliasDeclaration;
export type AnyJsDecorator = JsBogusExpression | JsCallExpression | JsIdentifierExpression | JsParenthesizedExpression | JsStaticMemberExpression;
export type AnyJsExportClause = AnyJsDeclarationClause | JsExportDefaultDeclarationClause | JsExportDefaultExpressionClause | JsExportFromClause | JsExportNamedClause | JsExportNamedFromClause | TsExportAsNamespaceClause | TsExportAssignmentClause | TsExportDeclareClause;
export type AnyJsExportDefaultDeclaration = JsClassExportDefaultDeclaration | JsFunctionExportDefaultDeclaration | TsDeclareFunctionExportDefaultDeclaration | TsInterfaceDeclaration;
export type AnyJsExportNamedSpecifier = JsExportNamedShorthandSpecifier | JsExportNamedSpecifier;
export type AnyJsExpression = AnyJsLiteralExpression | JsArrayExpression | JsArrowFunctionExpression | JsAssignmentExpression | JsAwaitExpression | JsBinaryExpression | JsBogusExpression | JsCallExpression | JsClassExpression | JsComputedMemberExpression | JsConditionalExpression | JsFunctionExpression | JsIdentifierExpression | JsImportCallExpression | JsImportMetaExpression | JsInExpression | JsInstanceofExpression | JsLogicalExpression | JsMetavariable | JsNewExpression | JsNewTargetExpression | JsObjectExpression | JsParenthesizedExpression | JsPostUpdateExpression | JsPreUpdateExpression | JsSequenceExpression | JsStaticMemberExpression | JsSuperExpression | JsTemplateExpression | JsThisExpression | JsUnaryExpression | JsYieldExpression | JsxTagExpression | TsAsExpression | TsInstantiationExpression | TsNonNullAssertionExpression | TsSatisfiesExpression | TsTypeAssertionExpression;
export type AnyJsForInOrOfInitializer = AnyJsAssignmentPattern | JsForVariableDeclaration;
export type AnyJsForInitializer = AnyJsExpression | JsVariableDeclaration;
export type AnyJsFormalParameter = JsBogusParameter | JsFormalParameter | JsMetavariable;
export type AnyJsFunction = JsArrowFunctionExpression | JsFunctionDeclaration | JsFunctionExportDefaultDeclaration | JsFunctionExpression;
export type AnyJsFunctionBody = AnyJsExpression | JsFunctionBody;
export type AnyJsImportAssertionEntry = JsBogusImportAssertionEntry | JsImportAssertionEntry;
export type AnyJsImportClause = JsImportBareClause | JsImportCombinedClause | JsImportDefaultClause | JsImportNamedClause | JsImportNamespaceClause;
export type AnyJsInProperty = AnyJsExpression | JsPrivateName;
export type AnyJsLiteralExportName = JsLiteralExportName | JsMetavariable;
export type AnyJsLiteralExpression = JsBigintLiteralExpression | JsBooleanLiteralExpression | JsNullLiteralExpression | JsNumberLiteralExpression | JsRegexLiteralExpression | JsStringLiteralExpression;
export type AnyJsMethodModifier = JsDecorator | JsStaticModifier | TsAccessibilityModifier | TsOverrideModifier;
export type AnyJsModuleItem = AnyJsStatement | JsExport | JsImport;
export type AnyJsModuleSource = JsMetavariable | JsModuleSource;
export type AnyJsName = JsMetavariable | JsName | JsPrivateName;
export type AnyJsNamedImportSpecifier = JsBogusNamedImportSpecifier | JsNamedImportSpecifier | JsShorthandNamedImportSpecifier;
export type AnyJsObjectAssignmentPatternMember = JsBogusAssignment | JsObjectAssignmentPatternProperty | JsObjectAssignmentPatternRest | JsObjectAssignmentPatternShorthandProperty;
export type AnyJsObjectBindingPatternMember = JsBogusBinding | JsMetavariable | JsObjectBindingPatternProperty | JsObjectBindingPatternRest | JsObjectBindingPatternShorthandProperty;
export type AnyJsObjectMember = JsBogusMember | JsGetterObjectMember | JsMetavariable | JsMethodObjectMember | JsPropertyObjectMember | JsSetterObjectMember | JsShorthandPropertyObjectMember | JsSpread;
export type AnyJsObjectMemberName = JsComputedMemberName | JsLiteralMemberName | JsMetavariable;
export type AnyJsParameter = AnyJsFormalParameter | JsRestParameter | TsThisParameter;
export type AnyJsPropertyModifier = JsAccessorModifier | JsDecorator | JsStaticModifier | TsAccessibilityModifier | TsOverrideModifier | TsReadonlyModifier;
export type AnyJsRoot = JsExpressionSnippet | JsExpressionTemplateRoot | JsModule | JsScript | JsSvelteSnippetRoot | TsDeclarationModule;
export type AnyJsStatement = JsBlockStatement | JsBogusStatement | JsBreakStatement | JsClassDeclaration | JsContinueStatement | JsDebuggerStatement | JsDoWhileStatement | JsEmptyStatement | JsExpressionStatement | JsForInStatement | JsForOfStatement | JsForStatement | JsFunctionDeclaration | JsIfStatement | JsLabeledStatement | JsMetavariable | JsReturnStatement | JsSwitchStatement | JsThrowStatement | JsTryFinallyStatement | JsTryStatement | JsVariableStatement | JsWhileStatement | JsWithStatement | TsDeclareFunctionDeclaration | TsDeclareStatement | TsEnumDeclaration | TsExternalModuleDeclaration | TsGlobalDeclaration | TsImportEqualsDeclaration | TsInterfaceDeclaration | TsModuleDeclaration | TsTypeAliasDeclaration;
export type AnyJsSwitchClause = JsCaseClause | JsDefaultClause;
export type AnyJsTemplateElement = JsTemplateChunkElement | JsTemplateElement;
export type AnyJsxAttribute = JsMetavariable | JsxAttribute | JsxShorthandAttribute | JsxSpreadAttribute;
export type AnyJsxAttributeName = JsxName | JsxNamespaceName;
export type AnyJsxAttributeValue = AnyJsxTag | JsxExpressionAttributeValue | JsxString;
export type AnyJsxChild = JsMetavariable | JsxElement | JsxExpressionChild | JsxFragment | JsxSelfClosingElement | JsxSpreadChild | JsxText;
export type AnyJsxElementName = JsMetavariable | JsxMemberName | JsxName | JsxNamespaceName | JsxReferenceIdentifier;
export type AnyJsxName = JsxName | JsxNamespaceName;
export type AnyJsxObjectName = JsxMemberName | JsxNamespaceName | JsxReferenceIdentifier;
export type AnyJsxTag = JsxElement | JsxFragment | JsxSelfClosingElement;
export type AnyTsEnumMemberName = JsComputedMemberName | TsLiteralEnumMemberName;
export type AnyTsExternalModuleDeclarationBody = TsEmptyExternalModuleDeclarationBody | TsModuleBlock;
export type AnyTsIdentifierBinding = JsMetavariable | TsIdentifierBinding;
export type AnyTsIndexSignatureModifier = JsStaticModifier | TsReadonlyModifier;
export type AnyTsMethodSignatureModifier = JsDecorator | JsStaticModifier | TsAbstractModifier | TsAccessibilityModifier | TsOverrideModifier;
export type AnyTsModuleName = AnyTsIdentifierBinding | TsQualifiedModuleName;
export type AnyTsModuleReference = AnyTsName | TsExternalModuleReference;
export type AnyTsName = JsReferenceIdentifier | TsQualifiedName;
export type AnyTsPropertyAnnotation = TsDefinitePropertyAnnotation | TsOptionalPropertyAnnotation | TsTypeAnnotation;
export type AnyTsPropertyParameterModifier = TsAccessibilityModifier | TsOverrideModifier | TsReadonlyModifier;
export type AnyTsPropertySignatureAnnotation = TsOptionalPropertyAnnotation | TsTypeAnnotation;
export type AnyTsPropertySignatureModifier = JsAccessorModifier | JsDecorator | JsStaticModifier | TsAbstractModifier | TsAccessibilityModifier | TsDeclareModifier | TsOverrideModifier | TsReadonlyModifier;
export type AnyTsReturnType = AnyTsType | TsAssertsReturnType | TsPredicateReturnType;
export type AnyTsTemplateElement = TsTemplateChunkElement | TsTemplateElement;
export type AnyTsTupleTypeElement = AnyTsType | TsNamedTupleTypeElement | TsOptionalTupleTypeElement | TsRestTupleTypeElement;
export type AnyTsType = JsMetavariable | TsAnyType | TsArrayType | TsBigintLiteralType | TsBigintType | TsBogusType | TsBooleanLiteralType | TsBooleanType | TsConditionalType | TsConstructorType | TsFunctionType | TsImportType | TsIndexedAccessType | TsInferType | TsIntersectionType | TsMappedType | TsNeverType | TsNonPrimitiveType | TsNullLiteralType | TsNumberLiteralType | TsNumberType | TsObjectType | TsParenthesizedType | TsReferenceType | TsStringLiteralType | TsStringType | TsSymbolType | TsTemplateLiteralType | TsThisType | TsTupleType | TsTypeOperatorType | TsTypeofType | TsUndefinedType | TsUnionType | TsUnknownType | TsVoidType;
export type AnyTsTypeMember = JsBogusMember | TsCallSignatureTypeMember | TsConstructSignatureTypeMember | TsGetterSignatureTypeMember | TsIndexSignatureTypeMember | TsMethodSignatureTypeMember | TsPropertySignatureTypeMember | TsSetterSignatureTypeMember;
export type AnyTsTypeParameterModifier = TsConstModifier | TsInModifier | TsOutModifier;
export type AnyTsTypePredicateParameterName = JsReferenceIdentifier | TsThisType;
export type AnyTsVariableAnnotation = TsDefiniteVariableAnnotation | TsTypeAnnotation;

export type JsArrayAssignmentPatternElementList = readonly AnyJsArrayAssignmentPatternElement[];
export type JsArrayBindingPatternElementList = readonly AnyJsArrayBindingPatternElement[];
export type JsArrayElementList = readonly AnyJsArrayElement[];
export type JsCallArgumentList = readonly AnyJsCallArgument[];
export type JsClassMemberList = readonly AnyJsClassMember[];
export type JsConstructorModifierList = readonly TsAccessibilityModifier[];
export type JsConstructorParameterList = readonly AnyJsConstructorParameter[];
export type JsDecoratorList = readonly JsDecorator[];
export type JsDirectiveList = readonly JsDirective[];
export type JsExportNamedFromSpecifierList = readonly JsExportNamedFromSpecifier[];
export type JsExportNamedSpecifierList = readonly AnyJsExportNamedSpecifier[];
export type JsImportAssertionEntryList = readonly AnyJsImportAssertionEntry[];
export type JsMethodModifierList = readonly AnyJsMethodModifier[];
export type JsModuleItemList = readonly AnyJsModuleItem[];
export type JsNamedImportSpecifierList = readonly AnyJsNamedImportSpecifier[];
export type JsObjectAssignmentPatternPropertyList = readonly AnyJsObjectAssignmentPatternMember[];
export type JsObjectBindingPatternPropertyList = readonly AnyJsObjectBindingPatternMember[];
export type JsObjectMemberList = readonly AnyJsObjectMember[];
export type JsParameterList = readonly AnyJsParameter[];
export type JsPropertyModifierList = readonly AnyJsPropertyModifier[];
export type JsStatementList = readonly AnyJsStatement[];
export type JsSwitchCaseList = readonly AnyJsSwitchClause[];
export type JsTemplateElementList = readonly AnyJsTemplateElement[];
export type JsVariableDeclaratorList = readonly JsVariableDeclarator[];
export type JsxAttributeList = readonly AnyJsxAttribute[];
export type JsxChildList = readonly AnyJsxChild[];
export type TsEnumMemberList = readonly TsEnumMember[];
export type TsIndexSignatureModifierList = readonly AnyTsIndexSignatureModifier[];
export type TsIntersectionTypeElementList = readonly AnyTsType[];
export type TsMethodSignatureModifierList = readonly AnyTsMethodSignatureModifier[];
export type TsPropertyParameterModifierList = readonly AnyTsPropertyParameterModifier[];
export type TsPropertySignatureModifierList = readonly AnyTsPropertySignatureModifier[];
export type TsTemplateElementList = readonly AnyTsTemplateElement[];
export type TsTupleTypeElementList = readonly AnyTsTupleTypeElement[];
export type TsTypeArgumentList = readonly AnyTsType[];
export type TsTypeList = readonly TsReferenceType[];
export type TsTypeMemberList = readonly AnyTsTypeMember[];
export type TsTypeParameterList = readonly TsTypeParameter[];
export type TsTypeParameterModifierList = readonly AnyTsTypeParameterModifier[];
export type TsUnionTypeVariantList = readonly AnyTsType[];
