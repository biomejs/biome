/* should generate diagnostics */
function f1() {
	return array.push(value);
}
function f2() {
	return array.unshift(value);
}
function f3() {
	return (array.push(value));
}
function f4() {
	return array?.push(value);
}
function f5() {
	return array.push?.(value);
}
function f6() {
	return array.push(/* comment */ value);
}
function f7() {
	// leading comment
	return array.push(value); // trailing comment
}
function f8() {
	return array.push(value) // trailing comment
}
function f9() {
	return this.items.push(value);
}
function f10() {
	switch (key) {
		case 1:
			return array.push(value);
	}
}
function f11() {
	return (array.push(value) /* comment */);
}
