Object.assign({}, foo);
Object.assign({}, {foo: 'bar'});
Object.assign({ foo: 'bar'}, baz);
Object.assign({}, baz, { foo: 'bar' });
Object.assign({}, { ...baz });
Object.assign({});
Object.assign({ foo: bar });
