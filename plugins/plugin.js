import { registerDiagnostic } from "@biomejs/plugin-api";

/**
 * @param {string} path
 * @param {import("@biomejs/plugin-api").AnyJsRoot} root
 */
export default function useMyPlugin(path, root) {
	if (path.endsWith("plugin.js")) {
		registerDiagnostic("warning", "Hello, world!");
	}

	const statement = root.children[1].children[1];
	if (statement.text.includes("useMyPlugin")) {
		registerDiagnostic("information", "Found useMyPlugin in AST");
	}
}
