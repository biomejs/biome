/* should not generate diagnostics */
const x: 1n = 1n;
const x: -1n = -1n;
const x: false = false;
const x: false = !true;
const x: false = !1;
const x: true = true;
const x: true = !false;
const x: true = !0;
const x: 1 = +1;
const x: -1 = -1;
const x: 1e-5 = 1e-5;
const x: RegExp = /a/;
const x: "str" = "str";
const x: "str" = `str`; // constant template string
const x: "str2" = `str${f()}`;
