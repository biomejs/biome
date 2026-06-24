// `Identity<bigint>` substitutes to `bigint`, so adding it to a number mixes numeric types.
type Identity<T> = T;
declare const big: Identity<bigint>;
declare const num: number;
const mixed = big + num;
