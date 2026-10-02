/* should generate diagnostics */

type Row = number[][][number];
declare const row: Row;
row.sort();
row.toSorted();

declare const tuple: [number[]][number];
tuple.sort();
