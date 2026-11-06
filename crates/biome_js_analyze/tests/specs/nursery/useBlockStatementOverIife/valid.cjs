/* should not generate diagnostics */

(() => {
	function foo() {}
	foo();
})();

(() => {
	if (condition) {
		function foo() {}
		foo();
	}
})();

(function () { arguments = [1]; })();
(function () { arguments++; })();
(function () { [arguments] = [1]; })();
