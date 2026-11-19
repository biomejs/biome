import { defineRule, grit, registerDiagnostic } from "@biomejs/runtime/plugin";

export const restrictedCallee = defineRule({
	query: grit(`\`$fn($args)\` where {
    $fn <: or { \`eval\`, \`setTimeout\`, \`setInterval\` },
	}
`),
	run(node) {
		registerDiagnostic(node.callee, "error", "Restricted callee");
	},
});
