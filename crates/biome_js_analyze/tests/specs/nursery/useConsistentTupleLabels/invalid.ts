/* should generate diagnostics */
type A1 = [a: string, number];
type A2 = [string, b: number];
type A3 = [a: string, b: number, c: boolean, d];
type A4 = [a: string, number, c: boolean];

// Each element without a label is reported
type B1 = [a: string, number, boolean];

// Optional elements
type C1 = [a?: string, number?];
type C2 = [string?, b: number];
type C3 = [a?: string, number];

// Rest elements
type D1 = [string, ...rest: number[]];
type D2 = [a: string, ...number[]];

type E1 = readonly [a: string, number];

// Nested tuple
type F1 = [[a: number, number]];
type F2 = [a: [x: number, number], b: string];

type G1 = [a: string, /* no label */ number];

function f(...args: [a: string, number]) {}
