/* should generate diagnostics */

const letters = ["A", "B", "C"] as const;
declare const letter: (typeof letters)[number];
String(letter);

declare const count: [1, 2, 3][number];
Number(count);

declare const flag: boolean[][number];
Boolean(flag);

const values = { A: 1, B: 2 } as const;
declare const key: keyof typeof values;
String(key);
