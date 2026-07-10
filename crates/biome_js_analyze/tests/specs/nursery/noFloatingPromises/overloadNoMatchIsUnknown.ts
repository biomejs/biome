/* should not generate diagnostics */

function onlyAsync(cb: () => Promise<void>): Promise<void>;
function onlyAsync(cb: () => Promise<void>, retries: number): Promise<void>;
function onlyAsync(cb: () => Promise<void>, retries?: number): Promise<void> {
	return cb();
}

function syncCb(): void {}

// Neither overload accepts a synchronous callback, so the call resolves to no
// signature. Its type is unknown rather than the first overload's
// `Promise<void>`, so it must NOT be flagged as a floating promise.
onlyAsync(syncCb);
