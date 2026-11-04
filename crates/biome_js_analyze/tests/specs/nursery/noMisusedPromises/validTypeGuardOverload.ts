/* should not generate diagnostics */

interface Refine {
	<T>(check: (value: unknown) => value is T): void;
	(check: (value: unknown) => unknown | Promise<unknown>): void;
}

declare const refine: Refine;

refine(async (value) => true);
