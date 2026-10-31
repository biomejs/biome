/* should generate diagnostics */
const a = parseInt(String(number), 10);
const b = Number.parseInt(String(number), 10);
const c = parseInt(String((number)), 10);
const d = parseInt(String(number + 1), 10);
const e = parseInt(String(number,), 10);
const f = parseInt(String(/* comment */ number), 10);
const g = parseInt(String(number /* comment */), 10);
const h = parseInt(String((0, number)), 10);
const i = parseInt((String(number)), (10));
const j = globalThis.parseInt(String(number), 10);
const k = parseInt(String(number), 0xA);
function l() {return parseInt(String(number), 10);}
