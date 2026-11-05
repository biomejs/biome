import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";

export const awaitInLoopGrit = defineRule({
	query: grit(`\`await $value\` as $await where {
    $await <: within or {
        JsForStatement(),
        JsForInStatement(),
        JsForOfStatement(),
        JsWhileStatement(),
        JsDoWhileStatement(),
    } until or {
        JsFunctionDeclaration(),
        JsFunctionExpression(),
        JsArrowFunctionExpression(),
        JsMethodClassMember(),
        JsMethodObjectMember(),
    }
  }`),
	run(node) {
		registerDiagnostic(node, "error", "Unexpected await inside a loop");
	},
});
