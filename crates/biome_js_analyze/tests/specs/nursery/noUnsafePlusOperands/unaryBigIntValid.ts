/* should not generate diagnostics */

declare const big: bigint;
const negated = -big + 1n;
const inverted = ~big + 1n;

declare const conditional: true extends true ? bigint : number;
const negatedConditional = -conditional + 1n;
