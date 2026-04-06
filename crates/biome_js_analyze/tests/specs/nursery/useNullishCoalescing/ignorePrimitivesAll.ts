// Tests for ignorePrimitives: true (ignore all primitives)
// These should NOT generate diagnostics because the non-nullish
// variants are all primitives

// string | null
declare const s: string | null;
const r1 = s || 'default';

// number | undefined
declare const n: number | undefined;
const r2 = n || 0;

// boolean | null
declare const b: boolean | null;
const r3 = b || false;

// bigint | null
declare const bi: bigint | null;
const r4 = bi || 0n;

// string literal | null
declare const sl: 'hello' | null;
const r5 = sl || 'default';

// number literal | undefined
declare const nl: 42 | undefined;
const r6 = nl || 0;

// string | null | undefined
declare const snu: string | null | undefined;
const r7 = snu || 'default';

// ||= with primitive type - should NOT generate diagnostic
declare let assignStr: string | null;
assignStr ||= 'default';

// Ternary with primitive type - should NOT generate diagnostic
declare const ts: string | null;
const r8 = ts !== null ? ts : 'default';

// If-statement with primitive type - should NOT generate diagnostic
declare let ifs: string | null;
if (ifs == null) { ifs = 'default'; }

// These SHOULD still generate diagnostics (non-primitive types)
declare const obj: { a: string } | null;
const r9 = obj || {};

// ||= with non-primitive type - SHOULD generate diagnostic
declare let assignObj: { a: string } | null;
assignObj ||= { a: 'default' };

// Ternary with non-primitive type - SHOULD generate diagnostic
declare const tobj: { a: string } | null;
const r10 = tobj !== null ? tobj : { a: 'default' };

// If-statement with non-primitive type - SHOULD generate diagnostic
declare let ifobj: { a: string } | null;
if (ifobj == null) { ifobj = { a: 'default' }; }
