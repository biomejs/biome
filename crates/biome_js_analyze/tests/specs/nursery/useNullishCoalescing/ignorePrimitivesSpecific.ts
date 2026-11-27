/* should generate diagnostics */

declare const s: string | null;
declare const n: number | null;
declare const b: boolean | null;
declare const big: bigint | null;

// Ignored kinds: not reported
const r1 = s || 'x';
const r2 = n || 0;

// Other kinds
const r3 = b || false;
const r4 = big || 0n;

// String literal union: not reported
declare const litStr: 'a' | 'b' | null;
const r5 = litStr || 'c';

// Every non-nullish variant ignored: not reported
declare const sn: string | number | null;
const r6 = sn || 'x';

// Only some variants ignored
declare const sb: string | boolean | null;
const r7 = sb || 'x';

// Ternary on an ignored kind: not reported
const r8 = s != null ? s : 'x';

// Ternary on another kind
const r9 = b != null ? b : false;
