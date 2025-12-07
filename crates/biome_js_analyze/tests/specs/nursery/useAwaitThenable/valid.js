/* should not generate diagnostics */

await Promise.resolve('value');

const createValue = async () => 'value';
await createValue();
