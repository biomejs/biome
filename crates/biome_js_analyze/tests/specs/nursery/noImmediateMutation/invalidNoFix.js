/* should generate diagnostics */
const array1 = [foo()];
array1.unshift(1);

const array2 = [1];
// comment
array2.push(2);

const array3 = [1 /* comment */];
array3.push(2);

const object1 = { get foo() { return 1; } };
object1.bar = 2;

const object2 = {};
object2.__proto__ = proto;

const object3 = {};
object3["__proto__"] = proto;

const object4 = {};
Object.assign(object4, { __proto__: proto });

const object5 = {};
Object.assign(object5, { get foo() { return 1; } });

const object6 = {};
Object.assign(object6, foo, bar);

const array4 = [1]
array4.push(2);[3].forEach(log)

let foo2;
foo2 = /* comment */ 1;

let i = 0;
const array5 = [i];
array5.unshift(++i);

const array6 = [...values];
array6.unshift(1);

const object7 = { __proto__: proto };
object7.foo = 1;
