/* should generate diagnostics */
function foo() {
	"use strict";
	this.a = 0;
}
function bar() {
	"use strict";
	return function () {
		this.a = 0;
	};
}
class A {
	static foo = function () {
		return function () {
			this.a = 0;
		};
	};
}
