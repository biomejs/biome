/* should generate diagnostics */

type SyncMode = "sync";
type AsyncMode = "async";

export function schedule(mode: SyncMode, cb: () => Promise<void>): void;
export function schedule(mode: AsyncMode, cb: () => Promise<void>): Promise<void>;
export function schedule(mode: SyncMode | AsyncMode, cb: () => Promise<void>) {
	return cb();
}

schedule("async", async () => {
	await Promise.resolve();
});
