import { registerDiagnostic } from "@biomejs/plugin-api";

/**
 * @param {string} _path
 * @param {import("@biomejs/plugin-api").AnyJsRoot} root
 */
export default function noTopLevelVar(_path, root) {
	const statements =
		root.kind === "JS_MODULE" || root.kind === "TS_DECLARATION_MODULE"
			? root.items
			: root.kind === "JS_SCRIPT"
				? root.statements
				: [];

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
}
