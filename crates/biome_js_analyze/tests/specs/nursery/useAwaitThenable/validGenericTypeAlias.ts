// should not generate diagnostics

// `Wrap<Promise<void>>` substitutes to `Promise<void>`, so awaiting it is valid.
type Wrap<T> = T;
async function run(task: Wrap<Promise<void>>) {
	await task;
}
