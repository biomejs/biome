/* should not generate diagnostics */
Object.assign();
let a = Object.assign(a, b);
Object.assign(a, b);
let c = Object.assign(b, { c: 1 });
const bar = { ...foo };
Object.assign(...foo);
Object.assign(foo, { bar: baz });
Object.assign({}, ...objects);
foo({ foo: 'bar' });
