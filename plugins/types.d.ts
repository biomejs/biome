declare module "@biomejs/plugin-api" {
	export type Severity = "fatal" | "error" | "warning" | "information" | "hint";

	export function addDiagnostic(severity: Severity, message: string): void;
}
