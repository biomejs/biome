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

// Nested `||=` with a `&&` in the inner right-hand side: only the inner `||=` is not reported
declare let outerAssign: string | null;
declare let innerAssign: string | null;
outerAssign ||= innerAssign ||= b && c;

// The same with parentheses around the inner `||=`
declare let parenOuter: string | null;
declare let parenInner: string | null;
parenOuter ||= (parenInner ||= b && c);

// `||=` with a `&&` right-hand side inside `||`: the `||=` is not reported, both `||` are
declare const left: string | null;
declare const right: string | null;
declare let tailAssign: string | null;
const r9 = (left || right) || (tailAssign ||= z && w);

declare let nestedAssign: string | null;
const r10 = x || (nestedAssign ||= b && c);

// `||` operand of a `||=` under `&&`: the `||=` is not reported, the `||` is
declare let wrapAssign: string | null;
const r11 = (wrapAssign ||= x || y) && c;

// `||` without `&&`
declare const d: string | null;
const r7 = d || 'default';

declare const e: number | undefined;
const r8 = e || 0;

// `||=` without `&&`
declare let assignPlain: string | null;
assignPlain ||= 'default';

// `&&` as the left operand of a `||` chain: not reported
declare const lead: string | null;
const r12 = (lead && b) || c;
const r13 = (lead && b) || c || d;
