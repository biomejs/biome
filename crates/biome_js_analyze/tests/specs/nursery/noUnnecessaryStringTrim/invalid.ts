/* should generate diagnostics */
function a(value: string) { value.trim().endsWith("-"); }
function b(value: string) { value.trim().startsWith("-" as const); }
function c(value: string) { value.trim().endsWith("-" satisfies string); }
(value as string).trim().startsWith("-");
value!.trim().startsWith("-");
{ const prefix = "foo" as const; value.trim().startsWith(prefix); }
