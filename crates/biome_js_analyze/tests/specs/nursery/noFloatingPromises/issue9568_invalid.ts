function bestEffort(cb: () => Promise<number>): Promise<number>;
function bestEffort(cb: () => number): number;
function bestEffort(cb: () => number | Promise<number>): Promise<number> | number {
	return cb() as Promise<number> | number;
}

async function asyncWork(): Promise<number> {
	return 42;
}

// Resolves to the first overload (async callback) -> returns a promise,
// so this floating call must still be flagged.
bestEffort(asyncWork);
