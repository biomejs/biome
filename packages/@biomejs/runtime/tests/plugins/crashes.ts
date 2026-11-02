import { ast, defineRule } from "@biomejs/runtime/plugin";

export const crashes = defineRule({
	query: ast("JS_MODULE"),
	run() {
		throw new Error("Deliberate rule crash");
	},
});
