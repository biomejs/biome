/* should generate diagnostics */
const array1 = [1, 2];
array1.push(3, 4);

const array2 = [3, 4];
array2.unshift(1, 2);

const array3 = [];
array3.push(1);

const array4 = [1, 2,];
array4.push(...rest);

let array5;
array5 = [1];
array5.push(2);

const array6 = ([1]);
(array6).push(2);

const object1 = { foo: 1 };
object1.bar = 2;

const object2 = {};
object2[key] = value;

const object3 = { foo: 1 };
Object.assign(object3, { bar: 2, baz: 3 });

const object4 = { foo: 1 };
Object.assign(object4, bar);

const object5 = { foo: 1 };
Object.assign(object5, {});

const set1 = new Set([1, 2]);
set1.add(3);

const set2 = new Set();
set2.add(1);

const set3 = new Set;
set3.add(1);

const weakSet = new WeakSet([foo, bar]);
weakSet.add(baz);

const map1 = new Map([["foo", 1]]);
map1.set("bar", 2);

const map2 = new Map();
map2.set("foo", 1);

const weakMap = new WeakMap([[foo, 1]]);
weakMap.set(bar, 2);

var array7 = [1];
array7.push(foo());

function inFunction() {
	const array = [1];
	array.push(2);
}

switch (foo) {
	case 1:
		const array = [1];
		array.push(2);
}

const globalSet = new globalThis.Set();
globalSet.add(1);

const a = [], array8 = [1];
array8.push(2);

const set4 = new Set // comment
set4.add(1)
