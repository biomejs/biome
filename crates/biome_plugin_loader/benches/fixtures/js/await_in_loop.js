import { ast, defineRule, registerDiagnostic } from "@biomejs/runtime/plugin";

const LOOPS = new Set([
	"JS_FOR_STATEMENT",
	"JS_FOR_IN_STATEMENT",
	"JS_FOR_OF_STATEMENT",
	"JS_WHILE_STATEMENT",
	"JS_DO_WHILE_STATEMENT",
]);

const FUNCTIONS = new Set([
	"JS_FUNCTION_DECLARATION",
	"JS_FUNCTION_EXPRESSION",
	"JS_ARROW_FUNCTION_EXPRESSION",
	"JS_METHOD_CLASS_MEMBER",
	"JS_METHOD_OBJECT_MEMBER",
]);

export const awaitInLoop = defineRule({
	query: ast("JS_AWAIT_EXPRESSION"),
	run(node) {
		for (let current = node.parent; current; current = current.parent) {
			if (FUNCTIONS.has(current.kind)) return;
			if (LOOPS.has(current.kind)) {
				registerDiagnostic(node, "error", "Unexpected await inside a loop");
				return;
			}
		}
	},
});
