/* should not generate diagnostics */
const Object = {};
Object.assign({}, foo);
Object.assign({}, foo);
Object.assign({ foo: 'bar' });
Object.assign({ foo: 'bar' });
