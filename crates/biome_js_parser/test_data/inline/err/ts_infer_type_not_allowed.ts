type WithSelectors<S> = S extends { getState: () => infer T }
  ? { use: { [K in keyof infer /*error*/ T]: () => T[K] } }
  : never;
type TV1 = `${infer X}`;
type T61<T> = (infer A) extends infer B ? infer C : infer D;
type A = {a: infer T}
type A = () => infer T;
let s: (infer string)[] = symbol();
let s: unique (infer string);
let s: [number, ...infer string]
let s: [(infer string)?]
let s: (infer string)[a]
let s: a[(infer string)]
