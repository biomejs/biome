var bar = Symbol('bar');

function baz() {
    function Symbol() { }
    new Symbol();
}