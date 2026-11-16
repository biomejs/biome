/* should generate diagnostics */

declare const s: string | null;
declare const n: number | null;

const r1 = s || 'x';
const r2 = n || 0;

// `any` operand: reported
declare const anyValue: any;
const r3 = anyValue || 'x';
