/* should generate diagnostics */

const counts = [1, 2, 3] as const;
declare const count: (typeof counts)[number];
count + 1n;

declare const bigint: bigint[][number];
bigint + 1;
