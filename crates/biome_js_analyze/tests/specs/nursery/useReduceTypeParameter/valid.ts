/* should not generate diagnostics */

// no initial value
const a1: number[] = [1, 2, 3];
a1.reduce((a, b) => a + b);

// initial value without type assertion
const a2: number[] = [1, 2, 3];
a2.reduce((arr, num) => arr.concat(num), []);

// already using type parameter
const a3: number[] = [1, 2, 3];
a3.reduce<number[]>((arr, num) => arr.concat(num), []);

// satisfies instead of as (different semantics, not a type assertion)
const a4: number[] = [1, 2, 3];
a4.reduce((arr, num) => arr.concat(num), [] satisfies number[]);

// not reduce/reduceRight
const a5: number[] = [1, 2, 3];
a5.map((n) => n * 2);
