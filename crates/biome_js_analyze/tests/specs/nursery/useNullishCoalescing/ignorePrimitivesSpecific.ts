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

// One ignored variant: not reported
declare const sb: string | boolean | null;
const r7 = sb || 'x';

// Template literal type: not reported
declare const template: `id-${number}` | undefined;
const r8 = template || 'id-0';

// Branded string: not reported
declare const branded: (string & { __brand: 'id' }) | null;
const r9 = branded || 'x';

// `any`: not reported
declare const anyValue: any;
const r10 = anyValue || 'x';

// Truthiness ternary on a string: not reported
const r11 = s ? s : 'x';

// Truthiness ternary on a boolean: reported
const r12 = b ? b : false;

// Comparisons with `null`: reported
const r13 = s != null ? s : 'x';
const r14 = b != null ? b : false;
