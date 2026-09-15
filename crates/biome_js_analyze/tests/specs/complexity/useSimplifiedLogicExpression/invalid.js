const r = true && boolExp;
const nonNullExp = 123;
const r3 = null ?? nonNullExp;
const boolExpr1 = true;
const boolExpr2 = false;
const r4 = /*1*/ !boolExpr1 /*2*/  || /*3*/ !boolExpr2 /*4*/ 

if (
    !showThinking && // while it's thinking there is hope to get suggestions
    !comments?.length
) {
    console.log();
}

if (x || false) {}
while (x && true) {}
do {} while (x && true);
for (; x || false; ) {}
const t = x || false ? 1 : 2;
const n = !(x && true);
if (a && (x || false)) {}
if ((x || false)) {}
