import {
	type AnyJsAstNode,
	ast,
	type CodeFix,
	createMutation,
	defineRule,
	factory,
	type JsAstElement,
	type JsAstToken,
	type JsCallArgumentList,
	type JsCallArgumentListNode,
	type JsCallArguments,
	type JsCallExpression,
	type JsMutation,
	type JsTokenKind,
	type JsVariableDeclaration,
	type JsVariableStatement,
	registerDiagnostic,
} from "@biomejs/plugin-api";

declare const node: AnyJsAstNode;
declare const nextNode: AnyJsAstNode;
declare const token: JsAstToken;
declare const nextToken: JsAstToken;
declare const element: JsAstElement;
declare const nextElement: JsAstElement;
declare const optionalElement: JsAstElement | undefined;
declare const tokenKind: JsTokenKind;
declare const statement: JsVariableStatement;
declare const declaration: JsVariableDeclaration;
declare const call: JsCallExpression;
declare const args: JsCallArguments;
declare const list: JsCallArgumentListNode;

const mutation = createMutation(node);
mutation satisfies JsMutation;
mutation.replaceNode(node, nextNode) satisfies void;
mutation.replaceToken(token, nextToken) satisfies void;
mutation.replaceElement(node, nextNode) satisfies void;
mutation.replaceElement(token, nextToken) satisfies void;
mutation.replaceElement(element, nextElement) satisfies void;
mutation.removeNode(node) satisfies void;
mutation.removeToken(token) satisfies void;
mutation.removeElement(element) satisfies void;
createMutation(list) satisfies JsMutation;
mutation.replaceNode(list, list) satisfies void;

// @ts-expect-error A mutation requires a node anchor, not a token.
createMutation(token);
// @ts-expect-error Source text is not a node handle.
createMutation(node.text);
// @ts-expect-error A possibly absent anchor must be checked first.
createMutation(node.parent);
// @ts-expect-error Tokens cannot be node targets.
mutation.replaceNode(token, node);
// @ts-expect-error Node replacements cannot be tokens.
mutation.replaceNode(node, token);
// @ts-expect-error Nodes cannot be token targets.
mutation.replaceToken(node, token);
// @ts-expect-error Token replacements cannot be nodes.
mutation.replaceToken(token, node);
// @ts-expect-error Tokens must be removed with a token or element operation.
mutation.removeNode(token);
// @ts-expect-error Nodes must be removed with a node or element operation.
mutation.removeToken(node);
// @ts-expect-error Element replacements require handles, not text.
mutation.replaceElement(element, "replacement");
// @ts-expect-error Element targets require handles, not text.
mutation.removeElement("target");
// @ts-expect-error Arbitrary kind strings do not describe AST elements.
mutation.removeElement({ kind: "FAKE_KIND", text: "fake" });
// @ts-expect-error Optional node targets must be checked first.
mutation.replaceNode(statement.declaration, declaration);
// @ts-expect-error Optional token targets must be checked first.
mutation.removeToken(statement.token("semicolonToken"));
// @ts-expect-error Optional element targets must be checked first.
mutation.removeElement(optionalElement);
// @ts-expect-error Replacements cannot be absent; use a removal operation.
mutation.replaceElement(element, undefined);
// @ts-expect-error Mutations have no direct commit API.
mutation.commit();
// @ts-expect-error Mutation operations return void, not a chainable builder.
mutation.removeNode(node).removeNode(nextNode);

declare const fakeMutation: Pick<JsMutation, Extract<keyof JsMutation, string>>;
// @ts-expect-error An object with the six methods is not a native builder.
fakeMutation satisfies JsMutation;

factory.token("LET_KW") satisfies JsAstToken;
factory.token("LET_KW", "let") satisfies JsAstToken;
factory.token("SEMICOLON") satisfies JsAstToken;
factory.token("IDENT", "renamed") satisfies JsAstToken;
factory.token("JS_STRING_LITERAL", '"value"') satisfies JsAstToken;
factory.token(tokenKind, "text") satisfies JsAstToken;
token.kind satisfies JsAstToken["kind"];
token.text satisfies string;
// @ts-expect-error Factory methods cannot be replaced.
factory.token = () => token;
// @ts-expect-error Token kinds are readonly.
token.kind = "IDENT";
// @ts-expect-error Token text is readonly.
token.text = "changed";
// @ts-expect-error Canonical token kinds are not source spellings.
factory.token("let");
// @ts-expect-error Unknown token kinds are not in the factory allowlist.
factory.token("FAKE_KIND", "fake");
// @ts-expect-error Node kinds are not token kinds.
factory.token("JS_VARIABLE_STATEMENT");
// @ts-expect-error Trivia tokens are not in the factory allowlist.
factory.token("WHITESPACE", " ");
// @ts-expect-error EOF is not in the factory allowlist.
factory.token("EOF");
// @ts-expect-error Token text must be a string.
factory.token("IDENT", 123);

const kindToken = declaration.token("kindToken");
kindToken satisfies JsAstToken | undefined;
declaration.kindToken satisfies string | undefined;
if (kindToken) {
	mutation.replaceToken(kindToken, factory.token("LET_KW")) satisfies void;
}
node.childrenWithTokens() satisfies readonly JsAstElement[];
for (const child of node.childrenWithTokens()) {
	mutation.removeElement(child) satisfies void;
}
// @ts-expect-error Token-aware traversal returns a readonly array.
node.childrenWithTokens().push(token);

const updatedDeclaration = declaration.withKindToken(factory.token("LET_KW"));
updatedDeclaration satisfies JsVariableDeclaration;
updatedDeclaration.kind satisfies "JS_VARIABLE_DECLARATION";
mutation.replaceNode(declaration, updatedDeclaration) satisfies void;
statement.withDeclaration(updatedDeclaration) satisfies JsVariableStatement;
statement.withSemicolonToken(
	factory.token("SEMICOLON"),
) satisfies JsVariableStatement;
statement.withSemicolonToken(undefined) satisfies JsVariableStatement;
call.withArguments(args) satisfies JsCallExpression;
call.withTypeArguments(undefined) satisfies JsCallExpression;
args.withArgs(list) satisfies JsCallArguments;
args.args satisfies JsCallArgumentList;
// @ts-expect-error Updated nodes preserve their concrete type.
updatedDeclaration satisfies JsVariableStatement;
// @ts-expect-error Token updates require handles, not strings.
declaration.withKindToken("let");
// @ts-expect-error Required token fields cannot be cleared.
declaration.withKindToken(undefined);
// @ts-expect-error Required node fields cannot be cleared.
statement.withDeclaration(undefined);
// @ts-expect-error Node updates reject unrelated node categories.
statement.withDeclaration(args);
// @ts-expect-error List updates require a native wrapper, not a named-field array.
args.withArgs(args.args);
// @ts-expect-error List updates do not accept constructed arrays.
args.withArgs([]);
// @ts-expect-error Required list fields cannot be cleared.
args.withArgs(undefined);
// @ts-expect-error Named-field arrays are not mutation anchors.
createMutation(args.args);
// @ts-expect-error Named-field arrays are not mutation targets.
mutation.removeNode(args.args);
// @ts-expect-error Named-field arrays are not replacement elements.
mutation.replaceElement(list, args.args);
// @ts-expect-error Named token fields remain readonly strings.
declaration.kindToken = "let";
// @ts-expect-error Named node fields remain readonly.
statement.declaration = declaration;
// @ts-expect-error Named list fields remain readonly arrays.
args.args.push(args.args[0]);

const safeFix: CodeFix = {
	mutation,
	message: "Apply edit.",
	kind: "safe",
};
declare const optionalFix: CodeFix | undefined;
({ mutation, message: "Apply edit.", kind: "unsafe" }) satisfies CodeFix;
registerDiagnostic(node, "warning", "Diagnostic.") satisfies void;
registerDiagnostic(node, "warning", "Diagnostic.", safeFix) satisfies void;
registerDiagnostic(node, "warning", "Diagnostic.", optionalFix) satisfies void;
registerDiagnostic(node, "warning", "Diagnostic.", undefined) satisfies void;
// @ts-expect-error Fix kinds are limited to safe and unsafe.
({ mutation, message: "Edit.", kind: "invalid" }) satisfies CodeFix;
// @ts-expect-error Fix messages must be strings.
({ mutation, message: 123, kind: "safe" }) satisfies CodeFix;
// @ts-expect-error A fix requires a native mutation, not source text.
({ mutation: "edit", message: "Edit.", kind: "safe" }) satisfies CodeFix;
// @ts-expect-error Fixes require a mutation.
({ message: "Edit.", kind: "safe" }) satisfies CodeFix;
// @ts-expect-error Fix mutation references are readonly.
safeFix.mutation = mutation;
// @ts-expect-error Fix messages are readonly.
safeFix.message = "Changed.";
// @ts-expect-error Fix kinds are readonly.
safeFix.kind = "unsafe";
// @ts-expect-error Registration takes a CodeFix, not a bare mutation.
registerDiagnostic(node, "warning", "Diagnostic.", mutation);
// @ts-expect-error Optional diagnostic targets must be checked first.
registerDiagnostic(node.parent, "warning", "Diagnostic.", safeFix);
// @ts-expect-error Diagnostic targets cannot be tokens, even with a fix.
registerDiagnostic(token, "warning", "Diagnostic.", safeFix);

defineRule({
	query: ast("JS_VARIABLE_DECLARATION"),
	run(node) {
		const mutation = createMutation(node);
		const updated = node.withKindToken(factory.token("LET_KW"));
		mutation.replaceNode(node, updated);
		registerDiagnostic(node, "warning", "Use let.", {
			mutation,
			message: "Replace the declaration kind with let.",
			kind: "unsafe",
		}) satisfies void;
	},
});
