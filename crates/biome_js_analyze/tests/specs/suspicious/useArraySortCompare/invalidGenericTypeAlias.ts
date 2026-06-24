// `Id<number[]>` substitutes to `number[]`, so the compare-less sort is detectable.
type Id<T> = T;
declare const xs: Id<number[]>;
xs.sort();
