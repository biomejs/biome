/* should generate diagnostics */

declare const s: string | null;
declare const n: number | null;
declare const b: boolean | null;
declare const big: bigint | null;

// Kinds that are not ignored: reported
const r1 = s || 'x';
const r2 = n || 0;
const r3 = b || false;

// Ignored kind: not reported
const r4 = big || 0n;
