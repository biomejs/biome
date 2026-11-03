/* should generate diagnostics */

// Boolean values without a prefix
const completed = true;
const failed = false;
const finished = progress === 100;
const converted = Boolean(value);
const negated = !value;
const list = Array.isArray(value);
const found = "hello".includes("h");
const matched = /unicorn/.test(value);
const either = condition ? true : false;
const last = (doSomething(), true);
const inside = "key" in object;
const instance = value instanceof Date;
const removed = delete object.key;
const both = a === b && c !== d;
const flags = Number.isInteger(value) || Object.is(a, b);
const global = globalThis.Boolean(value);
const nested = a === 1 && (b === 2 || (c ? d === 3 : !e)) && (f, await g instanceof Date);

// Prefix-like words that aren't separate words
const island = true;
const area = true;
const haven = false;
const candy = true;
const willow = true;
const haste = true;
const doesntMatch = true;
const ISREADY = true;

// Values that refer to other booleans
const alias = both;

// Variables that only receive booleans
let toggled;
toggled = true;
toggled = false;
let flipped = true;
flipped &&= false;

// Functions that return booleans
function done() {
	return true;
}
function check(value) {
	if (value) {
		return value === 1;
	}
	return false;
}
const arrow = () => true;
const expression = function () {
	return false;
};
export function exported() {
	return true;
}

// Parameters with boolean default values
function download(showProgress = false) {}
const handler = (verbose = true) => {};
class Downloader {
	constructor(showProgress = false) {}
	download(retry = true) {}
}
const object = {
	method(silent = false) {},
};

// Non-boolean values with a prefix
const hasName = "Sindre";
const wasRed = 2;
const requiresLogin = "yes";
const isList = [];
const isObject = {};
const hasTemplate = `template`;
const isNew = new Boolean(value);
const isSum = a + b;
const isType = typeof value;
const is_ready = 1;
const IS_READY = 1;
const _isReady = 1;
const is2FA = 1;
let isChanged = true;
isChanged = "yes";
let isCount = true;
isCount++;
let isLoop = true;
for (isLoop in object) {}

// Functions with a prefix that don't return booleans
const isReadyArrow = () => "yes";
function hasTitle() {
	return "Unicorn";
}
function isEmptyBody() {}
function isBareReturn() {
	return;
}
async function isAsyncString() {
	return "yes";
}
function* isGenerator() {
	yield true;
}
function isFunctionFactory() {
	return () => true;
}
function hasNestedReturn() {
	function inner() {
		return getValue();
	}
	return "outer";
}

// Hooks
function useReady() {
	return true;
}
const isTemplate = `text`;
function use2FA() {
	return true;
}
let check = () => true;
check = () => false;
function useIS_COOL() {
	return 1;
}
