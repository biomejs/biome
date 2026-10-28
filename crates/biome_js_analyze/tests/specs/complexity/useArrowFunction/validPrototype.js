/* should not generate diagnostics */

// https://github.com/biomejs/biome/issues/12372
const f = function () {};
console.log(f.prototype.toString());

const computedRead = function () {};
console.log(computedRead["prototype"]);

const parenthesizedRead = function () {};
console.log((parenthesizedRead).prototype);

const prototypeWrite = function () {};
prototypeWrite.prototype.x = 0;

const prototypeAssignment = function () {};
prototypeAssignment.prototype = {};

const computedAssignment = function () {};
computedAssignment["prototype"] = {};

const constructed = function () {};
new constructed();

const extended = function () {};
class Derived extends extended {}

const instance = function () {};
console.log({} instanceof instance);

const usedInClosure = function () {};
function later() {
	return usedInClosure.prototype;
}

var reassigned;
reassigned = function () {};
reassigned.prototype.y = 1;

const parenthesizedFunction = (function () {});
new parenthesizedFunction();
