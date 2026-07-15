/* should not generate diagnostics */

declare const unknownValue: unknown;
declare const anyValue: any;
declare const poisoned: number | unknown;

interface Cycle extends Cycle {}
declare const cycle: Cycle;

async function consume(): Promise<void> {
	await unknownValue;
	await anyValue;
	await poisoned;
	await cycle;
}
