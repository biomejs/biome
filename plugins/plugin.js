import { ast, defineRule, registerDiagnostic } from "@biomejs/plugin-api";

export const noTopLevelVar = defineRule({
	query: ast("JS_MODULE", "JS_SCRIPT", "TS_DECLARATION_MODULE"),
	run(root) {
		const statements = root.kind === "JS_SCRIPT" ? root.statements : root.items;

		for (const statement of statements) {
			if (
				statement.kind === "JS_VARIABLE_STATEMENT" &&
				statement.declaration?.kindToken === "var"
			) {
				registerDiagnostic(
					statement,
					"warning",
					"Use let or const instead of a top-level var declaration.",
				);
			}
		}
	},
});
