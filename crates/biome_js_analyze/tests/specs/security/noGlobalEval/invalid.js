var foo = "foo";
eval(foo);

eval("foo");

(0, eval)("foo");

(0, window.eval)("foo");

(0, window["eval"])("foo");

var EVAL = eval;
EVAL("foo");

var EVAL = this.eval;
EVAL("foo");

("use strict");
var EVAL = this.eval;
EVAL("foo");

() => {
	this.eval("foo");
};

() => {
	"use strict";
	this.eval("foo");
};

("use strict");
() => {
	this.eval("foo");
};

() => {
	"use strict";
	() => {
		this.eval("foo");
	};
};

(function (exe) {
	exe("foo");
})(eval);

window.eval("foo");

window.window.eval("foo");

window.window["eval"]("foo");

this.eval("foo");

("use strict");
this.eval("foo");

function foo() {
	this.eval("foo");
}

var EVAL = globalThis.eval;
EVAL("foo");

globalThis.eval("foo");

globalThis.globalThis.eval("foo");

globalThis.globalThis["eval"]("foo");

(0, globalThis.eval)("foo");

(0, globalThis["eval"])("foo");
