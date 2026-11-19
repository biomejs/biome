/* should generate diagnostics */

declare const s: string | null;
declare const anyValue: any;

// String and `any` operands: reported
const r1 = s || 'x';
const r2 = anyValue || 'x';
