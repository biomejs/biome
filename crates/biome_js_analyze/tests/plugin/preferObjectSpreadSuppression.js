// biome-ignore plugin/preferObjectSpreadSuppression: reason
Object.assign({ foo: 'bar'}, baz);

// biome-ignore-start plugin/preferObjectSpreadSuppression: reason
Object.assign({}, {foo: 'bar'});
// biome-ignore-end plugin/preferObjectSpreadSuppression: reason

// if no name is specified, should suppress all plugins
// biome-ignore plugin: reason
Object.assign({}, foo);

// only suppress specified plugin
// biome-ignore plugin/anotherPlugin: reason
Object.assign({ foo: 'bar'}, baz);
