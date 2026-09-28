var xs = Array();

var xs = Array(0, 1, 2);

var xs = Array(...args);

var xs = new Array;

var xs = new Array();

var xs = new Array(0, 1, 2);

var xs = new Array(...args);

// A spread can leave a single numeric argument, so these are not fixed
var xs = Array(x, ...args);

var xs = new Array(...a, ...b);

// At least two arguments remain whatever the spread holds
var xs = Array(x, y, ...args);

var xs = /**A*/ new /**B*/ Array /**C*/ ( /**D*/ 0 /**E*/, /**F*/ 1 /**G*/) /**H*/;

var xs = (Array)(
    /* foo */ a,
    b = c() // bar
);

var xs = Array?.();

// ASI
foo
new Array

var xs = globalThis.Array();

var xs = window.Array();
