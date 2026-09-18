// Tests for ignorePrimitives: true
// `||` and `||=` on primitive-typed operands should NOT report; non-primitives still do.

declare const s: string | null;
declare const n: number | null;
declare const b: boolean | null;
declare const big: bigint | null;
declare const fallbackS: string;
declare const fallbackN: number;

// primitive left operands: suppressed
const r1 = s || fallbackS;
const r2 = n || fallbackN;
const r3 = b || false;
const r4 = big || 0n;

// ||= on a primitive: suppressed
declare let assigned: string | null;
assigned ||= fallbackS;

// literal-typed unions: suppressed (exercises the TypeData::Literal arm)
declare const litStr: 'a' | 'b' | null;
declare const litNum: 0 | 1 | null;
declare const litBool: true | null;
declare const litBig: 1n | 2n | null;
const r5 = litStr || 'c';
const r6 = litNum || 2;
const r7 = litBool || false;
const r8 = litBig || 3n;

// non-primitive (object) left operand: SHOULD still report
declare const obj: { x: string } | null;
declare const fallbackObj: { x: string };
const r9 = obj || fallbackObj;
