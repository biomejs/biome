/* should generate diagnostics */
define([], function () {});
define(["a"], function (a) { console.log(a); });
require([], function () {});
require(["a"], function (a) { console.log(a); });
define(["a", "b"], factory);
(define)(["a"], function (a) {});
define((["a"]), function (a) {});
define?.(["a"], function (a) {});
const result = require(["a"], function (a) {});
