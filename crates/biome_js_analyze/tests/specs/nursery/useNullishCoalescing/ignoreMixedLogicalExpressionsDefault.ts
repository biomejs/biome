// Tests for ignoreMixedLogicalExpressions: true
// These should NOT generate diagnostics because || is mixed with &&

declare const a: string | null;
declare const b: string;
declare const c: string;

// || as child of &&
const r1 = (a || 'default') && b;

// || with && child
const r2 = a || (b && c);

// || as left operand of &&
const r3 = (a || b) && c;

// Nested: || inside && inside another &&
const r4 = (a || b) && c && b;

// ||= mixed with && - should NOT generate diagnostic
declare let assignMixed: string | null;
assignMixed ||= b && c;

// Chained || with && sibling: inner || should also be suppressed
// In `(a || b) || (c && d)`, the inner `a || b` is part of the same
// mixed expression tree and should not generate a diagnostic.
declare const x: string | null;
declare const y: string;
declare const z: string;
declare const w: string;
const r5 = x || y || (z && w);

// Three-way chain: a || b || c && d (without parens around &&)
const r6 = x || y || z && w;

// ||= chained with || containing && sibling
// `a || (b && c)` as the RHS of ||=
declare let chainAssign: string | null;
chainAssign ||= y || (z && w);

// These SHOULD still generate diagnostics (no && mixing)
declare const d: string | null;
const r7 = d || 'default';

declare const e: number | undefined;
const r8 = e || 0;

// ||= without && mixing - SHOULD generate diagnostic
declare let assignPlain: string | null;
assignPlain ||= 'default';
