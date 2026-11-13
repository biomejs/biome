/* should generate diagnostics */

const a = 1;
const b = 2;
const c = 3;
const d = 4;

Math.min(a, b) < c ? Math.min(a, b) : c;
Math.max(a, b) > c ? Math.max(a, b) : c;
c < Math.min(a, b) ? c : Math.min(a, b);
Math.max(a, b) <= c ? c : Math.max(a, b);
Math.min(Math.max(a, b), c) > d ? d : Math.min(Math.max(a, b), c);

const alias = a;
Math.min(alias, -2) < c ? Math.min(alias, -2) : c;
Math.max("1", true, null) > c ? Math.max("1", true, null) : c;
Math.min(Math.max(a, b), 10) < c ? Math.min(Math.max(a, b), 10) : c;

const cached = Math.min(unknown, 10);
Math.min(cached, 10) < c ? Math.min(cached, 10) : c;
