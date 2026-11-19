/* should generate diagnostics */

declare const a: string | null;
declare const b: string;

// `||` inside `Boolean()`: reported
const r1 = Boolean(a || b);
