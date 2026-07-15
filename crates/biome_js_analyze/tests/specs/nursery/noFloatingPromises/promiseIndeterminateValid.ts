/* should not generate diagnostics */

declare const unknownValue: unknown;
declare const anyValue: any;
declare const unknownArray: Array<unknown>;
declare const poisoned: number | unknown;

interface Cycle extends Cycle {}
declare const cycle: Cycle;

unknownValue;
anyValue;
unknownArray;
poisoned;
cycle;
