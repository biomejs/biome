// biome-ignore lint/correctness/noUndeclaredDependencies: work in progress
import { addDiagnostic } from "@biomejs/plugin-api";

/** @param {string} path */
export default function useMyPlugin(path) {
	if (path.endsWith("plugin.js")) {
		addDiagnostic("warning", "Hello, world!");
	}
}
