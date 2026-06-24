// `MaybeAsync<string>` substitutes to `Promise<string> | undefined`, so the Promise must be handled.
type MaybeAsync<T> = Promise<T> | undefined;
declare function getWork(): MaybeAsync<string>;
async function main() {
	getWork();
}
