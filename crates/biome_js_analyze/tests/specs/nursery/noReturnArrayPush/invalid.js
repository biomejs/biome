/* should generate diagnostics */
const length1 = array.push(value);
const length2 = routerItems.push(value);
const length3 = array.unshift(value);
const length4 = stream.unshift(chunk);
console.log(array.push(value));
const length5 = await array.push(value);
async function f1() { await array.push(value); }
(array.push(value), sideEffect());
(sideEffect(), array.unshift(value));
condition && array.push(value);
array.unshift(value) || sideEffect();
condition ?? array.push(value);
condition ? array.push(value) : sideEffect();
array.unshift(value) ? sideEffect() : other();
for (array.push(value); condition; ) {}
for (; condition; array.unshift(value)) {}
const length6 = array.push(...values);
const length7 = new Array().push(value);
const f2 = (value) => array.push(value);
const f3 = async (value) => array.unshift(await value);
source.forEach((item) => target.push(item));
function f4() {
	if (condition) return array.push(value);
}
function f5() {
	return condition && array.push(value);
}
function f6() {
	return condition ? value : array.unshift(value);
}
function f7() {
	return (sideEffect(), array.push(value));
}
async function f8() {
	return await array.push(value);
}
const length8 = (array.push)(value);
const length9 = tag`items`.push(value);
