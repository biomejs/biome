/* should not generate diagnostics */

// Promise in comma operator
function pattern19() {
	let _x = 5;
	_x++ ?? new Promise((resolve) => resolve('comma'));
}
