import { registerDiagnostic } from "@biomejs/plugin-api";

/** @param {string} path */
export default function useMyPlugin(path) {
	if (path.endsWith("plugin.js")) {
		registerDiagnostic("warning", "Hello, world!");
	}
}
