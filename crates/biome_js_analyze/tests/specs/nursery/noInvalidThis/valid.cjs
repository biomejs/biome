/* should not generate diagnostics */
this.a = 0;
z((x) => console.log(x, this));
function foo() {
	this.a = 0;
}
(function () {
	this.a = 0;
})();
"use strict";
function bar() {
	this.a = 0;
}
function baz() {
	return function () {
		"not strict";
		this.a = 0;
	};
}
