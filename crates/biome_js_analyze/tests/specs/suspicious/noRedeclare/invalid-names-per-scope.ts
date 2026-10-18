/* should generate diagnostics */
// A body redeclares a parameter or type parameter of its function.
function f<T>(a, b) {
	type T = number;
	const a = 0;
}
const g = function g(x) {
	let x;
};
const h = (y) => {
	var y;
};

// Enum members.
enum E {
	A,
	A,
}
