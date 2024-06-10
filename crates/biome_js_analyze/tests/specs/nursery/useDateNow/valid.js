const ts = Date.now();
// Test `new Date()`
// Not `NewExpression`
+Date();
+Date;
// Not `Date`
+new Moments();
// More arguments
+new Date(0);
+new Date(...[]);

// Test `new Date().getTime()` and `new Date().valueOf()`
// Not `CallExpression`
new Date.getTime();
// Not `MemberExpression`
valueOf();
// `computed`
new Date()[getTime]();
// Not `Identifier`
new Date()["valueOf"]();
// Not listed names
new Date().notListed(0);
// More arguments
new Date().getTime(0);
new Date().valueOf(...[]);
new Date(0).getTime()
new Date(0).valueOf()

// Test `Number(new Date())` and `BigInt(new Date())`
// Not `CallExpression`
new Number(new Date());
// Not listed names
toNumber(new Date());
// More/less arguments
BigInt();
Number(new Date(), extraArgument);
BigInt([...new Date()]);

// Test `+ new Date()` / `- new Date()`
// Not `UnaryExpression`
throw new Date();
// Not `+/-`
typeof new Date();

// Test `AssignmentExpression`
// Not `AssignmentExpression`
const foo = () => {
	return new Date();
};
// `operator` not listed
foo += new Date();

// Test `BinaryExpression`
// Not `BinaryExpression`
function* foo() {
	yield new Date();
}
// `operator` not listed
new Date() + new Date();

// We are not checking these cases
foo = new Date() | 0;
foo &= new Date();
foo = new Date() >> 0;
