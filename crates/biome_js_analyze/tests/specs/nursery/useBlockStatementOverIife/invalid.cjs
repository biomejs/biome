/* should generate diagnostics */

(() => {
	const value = getValue();
	run(value);
})();

(() => {
	const nested = function () {
		function foo() {}
		foo();
	};
	nested();
})();
