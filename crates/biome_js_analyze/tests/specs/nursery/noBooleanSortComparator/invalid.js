/* should generate diagnostics */
array.sort((a, b) => a > b);
[].sort((a, b) => a > b);
Array.from(iterable).sort((a, b) => a > b);
new Array().toSorted((a, b) => a > b);
new Int8Array().sort((a, b) => a > b);
array.sort((a, b) => a >= b);
array.sort((a, b) => a < b);
array.sort((a, b) => a <= b);
array.sort((a, b) => b > a);
array.sort((a, b) => b >= a);
array.sort((a, b) => b < a);
array.sort((a, b) => b <= a);
array.sort((a, b) => a.score > b.score);
array.sort((a, b) => a.score >= b.score);
array.sort((a, b) => a[0] < b[0]);
array.sort((a, b) => a[i] <= b[i]);
array.sort((a, b) => a.foo.bar > b.foo.bar);
array.sort((a, b) => a?.score > b?.score);
array.sort((a, b) => (a) > (b));
array.sort((a, b) => (a > b));
array.sort((a, b) => a > c);
array.sort((a, b) => a.foo > b.bar);
array.sort((a, b) => a[i] > b[j]);
array.sort((a, b) => a.score > b);
array.sort(([a], [b]) => a > b);
array.sort((a, b = 0) => a > b);
array.sort((a) => a > 0);
array.sort((a, b) => a > b, extraArgument);
array["sort"]((a, b) => a > b);
array["toSorted"]((a, b) => a > b);
array[sortMethod]((a, b) => a > b);
array.sort((a, b) => a /* ascending */ > b);
array.sort((a, b) => (a) > /* ascending */ (b));
array.toSorted((a, b) => a > b);
array.sort?.((a, b) => a > b);
array?.sort((a, b) => a > b);
(array.sort)((a, b) => a > b);
array.sort(function (a, b) { return a > b; });
array.sort((a, b) => {
	return a > b;
});
array.sort((a, b) => {
	// Compare
	return a > b;
});
array.sort((a, b) => a === b);
strings.toSorted((a, b) => a === b);
array.sort((a, b) => a !== b);
array.sort((a, b) => a == b);
array.sort((a, b) => a != b);
array.sort((a, b) => true);
array.sort((a, b) => false);
array.sort((a, b) => Boolean(a - b));
array.sort((a, b) => !(a - b));
array.sort((a, b) => a > b && c > d);
array.sort((a, b) => a > b || c > d);
array.sort((a, b) => a > b ? c > d : e > f);
array.sort((a, b) => a in b);
array.sort((a, b) => a instanceof b);
array.sort((a, b) => (sideEffect(), a > b));
array.sort((a, b) => (result = a > b));
array.sort((a, b) => Number.isNaN(a - b));
array.sort((a, b) => isBigger(a, b));
array.sort(Boolean);
array.toSorted(Boolean);
array.sort(isFinite);
array.sort(isNaN);
array.sort(Array.isArray);
array.sort(ArrayBuffer.isView);
array.sort(Atomics.isLockFree);
array.sort(Error.isError);
array.sort(Number.isFinite);
array.sort(Number.isInteger);
array.sort(Number.isNaN);
array.sort(Number["isNaN"]);
array.sort(Number.isSafeInteger);
array.sort(Object.hasOwn);
array.sort(Object.is);
array.sort(Object.isExtensible);
array.sort(Object.isFrozen);
array.sort(Object.isSealed);
array.sort(Reflect.deleteProperty);
array.sort(Reflect.has);
array.sort(URL.canParse);
array.sort((Boolean));
array.sort(compareArrow);
array.sort(compareFunctionExpression);
array.sort(compareDeclaration);
array.sort(compareWithConstResult);

const sortMethod = "sort";
const compareArrow = (a, b) => a > b;
const compareFunctionExpression = function (a, b) {
	return a > b;
};
function compareDeclaration(a, b) {
	return a > b;
}
function isBigger(a, b) {
	return a > b;
}
const isGreater = true;
function compareWithConstResult() {
	return isGreater;
}
