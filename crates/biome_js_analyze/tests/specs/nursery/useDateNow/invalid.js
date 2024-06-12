const ts = new Date().getTime();
const ts1 = (new Date()).getTime();
const ts2 = (new Date().getTime());
const ts3 = new Date().valueOf();
const ts4 = (new Date()).valueOf();
const ts5 = (new Date().valueOf());

// `Number()` and `BigInt()`
const ts6 = /* 1 */ Number(/* 2 */ new /* 3 */ Date(/* 4 */) /* 5 */); /* 6 */
const tsBigInt = /* 1 */ BigInt(
	/* 2 */ new /* 3 */ Date(/* 4 */) /* 5 */
); /* 6 */

// `BinaryExpression`
const ts10 = new Date() - 0;
const bar = bar - new Date();
const bar1 = new Date() * bar;
const ts11 = new Date() / 1;
const ts12 = new Date() % Infinity;
const ts13 = new Date() ** 1;
const zero = new Date(/* 1 */) /* 2 */ /* 3 */ - /* 4 */ new Date();

// `AssignmentExpression`
foo -= new Date();
foo *= new Date();
foo /= new Date();
foo %= new Date();
foo **= (new Date());


`UnaryExpression`
const ts7 = +(/* 1 */ new Date());
const ts8 = -(/* 1 */ new Date());

function foo() {
	return +new Date();
}
function foo() {
	return -new Date();
}
await +new Date();
typeof+new Date();