/* should not generate diagnostics */
import "x";
import x from "x";
var y = require("y");
require("z");
// The first argument isn't an array.
require("x", "y");
define("name", function () {});
// The number of arguments isn't two.
define(0, 1, 2);
define("a");
define(["a"]);
define(["a"], function (a) {}, extra);
require(["a"], function (a) {}, function (error) {});
// Other functions and callees.
setTimeout(foo, 100);
(a || b)(1, 2, 3);
loader.define(["a"], function (a) {});
// Calls inside functions or blocks.
function f() { define(["a"], function (a) {}); }
function g() { require(["a"], function (a) {}); }
const h = () => define(["a"], function (a) {});
if (typeof define === "function") { define(["a"], function (a) {}); }
