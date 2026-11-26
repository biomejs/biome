import {
	type AnyJsAstNode,
	type JsCallArgumentListNode,
	type JsCallExpression,
	type JsModule,
	type JsNewExpression,
	type JsNodeByKind,
	type NodeKindSet,
	nodeKinds,
	tokenKinds,
} from "@biomejs/runtime/plugin";

declare const root: JsModule;
declare const node: AnyJsAstNode;

const calls = nodeKinds("JS_CALL_EXPRESSION", "JS_NEW_EXPRESSION");
const lists = nodeKinds("JS_CALL_ARGUMENT_LIST");
const empty = nodeKinds();

root.descendants(calls) satisfies readonly (
	| JsCallExpression
	| JsNewExpression
)[];
node.descendants(lists) satisfies readonly JsCallArgumentListNode[];
root.descendants(empty) satisfies readonly never[];

for (const call of root.descendants(calls)) {
	call.kind satisfies "JS_CALL_EXPRESSION" | "JS_NEW_EXPRESSION";
	if (call.kind === "JS_CALL_EXPRESSION") {
		call satisfies JsCallExpression;
		call.callee;
	} else {
		call satisfies JsNewExpression;
	}
	// @ts-expect-error Matches narrow to the kinds in the set.
	call satisfies JsCallExpression;
}

// @ts-expect-error The result is not a mutable array.
root.descendants(calls) satisfies AnyJsAstNode[];
// @ts-expect-error The result cannot be appended to.
root.descendants(calls).push(node);

root.walk(calls, (call) => {
	call satisfies JsCallExpression | JsNewExpression;
}) satisfies void;
root.walk(
	calls,
	(call) => {
		call.kind satisfies "JS_CALL_EXPRESSION" | "JS_NEW_EXPRESSION";
		return false;
	},
	(call) => {
		call satisfies JsCallExpression | JsNewExpression;
	},
);
root.walk(lists, () => true, undefined);
// Any return value other than false continues the walk.
const stack: AnyJsAstNode[] = [];
root.walk(
	calls,
	(call) => stack.push(call),
	() => stack.pop(),
);
// enter may return false on some paths and nothing on others.
root.walk(calls, (call) => {
	if (call.kind === "JS_NEW_EXPRESSION") return false;
});

// Functions that accept any set are generic over its kinds.
function count<K extends keyof JsNodeByKind>(
	set: NodeKindSet<K>,
	from: AnyJsAstNode,
): number {
	let total = 0;
	from.walk(set, (match) => {
		match satisfies JsNodeByKind[K];
		total++;
	});
	return from.descendants(set).length + total;
}
count(calls, root) satisfies number;

// @ts-expect-error Kind strings are passed to nodeKinds(), not to descendants().
root.descendants("JS_CALL_EXPRESSION");
// @ts-expect-error Kind arrays are passed to nodeKinds(), not to descendants().
root.descendants(["JS_CALL_EXPRESSION"]);
// @ts-expect-error Token kind sets do not select nodes.
root.descendants(tokenKinds("IDENT"));
// @ts-expect-error A kind set is required.
root.descendants();
// @ts-expect-error Token kind sets do not select nodes.
root.walk(tokenKinds("IDENT"), () => {});
// @ts-expect-error An enter function is required.
root.walk(calls);
root.walk(
	calls,
	// @ts-expect-error enter receives only the kinds in the set.
	(call: JsModule) => {},
);
// @ts-expect-error exit must be a function.
root.walk(calls, () => {}, null);
