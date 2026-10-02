/* should generate diagnostics */

const letters = ["A", "B"] as const;
declare const letter: (typeof letters)[number];
await letter;

declare const count: number[][number];
await count;
