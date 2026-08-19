/* should not generate diagnostics */

function run(cb: () => void): Promise<void>;
function run(cb: () => void, immediate: boolean): void;
function run(cb: () => void, immediate?: boolean): Promise<void> | void {
	return immediate ? cb() : Promise.resolve();
}

function task(): void {}

// Two arguments select the second overload (returns `void`, not a promise),
// so this must NOT be flagged.
run(task, true);
