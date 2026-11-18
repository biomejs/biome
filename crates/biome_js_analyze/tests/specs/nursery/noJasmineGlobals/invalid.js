/* should generate diagnostics */

spyOn(some, "object");
spyOnProperty(some, "object");
fail();
pending();
(fail)();
fail?.();
globalThis.spyOn(some, "object");
window.pending();

jasmine.addMatchers(matchers);
jasmine.createSpy();
jasmine.any();
jasmine.anything();
jasmine.arrayContaining();
jasmine.objectContaining();
jasmine.stringMatching();
jasmine.getEnv();
jasmine.empty();
jasmine.falsy();
jasmine.truthy();
jasmine.arrayWithExactContents();
jasmine.clock();
jasmine.getEnv().addReporter(reporter);
jasmine.matchersUtil.equals(first, second);

jasmine["any"](Number);
jasmine[`createSpy`]();
jasmine[name]();
(jasmine).anything();
jasmine?.any(Number);
globalThis.jasmine.any(Number);
window.jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;

jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
jasmine.DEFAULT_TIMEOUT_INTERVAL = 10 * 1000;
jasmine.DEFAULT_TIMEOUT_INTERVAL = function () {};
jasmine["DEFAULT_TIMEOUT_INTERVAL"] = 5000;
jasmine.DEFAULT_TIMEOUT_INTERVAL += 1000;
jasmine.MAX_PRETTY_PRINT_ARRAY_LENGTH = 42;
previousTimeout = jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
[jasmine.DEFAULT_TIMEOUT_INTERVAL] = [5000];
jasmine.any = matcher;

const originalTimeout = jasmine.DEFAULT_TIMEOUT_INTERVAL;
const anyMatcher = jasmine.any;

// Comments around a replaced node are kept.
jasmine /* object */ .any(Number);
// Before the timeout.
jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000; // After the timeout.
jasmine.DEFAULT_TIMEOUT_INTERVAL =
	5000;

// The fix would lose these comments, so it isn't offered.
jasmine.DEFAULT_TIMEOUT_INTERVAL = /* five seconds */ 5000;
(jasmine /* object */).any(Number);
globalThis /* global */ .jasmine.any(Number);

// `expect` and `jest` refer to something else here, so no fix is offered.
function withLocalExpect(expect) {
	return jasmine.any(Number);
}
function withLocalJest(jest) {
	jasmine.DEFAULT_TIMEOUT_INTERVAL = 5000;
}

// The parameter of the nested function doesn't affect the outer call.
function outer() {
	fail();
	function nested(fail) {}
}
