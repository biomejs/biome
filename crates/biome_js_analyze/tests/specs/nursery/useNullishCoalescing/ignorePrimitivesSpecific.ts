// Tests for ignorePrimitives: { number: true, string: true }
// Only number and string primitives are ignored

// string | null - should NOT generate diagnostic (string ignored)
declare const s: string | null;
const r1 = s || 'default';

// number | undefined - should NOT generate diagnostic (number ignored)
declare const n: number | undefined;
const r2 = n || 0;

// boolean | null - SHOULD generate diagnostic (boolean not ignored)
declare const b: boolean | null;
const r3 = b || false;

// bigint | null - SHOULD generate diagnostic (bigint not ignored)
declare const bi: bigint | null;
const r4 = bi || 0n;

// string | number | null - should NOT generate diagnostic (both ignored)
declare const sn: string | number | null;
const r5 = sn || 'default';

// string | boolean | null - SHOULD generate diagnostic (boolean not ignored)
declare const sb: string | boolean | null;
const r6 = sb || 'default';

// ||= with ignored primitive type - should NOT generate diagnostic
declare let assignStr: string | null;
assignStr ||= 'default';

// ||= with non-ignored primitive type - SHOULD generate diagnostic
declare let assignBool: boolean | null;
assignBool ||= true;

// Ternary with ignored primitive type - should NOT generate diagnostic
declare const tn: number | null;
const r7 = tn !== null ? tn : 0;

// Ternary with non-ignored primitive type - SHOULD generate diagnostic
declare const tb: boolean | null;
const r8 = tb !== null ? tb : false;

// If-statement with ignored primitive type - should NOT generate diagnostic
declare let ifn: number | null;
if (ifn == null) { ifn = 0; }

// If-statement with non-ignored primitive type - SHOULD generate diagnostic
declare let ifb: boolean | null;
if (ifb == null) { ifb = false; }
