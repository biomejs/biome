/* should not generate diagnostics */

declare const promise: Promise<number>[][number];
await promise;
void promise;

declare const count: [1, 2, 3][number];
count;

declare const unknown: [unknown][number];
unknown;
