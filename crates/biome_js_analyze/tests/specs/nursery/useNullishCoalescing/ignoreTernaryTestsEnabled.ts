// should not generate diagnostics for ternary (ignoreTernaryTests: true)

declare const a: string | null;
const r1 = a !== null ? a : 'default';

declare const b: string | undefined;
const r2 = b === undefined ? 'default' : b;

declare const c: string | null;
const r3 = c != null ? c : 'default';
