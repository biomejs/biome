/* should generate diagnostics */
const a = ~~3.9;
const b = ~~(1 + 2 / 3.4);
const c = ~~1 + 2 / 3.4;
const d = ~~(0, 1.4);
const e = ~~~10.01;
const f = ~~(~10.01);
const g = ~(~~10.01);
const h = ~~-10.01;
const i = ~~~~10.01;
function j() {return~~3.9;}
function k() {return ~~3.9;}
const l = ~~(a ? b : c);
const m = ~~a.b?.c;
const n = /* first comment */ ~~3.4; // A B C
const o = typeof~~x;
const p = ~~bar | 0;
const q = ~~(bar | 0);
function r() {return~~x + 1;}
function s() {throw~~x + 1;}
const t = a in~~b + 1;
switch (value) {case~~x ? a : b: break;}
const u = `${~~x}`;
