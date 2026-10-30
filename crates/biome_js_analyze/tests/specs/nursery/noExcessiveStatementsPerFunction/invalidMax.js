/* should generate diagnostics */
function foo() { var bar = 1; var baz = 2; var qux = 3; }
var foo = () => { var bar = 1; var baz = 2; var qux = 3; };
var foo = function() { var bar = 1; var baz = 2; var qux = 3; };
function foo() { var bar = 1; if (true) { while (false) { var qux = null; } } }
function foo() { var bar = 1; try { a; } catch (e) { b; } finally { c; } }
function foo() { "use strict"; a; b; }
function foo() { ;;; }
function foo() { a; { b; } }
function foo() { label: { a; b; } }
function foo() { var bar = 1; return function () { var z; var y; var x; }; }
function foo() { foo_1; /* foo_ 2 */ class C { static { one; two; three; four; { five; six; seven; eight; } } } foo_3 }
class C { static { one; two; three; four; function not_top_level() { 1; 2; 3; } five; six; seven; eight; } }
var foo = { thing() { a; b; c; } };
var foo = { get thing() { a; b; return c; } };
var foo = { set thing(v) { a; b; c; } };
class C { method() { a; b; c; } }
class C { get x() { a; b; return c; } }
class C { set x(v) { a; b; c; } }
class C { constructor() { a; b; c; } }
class C { field = () => { a; b; c; }; }
export default function () { a; b; c; }
function foo(cb = () => { a; b; c; }) {}
async function* foo() { a; b; yield c; }
