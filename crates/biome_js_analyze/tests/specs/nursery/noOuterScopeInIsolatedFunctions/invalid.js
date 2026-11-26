/* should generate diagnostics */
import makeSynchronous from "make-synchronous";
import { helper } from "./helper.js";

const foo = "hi";
let counter = 0;

makeSynchronous(() => foo.slice());
makeSynchronous(async () => foo.slice());
makeSynchronous(function () {
	return foo.slice();
});
makeSynchronous(async function named() {
	return foo.slice();
});
makeSynchronous((() => foo.slice()));
makeSynchronous(() => helper());
makeSynchronous(() => {
	counter = 1;
});
workerize(() => foo.slice());
browser.execute(() => foo.slice());
page.evaluate(() => foo.slice());

chrome.scripting.executeScript({
	target: { tabId: 1 },
	func: () => foo.slice(),
});
browser.scripting.executeScript({
	target: { tabId: 1 },
	func: function () {
		return foo.slice();
	},
});
browser.scripting.executeScript({
	func() {
		return foo.slice();
	},
});
chrome.scripting.executeScript({
	["func"]: () => foo.slice(),
});
chrome.scripting.executeScript({
	[`func`]: () => foo.slice(),
});

// A nested function still runs inside the isolated function.
makeSynchronous(() => {
	function getValue() {
		return foo.slice();
	}
	return getValue;
});

makeSynchronous(function () {
	return this.foo;
});

// Arrow functions use the `this` of the isolated function.
makeSynchronous(function () {
	return () => this.foo;
});

// The `extends` clause and computed names of a nested class run inside the isolated function.
makeSynchronous(() => class extends this.Base {});
makeSynchronous(() => class {
	[this.key]() {}
});

// Each use is reported.
makeSynchronous(() => foo + foo);
