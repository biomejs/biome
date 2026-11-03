/* should not generate diagnostics */

type StringReducer = (a: string, b: string) => string;

const strings = ["foo", "bar"] as const;
const largestString = ((strings as readonly string[]).reduce as any)(
    ((a, b) => (a > b ? a : b)) as StringReducer,
);
const smallestString = strings.reduceRight(
    (((a, b) => (a < b ? a : b)) satisfies StringReducer)!,
);
