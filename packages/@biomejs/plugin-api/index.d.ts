export type Severity = "fatal" | "error" | "warning" | "information" | "hint";

export function registerDiagnostic(severity: Severity, message: string): void;
