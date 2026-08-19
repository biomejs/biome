// Tests for ignorePrimitives: { string: true, number: true }
// string/number operands suppressed; boolean/bigint still report.

declare const s: string | null;
declare const n: number | null;
declare const b: boolean | null;
declare const big: bigint | null;

// ignored kinds: suppressed
const r1 = s || 'x';
const r2 = n || 0;

// non-ignored kinds: SHOULD still report
const r3 = b || false;
const r4 = big || 0n;

// string-literal union: suppressed (string ignored, exercises the literal arm)
declare const litStr: 'a' | 'b' | null;
const r5 = litStr || 'c';

// every non-nullish variant ignored: suppressed
declare const sn: string | number | null;
const r6 = sn || 'x';

// only some variants ignored (boolean not opted out): SHOULD still report
declare const sb: string | boolean | null;
const r7 = sb || 'x';

// ternary on a string operand: suppressed (ignorePrimitives applies to ternary too)
const r8 = s != null ? s : 'x';

// ternary on a boolean operand: SHOULD still report
const r9 = b != null ? b : false;
