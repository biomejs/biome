// biome-ignore plugin/preferObjectSpreadSuppression: reason
Object.assign({ foo: 'bar'}, baz);

// biome-ignore-start plugin/preferObjectSpreadSuppression: reason
Object.assign({}, {foo: 'bar'});
// biome-ignore-end plugin/preferObjectSpreadSuppression: reason
