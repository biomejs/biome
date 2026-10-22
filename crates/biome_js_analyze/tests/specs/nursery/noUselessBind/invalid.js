/* should generate diagnostics */
var a = function () { foo(); }.bind(bar);
var b = (function () { foo(); }).bind(bar);
var c = (() => { foo(); }).bind(bar);
var d = (() => { this.foo(); }).bind(bar);
var e = function () { (function () { this.foo(); }()); }.bind(bar);
var f = function () { function inner() { this.foo(); } }.bind(bar);
var g = function () { return { method() { return this.foo; } }; }.bind(bar);
var h = function () { class A { x = this.y; } }.bind(bar);
var i = function () { class A { static { this.y; } } }.bind(bar);
var j = function () { foo(); }["bind"](bar);
var k = function () { foo(); }[`bind`](bar);
var l = function () { foo(); }?.bind(bar);
var m = function () { foo(); }.bind?.(bar);
var n = async function () { await foo(); }.bind(bar);
var o = function* () { yield foo(); }.bind(bar);
var p = function () { foo(); }.bind(this);
var q = function () { foo(); }.bind("x");
var r = function () { foo(); }.bind(function () {});
var s = function () { foo(); }.bind(baz());
var t = function () { foo(); }.bind(obj.prop);
var u = function () { foo(); }.bind(bar) // trailing comment
var v = function () { foo(); }.bind(/* comment */ bar);
var w = function () { foo(); } /* comment */ .bind(bar);
var x = function () { foo(); }
    .bind(bar);
foo(function () { bar(); }.bind(this));
var y = function () { class A { get x() { return this.y; } set x(v) { this.y = v; } } }.bind(bar);
var z = function () { return { get x() { return this.y; } }; }.bind(bar);
(function () { foo(); }.bind)(bar);
