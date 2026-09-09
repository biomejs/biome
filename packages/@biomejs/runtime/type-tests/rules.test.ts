import {
	type AnyJsAstNode,
	ast,
	defineRule,
	type JsCallArgumentList,
	type JsCallArgumentListNode,
	type JsFunctionDeclaration,
	type JsModule,
	type JsVariableStatement,
	registerDiagnostic,
} from "@biomejs/runtime/plugin";

defineRule({
	query: ast("JS_VARIABLE_STATEMENT"),
	run(node) {
		node satisfies JsVariableStatement;
		node.kind satisfies "JS_VARIABLE_STATEMENT";
		// @ts-expect-error A single-kind query must not infer an unrelated node or any.
		node satisfies JsModule;
		registerDiagnostic(node, "warning", "Variable statement.") satisfies void;
	},
});

defineRule({
	query: ast("JS_VARIABLE_STATEMENT", "JS_FUNCTION_DECLARATION"),
	run(node) {
		node satisfies JsVariableStatement | JsFunctionDeclaration;
		if (node.kind === "JS_VARIABLE_STATEMENT") {
			node satisfies JsVariableStatement;
		} else {
			node satisfies JsFunctionDeclaration;
		}
		// @ts-expect-error A multi-kind query must preserve its union.
		node satisfies JsVariableStatement;
		registerDiagnostic(node, "information", "Matched node.");
	},
});

defineRule({
	query: ast("JS_CALL_ARGUMENT_LIST"),
	run(list) {
		list satisfies JsCallArgumentListNode;
		list.kind satisfies "JS_CALL_ARGUMENT_LIST";
		list.children() satisfies readonly AnyJsAstNode[];
		// @ts-expect-error A list query infers a wrapper, not a named-field array.
		list satisfies JsCallArgumentList;
		// @ts-expect-error A list query must not infer an unrelated node or any.
		list satisfies JsModule;
		registerDiagnostic(list, "warning", "Argument list.") satisfies void;
	},
});

defineRule({
	query: ast("JS_CALL_ARGUMENT_LIST", "JS_VARIABLE_STATEMENT"),
	run(node) {
		node satisfies JsCallArgumentListNode | JsVariableStatement;
		if (node.kind === "JS_CALL_ARGUMENT_LIST") {
			node satisfies JsCallArgumentListNode;
		} else {
			node satisfies JsVariableStatement;
		}
		// @ts-expect-error A mixed node/list query must preserve its union.
		node satisfies JsCallArgumentListNode;
		registerDiagnostic(node, "information", "Matched node or list.");
	},
});

declare const node: AnyJsAstNode;
declare const args: JsCallArgumentList;

registerDiagnostic(node, "warning", "Any AST node.") satisfies void;
// @ts-expect-error Named-field arrays are not eligible diagnostic targets.
registerDiagnostic(args, "warning", "Invalid argument array.");
// @ts-expect-error Diagnostics require an AST node, not its text.
registerDiagnostic(node.text, "warning", "Invalid node.");
// @ts-expect-error A possibly absent node must be checked before reporting.
registerDiagnostic(node.parent, "warning", "Unchecked parent.");
// @ts-expect-error Severity must belong to the supported severity union.
registerDiagnostic(node, "invalid", "Invalid severity.");
// @ts-expect-error Diagnostic messages must be strings.
registerDiagnostic(node, "warning", 123);
