/* should not generate diagnostics */

interface SyncCb { (): void }
interface AsyncCb { (): Promise<void> }

function run(cb: AsyncCb): Promise<void>;
function run(cb: SyncCb): void;
function run(cb: SyncCb | AsyncCb): Promise<void> | void {
	const result = cb();
	return result instanceof Promise ? result : undefined;
}

const sync: SyncCb = () => {};

// SyncCb argument selects the second overload (returns `void`), must NOT be flagged.
run(sync);
