/* should not generate diagnostics */
var bar = Symbol('bar');

function baz() {
    function Symbol() { }
    new Symbol();
}