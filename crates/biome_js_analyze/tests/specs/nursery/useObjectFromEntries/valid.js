/* should not generate diagnostics */
Object.fromEntries(pairs);
pairs.reduce(object => ({...object, key}), Object.create(null));
pairs.reduce(object => ({...object, __proto__: value}), {});
pairs.reduce(object => Object.assign(object, {"__proto__": value}), {});
pairs.reduce(object => ({...object, key}));
pairs.reduce(object => ({...object, key}), {}, extraArgument);
pairs.reduce({}, object => ({...object, key}));
reduce(object => ({...object, key}), {});
new pairs.reduce(object => ({...object, key}), {});
pairs.reduce?.(object => ({...object, key}), {});
pairs?.reduce(object => ({...object, key}), {});
pairs.notReduce(object => ({...object, key}), {});
pairs["reduce"](object => ({...object, key}), {});
pairs.reduce(object => ({...object, key}), {notEmpty});
pairs.reduce(object => ({...object, key}), []);
pairs.reduce(...[(object => ({...object, key}))], {});
pairs.reduce(object => ({...object, key}), ...[{}]);
pairs.reduce(callback, {});
pairs.reduce(function (object) { return Object.assign(object, {key}); }, {});
pairs.reduce(object => ({...object, key} + 1), {});
pairs.reduce((object = {}) => ({...object, key}), {});
pairs.reduce((object) => ({...notSameObject, key}), {});
pairs.reduce(object => ({...object, key, anotherKey}), {});
pairs.reduce(object => ({}), {});
pairs.reduce(object => ({keyFirst, ...object}), {});
pairs.reduce(async object => ({...object, key}), {});
pairs.reduce((...object) => ({...object, key}), {});
pairs.reduce(({object}) => ({...object, key}), {});
pairs.reduce(object => ({...object, ...key}), {});
pairs.reduce(object => Object.assign(notSameObject, {key}), {});
pairs.reduce(object => Object.assign(object, {}), {});
pairs.reduce(object => Object.assign(object, {...key}), {});
pairs.reduce(object => Object.assign?.(object, {key}), {});
pairs.reduce(object => Object?.assign(object, {key}), {});
pairs.reduce(object => Object.notAssign(object, {key}), {});
pairs.reduce(object => NotObject.assign(object, {key}), {});
pairs.reduce(object => ({...object, object}), {});
pairs.reduce(object => ({...object, key: Object.keys(object)}), {});
pairs.reduce((object, [key, value = object]) => ({...object, [key]: value}), {});
pairs.reduce(object => ({...object, key: function () { return object; }}), {});
pairs.reduce(object => ({...object, method() {}}), {});
pairs.reduce(object => ({...object, get key() {}}), {});
pairs.reduce(object => ({...object, set key(v) {}}), {});
arrayOfObjects.reduce((flattened, next) => Object.assign(flattened, next), {});
function shadowedObject(Object) {
	return pairs.reduce(object => Object.assign(object, {key}), {});
}

new Uint8Array([1, 2]).reduce((object, value) => ({...object, [value]: value}), {});
Uint16Array.from([1, 2]).reduce((object, value) => ({...object, [value]: value}), {});
Uint8Array.of(1, 2).reduce((object, value) => Object.assign(object, {[value]: value}), {});
((new Uint8Array(values))).reduce((object, value) => ({...object, [value]: value}), {});
const typedValues = new Int16Array([1, 2]);
typedValues.reduce((object, value) => Object.assign(object, {[value]: value}), {});
const typedAlias = typedValues;
typedAlias.reduce((object, value) => ({...object, [value]: value}), {});

underscore.fromPairs(pairs);
_.fromPairs;
_.fromPairs();
new _.fromPairs(pairs);
_.fromPairs(...[pairs]);
_?.fromPairs(pairs);
_.fromPairs?.(pairs);
_.fromPairs(pairs, extra);

let object1 = Object.create(null);
for (const [key, value] of pairs) {
	object1[key] = value;
}

const object2 = {};
for (const [key, value] of pairs) {
	object2[key] = doSomething(value);
}

const object3 = {};
for (const [key, value] of pairs) {
	object3[key] = value;
	count++;
}

const object4 = {};
foo();
for (const [key, value] of pairs) {
	object4[key] = value;
}

const object5 = {}, otherObject = {};
for (const [key, value] of pairs) {
	object5[key] = value;
}

async function asyncLoop() {
	const object = {};
	for await (const [key, value] of pairs) {
		object[key] = value;
	}
}

const object6 = {};
for (const [key, value] of pairs) {
	object6[key] += value;
}

const object7 = {};
for (const [key, value] of pairs) {
	object7.key = value;
}

const object8 = {};
for (const [key, value] of pairs) {
	otherObject[key] = value;
}

const object9 = {};
for (const [key, value] of pairs) {
	this.object9[key] = value;
}

const object10 = {};
for (const [key, value] of pairs) {
	object10[value] = key;
}

const object11 = {};
for (var [key, value] of pairs) {
	object11[key] = value;
}

const object12 = {};
for (const [key, value] of object12) {
	object12[key] = value;
}

const object13 = {};
for (const [object13, value] of pairs) {
	object13[object13] = value;
}

const object14 = {};
for (const [key = defaultKey, value] of pairs) {
	object14[key] = value;
}

const object15 = {};
for (const [key, ...value] of pairs) {
	object15[key] = value;
}

let object16;
for (const [key, value] of pairs) {
	object16[key] = value;
}

const object17 = {};
for (const [key, value] of [...pairs, ...Object.entries(object17)]) {
	object17[key] = value;
}
