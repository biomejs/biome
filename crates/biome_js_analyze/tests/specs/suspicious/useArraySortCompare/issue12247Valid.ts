/* should not generate diagnostics */
declare const tuple: [number, number];
tuple.sort();
tuple.toSorted();

const constArray = [3, 1, 2] as const;
constArray.toSorted();
