/* should not generate diagnostics */
const array1 = [1] as number[];
array1.push(2);

const array2 = <number[]>[1];
array2.push(2);

declare let foo1: number;
foo1 = 1;
