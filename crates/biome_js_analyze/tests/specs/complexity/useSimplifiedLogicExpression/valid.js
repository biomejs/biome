/* should not generate diagnostics */
const boolExpr3 = true;
const boolExpr4 = false;
const r5 = !(boolExpr1 && boolExpr2);
const boolExpr5 = true;
const boolExpr6 = false;
const r6 = !!boolExpr1 || !!boolExpr2;
!!x;
// Boolean literals on the right side are not simplified: `||` and `&&`
// return one of their operands, so removing the literal can change the result.
const r7 = x || false;
const r8 = x || true;
const r9 = x && true;
const r10 = x && false;
const r11 = obj?.prop || false;
