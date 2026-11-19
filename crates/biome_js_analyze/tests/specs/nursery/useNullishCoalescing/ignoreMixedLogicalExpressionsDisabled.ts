/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const c: string;

// `||` as an operand of `&&`
const r1 = (a || 'default') && b;

// `||` with a `&&` operand
const r2 = a || (b && c);

// `||` as the left operand of `&&`
const r3 = (a || b) && c;

// `||` inside nested `&&`
const r4 = (a || b) && c && b;

// `||=` with a `&&` right-hand side
declare let assignMixed: string | null;
assignMixed ||= b && c;

// Chained `||` with a `&&` operand
declare const x: string | null;
declare const y: string;
declare const z: string;
declare const w: string;
const r5 = x || y || (z && w);

// Nested `||=` with a `&&` in the inner right-hand side
declare let outerAssign: string | null;
declare let innerAssign: string | null;
outerAssign ||= innerAssign ||= b && c;

// The same with parentheses around the inner `||=`
declare let parenOuter: string | null;
declare let parenInner: string | null;
parenOuter ||= (parenInner ||= b && c);
