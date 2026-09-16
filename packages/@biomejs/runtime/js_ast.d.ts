// Generated file, do not edit by hand, see `xtask/codegen`.

export interface JsAstNode {
	readonly kind: string;
	readonly text: string;
	/** The immediate parent node, including list containers. Undefined at the root.
	 * Repeated access does not guarantee the same JavaScript object identity. */
	readonly parent: AnyJsAstNode | undefined;
	/** Returns a fresh array of enclosing nodes, nearest first, excluding this node
	 * and including list containers and the root. Roots return an empty array.
	 * Returned nodes do not have stable JavaScript object identity. */
	ancestors(): readonly AnyJsAstNode[];
	/** Returns a fresh array of immediate child nodes in source order, including list
	 * containers and omitting tokens. Call children() on a list node to iterate its elements.
	 * Nodes without child nodes return an empty array. Named list fields remain arrays.
	 * Returned nodes do not have stable JavaScript object identity. */
	children(): readonly AnyJsAstNode[];
	/**
	 * Returns the native token handle for a named token field.
	 * Use the public field name, such as `kindToken` or `operatorToken`. Unlike the
	 * string-valued field getter, this handle can be passed to token mutation methods.
	 * Returns `undefined` when the field is recognized but its token is absent.
	 * Repeated access does not guarantee the same JavaScript object identity.
	 *
	 * @throws {TypeError} If the field name is unknown, names a non-token field,
	 * or the receiver or arguments are invalid.
	 */
	token(field: string): JsAstToken | undefined;
	/**
	 * Returns a fresh array of immediate child nodes and tokens in source order.
	 * Includes list containers without flattening them or descending into child nodes.
	 * Call this method on a list node to obtain its elements and separator tokens.
	 * Whitespace and comments remain token trivia, not separate array entries. Missing
	 * slots are omitted; nodes without child nodes or tokens return an empty array.
	 * The returned handles can be passed to element mutation methods. Changing the
	 * array does not change the syntax tree, and handles have no stable JavaScript identity.
	 */
	childrenWithTokens(): readonly JsAstElement[];
}
export interface JsAstToken {
	readonly kind:
		| JsTokenKind
		| "ERROR_TOKEN"
		| "NEWLINE"
		| "WHITESPACE"
		| "COMMENT"
		| "MULTILINE_COMMENT"
		| "EOF"
		| "UNICODE_BOM";
	readonly text: string;
	readonly parent: AnyJsAstNode | undefined;
}
export type JsAstElement = AnyJsAstNode | JsAstToken;
export type JsTokenKind =
	| "SEMICOLON"
	| "COMMA"
	| "L_PAREN"
	| "R_PAREN"
	| "L_CURLY"
	| "R_CURLY"
	| "L_BRACK"
	| "R_BRACK"
	| "L_ANGLE"
	| "R_ANGLE"
	| "TILDE"
	| "QUESTION"
	| "QUESTION2"
	| "QUESTIONDOT"
	| "AMP"
	| "PIPE"
	| "PLUS"
	| "PLUS2"
	| "STAR"
	| "STAR2"
	| "SLASH"
	| "CARET"
	| "PERCENT"
	| "DOT"
	| "DOT3"
	| "COLON"
	| "EQ"
	| "EQ2"
	| "EQ3"
	| "FAT_ARROW"
	| "BANG"
	| "NEQ"
	| "NEQ2"
	| "MINUS"
	| "MINUS2"
	| "LTEQ"
	| "GTEQ"
	| "PLUSEQ"
	| "MINUSEQ"
	| "PIPEEQ"
	| "AMPEQ"
	| "CARETEQ"
	| "SLASHEQ"
	| "STAREQ"
	| "PERCENTEQ"
	| "AMP2"
	| "PIPE2"
	| "SHL"
	| "SHR"
	| "USHR"
	| "SHLEQ"
	| "SHREQ"
	| "USHREQ"
	| "AMP2EQ"
	| "PIPE2EQ"
	| "STAR2EQ"
	| "QUESTION2EQ"
	| "AT"
	| "BACKTICK"
	| "BREAK_KW"
	| "CASE_KW"
	| "CATCH_KW"
	| "CLASS_KW"
	| "CONST_KW"
	| "CONTINUE_KW"
	| "DEBUGGER_KW"
	| "DEFAULT_KW"
	| "DELETE_KW"
	| "DO_KW"
	| "ELSE_KW"
	| "ENUM_KW"
	| "EXPORT_KW"
	| "EXTENDS_KW"
	| "FALSE_KW"
	| "FINALLY_KW"
	| "FOR_KW"
	| "FUNCTION_KW"
	| "IF_KW"
	| "IN_KW"
	| "INSTANCEOF_KW"
	| "IMPORT_KW"
	| "NEW_KW"
	| "NULL_KW"
	| "RETURN_KW"
	| "SUPER_KW"
	| "SWITCH_KW"
	| "THIS_KW"
	| "THROW_KW"
	| "TRY_KW"
	| "TRUE_KW"
	| "TYPEOF_KW"
	| "VAR_KW"
	| "VOID_KW"
	| "WHILE_KW"
	| "WITH_KW"
	| "IMPLEMENTS_KW"
	| "INTERFACE_KW"
	| "LET_KW"
	| "PACKAGE_KW"
	| "PRIVATE_KW"
	| "PROTECTED_KW"
	| "PUBLIC_KW"
	| "STATIC_KW"
	| "YIELD_KW"
	| "ABSTRACT_KW"
	| "ACCESSOR_KW"
	| "AS_KW"
	| "SATISFIES_KW"
	| "ASSERTS_KW"
	| "ASSERT_KW"
	| "ANY_KW"
	| "ASYNC_KW"
	| "AWAIT_KW"
	| "BOOLEAN_KW"
	| "CONSTRUCTOR_KW"
	| "DECLARE_KW"
	| "DEFER_KW"
	| "GET_KW"
	| "INFER_KW"
	| "IS_KW"
	| "KEYOF_KW"
	| "MODULE_KW"
	| "NAMESPACE_KW"
	| "NEVER_KW"
	| "READONLY_KW"
	| "REQUIRE_KW"
	| "NUMBER_KW"
	| "OBJECT_KW"
	| "SET_KW"
	| "STRING_KW"
	| "SOURCE_KW"
	| "SYMBOL_KW"
	| "TYPE_KW"
	| "UNDEFINED_KW"
	| "UNIQUE_KW"
	| "UNKNOWN_KW"
	| "FROM_KW"
	| "GLOBAL_KW"
	| "BIGINT_KW"
	| "OVERRIDE_KW"
	| "OF_KW"
	| "OUT_KW"
	| "USING_KW"
	| "JS_NUMBER_LITERAL"
	| "JS_BIGINT_LITERAL"
	| "JS_STRING_LITERAL"
	| "JS_REGEX_LITERAL"
	| "JSX_TEXT_LITERAL"
	| "JSX_STRING_LITERAL"
	| "TARGET"
	| "META"
	| "HASH"
	| "TEMPLATE_CHUNK"
	| "DOLLAR_CURLY"
	| "IDENT"
	| "JSX_IDENT"
	| "JS_SHEBANG";
export type AnyJsAstNode = JsNodeByKind[keyof JsNodeByKind];
export interface AstroImplicitFragment extends JsAstNode {
	readonly kind: "ASTRO_IMPLICIT_FRAGMENT";
	readonly elements: JsxChildList;
	withElements(value: JsxChildListNode): AstroImplicitFragment;
}
export interface JsAccessorModifier extends JsAstNode {
	readonly kind: "JS_ACCESSOR_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): JsAccessorModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsArrayAssignmentPattern extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN";
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): JsArrayAssignmentPattern;
	readonly elements: JsArrayAssignmentPatternElementList;
	withElements(
		value: JsArrayAssignmentPatternElementListNode,
	): JsArrayAssignmentPattern;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): JsArrayAssignmentPattern;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsArrayAssignmentPatternElement extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT";
	readonly pattern: AnyJsAssignmentPattern | undefined;
	withPattern(value: AnyJsAssignmentPattern): JsArrayAssignmentPatternElement;
	readonly init: JsInitializerClause | undefined;
	withInit(
		value: JsInitializerClause | undefined,
	): JsArrayAssignmentPatternElement;
}
export interface JsArrayAssignmentPatternRestElement extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsArrayAssignmentPatternRestElement;
	readonly pattern: AnyJsAssignmentPattern | undefined;
	withPattern(
		value: AnyJsAssignmentPattern,
	): JsArrayAssignmentPatternRestElement;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsArrayBindingPattern extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN";
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): JsArrayBindingPattern;
	readonly elements: JsArrayBindingPatternElementList;
	withElements(
		value: JsArrayBindingPatternElementListNode,
	): JsArrayBindingPattern;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): JsArrayBindingPattern;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsArrayBindingPatternElement extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN_ELEMENT";
	readonly pattern: AnyJsBindingPattern | undefined;
	withPattern(value: AnyJsBindingPattern): JsArrayBindingPatternElement;
	readonly init: JsInitializerClause | undefined;
	withInit(
		value: JsInitializerClause | undefined,
	): JsArrayBindingPatternElement;
}
export interface JsArrayBindingPatternRestElement extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN_REST_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsArrayBindingPatternRestElement;
	readonly pattern: AnyJsBindingPattern | undefined;
	withPattern(value: AnyJsBindingPattern): JsArrayBindingPatternRestElement;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsArrayExpression extends JsAstNode {
	readonly kind: "JS_ARRAY_EXPRESSION";
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): JsArrayExpression;
	readonly elements: JsArrayElementList;
	withElements(value: JsArrayElementListNode): JsArrayExpression;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): JsArrayExpression;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsArrayHole extends JsAstNode {
	readonly kind: "JS_ARRAY_HOLE";
}
export interface JsArrowFunctionExpression extends JsAstNode {
	readonly kind: "JS_ARROW_FUNCTION_EXPRESSION";
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): JsArrowFunctionExpression;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): JsArrowFunctionExpression;
	readonly parameters: AnyJsArrowFunctionParameters | undefined;
	withParameters(
		value: AnyJsArrowFunctionParameters,
	): JsArrowFunctionExpression;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): JsArrowFunctionExpression;
	readonly fatArrowToken: string | undefined;
	withFatArrowToken(value: JsAstToken): JsArrowFunctionExpression;
	readonly body: AnyJsFunctionBody | undefined;
	withBody(value: AnyJsFunctionBody): JsArrowFunctionExpression;
	token(field: "asyncToken" | "fatArrowToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsAssignmentExpression extends JsAstNode {
	readonly kind: "JS_ASSIGNMENT_EXPRESSION";
	readonly left: AnyJsAssignmentPattern | undefined;
	withLeft(value: AnyJsAssignmentPattern): JsAssignmentExpression;
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsAssignmentExpression;
	readonly right: AnyJsExpression | undefined;
	withRight(value: AnyJsExpression): JsAssignmentExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsAwaitExpression extends JsAstNode {
	readonly kind: "JS_AWAIT_EXPRESSION";
	readonly awaitToken: string | undefined;
	withAwaitToken(value: JsAstToken): JsAwaitExpression;
	readonly argument: AnyJsExpression | undefined;
	withArgument(value: AnyJsExpression): JsAwaitExpression;
	token(field: "awaitToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsBigintLiteralExpression extends JsAstNode {
	readonly kind: "JS_BIGINT_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsBigintLiteralExpression;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsBinaryExpression extends JsAstNode {
	readonly kind: "JS_BINARY_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	withLeft(value: AnyJsExpression): JsBinaryExpression;
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsBinaryExpression;
	readonly right: AnyJsExpression | undefined;
	withRight(value: AnyJsExpression): JsBinaryExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsBlockStatement extends JsAstNode {
	readonly kind: "JS_BLOCK_STATEMENT";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsBlockStatement;
	readonly statements: JsStatementList;
	withStatements(value: JsStatementListNode): JsBlockStatement;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsBlockStatement;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsBooleanLiteralExpression extends JsAstNode {
	readonly kind: "JS_BOOLEAN_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsBooleanLiteralExpression;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsBreakStatement extends JsAstNode {
	readonly kind: "JS_BREAK_STATEMENT";
	readonly breakToken: string | undefined;
	withBreakToken(value: JsAstToken): JsBreakStatement;
	readonly label: JsLabel | undefined;
	withLabel(value: JsLabel | undefined): JsBreakStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsBreakStatement;
	token(field: "breakToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsCallArguments extends JsAstNode {
	readonly kind: "JS_CALL_ARGUMENTS";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsCallArguments;
	readonly args: JsCallArgumentList;
	withArgs(value: JsCallArgumentListNode): JsCallArguments;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsCallArguments;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsCallExpression extends JsAstNode {
	readonly kind: "JS_CALL_EXPRESSION";
	readonly callee: AnyJsExpression | undefined;
	withCallee(value: AnyJsExpression): JsCallExpression;
	readonly optionalChainToken: string | undefined;
	withOptionalChainToken(value: JsAstToken | undefined): JsCallExpression;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): JsCallExpression;
	readonly arguments: JsCallArguments | undefined;
	withArguments(value: JsCallArguments): JsCallExpression;
	token(field: "optionalChainToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsCaseClause extends JsAstNode {
	readonly kind: "JS_CASE_CLAUSE";
	readonly caseToken: string | undefined;
	withCaseToken(value: JsAstToken): JsCaseClause;
	readonly test: AnyJsExpression | undefined;
	withTest(value: AnyJsExpression): JsCaseClause;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsCaseClause;
	readonly consequent: JsStatementList;
	withConsequent(value: JsStatementListNode): JsCaseClause;
	token(field: "caseToken" | "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsCatchClause extends JsAstNode {
	readonly kind: "JS_CATCH_CLAUSE";
	readonly catchToken: string | undefined;
	withCatchToken(value: JsAstToken): JsCatchClause;
	readonly declaration: JsCatchDeclaration | undefined;
	withDeclaration(value: JsCatchDeclaration | undefined): JsCatchClause;
	readonly body: JsBlockStatement | undefined;
	withBody(value: JsBlockStatement): JsCatchClause;
	token(field: "catchToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsCatchDeclaration extends JsAstNode {
	readonly kind: "JS_CATCH_DECLARATION";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsCatchDeclaration;
	readonly binding: AnyJsBindingPattern | undefined;
	withBinding(value: AnyJsBindingPattern): JsCatchDeclaration;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation | undefined): JsCatchDeclaration;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsCatchDeclaration;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsClassDeclaration extends JsAstNode {
	readonly kind: "JS_CLASS_DECLARATION";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): JsClassDeclaration;
	readonly abstractToken: string | undefined;
	withAbstractToken(value: JsAstToken | undefined): JsClassDeclaration;
	readonly classToken: string | undefined;
	withClassToken(value: JsAstToken): JsClassDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding): JsClassDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): JsClassDeclaration;
	readonly extendsClause: JsExtendsClause | undefined;
	withExtendsClause(value: JsExtendsClause | undefined): JsClassDeclaration;
	readonly implementsClause: TsImplementsClause | undefined;
	withImplementsClause(
		value: TsImplementsClause | undefined,
	): JsClassDeclaration;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsClassDeclaration;
	readonly members: JsClassMemberList;
	withMembers(value: JsClassMemberListNode): JsClassDeclaration;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsClassDeclaration;
	token(
		field: "abstractToken" | "classToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsClassExportDefaultDeclaration extends JsAstNode {
	readonly kind: "JS_CLASS_EXPORT_DEFAULT_DECLARATION";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): JsClassExportDefaultDeclaration;
	readonly abstractToken: string | undefined;
	withAbstractToken(
		value: JsAstToken | undefined,
	): JsClassExportDefaultDeclaration;
	readonly classToken: string | undefined;
	withClassToken(value: JsAstToken): JsClassExportDefaultDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding | undefined): JsClassExportDefaultDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): JsClassExportDefaultDeclaration;
	readonly extendsClause: JsExtendsClause | undefined;
	withExtendsClause(
		value: JsExtendsClause | undefined,
	): JsClassExportDefaultDeclaration;
	readonly implementsClause: TsImplementsClause | undefined;
	withImplementsClause(
		value: TsImplementsClause | undefined,
	): JsClassExportDefaultDeclaration;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsClassExportDefaultDeclaration;
	readonly members: JsClassMemberList;
	withMembers(value: JsClassMemberListNode): JsClassExportDefaultDeclaration;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsClassExportDefaultDeclaration;
	token(
		field: "abstractToken" | "classToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsClassExpression extends JsAstNode {
	readonly kind: "JS_CLASS_EXPRESSION";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): JsClassExpression;
	readonly classToken: string | undefined;
	withClassToken(value: JsAstToken): JsClassExpression;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding | undefined): JsClassExpression;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): JsClassExpression;
	readonly extendsClause: JsExtendsClause | undefined;
	withExtendsClause(value: JsExtendsClause | undefined): JsClassExpression;
	readonly implementsClause: TsImplementsClause | undefined;
	withImplementsClause(
		value: TsImplementsClause | undefined,
	): JsClassExpression;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsClassExpression;
	readonly members: JsClassMemberList;
	withMembers(value: JsClassMemberListNode): JsClassExpression;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsClassExpression;
	token(
		field: "classToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsComputedMemberAssignment extends JsAstNode {
	readonly kind: "JS_COMPUTED_MEMBER_ASSIGNMENT";
	readonly object: AnyJsExpression | undefined;
	withObject(value: AnyJsExpression): JsComputedMemberAssignment;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): JsComputedMemberAssignment;
	readonly member: AnyJsExpression | undefined;
	withMember(value: AnyJsExpression): JsComputedMemberAssignment;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): JsComputedMemberAssignment;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsComputedMemberExpression extends JsAstNode {
	readonly kind: "JS_COMPUTED_MEMBER_EXPRESSION";
	readonly object: AnyJsExpression | undefined;
	withObject(value: AnyJsExpression): JsComputedMemberExpression;
	readonly optionalChainToken: string | undefined;
	withOptionalChainToken(
		value: JsAstToken | undefined,
	): JsComputedMemberExpression;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): JsComputedMemberExpression;
	readonly member: AnyJsExpression | undefined;
	withMember(value: AnyJsExpression): JsComputedMemberExpression;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): JsComputedMemberExpression;
	token(
		field: "optionalChainToken" | "lBrackToken" | "rBrackToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsComputedMemberName extends JsAstNode {
	readonly kind: "JS_COMPUTED_MEMBER_NAME";
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): JsComputedMemberName;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsComputedMemberName;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): JsComputedMemberName;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsConditionalExpression extends JsAstNode {
	readonly kind: "JS_CONDITIONAL_EXPRESSION";
	readonly test: AnyJsExpression | undefined;
	withTest(value: AnyJsExpression): JsConditionalExpression;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken): JsConditionalExpression;
	readonly consequent: AnyJsExpression | undefined;
	withConsequent(value: AnyJsExpression): JsConditionalExpression;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsConditionalExpression;
	readonly alternate: AnyJsExpression | undefined;
	withAlternate(value: AnyJsExpression): JsConditionalExpression;
	token(field: "questionMarkToken" | "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsConstructorClassMember extends JsAstNode {
	readonly kind: "JS_CONSTRUCTOR_CLASS_MEMBER";
	readonly modifiers: JsConstructorModifierList;
	withModifiers(value: JsConstructorModifierListNode): JsConstructorClassMember;
	readonly name: JsLiteralMemberName | undefined;
	withName(value: JsLiteralMemberName): JsConstructorClassMember;
	readonly parameters: JsConstructorParameters | undefined;
	withParameters(value: JsConstructorParameters): JsConstructorClassMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsConstructorClassMember;
}
export interface JsConstructorParameters extends JsAstNode {
	readonly kind: "JS_CONSTRUCTOR_PARAMETERS";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsConstructorParameters;
	readonly parameters: JsConstructorParameterList;
	withParameters(
		value: JsConstructorParameterListNode,
	): JsConstructorParameters;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsConstructorParameters;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsContinueStatement extends JsAstNode {
	readonly kind: "JS_CONTINUE_STATEMENT";
	readonly continueToken: string | undefined;
	withContinueToken(value: JsAstToken): JsContinueStatement;
	readonly label: JsLabel | undefined;
	withLabel(value: JsLabel | undefined): JsContinueStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsContinueStatement;
	token(field: "continueToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsDebuggerStatement extends JsAstNode {
	readonly kind: "JS_DEBUGGER_STATEMENT";
	readonly debuggerToken: string | undefined;
	withDebuggerToken(value: JsAstToken): JsDebuggerStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsDebuggerStatement;
	token(field: "debuggerToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsDecorator extends JsAstNode {
	readonly kind: "JS_DECORATOR";
	readonly atToken: string | undefined;
	withAtToken(value: JsAstToken): JsDecorator;
	readonly expression: AnyJsDecorator | undefined;
	withExpression(value: AnyJsDecorator): JsDecorator;
	token(field: "atToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsDefaultClause extends JsAstNode {
	readonly kind: "JS_DEFAULT_CLAUSE";
	readonly defaultToken: string | undefined;
	withDefaultToken(value: JsAstToken): JsDefaultClause;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsDefaultClause;
	readonly consequent: JsStatementList;
	withConsequent(value: JsStatementListNode): JsDefaultClause;
	token(field: "defaultToken" | "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsDefaultImportSpecifier extends JsAstNode {
	readonly kind: "JS_DEFAULT_IMPORT_SPECIFIER";
	readonly localName: AnyJsBinding | undefined;
	withLocalName(value: AnyJsBinding): JsDefaultImportSpecifier;
}
export interface JsDirective extends JsAstNode {
	readonly kind: "JS_DIRECTIVE";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsDirective;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsDirective;
	token(field: "valueToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsDoWhileStatement extends JsAstNode {
	readonly kind: "JS_DO_WHILE_STATEMENT";
	readonly doToken: string | undefined;
	withDoToken(value: JsAstToken): JsDoWhileStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsDoWhileStatement;
	readonly whileToken: string | undefined;
	withWhileToken(value: JsAstToken): JsDoWhileStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsDoWhileStatement;
	readonly test: AnyJsExpression | undefined;
	withTest(value: AnyJsExpression): JsDoWhileStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsDoWhileStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsDoWhileStatement;
	token(
		field:
			| "doToken"
			| "whileToken"
			| "lParenToken"
			| "rParenToken"
			| "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsElseClause extends JsAstNode {
	readonly kind: "JS_ELSE_CLAUSE";
	readonly elseToken: string | undefined;
	withElseToken(value: JsAstToken): JsElseClause;
	readonly alternate: AnyJsStatement | undefined;
	withAlternate(value: AnyJsStatement): JsElseClause;
	token(field: "elseToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsEmptyClassMember extends JsAstNode {
	readonly kind: "JS_EMPTY_CLASS_MEMBER";
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken): JsEmptyClassMember;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsEmptyStatement extends JsAstNode {
	readonly kind: "JS_EMPTY_STATEMENT";
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken): JsEmptyStatement;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExport extends JsAstNode {
	readonly kind: "JS_EXPORT";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): JsExport;
	readonly exportToken: string | undefined;
	withExportToken(value: JsAstToken): JsExport;
	readonly exportClause: AnyJsExportClause | undefined;
	withExportClause(value: AnyJsExportClause): JsExport;
	token(field: "exportToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportAsClause extends JsAstNode {
	readonly kind: "JS_EXPORT_AS_CLAUSE";
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): JsExportAsClause;
	readonly exportedName: AnyJsLiteralExportName | undefined;
	withExportedName(value: AnyJsLiteralExportName): JsExportAsClause;
	token(field: "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportDefaultDeclarationClause extends JsAstNode {
	readonly kind: "JS_EXPORT_DEFAULT_DECLARATION_CLAUSE";
	readonly defaultToken: string | undefined;
	withDefaultToken(value: JsAstToken): JsExportDefaultDeclarationClause;
	readonly declaration: AnyJsExportDefaultDeclaration | undefined;
	withDeclaration(
		value: AnyJsExportDefaultDeclaration,
	): JsExportDefaultDeclarationClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): JsExportDefaultDeclarationClause;
	token(field: "defaultToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportDefaultExpressionClause extends JsAstNode {
	readonly kind: "JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE";
	readonly defaultToken: string | undefined;
	withDefaultToken(value: JsAstToken): JsExportDefaultExpressionClause;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsExportDefaultExpressionClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): JsExportDefaultExpressionClause;
	token(field: "defaultToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportFromClause extends JsAstNode {
	readonly kind: "JS_EXPORT_FROM_CLAUSE";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsExportFromClause;
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken): JsExportFromClause;
	readonly exportAs: JsExportAsClause | undefined;
	withExportAs(value: JsExportAsClause | undefined): JsExportFromClause;
	readonly fromToken: string | undefined;
	withFromToken(value: JsAstToken): JsExportFromClause;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsExportFromClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsExportFromClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsExportFromClause;
	token(
		field: "typeToken" | "starToken" | "fromToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportNamedClause extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_CLAUSE";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsExportNamedClause;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsExportNamedClause;
	readonly specifiers: JsExportNamedSpecifierList;
	withSpecifiers(value: JsExportNamedSpecifierListNode): JsExportNamedClause;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsExportNamedClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsExportNamedClause;
	token(
		field: "typeToken" | "lCurlyToken" | "rCurlyToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportNamedFromClause extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_FROM_CLAUSE";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsExportNamedFromClause;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsExportNamedFromClause;
	readonly specifiers: JsExportNamedFromSpecifierList;
	withSpecifiers(
		value: JsExportNamedFromSpecifierListNode,
	): JsExportNamedFromClause;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsExportNamedFromClause;
	readonly fromToken: string | undefined;
	withFromToken(value: JsAstToken): JsExportNamedFromClause;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsExportNamedFromClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsExportNamedFromClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsExportNamedFromClause;
	token(
		field:
			| "typeToken"
			| "lCurlyToken"
			| "rCurlyToken"
			| "fromToken"
			| "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportNamedFromSpecifier extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_FROM_SPECIFIER";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsExportNamedFromSpecifier;
	readonly sourceName: AnyJsLiteralExportName | undefined;
	withSourceName(value: AnyJsLiteralExportName): JsExportNamedFromSpecifier;
	readonly exportAs: JsExportAsClause | undefined;
	withExportAs(value: JsExportAsClause | undefined): JsExportNamedFromSpecifier;
	token(field: "typeToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportNamedShorthandSpecifier extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_SHORTHAND_SPECIFIER";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsExportNamedShorthandSpecifier;
	readonly name: JsReferenceIdentifier | undefined;
	withName(value: JsReferenceIdentifier): JsExportNamedShorthandSpecifier;
	token(field: "typeToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExportNamedSpecifier extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_SPECIFIER";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsExportNamedSpecifier;
	readonly localName: JsReferenceIdentifier | undefined;
	withLocalName(value: JsReferenceIdentifier): JsExportNamedSpecifier;
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): JsExportNamedSpecifier;
	readonly exportedName: AnyJsLiteralExportName | undefined;
	withExportedName(value: AnyJsLiteralExportName): JsExportNamedSpecifier;
	token(field: "typeToken" | "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExpressionSnippet extends JsAstNode {
	readonly kind: "JS_EXPRESSION_SNIPPET";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsExpressionSnippet;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): JsExpressionSnippet;
	token(field: "eofToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExpressionStatement extends JsAstNode {
	readonly kind: "JS_EXPRESSION_STATEMENT";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsExpressionStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsExpressionStatement;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExpressionTemplateRoot extends JsAstNode {
	readonly kind: "JS_EXPRESSION_TEMPLATE_ROOT";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression | undefined): JsExpressionTemplateRoot;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): JsExpressionTemplateRoot;
	token(field: "eofToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsExtendsClause extends JsAstNode {
	readonly kind: "JS_EXTENDS_CLAUSE";
	readonly extendsToken: string | undefined;
	withExtendsToken(value: JsAstToken): JsExtendsClause;
	readonly superClass: AnyJsExpression | undefined;
	withSuperClass(value: AnyJsExpression): JsExtendsClause;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): JsExtendsClause;
	token(field: "extendsToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsFinallyClause extends JsAstNode {
	readonly kind: "JS_FINALLY_CLAUSE";
	readonly finallyToken: string | undefined;
	withFinallyToken(value: JsAstToken): JsFinallyClause;
	readonly body: JsBlockStatement | undefined;
	withBody(value: JsBlockStatement): JsFinallyClause;
	token(field: "finallyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsForInStatement extends JsAstNode {
	readonly kind: "JS_FOR_IN_STATEMENT";
	readonly forToken: string | undefined;
	withForToken(value: JsAstToken): JsForInStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsForInStatement;
	readonly initializer: AnyJsForInOrOfInitializer | undefined;
	withInitializer(value: AnyJsForInOrOfInitializer): JsForInStatement;
	readonly inToken: string | undefined;
	withInToken(value: JsAstToken): JsForInStatement;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsForInStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsForInStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsForInStatement;
	token(
		field: "forToken" | "lParenToken" | "inToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsForOfStatement extends JsAstNode {
	readonly kind: "JS_FOR_OF_STATEMENT";
	readonly forToken: string | undefined;
	withForToken(value: JsAstToken): JsForOfStatement;
	readonly awaitToken: string | undefined;
	withAwaitToken(value: JsAstToken | undefined): JsForOfStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsForOfStatement;
	readonly initializer: AnyJsForInOrOfInitializer | undefined;
	withInitializer(value: AnyJsForInOrOfInitializer): JsForOfStatement;
	readonly ofToken: string | undefined;
	withOfToken(value: JsAstToken): JsForOfStatement;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsForOfStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsForOfStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsForOfStatement;
	token(
		field:
			| "forToken"
			| "awaitToken"
			| "lParenToken"
			| "ofToken"
			| "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsForStatement extends JsAstNode {
	readonly kind: "JS_FOR_STATEMENT";
	readonly forToken: string | undefined;
	withForToken(value: JsAstToken): JsForStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsForStatement;
	readonly initializer: AnyJsForInitializer | undefined;
	withInitializer(value: AnyJsForInitializer | undefined): JsForStatement;
	readonly firstSemiToken: string | undefined;
	withFirstSemiToken(value: JsAstToken): JsForStatement;
	readonly test: AnyJsExpression | undefined;
	withTest(value: AnyJsExpression | undefined): JsForStatement;
	readonly secondSemiToken: string | undefined;
	withSecondSemiToken(value: JsAstToken): JsForStatement;
	readonly update: AnyJsExpression | undefined;
	withUpdate(value: AnyJsExpression | undefined): JsForStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsForStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsForStatement;
	token(
		field:
			| "forToken"
			| "lParenToken"
			| "firstSemiToken"
			| "secondSemiToken"
			| "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsForVariableDeclaration extends JsAstNode {
	readonly kind: "JS_FOR_VARIABLE_DECLARATION";
	readonly awaitToken: string | undefined;
	withAwaitToken(value: JsAstToken | undefined): JsForVariableDeclaration;
	readonly kindToken: string | undefined;
	withKindToken(value: JsAstToken): JsForVariableDeclaration;
	readonly declarator: JsVariableDeclarator | undefined;
	withDeclarator(value: JsVariableDeclarator): JsForVariableDeclaration;
	token(field: "awaitToken" | "kindToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsFormalParameter extends JsAstNode {
	readonly kind: "JS_FORMAL_PARAMETER";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): JsFormalParameter;
	readonly binding: AnyJsBindingPattern | undefined;
	withBinding(value: AnyJsBindingPattern): JsFormalParameter;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken | undefined): JsFormalParameter;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation | undefined): JsFormalParameter;
	readonly initializer: JsInitializerClause | undefined;
	withInitializer(value: JsInitializerClause | undefined): JsFormalParameter;
	token(field: "questionMarkToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsFunctionBody extends JsAstNode {
	readonly kind: "JS_FUNCTION_BODY";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsFunctionBody;
	readonly directives: JsDirectiveList;
	withDirectives(value: JsDirectiveListNode): JsFunctionBody;
	readonly statements: JsStatementList;
	withStatements(value: JsStatementListNode): JsFunctionBody;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsFunctionBody;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsFunctionDeclaration extends JsAstNode {
	readonly kind: "JS_FUNCTION_DECLARATION";
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): JsFunctionDeclaration;
	readonly functionToken: string | undefined;
	withFunctionToken(value: JsAstToken): JsFunctionDeclaration;
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken | undefined): JsFunctionDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding): JsFunctionDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): JsFunctionDeclaration;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): JsFunctionDeclaration;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): JsFunctionDeclaration;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsFunctionDeclaration;
	token(
		field: "asyncToken" | "functionToken" | "starToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsFunctionExportDefaultDeclaration extends JsAstNode {
	readonly kind: "JS_FUNCTION_EXPORT_DEFAULT_DECLARATION";
	readonly asyncToken: string | undefined;
	withAsyncToken(
		value: JsAstToken | undefined,
	): JsFunctionExportDefaultDeclaration;
	readonly functionToken: string | undefined;
	withFunctionToken(value: JsAstToken): JsFunctionExportDefaultDeclaration;
	readonly starToken: string | undefined;
	withStarToken(
		value: JsAstToken | undefined,
	): JsFunctionExportDefaultDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding | undefined): JsFunctionExportDefaultDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): JsFunctionExportDefaultDeclaration;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): JsFunctionExportDefaultDeclaration;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): JsFunctionExportDefaultDeclaration;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsFunctionExportDefaultDeclaration;
	token(
		field: "asyncToken" | "functionToken" | "starToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsFunctionExpression extends JsAstNode {
	readonly kind: "JS_FUNCTION_EXPRESSION";
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): JsFunctionExpression;
	readonly functionToken: string | undefined;
	withFunctionToken(value: JsAstToken): JsFunctionExpression;
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken | undefined): JsFunctionExpression;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding | undefined): JsFunctionExpression;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): JsFunctionExpression;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): JsFunctionExpression;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): JsFunctionExpression;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsFunctionExpression;
	token(
		field: "asyncToken" | "functionToken" | "starToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsGetterClassMember extends JsAstNode {
	readonly kind: "JS_GETTER_CLASS_MEMBER";
	readonly modifiers: JsMethodModifierList;
	withModifiers(value: JsMethodModifierListNode): JsGetterClassMember;
	readonly getToken: string | undefined;
	withGetToken(value: JsAstToken): JsGetterClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): JsGetterClassMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsGetterClassMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsGetterClassMember;
	readonly returnType: TsTypeAnnotation | undefined;
	withReturnType(value: TsTypeAnnotation | undefined): JsGetterClassMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsGetterClassMember;
	token(
		field: "getToken" | "lParenToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsGetterObjectMember extends JsAstNode {
	readonly kind: "JS_GETTER_OBJECT_MEMBER";
	readonly getToken: string | undefined;
	withGetToken(value: JsAstToken): JsGetterObjectMember;
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): JsGetterObjectMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsGetterObjectMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsGetterObjectMember;
	readonly returnType: TsTypeAnnotation | undefined;
	withReturnType(value: TsTypeAnnotation | undefined): JsGetterObjectMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsGetterObjectMember;
	token(
		field: "getToken" | "lParenToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsIdentifierAssignment extends JsAstNode {
	readonly kind: "JS_IDENTIFIER_ASSIGNMENT";
	readonly nameToken: string | undefined;
	withNameToken(value: JsAstToken): JsIdentifierAssignment;
	token(field: "nameToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsIdentifierBinding extends JsAstNode {
	readonly kind: "JS_IDENTIFIER_BINDING";
	readonly nameToken: string | undefined;
	withNameToken(value: JsAstToken): JsIdentifierBinding;
	token(field: "nameToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsIdentifierExpression extends JsAstNode {
	readonly kind: "JS_IDENTIFIER_EXPRESSION";
	readonly name: JsReferenceIdentifier | undefined;
	withName(value: JsReferenceIdentifier): JsIdentifierExpression;
}
export interface JsIfStatement extends JsAstNode {
	readonly kind: "JS_IF_STATEMENT";
	readonly ifToken: string | undefined;
	withIfToken(value: JsAstToken): JsIfStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsIfStatement;
	readonly test: AnyJsExpression | undefined;
	withTest(value: AnyJsExpression): JsIfStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsIfStatement;
	readonly consequent: AnyJsStatement | undefined;
	withConsequent(value: AnyJsStatement): JsIfStatement;
	readonly elseClause: JsElseClause | undefined;
	withElseClause(value: JsElseClause | undefined): JsIfStatement;
	token(
		field: "ifToken" | "lParenToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImport extends JsAstNode {
	readonly kind: "JS_IMPORT";
	readonly importToken: string | undefined;
	withImportToken(value: JsAstToken): JsImport;
	readonly importClause: AnyJsImportClause | undefined;
	withImportClause(value: AnyJsImportClause): JsImport;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsImport;
	token(field: "importToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportAssertion extends JsAstNode {
	readonly kind: "JS_IMPORT_ASSERTION";
	readonly withToken: string | undefined;
	withWithToken(value: JsAstToken): JsImportAssertion;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsImportAssertion;
	readonly assertions: JsImportAssertionEntryList;
	withAssertions(value: JsImportAssertionEntryListNode): JsImportAssertion;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsImportAssertion;
	token(
		field: "withToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportAssertionEntry extends JsAstNode {
	readonly kind: "JS_IMPORT_ASSERTION_ENTRY";
	readonly key: string | undefined;
	withKey(value: JsAstToken): JsImportAssertionEntry;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsImportAssertionEntry;
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsImportAssertionEntry;
	token(field: "key" | "colonToken" | "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportBareClause extends JsAstNode {
	readonly kind: "JS_IMPORT_BARE_CLAUSE";
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsImportBareClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsImportBareClause;
}
export interface JsImportCallExpression extends JsAstNode {
	readonly kind: "JS_IMPORT_CALL_EXPRESSION";
	readonly importToken: string | undefined;
	withImportToken(value: JsAstToken): JsImportCallExpression;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken | undefined): JsImportCallExpression;
	readonly phase: string | undefined;
	withPhase(value: JsAstToken | undefined): JsImportCallExpression;
	readonly arguments: JsCallArguments | undefined;
	withArguments(value: JsCallArguments): JsImportCallExpression;
	token(field: "importToken" | "dotToken" | "phase"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportCombinedClause extends JsAstNode {
	readonly kind: "JS_IMPORT_COMBINED_CLAUSE";
	readonly defaultSpecifier: JsDefaultImportSpecifier | undefined;
	withDefaultSpecifier(value: JsDefaultImportSpecifier): JsImportCombinedClause;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken): JsImportCombinedClause;
	readonly specifier: AnyJsCombinedSpecifier | undefined;
	withSpecifier(value: AnyJsCombinedSpecifier): JsImportCombinedClause;
	readonly fromToken: string | undefined;
	withFromToken(value: JsAstToken): JsImportCombinedClause;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsImportCombinedClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsImportCombinedClause;
	token(field: "commaToken" | "fromToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportDefaultClause extends JsAstNode {
	readonly kind: "JS_IMPORT_DEFAULT_CLAUSE";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsImportDefaultClause;
	readonly phaseToken: string | undefined;
	withPhaseToken(value: JsAstToken | undefined): JsImportDefaultClause;
	readonly defaultSpecifier: JsDefaultImportSpecifier | undefined;
	withDefaultSpecifier(value: JsDefaultImportSpecifier): JsImportDefaultClause;
	readonly fromToken: string | undefined;
	withFromToken(value: JsAstToken): JsImportDefaultClause;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsImportDefaultClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsImportDefaultClause;
	token(
		field: "typeToken" | "phaseToken" | "fromToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportMetaExpression extends JsAstNode {
	readonly kind: "JS_IMPORT_META_EXPRESSION";
	readonly importToken: string | undefined;
	withImportToken(value: JsAstToken): JsImportMetaExpression;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): JsImportMetaExpression;
	readonly metaToken: string | undefined;
	withMetaToken(value: JsAstToken): JsImportMetaExpression;
	token(
		field: "importToken" | "dotToken" | "metaToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportNamedClause extends JsAstNode {
	readonly kind: "JS_IMPORT_NAMED_CLAUSE";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsImportNamedClause;
	readonly namedSpecifiers: JsNamedImportSpecifiers | undefined;
	withNamedSpecifiers(value: JsNamedImportSpecifiers): JsImportNamedClause;
	readonly fromToken: string | undefined;
	withFromToken(value: JsAstToken): JsImportNamedClause;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsImportNamedClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsImportNamedClause;
	token(field: "typeToken" | "fromToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsImportNamespaceClause extends JsAstNode {
	readonly kind: "JS_IMPORT_NAMESPACE_CLAUSE";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsImportNamespaceClause;
	readonly phaseToken: string | undefined;
	withPhaseToken(value: JsAstToken | undefined): JsImportNamespaceClause;
	readonly namespaceSpecifier: JsNamespaceImportSpecifier | undefined;
	withNamespaceSpecifier(
		value: JsNamespaceImportSpecifier,
	): JsImportNamespaceClause;
	readonly fromToken: string | undefined;
	withFromToken(value: JsAstToken): JsImportNamespaceClause;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): JsImportNamespaceClause;
	readonly assertion: JsImportAssertion | undefined;
	withAssertion(value: JsImportAssertion | undefined): JsImportNamespaceClause;
	token(
		field: "typeToken" | "phaseToken" | "fromToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsInExpression extends JsAstNode {
	readonly kind: "JS_IN_EXPRESSION";
	readonly property: AnyJsInProperty | undefined;
	withProperty(value: AnyJsInProperty): JsInExpression;
	readonly inToken: string | undefined;
	withInToken(value: JsAstToken): JsInExpression;
	readonly object: AnyJsExpression | undefined;
	withObject(value: AnyJsExpression): JsInExpression;
	token(field: "inToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsInitializerClause extends JsAstNode {
	readonly kind: "JS_INITIALIZER_CLAUSE";
	readonly eqToken: string | undefined;
	withEqToken(value: JsAstToken): JsInitializerClause;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsInitializerClause;
	token(field: "eqToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsInstanceofExpression extends JsAstNode {
	readonly kind: "JS_INSTANCEOF_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	withLeft(value: AnyJsExpression): JsInstanceofExpression;
	readonly instanceofToken: string | undefined;
	withInstanceofToken(value: JsAstToken): JsInstanceofExpression;
	readonly right: AnyJsExpression | undefined;
	withRight(value: AnyJsExpression): JsInstanceofExpression;
	token(field: "instanceofToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsLabel extends JsAstNode {
	readonly kind: "JS_LABEL";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsLabel;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsLabeledStatement extends JsAstNode {
	readonly kind: "JS_LABELED_STATEMENT";
	readonly label: JsLabel | undefined;
	withLabel(value: JsLabel): JsLabeledStatement;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsLabeledStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsLabeledStatement;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsLiteralExportName extends JsAstNode {
	readonly kind: "JS_LITERAL_EXPORT_NAME";
	readonly value: string | undefined;
	withValue(value: JsAstToken): JsLiteralExportName;
	token(field: "value"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsLiteralMemberName extends JsAstNode {
	readonly kind: "JS_LITERAL_MEMBER_NAME";
	readonly value: string | undefined;
	withValue(value: JsAstToken): JsLiteralMemberName;
	token(field: "value"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsLogicalExpression extends JsAstNode {
	readonly kind: "JS_LOGICAL_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	withLeft(value: AnyJsExpression): JsLogicalExpression;
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsLogicalExpression;
	readonly right: AnyJsExpression | undefined;
	withRight(value: AnyJsExpression): JsLogicalExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsMetavariable extends JsAstNode {
	readonly kind: "JS_METAVARIABLE";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsMetavariable;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsMethodClassMember extends JsAstNode {
	readonly kind: "JS_METHOD_CLASS_MEMBER";
	readonly modifiers: JsMethodModifierList;
	withModifiers(value: JsMethodModifierListNode): JsMethodClassMember;
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): JsMethodClassMember;
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken | undefined): JsMethodClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): JsMethodClassMember;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken | undefined): JsMethodClassMember;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): JsMethodClassMember;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): JsMethodClassMember;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): JsMethodClassMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsMethodClassMember;
	token(
		field: "asyncToken" | "starToken" | "questionMarkToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsMethodObjectMember extends JsAstNode {
	readonly kind: "JS_METHOD_OBJECT_MEMBER";
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): JsMethodObjectMember;
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken | undefined): JsMethodObjectMember;
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): JsMethodObjectMember;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): JsMethodObjectMember;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): JsMethodObjectMember;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): JsMethodObjectMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsMethodObjectMember;
	token(field: "asyncToken" | "starToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsModule extends JsAstNode {
	readonly kind: "JS_MODULE";
	readonly bomToken: string | undefined;
	withBomToken(value: JsAstToken | undefined): JsModule;
	readonly interpreterToken: string | undefined;
	withInterpreterToken(value: JsAstToken | undefined): JsModule;
	readonly directives: JsDirectiveList;
	withDirectives(value: JsDirectiveListNode): JsModule;
	readonly items: JsModuleItemList;
	withItems(value: JsModuleItemListNode): JsModule;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): JsModule;
	token(
		field: "bomToken" | "interpreterToken" | "eofToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsModuleSource extends JsAstNode {
	readonly kind: "JS_MODULE_SOURCE";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsModuleSource;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsName extends JsAstNode {
	readonly kind: "JS_NAME";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsName;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNamedImportSpecifier extends JsAstNode {
	readonly kind: "JS_NAMED_IMPORT_SPECIFIER";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsNamedImportSpecifier;
	readonly name: AnyJsLiteralExportName | undefined;
	withName(value: AnyJsLiteralExportName): JsNamedImportSpecifier;
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): JsNamedImportSpecifier;
	readonly localName: AnyJsBinding | undefined;
	withLocalName(value: AnyJsBinding): JsNamedImportSpecifier;
	token(field: "typeToken" | "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNamedImportSpecifiers extends JsAstNode {
	readonly kind: "JS_NAMED_IMPORT_SPECIFIERS";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsNamedImportSpecifiers;
	readonly specifiers: JsNamedImportSpecifierList;
	withSpecifiers(
		value: JsNamedImportSpecifierListNode,
	): JsNamedImportSpecifiers;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsNamedImportSpecifiers;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNamespaceImportSpecifier extends JsAstNode {
	readonly kind: "JS_NAMESPACE_IMPORT_SPECIFIER";
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken): JsNamespaceImportSpecifier;
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): JsNamespaceImportSpecifier;
	readonly localName: AnyJsBinding | undefined;
	withLocalName(value: AnyJsBinding): JsNamespaceImportSpecifier;
	token(field: "starToken" | "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNewExpression extends JsAstNode {
	readonly kind: "JS_NEW_EXPRESSION";
	readonly newToken: string | undefined;
	withNewToken(value: JsAstToken): JsNewExpression;
	readonly callee: AnyJsExpression | undefined;
	withCallee(value: AnyJsExpression): JsNewExpression;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): JsNewExpression;
	readonly arguments: JsCallArguments | undefined;
	withArguments(value: JsCallArguments | undefined): JsNewExpression;
	token(field: "newToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNewTargetExpression extends JsAstNode {
	readonly kind: "JS_NEW_TARGET_EXPRESSION";
	readonly newToken: string | undefined;
	withNewToken(value: JsAstToken): JsNewTargetExpression;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): JsNewTargetExpression;
	readonly targetToken: string | undefined;
	withTargetToken(value: JsAstToken): JsNewTargetExpression;
	token(field: "newToken" | "dotToken" | "targetToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNullLiteralExpression extends JsAstNode {
	readonly kind: "JS_NULL_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsNullLiteralExpression;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsNumberLiteralExpression extends JsAstNode {
	readonly kind: "JS_NUMBER_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsNumberLiteralExpression;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectAssignmentPattern extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsObjectAssignmentPattern;
	readonly properties: JsObjectAssignmentPatternPropertyList;
	withProperties(
		value: JsObjectAssignmentPatternPropertyListNode,
	): JsObjectAssignmentPattern;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsObjectAssignmentPattern;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectAssignmentPatternProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY";
	readonly member: AnyJsObjectMemberName | undefined;
	withMember(value: AnyJsObjectMemberName): JsObjectAssignmentPatternProperty;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsObjectAssignmentPatternProperty;
	readonly pattern: AnyJsAssignmentPattern | undefined;
	withPattern(value: AnyJsAssignmentPattern): JsObjectAssignmentPatternProperty;
	readonly init: JsInitializerClause | undefined;
	withInit(
		value: JsInitializerClause | undefined,
	): JsObjectAssignmentPatternProperty;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectAssignmentPatternRest extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_REST";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsObjectAssignmentPatternRest;
	readonly target: AnyJsAssignment | undefined;
	withTarget(value: AnyJsAssignment): JsObjectAssignmentPatternRest;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectAssignmentPatternShorthandProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY";
	readonly identifier: JsIdentifierAssignment | undefined;
	withIdentifier(
		value: JsIdentifierAssignment,
	): JsObjectAssignmentPatternShorthandProperty;
	readonly init: JsInitializerClause | undefined;
	withInit(
		value: JsInitializerClause | undefined,
	): JsObjectAssignmentPatternShorthandProperty;
}
export interface JsObjectBindingPattern extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsObjectBindingPattern;
	readonly properties: JsObjectBindingPatternPropertyList;
	withProperties(
		value: JsObjectBindingPatternPropertyListNode,
	): JsObjectBindingPattern;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsObjectBindingPattern;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectBindingPatternProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_PROPERTY";
	readonly member: AnyJsObjectMemberName | undefined;
	withMember(value: AnyJsObjectMemberName): JsObjectBindingPatternProperty;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsObjectBindingPatternProperty;
	readonly pattern: AnyJsBindingPattern | undefined;
	withPattern(value: AnyJsBindingPattern): JsObjectBindingPatternProperty;
	readonly init: JsInitializerClause | undefined;
	withInit(
		value: JsInitializerClause | undefined,
	): JsObjectBindingPatternProperty;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectBindingPatternRest extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_REST";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsObjectBindingPatternRest;
	readonly binding: AnyJsBinding | undefined;
	withBinding(value: AnyJsBinding): JsObjectBindingPatternRest;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsObjectBindingPatternShorthandProperty extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY";
	readonly identifier: AnyJsBinding | undefined;
	withIdentifier(value: AnyJsBinding): JsObjectBindingPatternShorthandProperty;
	readonly init: JsInitializerClause | undefined;
	withInit(
		value: JsInitializerClause | undefined,
	): JsObjectBindingPatternShorthandProperty;
}
export interface JsObjectExpression extends JsAstNode {
	readonly kind: "JS_OBJECT_EXPRESSION";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsObjectExpression;
	readonly members: JsObjectMemberList;
	withMembers(value: JsObjectMemberListNode): JsObjectExpression;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsObjectExpression;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsParameters extends JsAstNode {
	readonly kind: "JS_PARAMETERS";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsParameters;
	readonly items: JsParameterList;
	withItems(value: JsParameterListNode): JsParameters;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsParameters;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsParenthesizedAssignment extends JsAstNode {
	readonly kind: "JS_PARENTHESIZED_ASSIGNMENT";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsParenthesizedAssignment;
	readonly assignment: AnyJsAssignment | undefined;
	withAssignment(value: AnyJsAssignment): JsParenthesizedAssignment;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsParenthesizedAssignment;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsParenthesizedExpression extends JsAstNode {
	readonly kind: "JS_PARENTHESIZED_EXPRESSION";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsParenthesizedExpression;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsParenthesizedExpression;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsParenthesizedExpression;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsPostUpdateExpression extends JsAstNode {
	readonly kind: "JS_POST_UPDATE_EXPRESSION";
	readonly operand: AnyJsAssignment | undefined;
	withOperand(value: AnyJsAssignment): JsPostUpdateExpression;
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsPostUpdateExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsPreUpdateExpression extends JsAstNode {
	readonly kind: "JS_PRE_UPDATE_EXPRESSION";
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsPreUpdateExpression;
	readonly operand: AnyJsAssignment | undefined;
	withOperand(value: AnyJsAssignment): JsPreUpdateExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsPrivateClassMemberName extends JsAstNode {
	readonly kind: "JS_PRIVATE_CLASS_MEMBER_NAME";
	readonly hashToken: string | undefined;
	withHashToken(value: JsAstToken): JsPrivateClassMemberName;
	readonly idToken: string | undefined;
	withIdToken(value: JsAstToken): JsPrivateClassMemberName;
	token(field: "hashToken" | "idToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsPrivateName extends JsAstNode {
	readonly kind: "JS_PRIVATE_NAME";
	readonly hashToken: string | undefined;
	withHashToken(value: JsAstToken): JsPrivateName;
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsPrivateName;
	token(field: "hashToken" | "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsPropertyClassMember extends JsAstNode {
	readonly kind: "JS_PROPERTY_CLASS_MEMBER";
	readonly modifiers: JsPropertyModifierList;
	withModifiers(value: JsPropertyModifierListNode): JsPropertyClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): JsPropertyClassMember;
	readonly propertyAnnotation: AnyTsPropertyAnnotation | undefined;
	withPropertyAnnotation(
		value: AnyTsPropertyAnnotation | undefined,
	): JsPropertyClassMember;
	readonly value: JsInitializerClause | undefined;
	withValue(value: JsInitializerClause | undefined): JsPropertyClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsPropertyClassMember;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsPropertyObjectMember extends JsAstNode {
	readonly kind: "JS_PROPERTY_OBJECT_MEMBER";
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): JsPropertyObjectMember;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsPropertyObjectMember;
	readonly value: AnyJsExpression | undefined;
	withValue(value: AnyJsExpression): JsPropertyObjectMember;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsReferenceIdentifier extends JsAstNode {
	readonly kind: "JS_REFERENCE_IDENTIFIER";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsReferenceIdentifier;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsRegexLiteralExpression extends JsAstNode {
	readonly kind: "JS_REGEX_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsRegexLiteralExpression;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsRestParameter extends JsAstNode {
	readonly kind: "JS_REST_PARAMETER";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): JsRestParameter;
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsRestParameter;
	readonly binding: AnyJsBindingPattern | undefined;
	withBinding(value: AnyJsBindingPattern): JsRestParameter;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation | undefined): JsRestParameter;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsReturnStatement extends JsAstNode {
	readonly kind: "JS_RETURN_STATEMENT";
	readonly returnToken: string | undefined;
	withReturnToken(value: JsAstToken): JsReturnStatement;
	readonly argument: AnyJsExpression | undefined;
	withArgument(value: AnyJsExpression | undefined): JsReturnStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsReturnStatement;
	token(field: "returnToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsScript extends JsAstNode {
	readonly kind: "JS_SCRIPT";
	readonly bomToken: string | undefined;
	withBomToken(value: JsAstToken | undefined): JsScript;
	readonly interpreterToken: string | undefined;
	withInterpreterToken(value: JsAstToken | undefined): JsScript;
	readonly directives: JsDirectiveList;
	withDirectives(value: JsDirectiveListNode): JsScript;
	readonly statements: JsStatementList;
	withStatements(value: JsStatementListNode): JsScript;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): JsScript;
	token(
		field: "bomToken" | "interpreterToken" | "eofToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSequenceExpression extends JsAstNode {
	readonly kind: "JS_SEQUENCE_EXPRESSION";
	readonly left: AnyJsExpression | undefined;
	withLeft(value: AnyJsExpression): JsSequenceExpression;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken): JsSequenceExpression;
	readonly right: AnyJsExpression | undefined;
	withRight(value: AnyJsExpression): JsSequenceExpression;
	token(field: "commaToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSetterClassMember extends JsAstNode {
	readonly kind: "JS_SETTER_CLASS_MEMBER";
	readonly modifiers: JsMethodModifierList;
	withModifiers(value: JsMethodModifierListNode): JsSetterClassMember;
	readonly setToken: string | undefined;
	withSetToken(value: JsAstToken): JsSetterClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): JsSetterClassMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsSetterClassMember;
	readonly parameter: AnyJsFormalParameter | undefined;
	withParameter(value: AnyJsFormalParameter): JsSetterClassMember;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken | undefined): JsSetterClassMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsSetterClassMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsSetterClassMember;
	token(
		field: "setToken" | "lParenToken" | "commaToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSetterObjectMember extends JsAstNode {
	readonly kind: "JS_SETTER_OBJECT_MEMBER";
	readonly setToken: string | undefined;
	withSetToken(value: JsAstToken): JsSetterObjectMember;
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): JsSetterObjectMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsSetterObjectMember;
	readonly parameter: AnyJsFormalParameter | undefined;
	withParameter(value: AnyJsFormalParameter): JsSetterObjectMember;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken | undefined): JsSetterObjectMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsSetterObjectMember;
	readonly body: JsFunctionBody | undefined;
	withBody(value: JsFunctionBody): JsSetterObjectMember;
	token(
		field: "setToken" | "lParenToken" | "commaToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsShorthandNamedImportSpecifier extends JsAstNode {
	readonly kind: "JS_SHORTHAND_NAMED_IMPORT_SPECIFIER";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): JsShorthandNamedImportSpecifier;
	readonly localName: AnyJsBinding | undefined;
	withLocalName(value: AnyJsBinding): JsShorthandNamedImportSpecifier;
	token(field: "typeToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsShorthandPropertyObjectMember extends JsAstNode {
	readonly kind: "JS_SHORTHAND_PROPERTY_OBJECT_MEMBER";
	readonly name: JsReferenceIdentifier | undefined;
	withName(value: JsReferenceIdentifier): JsShorthandPropertyObjectMember;
}
export interface JsSpread extends JsAstNode {
	readonly kind: "JS_SPREAD";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsSpread;
	readonly argument: AnyJsExpression | undefined;
	withArgument(value: AnyJsExpression): JsSpread;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsStaticInitializationBlockClassMember extends JsAstNode {
	readonly kind: "JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER";
	readonly staticToken: string | undefined;
	withStaticToken(value: JsAstToken): JsStaticInitializationBlockClassMember;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsStaticInitializationBlockClassMember;
	readonly statements: JsStatementList;
	withStatements(
		value: JsStatementListNode,
	): JsStaticInitializationBlockClassMember;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsStaticInitializationBlockClassMember;
	token(
		field: "staticToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsStaticMemberAssignment extends JsAstNode {
	readonly kind: "JS_STATIC_MEMBER_ASSIGNMENT";
	readonly object: AnyJsExpression | undefined;
	withObject(value: AnyJsExpression): JsStaticMemberAssignment;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): JsStaticMemberAssignment;
	readonly member: AnyJsName | undefined;
	withMember(value: AnyJsName): JsStaticMemberAssignment;
	token(field: "dotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsStaticMemberExpression extends JsAstNode {
	readonly kind: "JS_STATIC_MEMBER_EXPRESSION";
	readonly object: AnyJsExpression | undefined;
	withObject(value: AnyJsExpression): JsStaticMemberExpression;
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsStaticMemberExpression;
	readonly member: AnyJsName | undefined;
	withMember(value: AnyJsName): JsStaticMemberExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsStaticModifier extends JsAstNode {
	readonly kind: "JS_STATIC_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): JsStaticModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsStringLiteralExpression extends JsAstNode {
	readonly kind: "JS_STRING_LITERAL_EXPRESSION";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsStringLiteralExpression;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSuperExpression extends JsAstNode {
	readonly kind: "JS_SUPER_EXPRESSION";
	readonly superToken: string | undefined;
	withSuperToken(value: JsAstToken): JsSuperExpression;
	token(field: "superToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSvelteDeclarationRoot extends JsAstNode {
	readonly kind: "JS_SVELTE_DECLARATION_ROOT";
	readonly declaration: AnyJsSvelteDeclaration | undefined;
	withDeclaration(value: AnyJsSvelteDeclaration): JsSvelteDeclarationRoot;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsSvelteDeclarationRoot;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): JsSvelteDeclarationRoot;
	token(field: "semicolonToken" | "eofToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSvelteSnippetRoot extends JsAstNode {
	readonly kind: "JS_SVELTE_SNIPPET_ROOT";
	readonly name: AnyJsBinding | undefined;
	withName(value: AnyJsBinding): JsSvelteSnippetRoot;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): JsSvelteSnippetRoot;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): JsSvelteSnippetRoot;
	token(field: "eofToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsSwitchStatement extends JsAstNode {
	readonly kind: "JS_SWITCH_STATEMENT";
	readonly switchToken: string | undefined;
	withSwitchToken(value: JsAstToken): JsSwitchStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsSwitchStatement;
	readonly discriminant: AnyJsExpression | undefined;
	withDiscriminant(value: AnyJsExpression): JsSwitchStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsSwitchStatement;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsSwitchStatement;
	readonly cases: JsSwitchCaseList;
	withCases(value: JsSwitchCaseListNode): JsSwitchStatement;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsSwitchStatement;
	token(
		field:
			| "switchToken"
			| "lParenToken"
			| "rParenToken"
			| "lCurlyToken"
			| "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsTemplateChunkElement extends JsAstNode {
	readonly kind: "JS_TEMPLATE_CHUNK_ELEMENT";
	readonly templateChunkToken: string | undefined;
	withTemplateChunkToken(value: JsAstToken): JsTemplateChunkElement;
	token(field: "templateChunkToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsTemplateElement extends JsAstNode {
	readonly kind: "JS_TEMPLATE_ELEMENT";
	readonly dollarCurlyToken: string | undefined;
	withDollarCurlyToken(value: JsAstToken): JsTemplateElement;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsTemplateElement;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsTemplateElement;
	token(field: "dollarCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsTemplateExpression extends JsAstNode {
	readonly kind: "JS_TEMPLATE_EXPRESSION";
	readonly tag: AnyJsExpression | undefined;
	withTag(value: AnyJsExpression | undefined): JsTemplateExpression;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): JsTemplateExpression;
	readonly lTickToken: string | undefined;
	withLTickToken(value: JsAstToken): JsTemplateExpression;
	readonly elements: JsTemplateElementList;
	withElements(value: JsTemplateElementListNode): JsTemplateExpression;
	readonly rTickToken: string | undefined;
	withRTickToken(value: JsAstToken): JsTemplateExpression;
	token(field: "lTickToken" | "rTickToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsThisExpression extends JsAstNode {
	readonly kind: "JS_THIS_EXPRESSION";
	readonly thisToken: string | undefined;
	withThisToken(value: JsAstToken): JsThisExpression;
	token(field: "thisToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsThrowStatement extends JsAstNode {
	readonly kind: "JS_THROW_STATEMENT";
	readonly throwToken: string | undefined;
	withThrowToken(value: JsAstToken): JsThrowStatement;
	readonly argument: AnyJsExpression | undefined;
	withArgument(value: AnyJsExpression): JsThrowStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsThrowStatement;
	token(field: "throwToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsTryFinallyStatement extends JsAstNode {
	readonly kind: "JS_TRY_FINALLY_STATEMENT";
	readonly tryToken: string | undefined;
	withTryToken(value: JsAstToken): JsTryFinallyStatement;
	readonly body: JsBlockStatement | undefined;
	withBody(value: JsBlockStatement): JsTryFinallyStatement;
	readonly catchClause: JsCatchClause | undefined;
	withCatchClause(value: JsCatchClause | undefined): JsTryFinallyStatement;
	readonly finallyClause: JsFinallyClause | undefined;
	withFinallyClause(value: JsFinallyClause): JsTryFinallyStatement;
	token(field: "tryToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsTryStatement extends JsAstNode {
	readonly kind: "JS_TRY_STATEMENT";
	readonly tryToken: string | undefined;
	withTryToken(value: JsAstToken): JsTryStatement;
	readonly body: JsBlockStatement | undefined;
	withBody(value: JsBlockStatement): JsTryStatement;
	readonly catchClause: JsCatchClause | undefined;
	withCatchClause(value: JsCatchClause): JsTryStatement;
	token(field: "tryToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsUnaryExpression extends JsAstNode {
	readonly kind: "JS_UNARY_EXPRESSION";
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): JsUnaryExpression;
	readonly argument: AnyJsExpression | undefined;
	withArgument(value: AnyJsExpression): JsUnaryExpression;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsVariableDeclaration extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATION";
	readonly awaitToken: string | undefined;
	withAwaitToken(value: JsAstToken | undefined): JsVariableDeclaration;
	readonly kindToken: string | undefined;
	withKindToken(value: JsAstToken): JsVariableDeclaration;
	readonly declarators: JsVariableDeclaratorList;
	withDeclarators(value: JsVariableDeclaratorListNode): JsVariableDeclaration;
	token(field: "awaitToken" | "kindToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsVariableDeclarationClause extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATION_CLAUSE";
	readonly declaration: JsVariableDeclaration | undefined;
	withDeclaration(value: JsVariableDeclaration): JsVariableDeclarationClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): JsVariableDeclarationClause;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsVariableDeclarator extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATOR";
	readonly id: AnyJsBindingPattern | undefined;
	withId(value: AnyJsBindingPattern): JsVariableDeclarator;
	readonly variableAnnotation: AnyTsVariableAnnotation | undefined;
	withVariableAnnotation(
		value: AnyTsVariableAnnotation | undefined,
	): JsVariableDeclarator;
	readonly initializer: JsInitializerClause | undefined;
	withInitializer(value: JsInitializerClause | undefined): JsVariableDeclarator;
}
export interface JsVariableStatement extends JsAstNode {
	readonly kind: "JS_VARIABLE_STATEMENT";
	readonly declaration: JsVariableDeclaration | undefined;
	withDeclaration(value: JsVariableDeclaration): JsVariableStatement;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): JsVariableStatement;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsWhileStatement extends JsAstNode {
	readonly kind: "JS_WHILE_STATEMENT";
	readonly whileToken: string | undefined;
	withWhileToken(value: JsAstToken): JsWhileStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsWhileStatement;
	readonly test: AnyJsExpression | undefined;
	withTest(value: AnyJsExpression): JsWhileStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsWhileStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsWhileStatement;
	token(
		field: "whileToken" | "lParenToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsWithStatement extends JsAstNode {
	readonly kind: "JS_WITH_STATEMENT";
	readonly withToken: string | undefined;
	withWithToken(value: JsAstToken): JsWithStatement;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): JsWithStatement;
	readonly object: AnyJsExpression | undefined;
	withObject(value: AnyJsExpression): JsWithStatement;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): JsWithStatement;
	readonly body: AnyJsStatement | undefined;
	withBody(value: AnyJsStatement): JsWithStatement;
	token(
		field: "withToken" | "lParenToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsYieldArgument extends JsAstNode {
	readonly kind: "JS_YIELD_ARGUMENT";
	readonly starToken: string | undefined;
	withStarToken(value: JsAstToken | undefined): JsYieldArgument;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsYieldArgument;
	token(field: "starToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsYieldExpression extends JsAstNode {
	readonly kind: "JS_YIELD_EXPRESSION";
	readonly yieldToken: string | undefined;
	withYieldToken(value: JsAstToken): JsYieldExpression;
	readonly argument: JsYieldArgument | undefined;
	withArgument(value: JsYieldArgument | undefined): JsYieldExpression;
	token(field: "yieldToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxAttribute extends JsAstNode {
	readonly kind: "JSX_ATTRIBUTE";
	readonly name: AnyJsxAttributeName | undefined;
	withName(value: AnyJsxAttributeName): JsxAttribute;
	readonly initializer: JsxAttributeInitializerClause | undefined;
	withInitializer(
		value: JsxAttributeInitializerClause | undefined,
	): JsxAttribute;
}
export interface JsxAttributeInitializerClause extends JsAstNode {
	readonly kind: "JSX_ATTRIBUTE_INITIALIZER_CLAUSE";
	readonly eqToken: string | undefined;
	withEqToken(value: JsAstToken): JsxAttributeInitializerClause;
	readonly value: AnyJsxAttributeValue | undefined;
	withValue(value: AnyJsxAttributeValue): JsxAttributeInitializerClause;
	token(field: "eqToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxClosingElement extends JsAstNode {
	readonly kind: "JSX_CLOSING_ELEMENT";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): JsxClosingElement;
	readonly slashToken: string | undefined;
	withSlashToken(value: JsAstToken): JsxClosingElement;
	readonly name: AnyJsxElementName | undefined;
	withName(value: AnyJsxElementName): JsxClosingElement;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): JsxClosingElement;
	token(
		field: "lAngleToken" | "slashToken" | "rAngleToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxClosingFragment extends JsAstNode {
	readonly kind: "JSX_CLOSING_FRAGMENT";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): JsxClosingFragment;
	readonly slashToken: string | undefined;
	withSlashToken(value: JsAstToken): JsxClosingFragment;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): JsxClosingFragment;
	token(
		field: "lAngleToken" | "slashToken" | "rAngleToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxElement extends JsAstNode {
	readonly kind: "JSX_ELEMENT";
	readonly openingElement: JsxOpeningElement | undefined;
	withOpeningElement(value: JsxOpeningElement): JsxElement;
	readonly elements: JsxChildList;
	withElements(value: JsxChildListNode): JsxElement;
	readonly closingElement: JsxClosingElement | undefined;
	withClosingElement(value: JsxClosingElement): JsxElement;
}
export interface JsxExpressionAttributeValue extends JsAstNode {
	readonly kind: "JSX_EXPRESSION_ATTRIBUTE_VALUE";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsxExpressionAttributeValue;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsxExpressionAttributeValue;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsxExpressionAttributeValue;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxExpressionChild extends JsAstNode {
	readonly kind: "JSX_EXPRESSION_CHILD";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsxExpressionChild;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression | undefined): JsxExpressionChild;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsxExpressionChild;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxFragment extends JsAstNode {
	readonly kind: "JSX_FRAGMENT";
	readonly openingFragment: JsxOpeningFragment | undefined;
	withOpeningFragment(value: JsxOpeningFragment): JsxFragment;
	readonly elements: JsxChildList;
	withElements(value: JsxChildListNode): JsxFragment;
	readonly closingFragment: JsxClosingFragment | undefined;
	withClosingFragment(value: JsxClosingFragment): JsxFragment;
}
export interface JsxMemberName extends JsAstNode {
	readonly kind: "JSX_MEMBER_NAME";
	readonly object: AnyJsxObjectName | undefined;
	withObject(value: AnyJsxObjectName): JsxMemberName;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): JsxMemberName;
	readonly member: JsName | undefined;
	withMember(value: JsName): JsxMemberName;
	token(field: "dotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxName extends JsAstNode {
	readonly kind: "JSX_NAME";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsxName;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxNamespaceName extends JsAstNode {
	readonly kind: "JSX_NAMESPACE_NAME";
	readonly namespace: JsxName | undefined;
	withNamespace(value: JsxName): JsxNamespaceName;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): JsxNamespaceName;
	readonly name: JsxName | undefined;
	withName(value: JsxName): JsxNamespaceName;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxOpeningElement extends JsAstNode {
	readonly kind: "JSX_OPENING_ELEMENT";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): JsxOpeningElement;
	readonly name: AnyJsxElementName | undefined;
	withName(value: AnyJsxElementName): JsxOpeningElement;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): JsxOpeningElement;
	readonly attributes: JsxAttributeList;
	withAttributes(value: JsxAttributeListNode): JsxOpeningElement;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): JsxOpeningElement;
	token(field: "lAngleToken" | "rAngleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxOpeningFragment extends JsAstNode {
	readonly kind: "JSX_OPENING_FRAGMENT";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): JsxOpeningFragment;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): JsxOpeningFragment;
	token(field: "lAngleToken" | "rAngleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxReferenceIdentifier extends JsAstNode {
	readonly kind: "JSX_REFERENCE_IDENTIFIER";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsxReferenceIdentifier;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxSelfClosingElement extends JsAstNode {
	readonly kind: "JSX_SELF_CLOSING_ELEMENT";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): JsxSelfClosingElement;
	readonly name: AnyJsxElementName | undefined;
	withName(value: AnyJsxElementName): JsxSelfClosingElement;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): JsxSelfClosingElement;
	readonly attributes: JsxAttributeList;
	withAttributes(value: JsxAttributeListNode): JsxSelfClosingElement;
	readonly slashToken: string | undefined;
	withSlashToken(value: JsAstToken | undefined): JsxSelfClosingElement;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): JsxSelfClosingElement;
	token(
		field: "lAngleToken" | "slashToken" | "rAngleToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxShorthandAttribute extends JsAstNode {
	readonly kind: "JSX_SHORTHAND_ATTRIBUTE";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsxShorthandAttribute;
	readonly name: JsReferenceIdentifier | undefined;
	withName(value: JsReferenceIdentifier): JsxShorthandAttribute;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsxShorthandAttribute;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxSpreadAttribute extends JsAstNode {
	readonly kind: "JSX_SPREAD_ATTRIBUTE";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsxSpreadAttribute;
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsxSpreadAttribute;
	readonly argument: AnyJsExpression | undefined;
	withArgument(value: AnyJsExpression): JsxSpreadAttribute;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsxSpreadAttribute;
	token(
		field: "lCurlyToken" | "dotdotdotToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxSpreadChild extends JsAstNode {
	readonly kind: "JSX_SPREAD_CHILD";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): JsxSpreadChild;
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): JsxSpreadChild;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): JsxSpreadChild;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): JsxSpreadChild;
	token(
		field: "lCurlyToken" | "dotdotdotToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxString extends JsAstNode {
	readonly kind: "JSX_STRING";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsxString;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface JsxTagExpression extends JsAstNode {
	readonly kind: "JSX_TAG_EXPRESSION";
	readonly tag: AnyJsxTag | undefined;
	withTag(value: AnyJsxTag): JsxTagExpression;
}
export interface JsxText extends JsAstNode {
	readonly kind: "JSX_TEXT";
	readonly valueToken: string | undefined;
	withValueToken(value: JsAstToken): JsxText;
	token(field: "valueToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAbstractModifier extends JsAstNode {
	readonly kind: "TS_ABSTRACT_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsAbstractModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAccessibilityModifier extends JsAstNode {
	readonly kind: "TS_ACCESSIBILITY_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsAccessibilityModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAnyType extends JsAstNode {
	readonly kind: "TS_ANY_TYPE";
	readonly anyToken: string | undefined;
	withAnyToken(value: JsAstToken): TsAnyType;
	token(field: "anyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsArrayType extends JsAstNode {
	readonly kind: "TS_ARRAY_TYPE";
	readonly elementType: AnyTsType | undefined;
	withElementType(value: AnyTsType): TsArrayType;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): TsArrayType;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): TsArrayType;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAsAssignment extends JsAstNode {
	readonly kind: "TS_AS_ASSIGNMENT";
	readonly assignment: AnyJsAssignment | undefined;
	withAssignment(value: AnyJsAssignment): TsAsAssignment;
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): TsAsAssignment;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsAsAssignment;
	token(field: "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAsExpression extends JsAstNode {
	readonly kind: "TS_AS_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): TsAsExpression;
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): TsAsExpression;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsAsExpression;
	token(field: "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAssertsCondition extends JsAstNode {
	readonly kind: "TS_ASSERTS_CONDITION";
	readonly isToken: string | undefined;
	withIsToken(value: JsAstToken): TsAssertsCondition;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsAssertsCondition;
	token(field: "isToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsAssertsReturnType extends JsAstNode {
	readonly kind: "TS_ASSERTS_RETURN_TYPE";
	readonly assertsToken: string | undefined;
	withAssertsToken(value: JsAstToken): TsAssertsReturnType;
	readonly parameterName: AnyTsTypePredicateParameterName | undefined;
	withParameterName(
		value: AnyTsTypePredicateParameterName,
	): TsAssertsReturnType;
	readonly predicate: TsAssertsCondition | undefined;
	withPredicate(value: TsAssertsCondition | undefined): TsAssertsReturnType;
	token(field: "assertsToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsBigintLiteralType extends JsAstNode {
	readonly kind: "TS_BIGINT_LITERAL_TYPE";
	readonly minusToken: string | undefined;
	withMinusToken(value: JsAstToken | undefined): TsBigintLiteralType;
	readonly literalToken: string | undefined;
	withLiteralToken(value: JsAstToken): TsBigintLiteralType;
	token(field: "minusToken" | "literalToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsBigintType extends JsAstNode {
	readonly kind: "TS_BIGINT_TYPE";
	readonly bigintToken: string | undefined;
	withBigintToken(value: JsAstToken): TsBigintType;
	token(field: "bigintToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsBooleanLiteralType extends JsAstNode {
	readonly kind: "TS_BOOLEAN_LITERAL_TYPE";
	readonly literal: string | undefined;
	withLiteral(value: JsAstToken): TsBooleanLiteralType;
	token(field: "literal"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsBooleanType extends JsAstNode {
	readonly kind: "TS_BOOLEAN_TYPE";
	readonly booleanToken: string | undefined;
	withBooleanToken(value: JsAstToken): TsBooleanType;
	token(field: "booleanToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsCallSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_CALL_SIGNATURE_TYPE_MEMBER";
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsCallSignatureTypeMember;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsCallSignatureTypeMember;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): TsCallSignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(value: JsAstToken | undefined): TsCallSignatureTypeMember;
	token(field: "separatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsConditionalType extends JsAstNode {
	readonly kind: "TS_CONDITIONAL_TYPE";
	readonly checkType: AnyTsType | undefined;
	withCheckType(value: AnyTsType): TsConditionalType;
	readonly extendsToken: string | undefined;
	withExtendsToken(value: JsAstToken): TsConditionalType;
	readonly extendsType: AnyTsType | undefined;
	withExtendsType(value: AnyTsType): TsConditionalType;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken): TsConditionalType;
	readonly trueType: AnyTsType | undefined;
	withTrueType(value: AnyTsType): TsConditionalType;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): TsConditionalType;
	readonly falseType: AnyTsType | undefined;
	withFalseType(value: AnyTsType): TsConditionalType;
	token(
		field: "extendsToken" | "questionMarkToken" | "colonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsConstModifier extends JsAstNode {
	readonly kind: "TS_CONST_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsConstModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsConstructSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER";
	readonly newToken: string | undefined;
	withNewToken(value: JsAstToken): TsConstructSignatureTypeMember;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsConstructSignatureTypeMember;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsConstructSignatureTypeMember;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(
		value: TsTypeAnnotation | undefined,
	): TsConstructSignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(
		value: JsAstToken | undefined,
	): TsConstructSignatureTypeMember;
	token(field: "newToken" | "separatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsConstructorSignatureClassMember extends JsAstNode {
	readonly kind: "TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: JsConstructorModifierList;
	withModifiers(
		value: JsConstructorModifierListNode,
	): TsConstructorSignatureClassMember;
	readonly name: JsLiteralMemberName | undefined;
	withName(value: JsLiteralMemberName): TsConstructorSignatureClassMember;
	readonly parameters: JsConstructorParameters | undefined;
	withParameters(
		value: JsConstructorParameters,
	): TsConstructorSignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsConstructorSignatureClassMember;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsConstructorType extends JsAstNode {
	readonly kind: "TS_CONSTRUCTOR_TYPE";
	readonly abstractToken: string | undefined;
	withAbstractToken(value: JsAstToken | undefined): TsConstructorType;
	readonly newToken: string | undefined;
	withNewToken(value: JsAstToken): TsConstructorType;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): TsConstructorType;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsConstructorType;
	readonly fatArrowToken: string | undefined;
	withFatArrowToken(value: JsAstToken): TsConstructorType;
	readonly returnType: AnyTsType | undefined;
	withReturnType(value: AnyTsType): TsConstructorType;
	token(
		field: "abstractToken" | "newToken" | "fatArrowToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDeclarationModule extends JsAstNode {
	readonly kind: "TS_DECLARATION_MODULE";
	readonly bomToken: string | undefined;
	withBomToken(value: JsAstToken | undefined): TsDeclarationModule;
	readonly interpreterToken: string | undefined;
	withInterpreterToken(value: JsAstToken | undefined): TsDeclarationModule;
	readonly directives: JsDirectiveList;
	withDirectives(value: JsDirectiveListNode): TsDeclarationModule;
	readonly items: JsModuleItemList;
	withItems(value: JsModuleItemListNode): TsDeclarationModule;
	readonly eofToken: string | undefined;
	withEofToken(value: JsAstToken): TsDeclarationModule;
	token(
		field: "bomToken" | "interpreterToken" | "eofToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDeclareFunctionDeclaration extends JsAstNode {
	readonly kind: "TS_DECLARE_FUNCTION_DECLARATION";
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): TsDeclareFunctionDeclaration;
	readonly functionToken: string | undefined;
	withFunctionToken(value: JsAstToken): TsDeclareFunctionDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding): TsDeclareFunctionDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsDeclareFunctionDeclaration;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsDeclareFunctionDeclaration;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): TsDeclareFunctionDeclaration;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsDeclareFunctionDeclaration;
	token(
		field: "asyncToken" | "functionToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDeclareFunctionExportDefaultDeclaration extends JsAstNode {
	readonly kind: "TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION";
	readonly asyncToken: string | undefined;
	withAsyncToken(
		value: JsAstToken | undefined,
	): TsDeclareFunctionExportDefaultDeclaration;
	readonly functionToken: string | undefined;
	withFunctionToken(
		value: JsAstToken,
	): TsDeclareFunctionExportDefaultDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(
		value: AnyJsBinding | undefined,
	): TsDeclareFunctionExportDefaultDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsDeclareFunctionExportDefaultDeclaration;
	readonly parameters: JsParameters | undefined;
	withParameters(
		value: JsParameters,
	): TsDeclareFunctionExportDefaultDeclaration;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): TsDeclareFunctionExportDefaultDeclaration;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsDeclareFunctionExportDefaultDeclaration;
	token(
		field: "asyncToken" | "functionToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDeclareModifier extends JsAstNode {
	readonly kind: "TS_DECLARE_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsDeclareModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDeclareStatement extends JsAstNode {
	readonly kind: "TS_DECLARE_STATEMENT";
	readonly declareToken: string | undefined;
	withDeclareToken(value: JsAstToken): TsDeclareStatement;
	readonly declaration: AnyJsDeclarationClause | undefined;
	withDeclaration(value: AnyJsDeclarationClause): TsDeclareStatement;
	token(field: "declareToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDefaultTypeClause extends JsAstNode {
	readonly kind: "TS_DEFAULT_TYPE_CLAUSE";
	readonly eqToken: string | undefined;
	withEqToken(value: JsAstToken): TsDefaultTypeClause;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsDefaultTypeClause;
	token(field: "eqToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDefinitePropertyAnnotation extends JsAstNode {
	readonly kind: "TS_DEFINITE_PROPERTY_ANNOTATION";
	readonly exclToken: string | undefined;
	withExclToken(value: JsAstToken): TsDefinitePropertyAnnotation;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation): TsDefinitePropertyAnnotation;
	token(field: "exclToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsDefiniteVariableAnnotation extends JsAstNode {
	readonly kind: "TS_DEFINITE_VARIABLE_ANNOTATION";
	readonly exclToken: string | undefined;
	withExclToken(value: JsAstToken): TsDefiniteVariableAnnotation;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation): TsDefiniteVariableAnnotation;
	token(field: "exclToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsEmptyExternalModuleDeclarationBody extends JsAstNode {
	readonly kind: "TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY";
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken): TsEmptyExternalModuleDeclarationBody;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsEnumDeclaration extends JsAstNode {
	readonly kind: "TS_ENUM_DECLARATION";
	readonly constToken: string | undefined;
	withConstToken(value: JsAstToken | undefined): TsEnumDeclaration;
	readonly enumToken: string | undefined;
	withEnumToken(value: JsAstToken): TsEnumDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding): TsEnumDeclaration;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsEnumDeclaration;
	readonly members: TsEnumMemberList;
	withMembers(value: TsEnumMemberListNode): TsEnumDeclaration;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsEnumDeclaration;
	token(
		field: "constToken" | "enumToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsEnumMember extends JsAstNode {
	readonly kind: "TS_ENUM_MEMBER";
	readonly name: AnyTsEnumMemberName | undefined;
	withName(value: AnyTsEnumMemberName): TsEnumMember;
	readonly initializer: JsInitializerClause | undefined;
	withInitializer(value: JsInitializerClause | undefined): TsEnumMember;
}
export interface TsExportAsNamespaceClause extends JsAstNode {
	readonly kind: "TS_EXPORT_AS_NAMESPACE_CLAUSE";
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): TsExportAsNamespaceClause;
	readonly namespaceToken: string | undefined;
	withNamespaceToken(value: JsAstToken): TsExportAsNamespaceClause;
	readonly name: JsName | undefined;
	withName(value: JsName): TsExportAsNamespaceClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): TsExportAsNamespaceClause;
	token(
		field: "asToken" | "namespaceToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsExportAssignmentClause extends JsAstNode {
	readonly kind: "TS_EXPORT_ASSIGNMENT_CLAUSE";
	readonly eqToken: string | undefined;
	withEqToken(value: JsAstToken): TsExportAssignmentClause;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): TsExportAssignmentClause;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): TsExportAssignmentClause;
	token(field: "eqToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsExportDeclareClause extends JsAstNode {
	readonly kind: "TS_EXPORT_DECLARE_CLAUSE";
	readonly declareToken: string | undefined;
	withDeclareToken(value: JsAstToken): TsExportDeclareClause;
	readonly declaration: AnyJsDeclarationClause | undefined;
	withDeclaration(value: AnyJsDeclarationClause): TsExportDeclareClause;
	token(field: "declareToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsExtendsClause extends JsAstNode {
	readonly kind: "TS_EXTENDS_CLAUSE";
	readonly extendsToken: string | undefined;
	withExtendsToken(value: JsAstToken): TsExtendsClause;
	readonly types: TsTypeList;
	withTypes(value: TsTypeListNode): TsExtendsClause;
	token(field: "extendsToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsExternalModuleDeclaration extends JsAstNode {
	readonly kind: "TS_EXTERNAL_MODULE_DECLARATION";
	readonly moduleToken: string | undefined;
	withModuleToken(value: JsAstToken): TsExternalModuleDeclaration;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): TsExternalModuleDeclaration;
	readonly body: AnyTsExternalModuleDeclarationBody | undefined;
	withBody(
		value: AnyTsExternalModuleDeclarationBody | undefined,
	): TsExternalModuleDeclaration;
	token(field: "moduleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsExternalModuleReference extends JsAstNode {
	readonly kind: "TS_EXTERNAL_MODULE_REFERENCE";
	readonly requireToken: string | undefined;
	withRequireToken(value: JsAstToken): TsExternalModuleReference;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsExternalModuleReference;
	readonly source: AnyJsModuleSource | undefined;
	withSource(value: AnyJsModuleSource): TsExternalModuleReference;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsExternalModuleReference;
	token(
		field: "requireToken" | "lParenToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsFunctionType extends JsAstNode {
	readonly kind: "TS_FUNCTION_TYPE";
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(value: TsTypeParameters | undefined): TsFunctionType;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsFunctionType;
	readonly fatArrowToken: string | undefined;
	withFatArrowToken(value: JsAstToken): TsFunctionType;
	readonly returnType: AnyTsReturnType | undefined;
	withReturnType(value: AnyTsReturnType): TsFunctionType;
	token(field: "fatArrowToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsGetterSignatureClassMember extends JsAstNode {
	readonly kind: "TS_GETTER_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsMethodSignatureModifierList;
	withModifiers(
		value: TsMethodSignatureModifierListNode,
	): TsGetterSignatureClassMember;
	readonly getToken: string | undefined;
	withGetToken(value: JsAstToken): TsGetterSignatureClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): TsGetterSignatureClassMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsGetterSignatureClassMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsGetterSignatureClassMember;
	readonly returnType: TsTypeAnnotation | undefined;
	withReturnType(
		value: TsTypeAnnotation | undefined,
	): TsGetterSignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsGetterSignatureClassMember;
	token(
		field: "getToken" | "lParenToken" | "rParenToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsGetterSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_GETTER_SIGNATURE_TYPE_MEMBER";
	readonly getToken: string | undefined;
	withGetToken(value: JsAstToken): TsGetterSignatureTypeMember;
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): TsGetterSignatureTypeMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsGetterSignatureTypeMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsGetterSignatureTypeMember;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(
		value: TsTypeAnnotation | undefined,
	): TsGetterSignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(
		value: JsAstToken | undefined,
	): TsGetterSignatureTypeMember;
	token(
		field: "getToken" | "lParenToken" | "rParenToken" | "separatorToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsGlobalDeclaration extends JsAstNode {
	readonly kind: "TS_GLOBAL_DECLARATION";
	readonly globalToken: string | undefined;
	withGlobalToken(value: JsAstToken): TsGlobalDeclaration;
	readonly body: TsModuleBlock | undefined;
	withBody(value: TsModuleBlock): TsGlobalDeclaration;
	token(field: "globalToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsIdentifierBinding extends JsAstNode {
	readonly kind: "TS_IDENTIFIER_BINDING";
	readonly nameToken: string | undefined;
	withNameToken(value: JsAstToken): TsIdentifierBinding;
	token(field: "nameToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImplementsClause extends JsAstNode {
	readonly kind: "TS_IMPLEMENTS_CLAUSE";
	readonly implementsToken: string | undefined;
	withImplementsToken(value: JsAstToken): TsImplementsClause;
	readonly types: TsTypeList;
	withTypes(value: TsTypeListNode): TsImplementsClause;
	token(field: "implementsToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImportEqualsDeclaration extends JsAstNode {
	readonly kind: "TS_IMPORT_EQUALS_DECLARATION";
	readonly importToken: string | undefined;
	withImportToken(value: JsAstToken): TsImportEqualsDeclaration;
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken | undefined): TsImportEqualsDeclaration;
	readonly id: AnyJsBinding | undefined;
	withId(value: AnyJsBinding): TsImportEqualsDeclaration;
	readonly eqToken: string | undefined;
	withEqToken(value: JsAstToken): TsImportEqualsDeclaration;
	readonly moduleReference: AnyTsModuleReference | undefined;
	withModuleReference(value: AnyTsModuleReference): TsImportEqualsDeclaration;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): TsImportEqualsDeclaration;
	token(
		field: "importToken" | "typeToken" | "eqToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImportType extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE";
	readonly typeofToken: string | undefined;
	withTypeofToken(value: JsAstToken | undefined): TsImportType;
	readonly importToken: string | undefined;
	withImportToken(value: JsAstToken): TsImportType;
	readonly arguments: TsImportTypeArguments | undefined;
	withArguments(value: TsImportTypeArguments): TsImportType;
	readonly qualifierClause: TsImportTypeQualifier | undefined;
	withQualifierClause(value: TsImportTypeQualifier | undefined): TsImportType;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): TsImportType;
	token(field: "typeofToken" | "importToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImportTypeArguments extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_ARGUMENTS";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsImportTypeArguments;
	readonly argument: AnyTsType | undefined;
	withArgument(value: AnyTsType): TsImportTypeArguments;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken | undefined): TsImportTypeArguments;
	readonly tsImportTypeAssertionBlock: TsImportTypeAssertionBlock | undefined;
	withTsImportTypeAssertionBlock(
		value: TsImportTypeAssertionBlock | undefined,
	): TsImportTypeArguments;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsImportTypeArguments;
	token(
		field: "lParenToken" | "commaToken" | "rParenToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImportTypeAssertion extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_ASSERTION";
	readonly withToken: string | undefined;
	withWithToken(value: JsAstToken): TsImportTypeAssertion;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): TsImportTypeAssertion;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsImportTypeAssertion;
	readonly assertions: JsImportAssertionEntryList;
	withAssertions(value: JsImportAssertionEntryListNode): TsImportTypeAssertion;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsImportTypeAssertion;
	token(
		field: "withToken" | "colonToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImportTypeAssertionBlock extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_ASSERTION_BLOCK";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsImportTypeAssertionBlock;
	readonly typeAssertion: TsImportTypeAssertion | undefined;
	withTypeAssertion(value: TsImportTypeAssertion): TsImportTypeAssertionBlock;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsImportTypeAssertionBlock;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsImportTypeQualifier extends JsAstNode {
	readonly kind: "TS_IMPORT_TYPE_QUALIFIER";
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): TsImportTypeQualifier;
	readonly right: AnyTsName | undefined;
	withRight(value: AnyTsName): TsImportTypeQualifier;
	token(field: "dotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsInModifier extends JsAstNode {
	readonly kind: "TS_IN_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsInModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsIndexSignatureClassMember extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsIndexSignatureModifierList;
	withModifiers(
		value: TsIndexSignatureModifierListNode,
	): TsIndexSignatureClassMember;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): TsIndexSignatureClassMember;
	readonly parameter: TsIndexSignatureParameter | undefined;
	withParameter(value: TsIndexSignatureParameter): TsIndexSignatureClassMember;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): TsIndexSignatureClassMember;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation): TsIndexSignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsIndexSignatureClassMember;
	token(
		field: "lBrackToken" | "rBrackToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsIndexSignatureParameter extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_PARAMETER";
	readonly binding: JsIdentifierBinding | undefined;
	withBinding(value: JsIdentifierBinding): TsIndexSignatureParameter;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation): TsIndexSignatureParameter;
}
export interface TsIndexSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_TYPE_MEMBER";
	readonly readonlyToken: string | undefined;
	withReadonlyToken(value: JsAstToken | undefined): TsIndexSignatureTypeMember;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): TsIndexSignatureTypeMember;
	readonly parameter: TsIndexSignatureParameter | undefined;
	withParameter(value: TsIndexSignatureParameter): TsIndexSignatureTypeMember;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): TsIndexSignatureTypeMember;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation): TsIndexSignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(value: JsAstToken | undefined): TsIndexSignatureTypeMember;
	token(
		field: "readonlyToken" | "lBrackToken" | "rBrackToken" | "separatorToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsIndexedAccessType extends JsAstNode {
	readonly kind: "TS_INDEXED_ACCESS_TYPE";
	readonly objectType: AnyTsType | undefined;
	withObjectType(value: AnyTsType): TsIndexedAccessType;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): TsIndexedAccessType;
	readonly indexType: AnyTsType | undefined;
	withIndexType(value: AnyTsType): TsIndexedAccessType;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): TsIndexedAccessType;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsInferType extends JsAstNode {
	readonly kind: "TS_INFER_TYPE";
	readonly inferToken: string | undefined;
	withInferToken(value: JsAstToken): TsInferType;
	readonly name: TsTypeParameterName | undefined;
	withName(value: TsTypeParameterName): TsInferType;
	readonly constraint: TsTypeConstraintClause | undefined;
	withConstraint(value: TsTypeConstraintClause | undefined): TsInferType;
	token(field: "inferToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsInitializedPropertySignatureClassMember extends JsAstNode {
	readonly kind: "TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsPropertySignatureModifierList;
	withModifiers(
		value: TsPropertySignatureModifierListNode,
	): TsInitializedPropertySignatureClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(
		value: AnyJsClassMemberName,
	): TsInitializedPropertySignatureClassMember;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(
		value: JsAstToken | undefined,
	): TsInitializedPropertySignatureClassMember;
	readonly value: JsInitializerClause | undefined;
	withValue(
		value: JsInitializerClause,
	): TsInitializedPropertySignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsInitializedPropertySignatureClassMember;
	token(field: "questionMarkToken" | "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsInstantiationExpression extends JsAstNode {
	readonly kind: "TS_INSTANTIATION_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): TsInstantiationExpression;
	readonly arguments: TsTypeArguments | undefined;
	withArguments(value: TsTypeArguments): TsInstantiationExpression;
}
export interface TsInterfaceDeclaration extends JsAstNode {
	readonly kind: "TS_INTERFACE_DECLARATION";
	readonly interfaceToken: string | undefined;
	withInterfaceToken(value: JsAstToken): TsInterfaceDeclaration;
	readonly id: AnyTsIdentifierBinding | undefined;
	withId(value: AnyTsIdentifierBinding): TsInterfaceDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsInterfaceDeclaration;
	readonly extendsClause: TsExtendsClause | undefined;
	withExtendsClause(value: TsExtendsClause | undefined): TsInterfaceDeclaration;
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsInterfaceDeclaration;
	readonly members: TsTypeMemberList;
	withMembers(value: TsTypeMemberListNode): TsInterfaceDeclaration;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsInterfaceDeclaration;
	token(
		field: "interfaceToken" | "lCurlyToken" | "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsIntersectionType extends JsAstNode {
	readonly kind: "TS_INTERSECTION_TYPE";
	readonly leadingSeparatorToken: string | undefined;
	withLeadingSeparatorToken(value: JsAstToken | undefined): TsIntersectionType;
	readonly types: TsIntersectionTypeElementList;
	withTypes(value: TsIntersectionTypeElementListNode): TsIntersectionType;
	token(field: "leadingSeparatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsLiteralEnumMemberName extends JsAstNode {
	readonly kind: "TS_LITERAL_ENUM_MEMBER_NAME";
	readonly value: string | undefined;
	withValue(value: JsAstToken): TsLiteralEnumMemberName;
	token(field: "value"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsMappedType extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsMappedType;
	readonly readonlyModifier: TsMappedTypeReadonlyModifierClause | undefined;
	withReadonlyModifier(
		value: TsMappedTypeReadonlyModifierClause | undefined,
	): TsMappedType;
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): TsMappedType;
	readonly propertyName: TsTypeParameterName | undefined;
	withPropertyName(value: TsTypeParameterName): TsMappedType;
	readonly inToken: string | undefined;
	withInToken(value: JsAstToken): TsMappedType;
	readonly keysType: AnyTsType | undefined;
	withKeysType(value: AnyTsType): TsMappedType;
	readonly asClause: TsMappedTypeAsClause | undefined;
	withAsClause(value: TsMappedTypeAsClause | undefined): TsMappedType;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): TsMappedType;
	readonly optionalModifier: TsMappedTypeOptionalModifierClause | undefined;
	withOptionalModifier(
		value: TsMappedTypeOptionalModifierClause | undefined,
	): TsMappedType;
	readonly mappedType: TsTypeAnnotation | undefined;
	withMappedType(value: TsTypeAnnotation | undefined): TsMappedType;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): TsMappedType;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsMappedType;
	token(
		field:
			| "lCurlyToken"
			| "lBrackToken"
			| "inToken"
			| "rBrackToken"
			| "semicolonToken"
			| "rCurlyToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsMappedTypeAsClause extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE_AS_CLAUSE";
	readonly asToken: string | undefined;
	withAsToken(value: JsAstToken): TsMappedTypeAsClause;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsMappedTypeAsClause;
	token(field: "asToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsMappedTypeOptionalModifierClause extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE";
	readonly operatorToken: string | undefined;
	withOperatorToken(
		value: JsAstToken | undefined,
	): TsMappedTypeOptionalModifierClause;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken): TsMappedTypeOptionalModifierClause;
	token(field: "operatorToken" | "questionMarkToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsMappedTypeReadonlyModifierClause extends JsAstNode {
	readonly kind: "TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE";
	readonly operatorToken: string | undefined;
	withOperatorToken(
		value: JsAstToken | undefined,
	): TsMappedTypeReadonlyModifierClause;
	readonly readonlyToken: string | undefined;
	withReadonlyToken(value: JsAstToken): TsMappedTypeReadonlyModifierClause;
	token(field: "operatorToken" | "readonlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsMethodSignatureClassMember extends JsAstNode {
	readonly kind: "TS_METHOD_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsMethodSignatureModifierList;
	withModifiers(
		value: TsMethodSignatureModifierListNode,
	): TsMethodSignatureClassMember;
	readonly asyncToken: string | undefined;
	withAsyncToken(value: JsAstToken | undefined): TsMethodSignatureClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): TsMethodSignatureClassMember;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(
		value: JsAstToken | undefined,
	): TsMethodSignatureClassMember;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsMethodSignatureClassMember;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsMethodSignatureClassMember;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): TsMethodSignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsMethodSignatureClassMember;
	token(
		field: "asyncToken" | "questionMarkToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsMethodSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_METHOD_SIGNATURE_TYPE_MEMBER";
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): TsMethodSignatureTypeMember;
	readonly optionalToken: string | undefined;
	withOptionalToken(value: JsAstToken | undefined): TsMethodSignatureTypeMember;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsMethodSignatureTypeMember;
	readonly parameters: JsParameters | undefined;
	withParameters(value: JsParameters): TsMethodSignatureTypeMember;
	readonly returnTypeAnnotation: TsReturnTypeAnnotation | undefined;
	withReturnTypeAnnotation(
		value: TsReturnTypeAnnotation | undefined,
	): TsMethodSignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(
		value: JsAstToken | undefined,
	): TsMethodSignatureTypeMember;
	token(field: "optionalToken" | "separatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsModuleBlock extends JsAstNode {
	readonly kind: "TS_MODULE_BLOCK";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsModuleBlock;
	readonly items: JsModuleItemList;
	withItems(value: JsModuleItemListNode): TsModuleBlock;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsModuleBlock;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsModuleDeclaration extends JsAstNode {
	readonly kind: "TS_MODULE_DECLARATION";
	readonly moduleOrNamespace: string | undefined;
	withModuleOrNamespace(value: JsAstToken): TsModuleDeclaration;
	readonly name: AnyTsModuleName | undefined;
	withName(value: AnyTsModuleName): TsModuleDeclaration;
	readonly body: TsModuleBlock | undefined;
	withBody(value: TsModuleBlock): TsModuleDeclaration;
	token(field: "moduleOrNamespace"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNamedTupleTypeElement extends JsAstNode {
	readonly kind: "TS_NAMED_TUPLE_TYPE_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken | undefined): TsNamedTupleTypeElement;
	readonly name: JsName | undefined;
	withName(value: JsName): TsNamedTupleTypeElement;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken | undefined): TsNamedTupleTypeElement;
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): TsNamedTupleTypeElement;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsNamedTupleTypeElement;
	token(
		field: "dotdotdotToken" | "questionMarkToken" | "colonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNeverType extends JsAstNode {
	readonly kind: "TS_NEVER_TYPE";
	readonly neverToken: string | undefined;
	withNeverToken(value: JsAstToken): TsNeverType;
	token(field: "neverToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNonNullAssertionAssignment extends JsAstNode {
	readonly kind: "TS_NON_NULL_ASSERTION_ASSIGNMENT";
	readonly assignment: AnyJsAssignment | undefined;
	withAssignment(value: AnyJsAssignment): TsNonNullAssertionAssignment;
	readonly exclToken: string | undefined;
	withExclToken(value: JsAstToken): TsNonNullAssertionAssignment;
	token(field: "exclToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNonNullAssertionExpression extends JsAstNode {
	readonly kind: "TS_NON_NULL_ASSERTION_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): TsNonNullAssertionExpression;
	readonly exclToken: string | undefined;
	withExclToken(value: JsAstToken): TsNonNullAssertionExpression;
	token(field: "exclToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNonPrimitiveType extends JsAstNode {
	readonly kind: "TS_NON_PRIMITIVE_TYPE";
	readonly objectToken: string | undefined;
	withObjectToken(value: JsAstToken): TsNonPrimitiveType;
	token(field: "objectToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNullLiteralType extends JsAstNode {
	readonly kind: "TS_NULL_LITERAL_TYPE";
	readonly literalToken: string | undefined;
	withLiteralToken(value: JsAstToken): TsNullLiteralType;
	token(field: "literalToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNumberLiteralType extends JsAstNode {
	readonly kind: "TS_NUMBER_LITERAL_TYPE";
	readonly minusToken: string | undefined;
	withMinusToken(value: JsAstToken | undefined): TsNumberLiteralType;
	readonly literalToken: string | undefined;
	withLiteralToken(value: JsAstToken): TsNumberLiteralType;
	token(field: "minusToken" | "literalToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsNumberType extends JsAstNode {
	readonly kind: "TS_NUMBER_TYPE";
	readonly numberToken: string | undefined;
	withNumberToken(value: JsAstToken): TsNumberType;
	token(field: "numberToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsObjectType extends JsAstNode {
	readonly kind: "TS_OBJECT_TYPE";
	readonly lCurlyToken: string | undefined;
	withLCurlyToken(value: JsAstToken): TsObjectType;
	readonly members: TsTypeMemberList;
	withMembers(value: TsTypeMemberListNode): TsObjectType;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsObjectType;
	token(field: "lCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsOptionalPropertyAnnotation extends JsAstNode {
	readonly kind: "TS_OPTIONAL_PROPERTY_ANNOTATION";
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken): TsOptionalPropertyAnnotation;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(
		value: TsTypeAnnotation | undefined,
	): TsOptionalPropertyAnnotation;
	token(field: "questionMarkToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsOptionalTupleTypeElement extends JsAstNode {
	readonly kind: "TS_OPTIONAL_TUPLE_TYPE_ELEMENT";
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsOptionalTupleTypeElement;
	readonly questionMarkToken: string | undefined;
	withQuestionMarkToken(value: JsAstToken): TsOptionalTupleTypeElement;
	token(field: "questionMarkToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsOutModifier extends JsAstNode {
	readonly kind: "TS_OUT_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsOutModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsOverrideModifier extends JsAstNode {
	readonly kind: "TS_OVERRIDE_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsOverrideModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsParenthesizedType extends JsAstNode {
	readonly kind: "TS_PARENTHESIZED_TYPE";
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsParenthesizedType;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsParenthesizedType;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsParenthesizedType;
	token(field: "lParenToken" | "rParenToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsPredicateReturnType extends JsAstNode {
	readonly kind: "TS_PREDICATE_RETURN_TYPE";
	readonly parameterName: AnyTsTypePredicateParameterName | undefined;
	withParameterName(
		value: AnyTsTypePredicateParameterName,
	): TsPredicateReturnType;
	readonly isToken: string | undefined;
	withIsToken(value: JsAstToken): TsPredicateReturnType;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsPredicateReturnType;
	token(field: "isToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsPropertyParameter extends JsAstNode {
	readonly kind: "TS_PROPERTY_PARAMETER";
	readonly decorators: JsDecoratorList;
	withDecorators(value: JsDecoratorListNode): TsPropertyParameter;
	readonly modifiers: TsPropertyParameterModifierList;
	withModifiers(
		value: TsPropertyParameterModifierListNode,
	): TsPropertyParameter;
	readonly formalParameter: AnyJsFormalParameter | undefined;
	withFormalParameter(value: AnyJsFormalParameter): TsPropertyParameter;
}
export interface TsPropertySignatureClassMember extends JsAstNode {
	readonly kind: "TS_PROPERTY_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsPropertySignatureModifierList;
	withModifiers(
		value: TsPropertySignatureModifierListNode,
	): TsPropertySignatureClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): TsPropertySignatureClassMember;
	readonly propertyAnnotation: AnyTsPropertySignatureAnnotation | undefined;
	withPropertyAnnotation(
		value: AnyTsPropertySignatureAnnotation | undefined,
	): TsPropertySignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsPropertySignatureClassMember;
	token(field: "semicolonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsPropertySignatureTypeMember extends JsAstNode {
	readonly kind: "TS_PROPERTY_SIGNATURE_TYPE_MEMBER";
	readonly readonlyToken: string | undefined;
	withReadonlyToken(
		value: JsAstToken | undefined,
	): TsPropertySignatureTypeMember;
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): TsPropertySignatureTypeMember;
	readonly optionalToken: string | undefined;
	withOptionalToken(
		value: JsAstToken | undefined,
	): TsPropertySignatureTypeMember;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(
		value: TsTypeAnnotation | undefined,
	): TsPropertySignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(
		value: JsAstToken | undefined,
	): TsPropertySignatureTypeMember;
	token(
		field: "readonlyToken" | "optionalToken" | "separatorToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsQualifiedModuleName extends JsAstNode {
	readonly kind: "TS_QUALIFIED_MODULE_NAME";
	readonly left: AnyTsModuleName | undefined;
	withLeft(value: AnyTsModuleName): TsQualifiedModuleName;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): TsQualifiedModuleName;
	readonly right: JsName | undefined;
	withRight(value: JsName): TsQualifiedModuleName;
	token(field: "dotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsQualifiedName extends JsAstNode {
	readonly kind: "TS_QUALIFIED_NAME";
	readonly left: AnyTsName | undefined;
	withLeft(value: AnyTsName): TsQualifiedName;
	readonly dotToken: string | undefined;
	withDotToken(value: JsAstToken): TsQualifiedName;
	readonly right: JsName | undefined;
	withRight(value: JsName): TsQualifiedName;
	token(field: "dotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsReadonlyModifier extends JsAstNode {
	readonly kind: "TS_READONLY_MODIFIER";
	readonly modifierToken: string | undefined;
	withModifierToken(value: JsAstToken): TsReadonlyModifier;
	token(field: "modifierToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsReferenceType extends JsAstNode {
	readonly kind: "TS_REFERENCE_TYPE";
	readonly name: AnyTsName | undefined;
	withName(value: AnyTsName): TsReferenceType;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): TsReferenceType;
}
export interface TsRestTupleTypeElement extends JsAstNode {
	readonly kind: "TS_REST_TUPLE_TYPE_ELEMENT";
	readonly dotdotdotToken: string | undefined;
	withDotdotdotToken(value: JsAstToken): TsRestTupleTypeElement;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsRestTupleTypeElement;
	token(field: "dotdotdotToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsReturnTypeAnnotation extends JsAstNode {
	readonly kind: "TS_RETURN_TYPE_ANNOTATION";
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): TsReturnTypeAnnotation;
	readonly ty: AnyTsReturnType | undefined;
	withTy(value: AnyTsReturnType): TsReturnTypeAnnotation;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsSatisfiesAssignment extends JsAstNode {
	readonly kind: "TS_SATISFIES_ASSIGNMENT";
	readonly assignment: AnyJsAssignment | undefined;
	withAssignment(value: AnyJsAssignment): TsSatisfiesAssignment;
	readonly satisfiesToken: string | undefined;
	withSatisfiesToken(value: JsAstToken): TsSatisfiesAssignment;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsSatisfiesAssignment;
	token(field: "satisfiesToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsSatisfiesExpression extends JsAstNode {
	readonly kind: "TS_SATISFIES_EXPRESSION";
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): TsSatisfiesExpression;
	readonly satisfiesToken: string | undefined;
	withSatisfiesToken(value: JsAstToken): TsSatisfiesExpression;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsSatisfiesExpression;
	token(field: "satisfiesToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsSetterSignatureClassMember extends JsAstNode {
	readonly kind: "TS_SETTER_SIGNATURE_CLASS_MEMBER";
	readonly modifiers: TsMethodSignatureModifierList;
	withModifiers(
		value: TsMethodSignatureModifierListNode,
	): TsSetterSignatureClassMember;
	readonly setToken: string | undefined;
	withSetToken(value: JsAstToken): TsSetterSignatureClassMember;
	readonly name: AnyJsClassMemberName | undefined;
	withName(value: AnyJsClassMemberName): TsSetterSignatureClassMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsSetterSignatureClassMember;
	readonly parameter: AnyJsFormalParameter | undefined;
	withParameter(value: AnyJsFormalParameter): TsSetterSignatureClassMember;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken | undefined): TsSetterSignatureClassMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsSetterSignatureClassMember;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(
		value: JsAstToken | undefined,
	): TsSetterSignatureClassMember;
	token(
		field:
			| "setToken"
			| "lParenToken"
			| "commaToken"
			| "rParenToken"
			| "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsSetterSignatureTypeMember extends JsAstNode {
	readonly kind: "TS_SETTER_SIGNATURE_TYPE_MEMBER";
	readonly setToken: string | undefined;
	withSetToken(value: JsAstToken): TsSetterSignatureTypeMember;
	readonly name: AnyJsObjectMemberName | undefined;
	withName(value: AnyJsObjectMemberName): TsSetterSignatureTypeMember;
	readonly lParenToken: string | undefined;
	withLParenToken(value: JsAstToken): TsSetterSignatureTypeMember;
	readonly parameter: AnyJsFormalParameter | undefined;
	withParameter(value: AnyJsFormalParameter): TsSetterSignatureTypeMember;
	readonly commaToken: string | undefined;
	withCommaToken(value: JsAstToken | undefined): TsSetterSignatureTypeMember;
	readonly rParenToken: string | undefined;
	withRParenToken(value: JsAstToken): TsSetterSignatureTypeMember;
	readonly separatorToken: string | undefined;
	withSeparatorToken(
		value: JsAstToken | undefined,
	): TsSetterSignatureTypeMember;
	token(
		field:
			| "setToken"
			| "lParenToken"
			| "commaToken"
			| "rParenToken"
			| "separatorToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsStringLiteralType extends JsAstNode {
	readonly kind: "TS_STRING_LITERAL_TYPE";
	readonly literalToken: string | undefined;
	withLiteralToken(value: JsAstToken): TsStringLiteralType;
	token(field: "literalToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsStringType extends JsAstNode {
	readonly kind: "TS_STRING_TYPE";
	readonly stringToken: string | undefined;
	withStringToken(value: JsAstToken): TsStringType;
	token(field: "stringToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsSymbolType extends JsAstNode {
	readonly kind: "TS_SYMBOL_TYPE";
	readonly symbolToken: string | undefined;
	withSymbolToken(value: JsAstToken): TsSymbolType;
	token(field: "symbolToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTemplateChunkElement extends JsAstNode {
	readonly kind: "TS_TEMPLATE_CHUNK_ELEMENT";
	readonly templateChunkToken: string | undefined;
	withTemplateChunkToken(value: JsAstToken): TsTemplateChunkElement;
	token(field: "templateChunkToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTemplateElement extends JsAstNode {
	readonly kind: "TS_TEMPLATE_ELEMENT";
	readonly dollarCurlyToken: string | undefined;
	withDollarCurlyToken(value: JsAstToken): TsTemplateElement;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTemplateElement;
	readonly rCurlyToken: string | undefined;
	withRCurlyToken(value: JsAstToken): TsTemplateElement;
	token(field: "dollarCurlyToken" | "rCurlyToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTemplateLiteralType extends JsAstNode {
	readonly kind: "TS_TEMPLATE_LITERAL_TYPE";
	readonly lTickToken: string | undefined;
	withLTickToken(value: JsAstToken): TsTemplateLiteralType;
	readonly elements: TsTemplateElementList;
	withElements(value: TsTemplateElementListNode): TsTemplateLiteralType;
	readonly rTickToken: string | undefined;
	withRTickToken(value: JsAstToken): TsTemplateLiteralType;
	token(field: "lTickToken" | "rTickToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsThisParameter extends JsAstNode {
	readonly kind: "TS_THIS_PARAMETER";
	readonly thisToken: string | undefined;
	withThisToken(value: JsAstToken): TsThisParameter;
	readonly typeAnnotation: TsTypeAnnotation | undefined;
	withTypeAnnotation(value: TsTypeAnnotation | undefined): TsThisParameter;
	token(field: "thisToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsThisType extends JsAstNode {
	readonly kind: "TS_THIS_TYPE";
	readonly thisToken: string | undefined;
	withThisToken(value: JsAstToken): TsThisType;
	token(field: "thisToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTupleType extends JsAstNode {
	readonly kind: "TS_TUPLE_TYPE";
	readonly lBrackToken: string | undefined;
	withLBrackToken(value: JsAstToken): TsTupleType;
	readonly elements: TsTupleTypeElementList;
	withElements(value: TsTupleTypeElementListNode): TsTupleType;
	readonly rBrackToken: string | undefined;
	withRBrackToken(value: JsAstToken): TsTupleType;
	token(field: "lBrackToken" | "rBrackToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeAliasDeclaration extends JsAstNode {
	readonly kind: "TS_TYPE_ALIAS_DECLARATION";
	readonly typeToken: string | undefined;
	withTypeToken(value: JsAstToken): TsTypeAliasDeclaration;
	readonly bindingIdentifier: AnyTsIdentifierBinding | undefined;
	withBindingIdentifier(value: AnyTsIdentifierBinding): TsTypeAliasDeclaration;
	readonly typeParameters: TsTypeParameters | undefined;
	withTypeParameters(
		value: TsTypeParameters | undefined,
	): TsTypeAliasDeclaration;
	readonly eqToken: string | undefined;
	withEqToken(value: JsAstToken): TsTypeAliasDeclaration;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTypeAliasDeclaration;
	readonly semicolonToken: string | undefined;
	withSemicolonToken(value: JsAstToken | undefined): TsTypeAliasDeclaration;
	token(
		field: "typeToken" | "eqToken" | "semicolonToken",
	): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeAnnotation extends JsAstNode {
	readonly kind: "TS_TYPE_ANNOTATION";
	readonly colonToken: string | undefined;
	withColonToken(value: JsAstToken): TsTypeAnnotation;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTypeAnnotation;
	token(field: "colonToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeArguments extends JsAstNode {
	readonly kind: "TS_TYPE_ARGUMENTS";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): TsTypeArguments;
	readonly tsTypeArgumentList: TsTypeArgumentList;
	withTsTypeArgumentList(value: TsTypeArgumentListNode): TsTypeArguments;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): TsTypeArguments;
	token(field: "lAngleToken" | "rAngleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeAssertionAssignment extends JsAstNode {
	readonly kind: "TS_TYPE_ASSERTION_ASSIGNMENT";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): TsTypeAssertionAssignment;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTypeAssertionAssignment;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): TsTypeAssertionAssignment;
	readonly assignment: AnyJsAssignment | undefined;
	withAssignment(value: AnyJsAssignment): TsTypeAssertionAssignment;
	token(field: "lAngleToken" | "rAngleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeAssertionExpression extends JsAstNode {
	readonly kind: "TS_TYPE_ASSERTION_EXPRESSION";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): TsTypeAssertionExpression;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTypeAssertionExpression;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): TsTypeAssertionExpression;
	readonly expression: AnyJsExpression | undefined;
	withExpression(value: AnyJsExpression): TsTypeAssertionExpression;
	token(field: "lAngleToken" | "rAngleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeConstraintClause extends JsAstNode {
	readonly kind: "TS_TYPE_CONSTRAINT_CLAUSE";
	readonly extendsToken: string | undefined;
	withExtendsToken(value: JsAstToken): TsTypeConstraintClause;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTypeConstraintClause;
	token(field: "extendsToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeOperatorType extends JsAstNode {
	readonly kind: "TS_TYPE_OPERATOR_TYPE";
	readonly operatorToken: string | undefined;
	withOperatorToken(value: JsAstToken): TsTypeOperatorType;
	readonly ty: AnyTsType | undefined;
	withTy(value: AnyTsType): TsTypeOperatorType;
	token(field: "operatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeParameter extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETER";
	readonly modifiers: TsTypeParameterModifierList;
	withModifiers(value: TsTypeParameterModifierListNode): TsTypeParameter;
	readonly name: TsTypeParameterName | undefined;
	withName(value: TsTypeParameterName): TsTypeParameter;
	readonly constraint: TsTypeConstraintClause | undefined;
	withConstraint(value: TsTypeConstraintClause | undefined): TsTypeParameter;
	readonly default: TsDefaultTypeClause | undefined;
	withDefault(value: TsDefaultTypeClause | undefined): TsTypeParameter;
}
export interface TsTypeParameterName extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETER_NAME";
	readonly identToken: string | undefined;
	withIdentToken(value: JsAstToken): TsTypeParameterName;
	token(field: "identToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeParameters extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETERS";
	readonly lAngleToken: string | undefined;
	withLAngleToken(value: JsAstToken): TsTypeParameters;
	readonly items: TsTypeParameterList;
	withItems(value: TsTypeParameterListNode): TsTypeParameters;
	readonly rAngleToken: string | undefined;
	withRAngleToken(value: JsAstToken): TsTypeParameters;
	token(field: "lAngleToken" | "rAngleToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsTypeofType extends JsAstNode {
	readonly kind: "TS_TYPEOF_TYPE";
	readonly typeofToken: string | undefined;
	withTypeofToken(value: JsAstToken): TsTypeofType;
	readonly expressionName: AnyTsName | undefined;
	withExpressionName(value: AnyTsName): TsTypeofType;
	readonly typeArguments: TsTypeArguments | undefined;
	withTypeArguments(value: TsTypeArguments | undefined): TsTypeofType;
	token(field: "typeofToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsUndefinedType extends JsAstNode {
	readonly kind: "TS_UNDEFINED_TYPE";
	readonly undefinedToken: string | undefined;
	withUndefinedToken(value: JsAstToken): TsUndefinedType;
	token(field: "undefinedToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsUnionType extends JsAstNode {
	readonly kind: "TS_UNION_TYPE";
	readonly leadingSeparatorToken: string | undefined;
	withLeadingSeparatorToken(value: JsAstToken | undefined): TsUnionType;
	readonly types: TsUnionTypeVariantList;
	withTypes(value: TsUnionTypeVariantListNode): TsUnionType;
	token(field: "leadingSeparatorToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsUnknownType extends JsAstNode {
	readonly kind: "TS_UNKNOWN_TYPE";
	readonly unknownToken: string | undefined;
	withUnknownToken(value: JsAstToken): TsUnknownType;
	token(field: "unknownToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
}
export interface TsVoidType extends JsAstNode {
	readonly kind: "TS_VOID_TYPE";
	readonly voidToken: string | undefined;
	withVoidToken(value: JsAstToken): TsVoidType;
	token(field: "voidToken"): JsAstToken | undefined;
	token(field: string): JsAstToken | undefined;
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
export interface JsBogusVariableDeclaration extends JsAstNode {
	readonly kind: "JS_BOGUS_VARIABLE_DECLARATION";
}
export interface TsBogusType extends JsAstNode {
	readonly kind: "TS_BOGUS_TYPE";
}
export type AnyJsArrayAssignmentPatternElement =
	| JsArrayAssignmentPatternElement
	| JsArrayAssignmentPatternRestElement
	| JsArrayHole;
export type AnyJsArrayBindingPatternElement =
	| JsArrayBindingPatternElement
	| JsArrayBindingPatternRestElement
	| JsArrayHole;
export type AnyJsArrayElement = AnyJsExpression | JsArrayHole | JsSpread;
export type AnyJsArrowFunctionParameters = AnyJsBinding | JsParameters;
export type AnyJsAssignment =
	| JsBogusAssignment
	| JsComputedMemberAssignment
	| JsIdentifierAssignment
	| JsParenthesizedAssignment
	| JsStaticMemberAssignment
	| TsAsAssignment
	| TsNonNullAssertionAssignment
	| TsSatisfiesAssignment
	| TsTypeAssertionAssignment;
export type AnyJsAssignmentPattern =
	| AnyJsAssignment
	| JsArrayAssignmentPattern
	| JsObjectAssignmentPattern;
export type AnyJsBinding =
	| JsBogusBinding
	| JsIdentifierBinding
	| JsMetavariable;
export type AnyJsBindingPattern =
	| AnyJsBinding
	| JsArrayBindingPattern
	| JsObjectBindingPattern;
export type AnyJsCallArgument = AnyJsExpression | JsSpread;
export type AnyJsClass =
	| JsClassDeclaration
	| JsClassExportDefaultDeclaration
	| JsClassExpression;
export type AnyJsClassMember =
	| JsBogusMember
	| JsConstructorClassMember
	| JsEmptyClassMember
	| JsGetterClassMember
	| JsMetavariable
	| JsMethodClassMember
	| JsPropertyClassMember
	| JsSetterClassMember
	| JsStaticInitializationBlockClassMember
	| TsConstructorSignatureClassMember
	| TsGetterSignatureClassMember
	| TsIndexSignatureClassMember
	| TsInitializedPropertySignatureClassMember
	| TsMethodSignatureClassMember
	| TsPropertySignatureClassMember
	| TsSetterSignatureClassMember;
export type AnyJsClassMemberName =
	| JsComputedMemberName
	| JsLiteralMemberName
	| JsMetavariable
	| JsPrivateClassMemberName;
export type AnyJsCombinedSpecifier =
	| JsNamedImportSpecifiers
	| JsNamespaceImportSpecifier;
export type AnyJsConstructorParameter =
	| AnyJsFormalParameter
	| JsRestParameter
	| TsPropertyParameter;
export type AnyJsDeclaration =
	| JsClassDeclaration
	| JsFunctionDeclaration
	| JsVariableDeclaration
	| TsDeclareFunctionDeclaration
	| TsEnumDeclaration
	| TsExternalModuleDeclaration
	| TsGlobalDeclaration
	| TsImportEqualsDeclaration
	| TsInterfaceDeclaration
	| TsModuleDeclaration
	| TsTypeAliasDeclaration;
export type AnyJsDeclarationClause =
	| JsClassDeclaration
	| JsFunctionDeclaration
	| JsVariableDeclarationClause
	| TsDeclareFunctionDeclaration
	| TsEnumDeclaration
	| TsExternalModuleDeclaration
	| TsGlobalDeclaration
	| TsImportEqualsDeclaration
	| TsInterfaceDeclaration
	| TsModuleDeclaration
	| TsTypeAliasDeclaration;
export type AnyJsDecorator =
	| JsBogusExpression
	| JsCallExpression
	| JsIdentifierExpression
	| JsParenthesizedExpression
	| JsStaticMemberExpression;
export type AnyJsExportClause =
	| AnyJsDeclarationClause
	| JsExportDefaultDeclarationClause
	| JsExportDefaultExpressionClause
	| JsExportFromClause
	| JsExportNamedClause
	| JsExportNamedFromClause
	| TsExportAsNamespaceClause
	| TsExportAssignmentClause
	| TsExportDeclareClause;
export type AnyJsExportDefaultDeclaration =
	| JsClassExportDefaultDeclaration
	| JsFunctionExportDefaultDeclaration
	| TsDeclareFunctionExportDefaultDeclaration
	| TsInterfaceDeclaration;
export type AnyJsExportNamedSpecifier =
	| JsExportNamedShorthandSpecifier
	| JsExportNamedSpecifier;
export type AnyJsExpression =
	| AnyJsLiteralExpression
	| JsArrayExpression
	| JsArrowFunctionExpression
	| JsAssignmentExpression
	| JsAwaitExpression
	| JsBinaryExpression
	| JsBogusExpression
	| JsCallExpression
	| JsClassExpression
	| JsComputedMemberExpression
	| JsConditionalExpression
	| JsFunctionExpression
	| JsIdentifierExpression
	| JsImportCallExpression
	| JsImportMetaExpression
	| JsInExpression
	| JsInstanceofExpression
	| JsLogicalExpression
	| JsMetavariable
	| JsNewExpression
	| JsNewTargetExpression
	| JsObjectExpression
	| JsParenthesizedExpression
	| JsPostUpdateExpression
	| JsPreUpdateExpression
	| JsSequenceExpression
	| JsStaticMemberExpression
	| JsSuperExpression
	| JsTemplateExpression
	| JsThisExpression
	| JsUnaryExpression
	| JsYieldExpression
	| JsxTagExpression
	| TsAsExpression
	| TsInstantiationExpression
	| TsNonNullAssertionExpression
	| TsSatisfiesExpression
	| TsTypeAssertionExpression;
export type AnyJsForInOrOfInitializer =
	| AnyJsAssignmentPattern
	| JsForVariableDeclaration;
export type AnyJsForInitializer = AnyJsExpression | JsVariableDeclaration;
export type AnyJsFormalParameter =
	| JsBogusParameter
	| JsFormalParameter
	| JsMetavariable;
export type AnyJsFunction =
	| JsArrowFunctionExpression
	| JsFunctionDeclaration
	| JsFunctionExportDefaultDeclaration
	| JsFunctionExpression;
export type AnyJsFunctionBody = AnyJsExpression | JsFunctionBody;
export type AnyJsImportAssertionEntry =
	| JsBogusImportAssertionEntry
	| JsImportAssertionEntry;
export type AnyJsImportClause =
	| JsImportBareClause
	| JsImportCombinedClause
	| JsImportDefaultClause
	| JsImportNamedClause
	| JsImportNamespaceClause;
export type AnyJsInProperty = AnyJsExpression | JsPrivateName;
export type AnyJsLiteralExportName = JsLiteralExportName | JsMetavariable;
export type AnyJsLiteralExpression =
	| JsBigintLiteralExpression
	| JsBooleanLiteralExpression
	| JsNullLiteralExpression
	| JsNumberLiteralExpression
	| JsRegexLiteralExpression
	| JsStringLiteralExpression;
export type AnyJsMethodModifier =
	| JsDecorator
	| JsStaticModifier
	| TsAccessibilityModifier
	| TsOverrideModifier;
export type AnyJsModuleItem = AnyJsStatement | JsExport | JsImport;
export type AnyJsModuleSource = JsMetavariable | JsModuleSource;
export type AnyJsName = JsMetavariable | JsName | JsPrivateName;
export type AnyJsNamedImportSpecifier =
	| JsBogusNamedImportSpecifier
	| JsNamedImportSpecifier
	| JsShorthandNamedImportSpecifier;
export type AnyJsObjectAssignmentPatternMember =
	| JsBogusAssignment
	| JsObjectAssignmentPatternProperty
	| JsObjectAssignmentPatternRest
	| JsObjectAssignmentPatternShorthandProperty;
export type AnyJsObjectBindingPatternMember =
	| JsBogusBinding
	| JsMetavariable
	| JsObjectBindingPatternProperty
	| JsObjectBindingPatternRest
	| JsObjectBindingPatternShorthandProperty;
export type AnyJsObjectMember =
	| JsBogusMember
	| JsGetterObjectMember
	| JsMetavariable
	| JsMethodObjectMember
	| JsPropertyObjectMember
	| JsSetterObjectMember
	| JsShorthandPropertyObjectMember
	| JsSpread;
export type AnyJsObjectMemberName =
	| JsComputedMemberName
	| JsLiteralMemberName
	| JsMetavariable;
export type AnyJsParameter =
	| AnyJsFormalParameter
	| JsRestParameter
	| TsThisParameter;
export type AnyJsPropertyModifier =
	| JsAccessorModifier
	| JsDecorator
	| JsStaticModifier
	| TsAccessibilityModifier
	| TsOverrideModifier
	| TsReadonlyModifier;
export type AnyJsRoot =
	| JsExpressionSnippet
	| JsExpressionTemplateRoot
	| JsModule
	| JsScript
	| JsSvelteDeclarationRoot
	| JsSvelteSnippetRoot
	| TsDeclarationModule;
export type AnyJsStatement =
	| JsBlockStatement
	| JsBogusStatement
	| JsBreakStatement
	| JsClassDeclaration
	| JsContinueStatement
	| JsDebuggerStatement
	| JsDoWhileStatement
	| JsEmptyStatement
	| JsExpressionStatement
	| JsForInStatement
	| JsForOfStatement
	| JsForStatement
	| JsFunctionDeclaration
	| JsIfStatement
	| JsLabeledStatement
	| JsMetavariable
	| JsReturnStatement
	| JsSwitchStatement
	| JsThrowStatement
	| JsTryFinallyStatement
	| JsTryStatement
	| JsVariableStatement
	| JsWhileStatement
	| JsWithStatement
	| TsDeclareFunctionDeclaration
	| TsDeclareStatement
	| TsEnumDeclaration
	| TsExternalModuleDeclaration
	| TsGlobalDeclaration
	| TsImportEqualsDeclaration
	| TsInterfaceDeclaration
	| TsModuleDeclaration
	| TsTypeAliasDeclaration;
export type AnyJsSvelteDeclaration =
	| JsBogusVariableDeclaration
	| JsVariableDeclaration;
export type AnyJsSwitchClause = JsCaseClause | JsDefaultClause;
export type AnyJsTemplateElement = JsTemplateChunkElement | JsTemplateElement;
export type AnyJsxAttribute =
	| JsMetavariable
	| JsxAttribute
	| JsxShorthandAttribute
	| JsxSpreadAttribute;
export type AnyJsxAttributeName = JsxName | JsxNamespaceName;
export type AnyJsxAttributeValue =
	| AnyJsxTag
	| JsTemplateExpression
	| JsxExpressionAttributeValue
	| JsxString;
export type AnyJsxChild =
	| JsMetavariable
	| JsxElement
	| JsxExpressionChild
	| JsxFragment
	| JsxSelfClosingElement
	| JsxSpreadChild
	| JsxText;
export type AnyJsxElementName =
	| JsMetavariable
	| JsxMemberName
	| JsxName
	| JsxNamespaceName
	| JsxReferenceIdentifier;
export type AnyJsxName = JsxName | JsxNamespaceName;
export type AnyJsxObjectName =
	| JsxMemberName
	| JsxNamespaceName
	| JsxReferenceIdentifier;
export type AnyJsxTag =
	| AstroImplicitFragment
	| JsxElement
	| JsxFragment
	| JsxSelfClosingElement;
export type AnyTsEnumMemberName =
	| JsComputedMemberName
	| TsLiteralEnumMemberName;
export type AnyTsExternalModuleDeclarationBody =
	| TsEmptyExternalModuleDeclarationBody
	| TsModuleBlock;
export type AnyTsIdentifierBinding = JsMetavariable | TsIdentifierBinding;
export type AnyTsIndexSignatureModifier = JsStaticModifier | TsReadonlyModifier;
export type AnyTsMethodSignatureModifier =
	| JsDecorator
	| JsStaticModifier
	| TsAbstractModifier
	| TsAccessibilityModifier
	| TsOverrideModifier;
export type AnyTsModuleName = AnyTsIdentifierBinding | TsQualifiedModuleName;
export type AnyTsModuleReference = AnyTsName | TsExternalModuleReference;
export type AnyTsName = JsReferenceIdentifier | TsQualifiedName;
export type AnyTsPropertyAnnotation =
	| TsDefinitePropertyAnnotation
	| TsOptionalPropertyAnnotation
	| TsTypeAnnotation;
export type AnyTsPropertyParameterModifier =
	| TsAccessibilityModifier
	| TsOverrideModifier
	| TsReadonlyModifier;
export type AnyTsPropertySignatureAnnotation =
	| TsOptionalPropertyAnnotation
	| TsTypeAnnotation;
export type AnyTsPropertySignatureModifier =
	| JsAccessorModifier
	| JsDecorator
	| JsStaticModifier
	| TsAbstractModifier
	| TsAccessibilityModifier
	| TsDeclareModifier
	| TsOverrideModifier
	| TsReadonlyModifier;
export type AnyTsReturnType =
	| AnyTsType
	| TsAssertsReturnType
	| TsPredicateReturnType;
export type AnyTsTemplateElement = TsTemplateChunkElement | TsTemplateElement;
export type AnyTsTupleTypeElement =
	| AnyTsType
	| TsNamedTupleTypeElement
	| TsOptionalTupleTypeElement
	| TsRestTupleTypeElement;
export type AnyTsType =
	| JsMetavariable
	| TsAnyType
	| TsArrayType
	| TsBigintLiteralType
	| TsBigintType
	| TsBogusType
	| TsBooleanLiteralType
	| TsBooleanType
	| TsConditionalType
	| TsConstructorType
	| TsFunctionType
	| TsImportType
	| TsIndexedAccessType
	| TsInferType
	| TsIntersectionType
	| TsMappedType
	| TsNeverType
	| TsNonPrimitiveType
	| TsNullLiteralType
	| TsNumberLiteralType
	| TsNumberType
	| TsObjectType
	| TsParenthesizedType
	| TsReferenceType
	| TsStringLiteralType
	| TsStringType
	| TsSymbolType
	| TsTemplateLiteralType
	| TsThisType
	| TsTupleType
	| TsTypeOperatorType
	| TsTypeofType
	| TsUndefinedType
	| TsUnionType
	| TsUnknownType
	| TsVoidType;
export type AnyTsTypeMember =
	| JsBogusMember
	| JsMetavariable
	| TsCallSignatureTypeMember
	| TsConstructSignatureTypeMember
	| TsGetterSignatureTypeMember
	| TsIndexSignatureTypeMember
	| TsMethodSignatureTypeMember
	| TsPropertySignatureTypeMember
	| TsSetterSignatureTypeMember;
export type AnyTsTypeParameterModifier =
	| TsConstModifier
	| TsInModifier
	| TsOutModifier;
export type AnyTsTypePredicateParameterName =
	| JsReferenceIdentifier
	| TsThisType;
export type AnyTsVariableAnnotation =
	| TsDefiniteVariableAnnotation
	| TsTypeAnnotation;
export interface JsNodeByKind {
	readonly ASTRO_IMPLICIT_FRAGMENT: AstroImplicitFragment;
	readonly JS_ACCESSOR_MODIFIER: JsAccessorModifier;
	readonly JS_ARRAY_ASSIGNMENT_PATTERN: JsArrayAssignmentPattern;
	readonly JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT: JsArrayAssignmentPatternElement;
	readonly JS_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT: JsArrayAssignmentPatternRestElement;
	readonly JS_ARRAY_BINDING_PATTERN: JsArrayBindingPattern;
	readonly JS_ARRAY_BINDING_PATTERN_ELEMENT: JsArrayBindingPatternElement;
	readonly JS_ARRAY_BINDING_PATTERN_REST_ELEMENT: JsArrayBindingPatternRestElement;
	readonly JS_ARRAY_EXPRESSION: JsArrayExpression;
	readonly JS_ARRAY_HOLE: JsArrayHole;
	readonly JS_ARROW_FUNCTION_EXPRESSION: JsArrowFunctionExpression;
	readonly JS_ASSIGNMENT_EXPRESSION: JsAssignmentExpression;
	readonly JS_AWAIT_EXPRESSION: JsAwaitExpression;
	readonly JS_BIGINT_LITERAL_EXPRESSION: JsBigintLiteralExpression;
	readonly JS_BINARY_EXPRESSION: JsBinaryExpression;
	readonly JS_BLOCK_STATEMENT: JsBlockStatement;
	readonly JS_BOOLEAN_LITERAL_EXPRESSION: JsBooleanLiteralExpression;
	readonly JS_BREAK_STATEMENT: JsBreakStatement;
	readonly JS_CALL_ARGUMENTS: JsCallArguments;
	readonly JS_CALL_EXPRESSION: JsCallExpression;
	readonly JS_CASE_CLAUSE: JsCaseClause;
	readonly JS_CATCH_CLAUSE: JsCatchClause;
	readonly JS_CATCH_DECLARATION: JsCatchDeclaration;
	readonly JS_CLASS_DECLARATION: JsClassDeclaration;
	readonly JS_CLASS_EXPORT_DEFAULT_DECLARATION: JsClassExportDefaultDeclaration;
	readonly JS_CLASS_EXPRESSION: JsClassExpression;
	readonly JS_COMPUTED_MEMBER_ASSIGNMENT: JsComputedMemberAssignment;
	readonly JS_COMPUTED_MEMBER_EXPRESSION: JsComputedMemberExpression;
	readonly JS_COMPUTED_MEMBER_NAME: JsComputedMemberName;
	readonly JS_CONDITIONAL_EXPRESSION: JsConditionalExpression;
	readonly JS_CONSTRUCTOR_CLASS_MEMBER: JsConstructorClassMember;
	readonly JS_CONSTRUCTOR_PARAMETERS: JsConstructorParameters;
	readonly JS_CONTINUE_STATEMENT: JsContinueStatement;
	readonly JS_DEBUGGER_STATEMENT: JsDebuggerStatement;
	readonly JS_DECORATOR: JsDecorator;
	readonly JS_DEFAULT_CLAUSE: JsDefaultClause;
	readonly JS_DEFAULT_IMPORT_SPECIFIER: JsDefaultImportSpecifier;
	readonly JS_DIRECTIVE: JsDirective;
	readonly JS_DO_WHILE_STATEMENT: JsDoWhileStatement;
	readonly JS_ELSE_CLAUSE: JsElseClause;
	readonly JS_EMPTY_CLASS_MEMBER: JsEmptyClassMember;
	readonly JS_EMPTY_STATEMENT: JsEmptyStatement;
	readonly JS_EXPORT: JsExport;
	readonly JS_EXPORT_AS_CLAUSE: JsExportAsClause;
	readonly JS_EXPORT_DEFAULT_DECLARATION_CLAUSE: JsExportDefaultDeclarationClause;
	readonly JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE: JsExportDefaultExpressionClause;
	readonly JS_EXPORT_FROM_CLAUSE: JsExportFromClause;
	readonly JS_EXPORT_NAMED_CLAUSE: JsExportNamedClause;
	readonly JS_EXPORT_NAMED_FROM_CLAUSE: JsExportNamedFromClause;
	readonly JS_EXPORT_NAMED_FROM_SPECIFIER: JsExportNamedFromSpecifier;
	readonly JS_EXPORT_NAMED_SHORTHAND_SPECIFIER: JsExportNamedShorthandSpecifier;
	readonly JS_EXPORT_NAMED_SPECIFIER: JsExportNamedSpecifier;
	readonly JS_EXPRESSION_SNIPPET: JsExpressionSnippet;
	readonly JS_EXPRESSION_STATEMENT: JsExpressionStatement;
	readonly JS_EXPRESSION_TEMPLATE_ROOT: JsExpressionTemplateRoot;
	readonly JS_EXTENDS_CLAUSE: JsExtendsClause;
	readonly JS_FINALLY_CLAUSE: JsFinallyClause;
	readonly JS_FOR_IN_STATEMENT: JsForInStatement;
	readonly JS_FOR_OF_STATEMENT: JsForOfStatement;
	readonly JS_FOR_STATEMENT: JsForStatement;
	readonly JS_FOR_VARIABLE_DECLARATION: JsForVariableDeclaration;
	readonly JS_FORMAL_PARAMETER: JsFormalParameter;
	readonly JS_FUNCTION_BODY: JsFunctionBody;
	readonly JS_FUNCTION_DECLARATION: JsFunctionDeclaration;
	readonly JS_FUNCTION_EXPORT_DEFAULT_DECLARATION: JsFunctionExportDefaultDeclaration;
	readonly JS_FUNCTION_EXPRESSION: JsFunctionExpression;
	readonly JS_GETTER_CLASS_MEMBER: JsGetterClassMember;
	readonly JS_GETTER_OBJECT_MEMBER: JsGetterObjectMember;
	readonly JS_IDENTIFIER_ASSIGNMENT: JsIdentifierAssignment;
	readonly JS_IDENTIFIER_BINDING: JsIdentifierBinding;
	readonly JS_IDENTIFIER_EXPRESSION: JsIdentifierExpression;
	readonly JS_IF_STATEMENT: JsIfStatement;
	readonly JS_IMPORT: JsImport;
	readonly JS_IMPORT_ASSERTION: JsImportAssertion;
	readonly JS_IMPORT_ASSERTION_ENTRY: JsImportAssertionEntry;
	readonly JS_IMPORT_BARE_CLAUSE: JsImportBareClause;
	readonly JS_IMPORT_CALL_EXPRESSION: JsImportCallExpression;
	readonly JS_IMPORT_COMBINED_CLAUSE: JsImportCombinedClause;
	readonly JS_IMPORT_DEFAULT_CLAUSE: JsImportDefaultClause;
	readonly JS_IMPORT_META_EXPRESSION: JsImportMetaExpression;
	readonly JS_IMPORT_NAMED_CLAUSE: JsImportNamedClause;
	readonly JS_IMPORT_NAMESPACE_CLAUSE: JsImportNamespaceClause;
	readonly JS_IN_EXPRESSION: JsInExpression;
	readonly JS_INITIALIZER_CLAUSE: JsInitializerClause;
	readonly JS_INSTANCEOF_EXPRESSION: JsInstanceofExpression;
	readonly JS_LABEL: JsLabel;
	readonly JS_LABELED_STATEMENT: JsLabeledStatement;
	readonly JS_LITERAL_EXPORT_NAME: JsLiteralExportName;
	readonly JS_LITERAL_MEMBER_NAME: JsLiteralMemberName;
	readonly JS_LOGICAL_EXPRESSION: JsLogicalExpression;
	readonly JS_METAVARIABLE: JsMetavariable;
	readonly JS_METHOD_CLASS_MEMBER: JsMethodClassMember;
	readonly JS_METHOD_OBJECT_MEMBER: JsMethodObjectMember;
	readonly JS_MODULE: JsModule;
	readonly JS_MODULE_SOURCE: JsModuleSource;
	readonly JS_NAME: JsName;
	readonly JS_NAMED_IMPORT_SPECIFIER: JsNamedImportSpecifier;
	readonly JS_NAMED_IMPORT_SPECIFIERS: JsNamedImportSpecifiers;
	readonly JS_NAMESPACE_IMPORT_SPECIFIER: JsNamespaceImportSpecifier;
	readonly JS_NEW_EXPRESSION: JsNewExpression;
	readonly JS_NEW_TARGET_EXPRESSION: JsNewTargetExpression;
	readonly JS_NULL_LITERAL_EXPRESSION: JsNullLiteralExpression;
	readonly JS_NUMBER_LITERAL_EXPRESSION: JsNumberLiteralExpression;
	readonly JS_OBJECT_ASSIGNMENT_PATTERN: JsObjectAssignmentPattern;
	readonly JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY: JsObjectAssignmentPatternProperty;
	readonly JS_OBJECT_ASSIGNMENT_PATTERN_REST: JsObjectAssignmentPatternRest;
	readonly JS_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY: JsObjectAssignmentPatternShorthandProperty;
	readonly JS_OBJECT_BINDING_PATTERN: JsObjectBindingPattern;
	readonly JS_OBJECT_BINDING_PATTERN_PROPERTY: JsObjectBindingPatternProperty;
	readonly JS_OBJECT_BINDING_PATTERN_REST: JsObjectBindingPatternRest;
	readonly JS_OBJECT_BINDING_PATTERN_SHORTHAND_PROPERTY: JsObjectBindingPatternShorthandProperty;
	readonly JS_OBJECT_EXPRESSION: JsObjectExpression;
	readonly JS_PARAMETERS: JsParameters;
	readonly JS_PARENTHESIZED_ASSIGNMENT: JsParenthesizedAssignment;
	readonly JS_PARENTHESIZED_EXPRESSION: JsParenthesizedExpression;
	readonly JS_POST_UPDATE_EXPRESSION: JsPostUpdateExpression;
	readonly JS_PRE_UPDATE_EXPRESSION: JsPreUpdateExpression;
	readonly JS_PRIVATE_CLASS_MEMBER_NAME: JsPrivateClassMemberName;
	readonly JS_PRIVATE_NAME: JsPrivateName;
	readonly JS_PROPERTY_CLASS_MEMBER: JsPropertyClassMember;
	readonly JS_PROPERTY_OBJECT_MEMBER: JsPropertyObjectMember;
	readonly JS_REFERENCE_IDENTIFIER: JsReferenceIdentifier;
	readonly JS_REGEX_LITERAL_EXPRESSION: JsRegexLiteralExpression;
	readonly JS_REST_PARAMETER: JsRestParameter;
	readonly JS_RETURN_STATEMENT: JsReturnStatement;
	readonly JS_SCRIPT: JsScript;
	readonly JS_SEQUENCE_EXPRESSION: JsSequenceExpression;
	readonly JS_SETTER_CLASS_MEMBER: JsSetterClassMember;
	readonly JS_SETTER_OBJECT_MEMBER: JsSetterObjectMember;
	readonly JS_SHORTHAND_NAMED_IMPORT_SPECIFIER: JsShorthandNamedImportSpecifier;
	readonly JS_SHORTHAND_PROPERTY_OBJECT_MEMBER: JsShorthandPropertyObjectMember;
	readonly JS_SPREAD: JsSpread;
	readonly JS_STATIC_INITIALIZATION_BLOCK_CLASS_MEMBER: JsStaticInitializationBlockClassMember;
	readonly JS_STATIC_MEMBER_ASSIGNMENT: JsStaticMemberAssignment;
	readonly JS_STATIC_MEMBER_EXPRESSION: JsStaticMemberExpression;
	readonly JS_STATIC_MODIFIER: JsStaticModifier;
	readonly JS_STRING_LITERAL_EXPRESSION: JsStringLiteralExpression;
	readonly JS_SUPER_EXPRESSION: JsSuperExpression;
	readonly JS_SVELTE_DECLARATION_ROOT: JsSvelteDeclarationRoot;
	readonly JS_SVELTE_SNIPPET_ROOT: JsSvelteSnippetRoot;
	readonly JS_SWITCH_STATEMENT: JsSwitchStatement;
	readonly JS_TEMPLATE_CHUNK_ELEMENT: JsTemplateChunkElement;
	readonly JS_TEMPLATE_ELEMENT: JsTemplateElement;
	readonly JS_TEMPLATE_EXPRESSION: JsTemplateExpression;
	readonly JS_THIS_EXPRESSION: JsThisExpression;
	readonly JS_THROW_STATEMENT: JsThrowStatement;
	readonly JS_TRY_FINALLY_STATEMENT: JsTryFinallyStatement;
	readonly JS_TRY_STATEMENT: JsTryStatement;
	readonly JS_UNARY_EXPRESSION: JsUnaryExpression;
	readonly JS_VARIABLE_DECLARATION: JsVariableDeclaration;
	readonly JS_VARIABLE_DECLARATION_CLAUSE: JsVariableDeclarationClause;
	readonly JS_VARIABLE_DECLARATOR: JsVariableDeclarator;
	readonly JS_VARIABLE_STATEMENT: JsVariableStatement;
	readonly JS_WHILE_STATEMENT: JsWhileStatement;
	readonly JS_WITH_STATEMENT: JsWithStatement;
	readonly JS_YIELD_ARGUMENT: JsYieldArgument;
	readonly JS_YIELD_EXPRESSION: JsYieldExpression;
	readonly JSX_ATTRIBUTE: JsxAttribute;
	readonly JSX_ATTRIBUTE_INITIALIZER_CLAUSE: JsxAttributeInitializerClause;
	readonly JSX_CLOSING_ELEMENT: JsxClosingElement;
	readonly JSX_CLOSING_FRAGMENT: JsxClosingFragment;
	readonly JSX_ELEMENT: JsxElement;
	readonly JSX_EXPRESSION_ATTRIBUTE_VALUE: JsxExpressionAttributeValue;
	readonly JSX_EXPRESSION_CHILD: JsxExpressionChild;
	readonly JSX_FRAGMENT: JsxFragment;
	readonly JSX_MEMBER_NAME: JsxMemberName;
	readonly JSX_NAME: JsxName;
	readonly JSX_NAMESPACE_NAME: JsxNamespaceName;
	readonly JSX_OPENING_ELEMENT: JsxOpeningElement;
	readonly JSX_OPENING_FRAGMENT: JsxOpeningFragment;
	readonly JSX_REFERENCE_IDENTIFIER: JsxReferenceIdentifier;
	readonly JSX_SELF_CLOSING_ELEMENT: JsxSelfClosingElement;
	readonly JSX_SHORTHAND_ATTRIBUTE: JsxShorthandAttribute;
	readonly JSX_SPREAD_ATTRIBUTE: JsxSpreadAttribute;
	readonly JSX_SPREAD_CHILD: JsxSpreadChild;
	readonly JSX_STRING: JsxString;
	readonly JSX_TAG_EXPRESSION: JsxTagExpression;
	readonly JSX_TEXT: JsxText;
	readonly TS_ABSTRACT_MODIFIER: TsAbstractModifier;
	readonly TS_ACCESSIBILITY_MODIFIER: TsAccessibilityModifier;
	readonly TS_ANY_TYPE: TsAnyType;
	readonly TS_ARRAY_TYPE: TsArrayType;
	readonly TS_AS_ASSIGNMENT: TsAsAssignment;
	readonly TS_AS_EXPRESSION: TsAsExpression;
	readonly TS_ASSERTS_CONDITION: TsAssertsCondition;
	readonly TS_ASSERTS_RETURN_TYPE: TsAssertsReturnType;
	readonly TS_BIGINT_LITERAL_TYPE: TsBigintLiteralType;
	readonly TS_BIGINT_TYPE: TsBigintType;
	readonly TS_BOOLEAN_LITERAL_TYPE: TsBooleanLiteralType;
	readonly TS_BOOLEAN_TYPE: TsBooleanType;
	readonly TS_CALL_SIGNATURE_TYPE_MEMBER: TsCallSignatureTypeMember;
	readonly TS_CONDITIONAL_TYPE: TsConditionalType;
	readonly TS_CONST_MODIFIER: TsConstModifier;
	readonly TS_CONSTRUCT_SIGNATURE_TYPE_MEMBER: TsConstructSignatureTypeMember;
	readonly TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER: TsConstructorSignatureClassMember;
	readonly TS_CONSTRUCTOR_TYPE: TsConstructorType;
	readonly TS_DECLARATION_MODULE: TsDeclarationModule;
	readonly TS_DECLARE_FUNCTION_DECLARATION: TsDeclareFunctionDeclaration;
	readonly TS_DECLARE_FUNCTION_EXPORT_DEFAULT_DECLARATION: TsDeclareFunctionExportDefaultDeclaration;
	readonly TS_DECLARE_MODIFIER: TsDeclareModifier;
	readonly TS_DECLARE_STATEMENT: TsDeclareStatement;
	readonly TS_DEFAULT_TYPE_CLAUSE: TsDefaultTypeClause;
	readonly TS_DEFINITE_PROPERTY_ANNOTATION: TsDefinitePropertyAnnotation;
	readonly TS_DEFINITE_VARIABLE_ANNOTATION: TsDefiniteVariableAnnotation;
	readonly TS_EMPTY_EXTERNAL_MODULE_DECLARATION_BODY: TsEmptyExternalModuleDeclarationBody;
	readonly TS_ENUM_DECLARATION: TsEnumDeclaration;
	readonly TS_ENUM_MEMBER: TsEnumMember;
	readonly TS_EXPORT_AS_NAMESPACE_CLAUSE: TsExportAsNamespaceClause;
	readonly TS_EXPORT_ASSIGNMENT_CLAUSE: TsExportAssignmentClause;
	readonly TS_EXPORT_DECLARE_CLAUSE: TsExportDeclareClause;
	readonly TS_EXTENDS_CLAUSE: TsExtendsClause;
	readonly TS_EXTERNAL_MODULE_DECLARATION: TsExternalModuleDeclaration;
	readonly TS_EXTERNAL_MODULE_REFERENCE: TsExternalModuleReference;
	readonly TS_FUNCTION_TYPE: TsFunctionType;
	readonly TS_GETTER_SIGNATURE_CLASS_MEMBER: TsGetterSignatureClassMember;
	readonly TS_GETTER_SIGNATURE_TYPE_MEMBER: TsGetterSignatureTypeMember;
	readonly TS_GLOBAL_DECLARATION: TsGlobalDeclaration;
	readonly TS_IDENTIFIER_BINDING: TsIdentifierBinding;
	readonly TS_IMPLEMENTS_CLAUSE: TsImplementsClause;
	readonly TS_IMPORT_EQUALS_DECLARATION: TsImportEqualsDeclaration;
	readonly TS_IMPORT_TYPE: TsImportType;
	readonly TS_IMPORT_TYPE_ARGUMENTS: TsImportTypeArguments;
	readonly TS_IMPORT_TYPE_ASSERTION: TsImportTypeAssertion;
	readonly TS_IMPORT_TYPE_ASSERTION_BLOCK: TsImportTypeAssertionBlock;
	readonly TS_IMPORT_TYPE_QUALIFIER: TsImportTypeQualifier;
	readonly TS_IN_MODIFIER: TsInModifier;
	readonly TS_INDEX_SIGNATURE_CLASS_MEMBER: TsIndexSignatureClassMember;
	readonly TS_INDEX_SIGNATURE_PARAMETER: TsIndexSignatureParameter;
	readonly TS_INDEX_SIGNATURE_TYPE_MEMBER: TsIndexSignatureTypeMember;
	readonly TS_INDEXED_ACCESS_TYPE: TsIndexedAccessType;
	readonly TS_INFER_TYPE: TsInferType;
	readonly TS_INITIALIZED_PROPERTY_SIGNATURE_CLASS_MEMBER: TsInitializedPropertySignatureClassMember;
	readonly TS_INSTANTIATION_EXPRESSION: TsInstantiationExpression;
	readonly TS_INTERFACE_DECLARATION: TsInterfaceDeclaration;
	readonly TS_INTERSECTION_TYPE: TsIntersectionType;
	readonly TS_LITERAL_ENUM_MEMBER_NAME: TsLiteralEnumMemberName;
	readonly TS_MAPPED_TYPE: TsMappedType;
	readonly TS_MAPPED_TYPE_AS_CLAUSE: TsMappedTypeAsClause;
	readonly TS_MAPPED_TYPE_OPTIONAL_MODIFIER_CLAUSE: TsMappedTypeOptionalModifierClause;
	readonly TS_MAPPED_TYPE_READONLY_MODIFIER_CLAUSE: TsMappedTypeReadonlyModifierClause;
	readonly TS_METHOD_SIGNATURE_CLASS_MEMBER: TsMethodSignatureClassMember;
	readonly TS_METHOD_SIGNATURE_TYPE_MEMBER: TsMethodSignatureTypeMember;
	readonly TS_MODULE_BLOCK: TsModuleBlock;
	readonly TS_MODULE_DECLARATION: TsModuleDeclaration;
	readonly TS_NAMED_TUPLE_TYPE_ELEMENT: TsNamedTupleTypeElement;
	readonly TS_NEVER_TYPE: TsNeverType;
	readonly TS_NON_NULL_ASSERTION_ASSIGNMENT: TsNonNullAssertionAssignment;
	readonly TS_NON_NULL_ASSERTION_EXPRESSION: TsNonNullAssertionExpression;
	readonly TS_NON_PRIMITIVE_TYPE: TsNonPrimitiveType;
	readonly TS_NULL_LITERAL_TYPE: TsNullLiteralType;
	readonly TS_NUMBER_LITERAL_TYPE: TsNumberLiteralType;
	readonly TS_NUMBER_TYPE: TsNumberType;
	readonly TS_OBJECT_TYPE: TsObjectType;
	readonly TS_OPTIONAL_PROPERTY_ANNOTATION: TsOptionalPropertyAnnotation;
	readonly TS_OPTIONAL_TUPLE_TYPE_ELEMENT: TsOptionalTupleTypeElement;
	readonly TS_OUT_MODIFIER: TsOutModifier;
	readonly TS_OVERRIDE_MODIFIER: TsOverrideModifier;
	readonly TS_PARENTHESIZED_TYPE: TsParenthesizedType;
	readonly TS_PREDICATE_RETURN_TYPE: TsPredicateReturnType;
	readonly TS_PROPERTY_PARAMETER: TsPropertyParameter;
	readonly TS_PROPERTY_SIGNATURE_CLASS_MEMBER: TsPropertySignatureClassMember;
	readonly TS_PROPERTY_SIGNATURE_TYPE_MEMBER: TsPropertySignatureTypeMember;
	readonly TS_QUALIFIED_MODULE_NAME: TsQualifiedModuleName;
	readonly TS_QUALIFIED_NAME: TsQualifiedName;
	readonly TS_READONLY_MODIFIER: TsReadonlyModifier;
	readonly TS_REFERENCE_TYPE: TsReferenceType;
	readonly TS_REST_TUPLE_TYPE_ELEMENT: TsRestTupleTypeElement;
	readonly TS_RETURN_TYPE_ANNOTATION: TsReturnTypeAnnotation;
	readonly TS_SATISFIES_ASSIGNMENT: TsSatisfiesAssignment;
	readonly TS_SATISFIES_EXPRESSION: TsSatisfiesExpression;
	readonly TS_SETTER_SIGNATURE_CLASS_MEMBER: TsSetterSignatureClassMember;
	readonly TS_SETTER_SIGNATURE_TYPE_MEMBER: TsSetterSignatureTypeMember;
	readonly TS_STRING_LITERAL_TYPE: TsStringLiteralType;
	readonly TS_STRING_TYPE: TsStringType;
	readonly TS_SYMBOL_TYPE: TsSymbolType;
	readonly TS_TEMPLATE_CHUNK_ELEMENT: TsTemplateChunkElement;
	readonly TS_TEMPLATE_ELEMENT: TsTemplateElement;
	readonly TS_TEMPLATE_LITERAL_TYPE: TsTemplateLiteralType;
	readonly TS_THIS_PARAMETER: TsThisParameter;
	readonly TS_THIS_TYPE: TsThisType;
	readonly TS_TUPLE_TYPE: TsTupleType;
	readonly TS_TYPE_ALIAS_DECLARATION: TsTypeAliasDeclaration;
	readonly TS_TYPE_ANNOTATION: TsTypeAnnotation;
	readonly TS_TYPE_ARGUMENTS: TsTypeArguments;
	readonly TS_TYPE_ASSERTION_ASSIGNMENT: TsTypeAssertionAssignment;
	readonly TS_TYPE_ASSERTION_EXPRESSION: TsTypeAssertionExpression;
	readonly TS_TYPE_CONSTRAINT_CLAUSE: TsTypeConstraintClause;
	readonly TS_TYPE_OPERATOR_TYPE: TsTypeOperatorType;
	readonly TS_TYPE_PARAMETER: TsTypeParameter;
	readonly TS_TYPE_PARAMETER_NAME: TsTypeParameterName;
	readonly TS_TYPE_PARAMETERS: TsTypeParameters;
	readonly TS_TYPEOF_TYPE: TsTypeofType;
	readonly TS_UNDEFINED_TYPE: TsUndefinedType;
	readonly TS_UNION_TYPE: TsUnionType;
	readonly TS_UNKNOWN_TYPE: TsUnknownType;
	readonly TS_VOID_TYPE: TsVoidType;
	readonly JS_BOGUS: JsBogus;
	readonly JS_BOGUS_ASSIGNMENT: JsBogusAssignment;
	readonly JS_BOGUS_BINDING: JsBogusBinding;
	readonly JS_BOGUS_EXPRESSION: JsBogusExpression;
	readonly JS_BOGUS_IMPORT_ASSERTION_ENTRY: JsBogusImportAssertionEntry;
	readonly JS_BOGUS_MEMBER: JsBogusMember;
	readonly JS_BOGUS_NAMED_IMPORT_SPECIFIER: JsBogusNamedImportSpecifier;
	readonly JS_BOGUS_PARAMETER: JsBogusParameter;
	readonly JS_BOGUS_STATEMENT: JsBogusStatement;
	readonly JS_BOGUS_VARIABLE_DECLARATION: JsBogusVariableDeclaration;
	readonly TS_BOGUS_TYPE: TsBogusType;
	readonly JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST: JsArrayAssignmentPatternElementListNode;
	readonly JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST: JsArrayBindingPatternElementListNode;
	readonly JS_ARRAY_ELEMENT_LIST: JsArrayElementListNode;
	readonly JS_CALL_ARGUMENT_LIST: JsCallArgumentListNode;
	readonly JS_CLASS_MEMBER_LIST: JsClassMemberListNode;
	readonly JS_CONSTRUCTOR_MODIFIER_LIST: JsConstructorModifierListNode;
	readonly JS_CONSTRUCTOR_PARAMETER_LIST: JsConstructorParameterListNode;
	readonly JS_DECORATOR_LIST: JsDecoratorListNode;
	readonly JS_DIRECTIVE_LIST: JsDirectiveListNode;
	readonly JS_EXPORT_NAMED_FROM_SPECIFIER_LIST: JsExportNamedFromSpecifierListNode;
	readonly JS_EXPORT_NAMED_SPECIFIER_LIST: JsExportNamedSpecifierListNode;
	readonly JS_IMPORT_ASSERTION_ENTRY_LIST: JsImportAssertionEntryListNode;
	readonly JS_METHOD_MODIFIER_LIST: JsMethodModifierListNode;
	readonly JS_MODULE_ITEM_LIST: JsModuleItemListNode;
	readonly JS_NAMED_IMPORT_SPECIFIER_LIST: JsNamedImportSpecifierListNode;
	readonly JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST: JsObjectAssignmentPatternPropertyListNode;
	readonly JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST: JsObjectBindingPatternPropertyListNode;
	readonly JS_OBJECT_MEMBER_LIST: JsObjectMemberListNode;
	readonly JS_PARAMETER_LIST: JsParameterListNode;
	readonly JS_PROPERTY_MODIFIER_LIST: JsPropertyModifierListNode;
	readonly JS_STATEMENT_LIST: JsStatementListNode;
	readonly JS_SWITCH_CASE_LIST: JsSwitchCaseListNode;
	readonly JS_TEMPLATE_ELEMENT_LIST: JsTemplateElementListNode;
	readonly JS_VARIABLE_DECLARATOR_LIST: JsVariableDeclaratorListNode;
	readonly JSX_ATTRIBUTE_LIST: JsxAttributeListNode;
	readonly JSX_CHILD_LIST: JsxChildListNode;
	readonly TS_ENUM_MEMBER_LIST: TsEnumMemberListNode;
	readonly TS_INDEX_SIGNATURE_MODIFIER_LIST: TsIndexSignatureModifierListNode;
	readonly TS_INTERSECTION_TYPE_ELEMENT_LIST: TsIntersectionTypeElementListNode;
	readonly TS_METHOD_SIGNATURE_MODIFIER_LIST: TsMethodSignatureModifierListNode;
	readonly TS_PROPERTY_PARAMETER_MODIFIER_LIST: TsPropertyParameterModifierListNode;
	readonly TS_PROPERTY_SIGNATURE_MODIFIER_LIST: TsPropertySignatureModifierListNode;
	readonly TS_TEMPLATE_ELEMENT_LIST: TsTemplateElementListNode;
	readonly TS_TUPLE_TYPE_ELEMENT_LIST: TsTupleTypeElementListNode;
	readonly TS_TYPE_ARGUMENT_LIST: TsTypeArgumentListNode;
	readonly TS_TYPE_LIST: TsTypeListNode;
	readonly TS_TYPE_MEMBER_LIST: TsTypeMemberListNode;
	readonly TS_TYPE_PARAMETER_LIST: TsTypeParameterListNode;
	readonly TS_TYPE_PARAMETER_MODIFIER_LIST: TsTypeParameterModifierListNode;
	readonly TS_UNION_TYPE_VARIANT_LIST: TsUnionTypeVariantListNode;
}
export interface JsArrayAssignmentPatternElementListNode extends JsAstNode {
	readonly kind: "JS_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST";
}
export type JsArrayAssignmentPatternElementList =
	readonly AnyJsArrayAssignmentPatternElement[];
export interface JsArrayBindingPatternElementListNode extends JsAstNode {
	readonly kind: "JS_ARRAY_BINDING_PATTERN_ELEMENT_LIST";
}
export type JsArrayBindingPatternElementList =
	readonly AnyJsArrayBindingPatternElement[];
export interface JsArrayElementListNode extends JsAstNode {
	readonly kind: "JS_ARRAY_ELEMENT_LIST";
}
export type JsArrayElementList = readonly AnyJsArrayElement[];
export interface JsCallArgumentListNode extends JsAstNode {
	readonly kind: "JS_CALL_ARGUMENT_LIST";
}
export type JsCallArgumentList = readonly AnyJsCallArgument[];
export interface JsClassMemberListNode extends JsAstNode {
	readonly kind: "JS_CLASS_MEMBER_LIST";
}
export type JsClassMemberList = readonly AnyJsClassMember[];
export interface JsConstructorModifierListNode extends JsAstNode {
	readonly kind: "JS_CONSTRUCTOR_MODIFIER_LIST";
}
export type JsConstructorModifierList = readonly TsAccessibilityModifier[];
export interface JsConstructorParameterListNode extends JsAstNode {
	readonly kind: "JS_CONSTRUCTOR_PARAMETER_LIST";
}
export type JsConstructorParameterList = readonly AnyJsConstructorParameter[];
export interface JsDecoratorListNode extends JsAstNode {
	readonly kind: "JS_DECORATOR_LIST";
}
export type JsDecoratorList = readonly JsDecorator[];
export interface JsDirectiveListNode extends JsAstNode {
	readonly kind: "JS_DIRECTIVE_LIST";
}
export type JsDirectiveList = readonly JsDirective[];
export interface JsExportNamedFromSpecifierListNode extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_FROM_SPECIFIER_LIST";
}
export type JsExportNamedFromSpecifierList =
	readonly JsExportNamedFromSpecifier[];
export interface JsExportNamedSpecifierListNode extends JsAstNode {
	readonly kind: "JS_EXPORT_NAMED_SPECIFIER_LIST";
}
export type JsExportNamedSpecifierList = readonly AnyJsExportNamedSpecifier[];
export interface JsImportAssertionEntryListNode extends JsAstNode {
	readonly kind: "JS_IMPORT_ASSERTION_ENTRY_LIST";
}
export type JsImportAssertionEntryList = readonly AnyJsImportAssertionEntry[];
export interface JsMethodModifierListNode extends JsAstNode {
	readonly kind: "JS_METHOD_MODIFIER_LIST";
}
export type JsMethodModifierList = readonly AnyJsMethodModifier[];
export interface JsModuleItemListNode extends JsAstNode {
	readonly kind: "JS_MODULE_ITEM_LIST";
}
export type JsModuleItemList = readonly AnyJsModuleItem[];
export interface JsNamedImportSpecifierListNode extends JsAstNode {
	readonly kind: "JS_NAMED_IMPORT_SPECIFIER_LIST";
}
export type JsNamedImportSpecifierList = readonly AnyJsNamedImportSpecifier[];
export interface JsObjectAssignmentPatternPropertyListNode extends JsAstNode {
	readonly kind: "JS_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST";
}
export type JsObjectAssignmentPatternPropertyList =
	readonly AnyJsObjectAssignmentPatternMember[];
export interface JsObjectBindingPatternPropertyListNode extends JsAstNode {
	readonly kind: "JS_OBJECT_BINDING_PATTERN_PROPERTY_LIST";
}
export type JsObjectBindingPatternPropertyList =
	readonly AnyJsObjectBindingPatternMember[];
export interface JsObjectMemberListNode extends JsAstNode {
	readonly kind: "JS_OBJECT_MEMBER_LIST";
}
export type JsObjectMemberList = readonly AnyJsObjectMember[];
export interface JsParameterListNode extends JsAstNode {
	readonly kind: "JS_PARAMETER_LIST";
}
export type JsParameterList = readonly AnyJsParameter[];
export interface JsPropertyModifierListNode extends JsAstNode {
	readonly kind: "JS_PROPERTY_MODIFIER_LIST";
}
export type JsPropertyModifierList = readonly AnyJsPropertyModifier[];
export interface JsStatementListNode extends JsAstNode {
	readonly kind: "JS_STATEMENT_LIST";
}
export type JsStatementList = readonly AnyJsStatement[];
export interface JsSwitchCaseListNode extends JsAstNode {
	readonly kind: "JS_SWITCH_CASE_LIST";
}
export type JsSwitchCaseList = readonly AnyJsSwitchClause[];
export interface JsTemplateElementListNode extends JsAstNode {
	readonly kind: "JS_TEMPLATE_ELEMENT_LIST";
}
export type JsTemplateElementList = readonly AnyJsTemplateElement[];
export interface JsVariableDeclaratorListNode extends JsAstNode {
	readonly kind: "JS_VARIABLE_DECLARATOR_LIST";
}
export type JsVariableDeclaratorList = readonly JsVariableDeclarator[];
export interface JsxAttributeListNode extends JsAstNode {
	readonly kind: "JSX_ATTRIBUTE_LIST";
}
export type JsxAttributeList = readonly AnyJsxAttribute[];
export interface JsxChildListNode extends JsAstNode {
	readonly kind: "JSX_CHILD_LIST";
}
export type JsxChildList = readonly AnyJsxChild[];
export interface TsEnumMemberListNode extends JsAstNode {
	readonly kind: "TS_ENUM_MEMBER_LIST";
}
export type TsEnumMemberList = readonly TsEnumMember[];
export interface TsIndexSignatureModifierListNode extends JsAstNode {
	readonly kind: "TS_INDEX_SIGNATURE_MODIFIER_LIST";
}
export type TsIndexSignatureModifierList =
	readonly AnyTsIndexSignatureModifier[];
export interface TsIntersectionTypeElementListNode extends JsAstNode {
	readonly kind: "TS_INTERSECTION_TYPE_ELEMENT_LIST";
}
export type TsIntersectionTypeElementList = readonly AnyTsType[];
export interface TsMethodSignatureModifierListNode extends JsAstNode {
	readonly kind: "TS_METHOD_SIGNATURE_MODIFIER_LIST";
}
export type TsMethodSignatureModifierList =
	readonly AnyTsMethodSignatureModifier[];
export interface TsPropertyParameterModifierListNode extends JsAstNode {
	readonly kind: "TS_PROPERTY_PARAMETER_MODIFIER_LIST";
}
export type TsPropertyParameterModifierList =
	readonly AnyTsPropertyParameterModifier[];
export interface TsPropertySignatureModifierListNode extends JsAstNode {
	readonly kind: "TS_PROPERTY_SIGNATURE_MODIFIER_LIST";
}
export type TsPropertySignatureModifierList =
	readonly AnyTsPropertySignatureModifier[];
export interface TsTemplateElementListNode extends JsAstNode {
	readonly kind: "TS_TEMPLATE_ELEMENT_LIST";
}
export type TsTemplateElementList = readonly AnyTsTemplateElement[];
export interface TsTupleTypeElementListNode extends JsAstNode {
	readonly kind: "TS_TUPLE_TYPE_ELEMENT_LIST";
}
export type TsTupleTypeElementList = readonly AnyTsTupleTypeElement[];
export interface TsTypeArgumentListNode extends JsAstNode {
	readonly kind: "TS_TYPE_ARGUMENT_LIST";
}
export type TsTypeArgumentList = readonly AnyTsType[];
export interface TsTypeListNode extends JsAstNode {
	readonly kind: "TS_TYPE_LIST";
}
export type TsTypeList = readonly TsReferenceType[];
export interface TsTypeMemberListNode extends JsAstNode {
	readonly kind: "TS_TYPE_MEMBER_LIST";
}
export type TsTypeMemberList = readonly AnyTsTypeMember[];
export interface TsTypeParameterListNode extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETER_LIST";
}
export type TsTypeParameterList = readonly TsTypeParameter[];
export interface TsTypeParameterModifierListNode extends JsAstNode {
	readonly kind: "TS_TYPE_PARAMETER_MODIFIER_LIST";
}
export type TsTypeParameterModifierList = readonly AnyTsTypeParameterModifier[];
export interface TsUnionTypeVariantListNode extends JsAstNode {
	readonly kind: "TS_UNION_TYPE_VARIANT_LIST";
}
export type TsUnionTypeVariantList = readonly AnyTsType[];
