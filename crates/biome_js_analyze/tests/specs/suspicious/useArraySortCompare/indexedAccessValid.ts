/* should not generate diagnostics */

declare const row: number[][][number];
row.sort((a, b) => a - b);
row.toSorted((a, b) => a - b);

declare const custom: [{ sort(): void }][number];
custom.sort();

declare const unknown: [unknown][number];
unknown.sort();
