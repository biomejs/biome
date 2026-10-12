/* should generate diagnostics */
const a = 'a' + 1e21;
const b = 'a' + 0.1e-6;
const c = 'a' + 0x10;
const d = 'a' + 1_000;
const e = 1e21 + 'a';
const f = 0.1e-6 + 'a';
const g = 0x10 + 'a';
const h = 1_000 + 'a';
const i = 'a' + 123;
const j = 'a' + 0;
const k = 'a' + 45.67;
const l = 'a' + 0x1000000000000081;
