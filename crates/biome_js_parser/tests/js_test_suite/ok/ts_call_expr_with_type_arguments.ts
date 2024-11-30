function a<A, B, C>() {}
a<A, B, C>();
(() => { a }).a<A, B, C>()
(() => a)<A, B, C>();
type A<T> = T;
a<<T>(arg: T) => number, number, string>();
