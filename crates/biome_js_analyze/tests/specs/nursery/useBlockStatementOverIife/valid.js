/* should not generate diagnostics */

{
	const value = getValue();
	run(value);
}

const value = (() => { return getValue(); })();
const other = (() => { getValue(); })();
run((() => { getValue(); })());
(() => value)();
(async () => { await run(); })();
(async function () { await run(); })();
(function* () { yield value; })();
(function named() { run(); })();
(function (value) { run(value); })(value);
((value) => { run(value); })(value);
(value => { run(value); })();
(() => { run(); })(value);
(() => { return value; })();
(() => { if (condition) { return; } })();
(() => { var value = 1; run(value); })();
(() => { if (condition) { var value = 1; } })();
(() => { for (var index = 0; index < 1; index++) {} })();
(() => { for (var key in object) {} })();
(() => { for (var item of items) {} })();
(() => { eval(code); })();
(() => { if (condition) { eval(code); } })();
(() => { (eval)(code); })();
(function () { this.run(); })();
(function () { arguments[0](); })();
(function () { console.log(new.target); })();
(function () { const run = () => this.run(); run(); })();
(function () { const getArguments = () => arguments; getArguments(); })();
(function () { const getNewTarget = () => new.target; getNewTarget(); })();
(function () { const nested = () => () => this.run(); nested(); })();
(function () { const callback = (value = this.value) => value; callback(); })();
(function () { const object = { [this.key]() {} }; run(object); })();
(function () { class Example { [this.key]() {} } run(Example); })();
(function () { class Example { value = this.value; } run(Example); })();
(function () { const read = () => eval(code); read(); })();
(function () { 'use strict'; run(); })();
(() => { 'use strict'; run(); })();
(function () { run(); }).call(this);
(() => { run(); })?.();
(() => { run(); })``;
new (function () { run(); })();
!(() => { run(); })();
void (() => { run(); })();
(() => { run(); })(), run();
(() => { run(); })().then(run);

loop: for (;;) {
	(() => {
		loop: for (;;) {
			break loop;
		}
	})();
}
