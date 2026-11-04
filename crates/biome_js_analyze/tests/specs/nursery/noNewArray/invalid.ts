/* should generate diagnostics */
const a1 = new Array<string>("x");
const a2 = new Array(1 as number);
const a3 = new Array("x" as string);
const a4 = new Array("x"!);
const a5 = new Array("x" satisfies string);
