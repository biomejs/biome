function run(cb: () => void): void;
function run(cb: () => void, immediate: boolean): Promise<void>;
function run(cb: () => void, immediate?: boolean): void | Promise<void> {
	return immediate ? Promise.resolve() : cb();
}

function task(): void {}

// Two arguments select the second overload, which returns `Promise<void>` (not
// the first, `void`-returning one). That promise floats here, so it must still
// be flagged.
run(task, true);
