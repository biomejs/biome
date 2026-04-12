// These should NOT produce diagnostics (global Boolean with ignoreBooleanCoercion: true)
declare const a: string | null;
const r1 = Boolean(a || 'default');
const r2 = Boolean((a || 'default'));

// Shadowed Boolean: SHOULD still produce diagnostics
function Boolean(x: unknown): boolean { return !!x; }
declare const b: string | null;
const r3 = Boolean(b || 'default');
