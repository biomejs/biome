/* should generate diagnostics */
const a = 1.1 | 0;
const b = (1 + 2 / 3.4) | 0;
const c = bar((1.4 | 0) + 2);
const d = (0, 1.4) | 0;
function e() {return.1 | 0;}
const f = 1.4 | 0.;
const g = 1.4 | .0;
const h = 1.4 | 0.0000_0000_0000;
const i = 1.4 | 0b0;
const j = 1.4 | 0x0000_0000_0000;
const k = 1.4 | 0o0;
const l = 1.4 | (0);
const m = 1.23 | 0 | 4;
const n = bar >> 0;
const o = (a + b) >> 0;
const p = bar << 0;
const q = bar ^ 0;
function r() {return.1 ^0;}
const s = a.b.c | 0;
const t = a.b?.c | 0;
const u = /* first comment */ 3.4 | 0; // A B C
const v = bar | 0 | 0;
function w() {return.1 | 0 | 4;}
