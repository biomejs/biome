/* should not generate diagnostics */

function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
function bestEffort<T>(cb: () => T): T | undefined;
function bestEffort<T>(cb: (() => T) | (() => Promise<T>)): Promise<T | undefined> | T | undefined {
	return undefined;
}

function syncWork(): number {
	return 42;
}

// Generic overload exactly like the reported issue: selects the second
// overload (sync), so it must NOT be flagged.
bestEffort(syncWork);
