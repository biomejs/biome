/* should generate diagnostics */
const a = ~~(/* keep */ 1 + 2 / 3.4);
const b = /* keep */ ~ /* drop */ ~ 3.4;
const c = x /* keep */ << 0;
const d = 3.4 /* drop */ | /* drop */ 0;
const e = parseInt(/* comment */ String(number), 10);
const f = parseInt(String /* comment */ (number), 10);
const g = parseInt(String(number), /* comment */ 10);
