/* should not generate diagnostics */

(function () {
	let x = 0;
	let y = 0;
	let z = x + y;
	let foo = {};
	return bar;
})();

(() => {
	let x = 0;
	let y = 0;
	let z = x + y;
	let foo = {};
	return bar;
})();
