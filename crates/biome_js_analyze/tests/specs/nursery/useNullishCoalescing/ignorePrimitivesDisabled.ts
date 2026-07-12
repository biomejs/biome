// Tests for ignorePrimitives: false
// The option is present but disabled, so primitive operands still report.

declare const s: string | null;
declare const n: number | null;

const r1 = s || 'x';
const r2 = n || 0;
