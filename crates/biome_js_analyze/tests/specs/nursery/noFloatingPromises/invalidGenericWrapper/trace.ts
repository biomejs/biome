/* should not generate diagnostics */

type Options<F extends (...args: any) => any> = {
	name?: string;
	skip?: (...args: Parameters<F>) => boolean;
};

export function trace<F extends (...args: any) => any>(
	fn: F,
	{
		name = fn.name,
		skip = () => false,
	}: Options<F> = {},
): F {
	return fn;
}
