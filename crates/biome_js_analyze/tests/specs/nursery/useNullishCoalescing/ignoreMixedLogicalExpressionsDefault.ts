/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const c: string;

// `||` mixed with `&&`
const r1 = (a || 'default') && b;
const r2 = a || (b && c);
