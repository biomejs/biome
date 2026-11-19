/* should generate diagnostics */

declare const a: string | null;
declare const b: string;
declare const c: string;

// `||` as an operand of `&&`: not reported
const r1 = (a || 'default') && b;

// `||` with a `&&` operand: not reported
const r2 = a || (b && c);

// `||` as the left operand of `&&`: not reported
const r3 = (a || b) && c;

// `||` inside nested `&&`: not reported
const r4 = (a || b) && c && b;

// `||=` with a `&&` right-hand side: not reported
declare let assignMixed: string | null;
assignMixed ||= b && c;

// Chained `||` with a parenthesized `&&` operand: not reported
declare const x: string | null;
declare const y: string;
declare const z: string;
declare const w: string;
const r5 = x || y || (z && w);

// Chained `||` with an unparenthesized `&&` operand: not reported
const r6 = x || y || z && w;

// `||=` whose right-hand side is a `||` with a `&&` operand: not reported
declare let chainAssign: string | null;
chainAssign ||= y || (z && w);

// Nested `||=` with a `&&` in the inner right-hand side: not reported
declare let outerAssign: string | null;
declare let innerAssign: string | null;
outerAssign ||= innerAssign ||= b && c;

// The same with parentheses around the inner `||=`: not reported
declare let parenOuter: string | null;
declare let parenInner: string | null;
parenOuter ||= (parenInner ||= b && c);

// `||` without `&&`
declare const d: string | null;
const r7 = d || 'default';

declare const e: number | undefined;
const r8 = e || 0;

// `||=` without `&&`
declare let assignPlain: string | null;
assignPlain ||= 'default';
