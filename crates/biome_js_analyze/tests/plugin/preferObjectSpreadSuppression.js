// biome-ignore lint/plugin/preferObjectSpreadSuppression: reason
Object.assign({ foo: 'bar'}, baz);

// biome-ignore-start lint/plugin/preferObjectSpreadSuppression: reason
Object.assign({}, {foo: 'bar'});
// biome-ignore-end lint/plugin/preferObjectSpreadSuppression: reason

// if no name is specified, should suppress all plugins
// biome-ignore lint/plugin: reason
Object.assign({}, foo);

// only suppress specified plugin
// biome-ignore lint/plugin/anotherPlugin: reason
Object.assign({ foo: 'bar'}, baz);
