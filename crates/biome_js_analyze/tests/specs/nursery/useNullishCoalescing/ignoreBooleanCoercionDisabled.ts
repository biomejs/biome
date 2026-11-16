/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const c: string;

// `||` inside `Boolean()`
const r1 = Boolean(a || b);

// Chained `||` inside `Boolean()`
const r2 = Boolean(a || b || c);

// `||=` inside `Boolean()`
declare let assigned: string | null;
const r3 = Boolean(assigned ||= b);

// Truthiness ternary inside `Boolean()`
declare const maybeObject: Date | null;
const r4 = Boolean(maybeObject ? maybeObject : new Date(b));
