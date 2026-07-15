export type Severity = "fatal" | "error" | "warning" | "information" | "hint";

export * from "./js_ast";

export function registerDiagnostic(severity: Severity, message: string): void;
