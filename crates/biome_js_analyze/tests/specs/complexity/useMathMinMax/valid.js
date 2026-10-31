/* should not generate diagnostics */
height > 10 ? height : 20;
height > 50 ? Math.min(50, height) : height;
foo ? foo : bar;

foo > 10n ? 10n : foo;
foo > BigInt(10) ? BigInt(10) : foo;
foo < -1n ? -1n : foo;
foo < ~1n ? ~1n : foo;
foo > 2n ** 62n - 1n ? 2n ** 62n - 1n : foo;
foo > (2n ** 62n - 1n) ? (2n ** 62n - 1n) : foo;
foo > (2n ** 62n) - 1n ? (2n ** 62n) - 1n : foo;
foo < -(1n) ? -(1n) : foo;
foo > (1n + 2n) ? (1n + 2n) : foo;
foo > (2n * 3n) ? (2n * 3n) : foo;
foo > (6n / 3n) ? (6n / 3n) : foo;
foo > (7n % 4n) ? (7n % 4n) : foo;
foo > (1n << 2n) ? (1n << 2n) : foo;
foo > (8n >> 2n) ? (8n >> 2n) : foo;
foo > (6n & 3n) ? (6n & 3n) : foo;
foo > (6n | 3n) ? (6n | 3n) : foo;
foo > (6n ^ 3n) ? (6n ^ 3n) : foo;

const NEG_BIGINT = -1n;
const MAX_BIGINT = 2n ** 62n - 1n;
const CALL_BIGINT = BigInt(10);
var inlineNeg = (x) => (x < NEG_BIGINT ? NEG_BIGINT : x);
var inlineMax = (x) => (x > MAX_BIGINT ? MAX_BIGINT : x);
var inlineCall = (x) => (x > CALL_BIGINT ? CALL_BIGINT : x);

function keepBigintDefault(a = -1n, b) {
    return a > b ? a : b;
}

foo > "10" ? "10" : foo;
foo > true ? true : foo;
foo > null ? null : foo;
"10" > foo ? foo : "10";
false > foo ? foo : false;
foo > /pattern/ ? /pattern/ : foo;

new Date() > foo ? foo : new Date();
foo > new Date(0) ? new Date(0) : foo;

var now = new Date();
var later = new Date();
var byDate = now > later ? later : now;

const start = new Date();
var mixedDate = start > foo ? foo : start;

function keepStringDefault(a = 'string', b) {
    return a > b ? a : b;
}

function shadowedMath(Math, a, b) {
    return a > b ? b : a;
}

{
    const Math = { min() {}, max() {} };
    Math;
    value > limit ? limit : value;
}

// the operands in these expressions are not idempotent
// and are guarenteed to have side effects. changing them
// to use Math.min or Math.max would change the behavior.
i++ > b ? b : i++;
