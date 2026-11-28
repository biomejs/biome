// should not generate diagnostics
declare const thenable: PromiseLike<number>;

await thenable.then((value) => value);
void thenable.then((value) => value);
const chained = await thenable.then((value) => value);
chained;
[chained];
