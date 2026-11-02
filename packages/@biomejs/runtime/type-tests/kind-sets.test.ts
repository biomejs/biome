import {
	type AnyJsAstNode,
	type JsAstElement,
	type JsAstToken,
	type JsCallExpression,
	type JsNewExpression,
	type JsNodeByKind,
	type NodeKindSet,
	nodeKinds,
	type TokenKindSet,
	tokenKinds,
} from "@biomejs/runtime/plugin";

const calls = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
const equality = tokenKinds("EQ2", "NEQ");
const sourceTokens = tokenKinds("EOF", "ERROR_TOKEN");
const empty = nodeKinds();

calls satisfies NodeKindSet<"JS_CALL_EXPRESSION" | "JS_NEW_EXPRESSION">;
equality satisfies TokenKindSet<"EQ2" | "NEQ">;
sourceTokens satisfies TokenKindSet<"EOF" | "ERROR_TOKEN">;
empty satisfies NodeKindSet<never>;

declare const element: JsAstElement;
declare const node: AnyJsAstNode;
declare const token: JsAstToken;
declare const optional: AnyJsAstNode | undefined;

calls.has(element) satisfies boolean;
calls.has(token) satisfies boolean;
calls.has(optional) satisfies boolean;
equality.has(node) satisfies boolean;
equality.has(undefined) satisfies boolean;

if (calls.has(element)) {
	element satisfies JsCallExpression | JsNewExpression;
	element.kind satisfies "JS_CALL_EXPRESSION" | "JS_NEW_EXPRESSION";
	// @ts-expect-error Membership narrows to the kinds in the set.
	element satisfies JsCallExpression;
}

if (equality.has(element)) {
	element satisfies JsAstToken;
	element.kind satisfies "EQ2" | "NEQ";
	// @ts-expect-error Membership narrows to the kinds in the set.
	element.kind satisfies "EQ2";
}

// Non-members keep every type outside the set's kinds.
if (!calls.has(node)) {
	node.kind satisfies string;
	// @ts-expect-error A non-member can still be a node of another kind.
	node satisfies never;
}
if (!equality.has(element)) {
	// @ts-expect-error A non-member can still be a token of another kind.
	element satisfies AnyJsAstNode;
}

// Functions that accept any set are generic over its kinds.
function countMembers<K extends keyof JsNodeByKind>(
	set: NodeKindSet<K>,
	nodes: readonly AnyJsAstNode[],
): number {
	return nodes.filter((child) => set.has(child)).length;
}
function firstMember<K extends JsAstToken["kind"]>(
	set: TokenKindSet<K>,
	elements: readonly JsAstElement[],
) {
	for (const child of elements) {
		if (set.has(child)) return child;
	}
	return undefined;
}
countMembers(calls, [node]) satisfies number;
firstMember(equality, [element])?.kind satisfies "EQ2" | "NEQ" | undefined;

// @ts-expect-error Unknown node kinds are rejected.
nodeKinds("NOT_A_KIND");
// @ts-expect-error Token kinds do not build node kind sets.
nodeKinds("IDENT");
// @ts-expect-error Node kinds do not build token kind sets.
tokenKinds("JS_CALL_EXPRESSION");
// @ts-expect-error Kinds are passed as separate arguments, not an array.
nodeKinds(["JS_CALL_EXPRESSION"]);
// @ts-expect-error Membership is tested with nodes and tokens, not kind strings.
calls.has("JS_CALL_EXPRESSION");
// @ts-expect-error Membership is tested with nodes and tokens, not null.
calls.has(null);
// @ts-expect-error Node kind sets are not token kind sets.
calls satisfies TokenKindSet;
// @ts-expect-error Token kind sets are not node kind sets.
equality satisfies NodeKindSet;
// @ts-expect-error A set does not narrow to fewer kinds.
calls satisfies NodeKindSet<"JS_CALL_EXPRESSION">;
// @ts-expect-error A set does not widen to every node kind.
calls satisfies NodeKindSet;
// @ts-expect-error A set does not widen to every token kind.
equality satisfies TokenKindSet;
