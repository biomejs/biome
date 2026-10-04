/* should generate diagnostics */
"use strict";
function foo() {
	this.a = 0;
}
function bar() {
	return function () {
		this.a = 0;
	};
}
