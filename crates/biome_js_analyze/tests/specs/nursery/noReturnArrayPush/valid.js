/* should not generate diagnostics */
function f1() {
	array.push(value);
	return;
}
function f2() {
	array.unshift(value);
	return;
}
function f3() {
	return array.length + 1;
}
function f4() {
	return array.push();
}
function f5() {
	return array.unshift();
}
function f6() {
	return stream.push(chunk);
}
function f7() {
	return this.push(chunk);
}
function f8() {
	return this.stream.push(chunk);
}
function f9() {
	return process.stdin.push(chunk);
}
function f10() {
	return process.stdout.push(chunk);
}
function f11() {
	return process.stderr.push(chunk);
}
function f12() {
	return router.push(to);
}
function f13() {
	return this.router.push(to);
}
function f14() {
	return this.$router.push(to);
}
function f15() {
	return stream?.push(chunk);
}
function f16() {
	return this?.push(chunk);
}
function f17() {
	return process.stdin?.push(chunk);
}
function f18() {
	return array["push"](value);
}
function f19() {
	return array[push](value);
}
function f20() {
	return push(value);
}
function f21() {
	array?.push(value);
}
function f22() {
	array?.unshift(value);
}
function f23() {
	return array.#push(value);
}
void array.push(value);
void array.unshift(value);
void array?.push(value);
const length1 = void array.push(value);
const f24 = (value) => void array.push(value);
function f25() {
	return void array.push(value);
}
function f26() {
	return router.push(to).catch(() => {});
}
function f27() {
	return router.push(to).then(onFulfilled);
}
const promise = router.push(to).finally(cleanup);
function f28() {
	return array.push(value).toString();
}
function f29() {
	return array.unshift(value).foo;
}
function f30() {
	return router.push(to)?.catch(() => {});
}
array.push(value)[0];
(array.push(value));
const f31 = () => ({}).push(value);
const f32 = () => new Stack().push(value);
const f33 = () => `${a}`.push(value);
const f34 = () => (function () {}).unshift(value);
