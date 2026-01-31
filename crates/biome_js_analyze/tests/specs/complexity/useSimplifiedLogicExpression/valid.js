/* should not generate diagnostics */
const boolExpr3 = true;
const boolExpr4 = false;
const r5 = !(boolExpr1 && boolExpr2);
const boolExpr5 = true;
const boolExpr6 = false;
const r6 = !!boolExpr1 || !!boolExpr2;
!!x;

// Issue #8577: optional chaining with || false should not be simplified
// because it changes semantics: `account?.test || false` returns false when account is undefined,
// but `account?.test` returns undefined.
const account = undefined;
const foo1 = { bar: account?.test || false };
const foo2 = account?.test || false;
const foo3 = account?.method() || false;
const foo4 = account?.[key] || false;
const foo5 = (account?.test) || false;
const foo6 = false || account?.test; // This also should not be simplified
