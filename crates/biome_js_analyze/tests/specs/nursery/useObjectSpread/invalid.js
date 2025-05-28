Object.assign({}, foo);
Object.assign  ({}, foo);
Object  .assign  ({}, foo);
Object.assign({}, {foo: 'bar'});
Object.assign({ foo: 'bar'}, baz);
Object.assign({}, baz, { foo: 'bar' });
Object.assign({}, { foo: 'bar', baz: 'foo' });
Object.assign({}, { ...baz });
Object.assign({});
Object.assign({ foo: bar });
Object.assign({ foo: 'bar' }, cats, dogs, trees, birds);
Object.assign({ foo: 'bar' }, Object.assign({ bar: 'foo' }, baz));
({foo: 'bar', ...Object.assign({ bar: 'foo' }, baz)});
Object.assign({ foo: 'bar' }, Object.assign({ bar: 'foo' }, Object.assign({}, { superNested: 'butwhy' })));
Object.assign({foo: 'bar', ...bar}, baz);
Object.assign({}, { foo, bar, baz });
Object.assign({}, { [bar]: 'foo' });
Object.assign({ ...bar }, { ...baz });
Object.assign({ ...bar }, {
    // this is a bar
    foo: 'bar',
    baz: "cats"
});
Object.assign({
    boo: "lol",
    // I'm a comment
    dog: "cat"
}, {
    // this is a bar
    foo: 'bar',
    baz: "cats"
});
const test1 = Object.assign({ ...bar }, {
    foo: 'bar', // inline comment
    baz: "cats"
});
const test2 = Object.assign({ ...bar }, {
    /**
     * foo
     */
    foo: 'bar',
    baz: "cats"
});
globalThis.Object.assign({}, foo);
