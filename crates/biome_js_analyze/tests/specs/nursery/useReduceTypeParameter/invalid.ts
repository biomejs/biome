// basic reduce with as assertion
const arr1: number[] = [1, 2, 3];
arr1.reduce((sum, num) => sum.concat(num * 2), [] as number[]);

// object accumulator
const arr2: string[] = ['a', 'b'];
arr2.reduce((acc, name) => ({ ...acc, [name]: true }), {} as Record<string, boolean>);

// reduceRight
const arr3: number[] = [1, 2, 3];
arr3.reduceRight((sum, num) => sum.concat(num * 2), [] as number[]);

// tuple type
const arr4: string[] = ['x'];
arr4.reduce((acc, item) => [...acc, item], [] as [string, number][]);

// union type
const arr5: number[] = [1, 2];
arr5.reduce((acc, val) => acc.concat(val), [] as (string | number)[]);

// intersection type
const arr6: object[] = [];
arr6.reduce((acc, entry) => ({ ...acc, ...entry }), {} as Foo & Bar);

// angle bracket assertion (<Type>expr syntax)
const arr7: number[] = [1, 2, 3];
arr7.reduce((sum, num) => sum + num, <number>0);

// parenthesized as assertion
const arr8: number[] = [1, 2, 3];
arr8.reduce((sum, num) => sum + num, (0 as number));

// parenthesized angle bracket assertion
const arr9: number[] = [1, 2, 3];
arr9.reduce((sum, num) => sum + num, (<number>0));

// tuple receiver
const tuple1: [number, number, number] = [1, 2, 3];
tuple1.reduce((sum, n) => sum + n, 0 as number);

// existing type arguments with as assertion (false negative fix)
const arr10: number[] = [1, 2, 3];
arr10.reduce<number[]>((arr, num) => arr.concat(num), [] as number[]);

// existing type arguments with angle bracket assertion
const arr11: number[] = [1, 2, 3];
arr11.reduce<number>((sum, num) => sum + num, <number>0);

// reduceRight with existing type arguments and assertion
const arr12: number[] = [1, 2, 3];
arr12.reduceRight<number[]>((arr, num) => arr.concat(num), [] as number[]);

