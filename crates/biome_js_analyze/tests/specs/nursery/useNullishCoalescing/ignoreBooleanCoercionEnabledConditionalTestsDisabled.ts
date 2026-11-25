/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const flag: boolean;

// Ternary test passed to `Boolean()`: reported
const r1 = Boolean((a || 'default') ? 1 : 2);

// `Boolean()` as a conditional test: not reported
if (Boolean(a || 'default')) {}

// Plain conditional test: reported
if (a || 'default') {}

// Only the first argument of `Boolean()` counts: reported
const r2 = Boolean(flag, a || b);
