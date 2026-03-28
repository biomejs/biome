/* should not generate diagnostics */

export function bestEffort<T>(cb: () => Promise<T>): Promise<T | undefined>;
export function bestEffort<T>(cb: () => T): T | undefined;
export function bestEffort<T>(cb: (() => T) | (() => Promise<T>)) {
	return cb();
}

bestEffort(() => {
	console.log("Hello");
});

bestEffort(() => 1);
