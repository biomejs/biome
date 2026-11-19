/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const c: string;

// `||` mixed with `&&`
const r1 = (a || 'default') && b;
const r2 = a || (b && c);

// `&&` as the left operand of a `||` chain
declare const lead: string | null;
declare const d: string;
const r3 = (lead && b) || c;
const r4 = (lead && b) || c || d;
