// Tests for ignoreMixedLogicalExpressions: false
// Mixed || + && expressions SHOULD generate diagnostics

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

// ||= mixed with &&
declare let assignMixed: string | null;
assignMixed ||= b && c;

// Chained || with && sibling
declare const x: string | null;
declare const y: string;
declare const z: string;
declare const w: string;
const r5 = x || y || (z && w);

// Nested ||= where the inner ||= contains && in its RHS.
// With the option disabled, both ||= operations should be diagnosed.
declare let outerAssign: string | null;
declare let innerAssign: string | null;
outerAssign ||= innerAssign ||= b && c;

// Same nested ||= shape with explicit parens around the inner assignment.
declare let parenOuter: string | null;
declare let parenInner: string | null;
parenOuter ||= (parenInner ||= b && c);
