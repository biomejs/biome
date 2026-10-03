/* should generate diagnostics */

new Promise(function (resolve, reject) { return 1; });
new Promise(function foo(resolve, reject) { return 1; });
new Promise((resolve, reject) => { return 1; });
new Promise(function (resolve, reject) { return undefined; });
new Promise((resolve, reject) => { return null; });
new Promise((resolve, reject) => { return reject(foo); });
new Promise((resolve, reject) => { return Promise.resolve(42); });
new Promise(function (resolve, reject) { if (foo) { return 1; } });
new Promise((resolve, reject) => { try { return 1; } catch (e) {} });
new Promise(function (resolve, reject) { while (foo) { if (bar) break; else return 1; } });
new Promise(() => { return void 1; });
new Promise((Promise) => { return 1; });
new Promise(function Promise(resolve, reject) { return 1; });
new (Promise)((resolve) => { return 1; });
new Promise(((resolve) => { return 1; }));
new Promise(async (resolve) => { return 1; });

// Multiple returns, and returns next to nested functions
new Promise(() => {
	if (foo) {
		return 0;
	} else bar(() => { return 1; });
	return 2;
});

// Arrow function expression bodies
new Promise(r => r(1));
new Promise((resolve, reject) => resolve);
new Promise((resolve, reject) => null);
new Promise((resolve, reject) => x + y);
new Promise(() => (1));
new Promise(() => ({}));
new Promise(r => /*hi*/ ~0);
new Promise(() => () => 1);
new Promise(() => async () => 1);
new Promise(() => function foo() {});
new Promise(() => class Foo {});
new Promise(() => []);
new Promise((resolve, reject) => getSomething((err, data) => {
	if (err) {
		reject(err);
	} else {
		resolve(data);
	}
}));
new Promise(r => r(1) // comment
);

// No fix: wrapping in braces would turn these into invalid declarations
new Promise(() => function () {});
new Promise(() => class {});

// Nested inside another function
function outer() {
	return new Promise(function () { return 1; });
}
