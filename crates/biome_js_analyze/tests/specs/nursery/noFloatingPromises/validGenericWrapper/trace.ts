/* should not generate diagnostics */

export function trace<F extends (...args: any) => any>(fn: F): F {
	return fn;
}
