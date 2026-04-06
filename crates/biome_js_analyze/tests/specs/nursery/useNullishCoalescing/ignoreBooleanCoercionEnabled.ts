// Tests for ignoreBooleanCoercion: true
// These should NOT generate diagnostics because || is inside Boolean()

declare const a: string | null;
declare const b: string;

// Direct Boolean() call
const r1 = Boolean(a || b);

// Boolean() with parenthesized expression
const r2 = Boolean((a || b));

// Nested || inside Boolean()
const r3 = Boolean(a || b || 'default');

// ||= inside Boolean() - should NOT generate diagnostic
declare let assignBool: string | null;
Boolean(assignBool ||= 'default');

// These SHOULD still generate diagnostics (not inside Boolean())

// Plain || expression
declare const c: string | null;
const r4 = c || 'default';

// Inside other function call
declare const d: string | null;
const r5 = String(d || 'default');

// Boolean used as identifier, not a call
declare const e: string | null;
const r6 = e || true;

// ||= not inside Boolean() - SHOULD generate diagnostic
declare let assignPlain: string | null;
assignPlain ||= 'default';
