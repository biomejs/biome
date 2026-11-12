/* should not generate diagnostics */

jest.spyOn();
jest.fn();
jest.setTimeout(5000);
expect.extend();
expect.any();
it("foo", function () {});
test("foo", function () {});
foo();
require("foo")("bar");
(function () {})();

// Names declared in the file aren't Jasmine globals.
function callback(fail) {
	fail();
}
function otherCallback(pending) {
	pending();
}
var spyOn = require("actions");
spyOn("foo");
function withLocalJasmine(jasmine) {
	jasmine.any(Number);
	jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
}

// Properties with the same name as a Jasmine global.
object.fail();
object.spyOn(some, "object");
this.pending();
object.jasmine.any(Number);
globalThis.object.jasmine.any(Number);

// Only calls to the global functions are reported.
promise.catch(fail);
new fail();

// Only properties of `jasmine` are reported.
typeof jasmine;
register(jasmine);
